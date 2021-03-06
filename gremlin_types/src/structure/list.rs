use crate::{
    error::{DecodeError, EncodeError},
    macros::{TryBorrowFrom, TryMutBorrowFrom},
    specs::CoreType,
};
use std::fmt::Display;

use crate::GremlinValue;

#[cfg(feature = "graph_binary")]
use crate::graph_binary::{Decode, Encode};

#[cfg(feature = "graph_son")]
use crate::error::GraphSonError;
#[cfg(feature = "graph_son")]
use crate::graphson::{validate_type, DecodeGraphSON, EncodeGraphSON};
#[cfg(feature = "graph_son")]
use serde_json::json;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Set<T>(Vec<T>);

impl<T> Set<T> {
    #[must_use]
    pub fn new(v: Vec<T>) -> Self {
        Set(v)
    }
    #[must_use]
    pub fn set(&self) -> &Vec<T> {
        &self.0
    }

    #[must_use]
    pub fn set_mut(&mut self) -> &mut Vec<T> {
        &mut self.0
    }
    #[must_use]
    pub fn inner(self) -> Vec<T> {
        self.0
    }
}

impl<T: Display> Display for Set<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for i in &self.0 {
            write!(f, "{i},")?;
        }
        write!(f, "]")
    }
}

#[cfg(feature = "graph_binary")]
impl<T: Encode> Encode for Set<T> {
    fn type_code() -> u8 {
        CoreType::Set.into()
    }

    fn partial_encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        self.0.partial_encode(writer)
    }
}

#[cfg(feature = "graph_binary")]
impl<T: Decode> Decode for Set<T> {
    fn expected_type_code() -> u8 {
        CoreType::Set.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        Ok(Set(Vec::<T>::partial_decode(reader)?))
    }
}

#[cfg(feature = "graph_son")]
impl<T: EncodeGraphSON> EncodeGraphSON for Set<T> {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
          "@type" : "g:Set",
          "@value" : self.0.iter().map(|t| t.encode_v3()).collect::<Vec<serde_json::Value>>()
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        json!(self
            .0
            .iter()
            .map(|t| t.encode_v2())
            .collect::<Vec<serde_json::Value>>())
    }

    fn encode_v1(&self) -> serde_json::Value {
        json!(self
            .0
            .iter()
            .map(|t| t.encode_v1())
            .collect::<Vec<serde_json::Value>>())
    }
}

#[cfg(feature = "graph_son")]
impl<T: DecodeGraphSON> DecodeGraphSON for Set<T> {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:Set")?;

        let result_vec = value_object
            .as_array()
            .ok_or_else(|| GraphSonError::WrongJsonType("array".to_string()))?
            .iter()
            .map(|v| T::decode_v3(v))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Set(result_vec))
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Ok(Set(Vec::<T>::decode_v2(j_val)?))
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}
#[cfg(feature = "graph_binary")]
impl<T: Encode> Encode for &[T] {
    fn type_code() -> u8 {
        CoreType::List.into()
    }

    fn partial_encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        let len = self.len() as i32;
        len.partial_encode(writer)?;

        for item in *self {
            item.encode(writer)?;
        }

        Ok(())
    }
}
#[cfg(feature = "graph_binary")]
impl<T: Encode> Encode for Vec<T> {
    fn type_code() -> u8 {
        CoreType::List.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        let len = self.len() as i32;
        len.partial_encode(writer)?;

        for item in self {
            item.encode(writer)?;
        }

        Ok(())
    }
}

impl<T: TryFrom<GremlinValue>> TryFrom<GremlinValue> for Vec<T> {
    type Error = DecodeError;

    fn try_from(value: GremlinValue) -> Result<Self, Self::Error> {
        match value {
            GremlinValue::List(list) => Ok(list
                .into_iter()
                .filter_map(|gb| gb.try_into().ok())
                .collect()),
            _ => Err(DecodeError::DecodeError("".to_string())),
        }
    }
}

impl TryBorrowFrom for Vec<GremlinValue> {
    fn try_borrow_from(graph_binary: &GremlinValue) -> Option<&Self> {
        match graph_binary {
            GremlinValue::List(list) => Some(list),
            _ => None,
        }
    }
}

impl TryMutBorrowFrom for Vec<GremlinValue> {
    fn try_mut_borrow_from(graph_binary: &mut GremlinValue) -> Option<&mut Self> {
        match graph_binary {
            GremlinValue::List(val) => Some(val),
            _ => None,
        }
    }
}

