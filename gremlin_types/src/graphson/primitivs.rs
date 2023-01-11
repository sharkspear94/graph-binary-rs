use std::str::FromStr;

use serde_json::json;
use uuid::Uuid;

use crate::error::GraphSonError;

use super::{validate_type, DecodeGraphSON, EncodeGraphSON};

#[cfg(feature = "graph_son")]
impl EncodeGraphSON for String {
    fn encode_v3(&self) -> serde_json::Value {
        json!(self)
    }

    fn encode_v2(&self) -> serde_json::Value {
        self.encode_v3()
    }

    fn encode_v1(&self) -> serde_json::Value {
        self.encode_v3()
    }
}
#[cfg(feature = "graph_son")]
impl DecodeGraphSON for String {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        j_val
            .as_str()
            .ok_or_else(|| GraphSonError::WrongJsonType("str".to_string()))
            .map(ToString::to_string)
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }
}

#[cfg(feature = "graph_son")]
impl EncodeGraphSON for &str {
    fn encode_v3(&self) -> serde_json::Value {
        json!(self)
    }

    fn encode_v2(&self) -> serde_json::Value {
        self.encode_v3()
    }

    fn encode_v1(&self) -> serde_json::Value {
        self.encode_v3()
    }
}

#[cfg(feature = "graph_son")]
impl DecodeGraphSON for u8 {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "gx:Byte")?;
        value_object
            .as_u64()
            .ok_or_else(|| GraphSonError::WrongJsonType("u64".to_string()))
            .map(|val| val as u8)
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        j_val
            .as_u64()
            .ok_or_else(|| GraphSonError::WrongJsonType("u64".to_string()))
            .map(|val| val as u8)
    }
}

#[cfg(feature = "graph_son")]
impl DecodeGraphSON for i16 {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "gx:Short")?;
        value_object
            .as_i64()
            .ok_or_else(|| GraphSonError::WrongJsonType("i64".to_string()))
            .map(|val| val as i16)
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        j_val
            .as_i64()
            .ok_or_else(|| GraphSonError::WrongJsonType("i64".to_string()))
            .map(|val| val as i16)
    }
}

#[cfg(feature = "graph_son")]
impl DecodeGraphSON for i32 {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:Int32")?;
        value_object
            .as_i64()
            .ok_or_else(|| GraphSonError::WrongJsonType("i64".to_string()))
            .map(|val| val as i32)
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        j_val
            .as_i64()
            .ok_or_else(|| GraphSonError::WrongJsonType("i64".to_string()))
            .map(|t| t as i32)
    }
}

#[cfg(feature = "graph_son")]
impl DecodeGraphSON for i64 {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:Int64")?;
        value_object
            .as_i64()
            .ok_or_else(|| GraphSonError::WrongJsonType("i64".to_string()))
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        j_val
            .as_i64()
            .ok_or_else(|| GraphSonError::WrongJsonType("i64".to_string()))
    }
}

#[cfg(feature = "graph_son")]
impl DecodeGraphSON for f32 {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:Float")?;

        if let Some(res) = value_object.as_f64().map(|f| f as f32) {
            return Ok(res);
        }
        if let Some(res) = value_object.as_str().and_then(|s| match s {
            "NaN" => Some(f32::NAN),
            "Infinity" => Some(f32::INFINITY),
            "-Infinity" => Some(f32::NEG_INFINITY),
            _ => None,
        }) {
            Ok(res)
        } else {
            Err(GraphSonError::WrongJsonType("f64 or str".to_string()))
        }
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        j_val
            .as_f64()
            .ok_or_else(|| GraphSonError::WrongJsonType("f64".to_string()))
            .map(|t| t as f32)
    }
}

#[cfg(feature = "graph_son")]
impl DecodeGraphSON for f64 {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:Double")?;

        if let Some(res) = value_object.as_f64() {
            return Ok(res);
        }
        if let Some(res) = value_object.as_str().and_then(|s| match s {
            "NaN" => Some(f64::NAN),
            "Infinity" => Some(f64::INFINITY),
            "-Infinity" => Some(f64::NEG_INFINITY),
            _ => None,
        }) {
            Ok(res)
        } else {
            Err(GraphSonError::WrongJsonType("f64 or str".to_string()))
        }
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        j_val
            .as_f64()
            .ok_or_else(|| GraphSonError::WrongJsonType("f64 or str".to_string()))
    }
}