impl<T> From<Vec<T>> for GremlinValue
where
    T: Into<GremlinValue>,
{
    fn from(v: Vec<T>) -> Self {
        GremlinValue::List(v.into_iter().map(Into::into).collect())
    }
}

#[cfg(feature = "graph_binary")]
impl<T: Decode> Decode for Vec<T> {
    fn expected_type_code() -> u8 {
        CoreType::List.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let len = i32::partial_decode(reader)?;
        if len.is_negative() {
            return Err(DecodeError::DecodeError("vec len negativ".to_string()));
        }
        let mut list: Vec<T> = Vec::with_capacity(len as usize);
        for _ in 0..len {
            list.push(T::decode(reader)?);
        }
        Ok(list)
    }
}

#[cfg(feature = "graph_son")]
impl<T: EncodeGraphSON> EncodeGraphSON for Vec<T> {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
            "@type" : "g:List",
            "@value" : self.iter().map(|t| t.encode_v3()).collect::<Vec<serde_json::Value>>(),
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        json!(self
            .iter()
            .map(|t| t.encode_v2())
            .collect::<Vec<serde_json::Value>>())
    }

    fn encode_v1(&self) -> serde_json::Value {
        json!(self
            .iter()
            .map(|t| t.encode_v1())
            .collect::<Vec<serde_json::Value>>())
    }
}

#[cfg(feature = "graph_son")]
impl<T: DecodeGraphSON> DecodeGraphSON for Vec<T> {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:List")?;

        value_object
            .as_array()
            .ok_or_else(|| GraphSonError::WrongJsonType("array".to_string()))?
            .iter()
            .map(|v| T::decode_v3(v))
            .collect::<Result<Vec<_>, _>>()
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        j_val
            .as_array()
            .ok_or_else(|| GraphSonError::WrongJsonType("array".to_string()))?
            .iter()
            .map(|v| T::decode_v2(v))
            .collect::<Result<Vec<_>, _>>()
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        j_val
            .as_array()
            .ok_or_else(|| GraphSonError::WrongJsonType("array".to_string()))?
            .iter()
            .map(|v| T::decode_v1(v))
            .collect::<Result<Vec<_>, _>>()
    }
}

#[cfg(feature = "graph_son")]
impl<T: EncodeGraphSON> EncodeGraphSON for &[T] {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
            "@type" : "g:List",
            "@value" : self.iter().map(|t| t.encode_v3()).collect::<Vec<serde_json::Value>>(),
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        json!(self
            .iter()
            .map(|t| t.encode_v2())
            .collect::<Vec<serde_json::Value>>())
    }

    fn encode_v1(&self) -> serde_json::Value {
        json!(self
            .iter()
            .map(|t| t.encode_v1())
            .collect::<Vec<serde_json::Value>>())
    }
}

#[cfg(feature = "graph_son")]
impl<T: EncodeGraphSON, const N: usize> EncodeGraphSON for [T; N] {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
            "@type" : "g:List",
            "@value" : self.iter().map(|t| t.encode_v3()).collect::<Vec<serde_json::Value>>(),
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        json!(self
            .iter()
            .map(|t| t.encode_v2())
            .collect::<Vec<serde_json::Value>>())
    }

    fn encode_v1(&self) -> serde_json::Value {
        json!(self
            .iter()
            .map(|t| t.encode_v1())
            .collect::<Vec<serde_json::Value>>())
    }
}

#[test]
fn vec_decode_test() {
    let reader: Vec<u8> = vec![
        0x0, 0x0, 0x0, 0x04, 0x01, 0x0, 0x0, 0x0, 0x0, 0x04, 0x01, 0x0, 0x0, 0x0, 0x0, 0x04, 0x01,
        0x0, 0x0, 0x0, 0x0, 0x04, 0x01, 0x0, 0x0, 0x0, 0x0, 0x04,
    ];

    let s = Vec::partial_decode(&mut &reader[..]);

    assert!(s.is_ok());
    let s = s.unwrap();
    assert_eq!(4, s.len());
    for gb in s {
        assert_eq!(
            4,
            match gb {
                GremlinValue::Int(s) => s,
                _ => panic!(),
            }
        )
    }
}