#[cfg(feature = "graph_son")]
impl EncodeGraphSON for Uuid {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
          "@type" : "g:UUID",
          "@value" : self.to_string()
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        self.encode_v3()
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

#[cfg(feature = "graph_son")]
impl DecodeGraphSON for Uuid {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:UUID")?;
        let s = value_object
            .as_str()
            .ok_or_else(|| GraphSonError::WrongJsonType("str".to_string()))?;
        Uuid::from_str(s).map_err(|err| GraphSonError::TryFrom(err.to_string()))
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let s = j_val
            .as_str()
            .ok_or_else(|| GraphSonError::WrongJsonType("str".to_string()))?;
        Uuid::from_str(s).map_err(|err| GraphSonError::TryFrom(err.to_string()))
    }
}

#[cfg(feature = "graph_son")]
impl EncodeGraphSON for bool {
    fn encode_v3(&self) -> serde_json::Value {
        json!(self)
    }

    fn encode_v2(&self) -> serde_json::Value {
        self.encode_v3()
    }

    fn encode_v1(&self) -> serde_json::Value {
        self.encode_v3()
    }
}

#[cfg(feature = "graph_son")]
impl DecodeGraphSON for bool {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        j_val
            .as_bool()
            .ok_or_else(|| GraphSonError::WrongJsonType("expected bool".to_string()))
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }
}

#[cfg(feature = "graph_son")]
impl<T: EncodeGraphSON> EncodeGraphSON for Option<T> {
    fn encode_v3(&self) -> serde_json::Value {
        match self {
            Some(val) => val.encode_v3(), // not sure if correct
            None => json!(null),
        }
    }

    fn encode_v2(&self) -> serde_json::Value {
        match self {
            Some(val) => val.encode_v2(),
            None => json!(null),
        }
    }

    fn encode_v1(&self) -> serde_json::Value {
        match self {
            Some(val) => val.encode_v1(),
            None => json!(null),
        }
    }
}

#[cfg(feature = "graph_son")]
impl<T: DecodeGraphSON> DecodeGraphSON for Option<T> {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        match j_val {
            serde_json::Value::Null => Ok(None),
            _ => Ok(Some(T::decode_v3(j_val)?)),
        }
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        match j_val {
            serde_json::Value::Null => Ok(None),
            _ => Ok(Some(T::decode_v2(j_val)?)),
        }
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        match j_val {
            serde_json::Value::Null => Ok(None),
            _ => Ok(Some(T::decode_v1(j_val)?)),
        }
    }
}

macro_rules! graphson_impl {
    ($(($t:ty,$type_sig:literal)),*$(,)?) => {
        $(
        #[cfg(feature = "graph_son")]
        impl EncodeGraphSON for $t {

            fn encode_v3(&self) -> serde_json::Value {
                json!({
                    "@type" : $type_sig,
                    "@value" : self
                })
            }

            fn encode_v2(&self) -> serde_json::Value {
                json!({
                    "@type" : $type_sig,
                    "@value" : self
                })
            }

            fn encode_v1(&self) -> serde_json::Value {
                json!(self)
            }
        }
    )*
}
}

graphson_impl!(
    (u8, "gx:Byte"),
    (i16, "gx:Int16"),
    (i32, "g:Int32"),
    (i64, "g:Int64"),
    (f32, "g:Float"),
    (f64, "g:Double"),
);

#[test]
fn int32_decode_v3() {
    let obj = r#"{"@type" : "g:Int32","@value" : 100}"#;
    let val = serde_json::from_str(obj).expect("a json value");
    assert_eq!(100, i32::decode_v3(&val).unwrap())
}

#[test]
fn f32_inf_decode_v3() {
    let f = r#"{
        "@type" : "g:Float",
        "@value" : "Infinity"
      }"#;
    let v = serde_json::from_str(f).unwrap();
    let a = f32::decode_v3(&v).unwrap();
    assert_eq!(a, f32::INFINITY)
}

#[test]
fn f64_neg_infinity_decode_v3() {
    let f = r#"{
        "@type" : "g:Double",
        "@value" : "-Infinity"
      }"#;
    let v = serde_json::from_str(f).unwrap();
    let a = f64::decode_v3(&v).unwrap();
    assert_eq!(a, f64::NEG_INFINITY)
}

#[test]
fn uuid_encode_v3() {
    let uuid = Uuid::from_str("41d2e28a-20a4-4ab0-b379-d810dede3786").unwrap();
    let v = uuid.encode_v3();
    let res = serde_json::to_string(&v).unwrap();

    let expected = r#"{"@type":"g:UUID","@value":"41d2e28a-20a4-4ab0-b379-d810dede3786"}"#;
    assert_eq!(res, expected)
}