#[test]
fn vec_decode_graphson_v3() {
    let str = r#"{
        "@type" : "g:List",
        "@value" : [ {
          "@type" : "g:Int32",
          "@value" : 1
        }, {
            "@type" : "g:Int32",
            "@value" : 2
          }]
      }"#;

    let s = serde_json::from_str(str).unwrap();
    let s: Vec<i32> = Vec::decode_v3(&s).unwrap();
    println!("{s:?}");

    assert_eq!(s, vec![1, 2]);
}

#[test]
fn vec_encode_graphson_v3() {
    let str = r#"{"@type":"g:List","@value":[{"@type":"g:Int32","@value":1},{"@type":"g:Int32","@value":2}]}"#;

    let v = vec![1, 2];
    let val = v.encode_v3();
    let val = serde_json::to_string(&val).unwrap();

    assert_eq!(str, val);
}

#[test]
fn empty_vec_decode_graphson_v3() {
    let str = r#"{
        "@type" : "g:List",
        "@value" : []
      }"#;

    let s = serde_json::from_str(str).unwrap();
    let s: Vec<i32> = Vec::decode_v3(&s).unwrap();

    assert_eq!(s, Vec::<i32>::new());
}

#[test]
fn vec_empty_encode_graphson_v3() {
    let str = r#"{"@type":"g:List","@value":[]}"#;

    let v: Vec<i32> = vec![];
    let val = v.encode_v3();
    let val = serde_json::to_string(&val).unwrap();

    assert_eq!(str, val);
}

#[test]
fn vec_decode_graphson_v3_error() {
    let str = r#"{
        "@type" : "g:List",
        "@value" : [ {
          "@type" : "g:Int32",
          "@value" : 1
        }, {
            "@type" : "g:Error",
            "@value" : 2
          }]
      }"#;

    let s = serde_json::from_str(str).unwrap();
    let s = Vec::<i32>::decode_v3(&s);
    assert!(s.is_err())
}

#[test]
fn vec_decode_graphson_v2() {
    let str = r#"[ {
          "@type" : "g:Int32",
          "@value" : 1
        }, {
            "@type" : "g:Int32",
            "@value" : 2
          }]"#;

    let s = serde_json::from_str(str).unwrap();
    let s: Vec<i32> = Vec::decode_v2(&s).unwrap();
    println!("{s:?}");

    assert_eq!(s, vec![1, 2]);
}

#[test]
fn vec_encode_graphson_v2() {
    let str = r#"[{"@type":"g:Int32","@value":1},{"@type":"g:Int32","@value":2}]"#;

    let v = vec![1, 2];
    let val = v.encode_v2();
    let val = serde_json::to_string(&val).unwrap();

    assert_eq!(str, val);
}

#[test]
fn empty_vec_decode_graphson_v2() {
    let str = r#"[]"#;

    let s = serde_json::from_str(str).unwrap();
    let s: Vec<i32> = Vec::decode_v2(&s).unwrap();

    assert_eq!(s, Vec::<i32>::new());
}

#[test]
fn empty_vec_encode_graphson_v2() {
    let str = r#"[]"#;

    let v: Vec<i32> = vec![];
    let val = v.encode_v2();
    let val = serde_json::to_string(&val).unwrap();

    assert_eq!(str, val);
}

#[test]
fn vec_decode_graphson_v1() {
    let str = r#"[1,2]"#;

    let s = serde_json::from_str(str).unwrap();
    let s: Vec<i32> = Vec::decode_v1(&s).unwrap();
    println!("{:?}", json!('a'));

    assert_eq!(s, vec![1, 2]);
}

#[test]
fn vec_encode_graphson_v1() {
    let str = r#"[1,2]"#;

    let v = vec![1, 2];
    let val = v.encode_v1();
    let val = serde_json::to_string(&val).unwrap();

    assert_eq!(str, val);
}

#[test]
fn empty_vec_decode_graphson_v1() {
    let str = r#"[]"#;

    let s = serde_json::from_str(str).unwrap();
    let s: Vec<i32> = Vec::decode_v2(&s).unwrap();

    assert_eq!(s, Vec::<i32>::new());
}

#[test]
fn empty_vec_encode_graphson_v1() {
    let str = r#"[]"#;

    let v: Vec<i32> = vec![];
    let val = v.encode_v2();
    let val = serde_json::to_string(&val).unwrap();

    assert_eq!(str, val);
}
