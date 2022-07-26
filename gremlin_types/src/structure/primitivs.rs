use crate::macros::{TryBorrowFrom, TryMutBorrowFrom};
use crate::specs::CoreType;
use crate::{conversion, GremlinValue};
use uuid::Uuid;

use std::io::Read;
use std::slice;
use std::str::FromStr;

#[cfg(any(feature = "graph_son", feature = "graph_binary"))]
use crate::error::DecodeError;
#[cfg(feature = "graph_son")]
use crate::error::GraphSonError;
#[cfg(feature = "graph_binary")]
use crate::graph_binary::{encode_null_object, Decode, Encode};

#[cfg(feature = "graph_binary")]
use crate::error::EncodeError;
#[cfg(feature = "graph_son")]
use crate::graphson::{validate_type, DecodeGraphSON, EncodeGraphSON};
#[cfg(feature = "graph_son")]
use serde_json::json;
#[cfg(feature = "graph_binary")]
impl Encode for String {
    fn type_code() -> u8 {
        CoreType::String.into()
    }

    fn partial_encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        let len = self.len() as i32;
        writer.write_all(&len.to_be_bytes())?;
        writer.write_all(self.as_bytes())?;
        Ok(())
    }
}

#[cfg(feature = "graph_binary")]
impl Decode for String {
    fn expected_type_code() -> u8 {
        CoreType::String.into()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<String, DecodeError> {
        let len = i32::partial_decode(reader)?;

        if len < 0 {
            return Err(DecodeError::DecodeError("size negativ".to_string()));
        }
        let s = String::from_utf8(
            reader
                .bytes()
                .take(len as usize)
                .filter_map(Result::ok)
                .collect(),
        )?;

        match s.len() {
            l if l != len as usize => Err(DecodeError::DecodeError(format!(
                "String {} len not expected lenth of {}",
                s, len
            ))),
            _ => Ok(s),
        }
    }
}
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

impl TryBorrowFrom for str {
    fn try_borrow_from(graph_binary: &GremlinValue) -> Option<&Self> {
        match graph_binary {
            GremlinValue::String(s) => Some(s),
            _ => None,
        }
    }
}

impl TryMutBorrowFrom for str {
    fn try_mut_borrow_from(graph_binary: &mut GremlinValue) -> Option<&mut Self> {
        match graph_binary {
            GremlinValue::String(s) => Some(s),
            _ => None,
        }
    }
}

#[cfg(feature = "graph_binary")]
impl Encode for &str {
    fn type_code() -> u8 {
        CoreType::String.into()
    }

    fn partial_encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        let len = self.len() as i32;
        writer.write_all(&len.to_be_bytes())?;
        writer.write_all(self.as_bytes())?;
        Ok(())
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

impl From<&str> for GremlinValue {
    fn from(s: &str) -> Self {
        GremlinValue::String(s.to_owned())
    }
}

#[cfg(feature = "graph_binary")]
impl Encode for u8 {
    fn type_code() -> u8 {
        CoreType::Byte.into()
    }

    fn partial_encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

#[cfg(feature = "graph_binary")]
impl Decode for u8 {
    fn expected_type_code() -> u8 {
        CoreType::Byte.into()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<u8, DecodeError> {
        let mut int = [0_u8; 1];
        reader.read_exact(&mut int)?;

        Ok(u8::from_be_bytes(int))
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

#[cfg(feature = "graph_binary")]
impl Encode for i16 {
    fn type_code() -> u8 {
        CoreType::Short.into()
    }

    fn partial_encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&self.to_be_bytes())?;

        Ok(())
    }
}

#[cfg(feature = "graph_binary")]
impl Decode for i16 {
    fn expected_type_code() -> u8 {
        CoreType::Short.into()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<i16, DecodeError> {
        let mut int = [0_u8; 2];
        reader.read_exact(&mut int)?;

        Ok(i16::from_be_bytes(int))
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

#[cfg(feature = "graph_binary")]
impl Encode for i32 {
    fn type_code() -> u8 {
        CoreType::Int32.into()
    }

    fn partial_encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

#[cfg(feature = "graph_binary")]
impl Decode for i32 {
    fn expected_type_code() -> u8 {
        CoreType::Int32.into()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<i32, DecodeError> {
        let mut buf = [0_u8; 4];
        reader.read_exact(&mut buf)?;

        Ok(i32::from_be_bytes(buf))
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
#[cfg(feature = "graph_binary")]
impl Encode for i64 {
    fn type_code() -> u8 {
        CoreType::Long.into()
    }

    fn partial_encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}
#[cfg(feature = "graph_binary")]
impl Decode for i64 {
    fn expected_type_code() -> u8 {
        CoreType::Long.into()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<i64, DecodeError> {
        let mut int = [0_u8; 8];
        reader.read_exact(&mut int)?;

        Ok(i64::from_be_bytes(int))
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

#[cfg(feature = "graph_binary")]
impl Encode for f32 {
    fn type_code() -> u8 {
        CoreType::Float.into()
    }

    fn partial_encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&self.to_be_bytes())?;

        Ok(())
    }
}

#[cfg(feature = "graph_binary")]
impl Decode for f32 {
    fn expected_type_code() -> u8 {
        CoreType::Float.into()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<f32, DecodeError> {
        let mut int = [0_u8; 4];
        reader.read_exact(&mut int)?;

        Ok(f32::from_be_bytes(int))
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

#[cfg(feature = "graph_binary")]
impl Encode for f64 {
    fn type_code() -> u8 {
        CoreType::Double.into()
    }

    fn partial_encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&self.to_be_bytes())?;

        Ok(())
    }
}

#[cfg(feature = "graph_binary")]
impl Decode for f64 {
    fn expected_type_code() -> u8 {
        CoreType::Double.into()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<f64, DecodeError> {
        let mut int = [0_u8; 8];
        reader.read_exact(&mut int)?;

        Ok(f64::from_be_bytes(int))
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

#[cfg(feature = "graph_binary")]
impl Encode for Uuid {
    fn type_code() -> u8 {
        CoreType::Uuid.into()
    }

    fn partial_encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(self.as_bytes())?;

        Ok(())
    }
}

#[cfg(feature = "graph_binary")]
impl Decode for Uuid {
    fn expected_type_code() -> u8 {
        CoreType::Uuid.into()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<Uuid, DecodeError> {
        let mut buf = [0_u8; 16];
        reader.read_exact(&mut buf)?;

        Ok(Uuid::from_bytes(buf))
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

#[cfg(feature = "graph_binary")]
impl Decode for u128 {
    fn expected_type_code() -> u8 {
        CoreType::Uuid.into()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<u128, DecodeError> {
        let mut buf = [0_u8; 16];
        reader.read_exact(&mut buf)?;
        let val = u128::from_be_bytes(buf);
        Ok(val)
    }
}

#[cfg(feature = "graph_binary")]
impl Encode for bool {
    fn type_code() -> u8 {
        CoreType::Boolean.into()
    }

    fn partial_encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        let b = match self {
            true => 0x00_u8,
            false => 0x01_u8,
        };

        writer.write_all(slice::from_ref(&b))?;

        Ok(())
    }
}

#[cfg(feature = "graph_binary")]
impl Decode for bool {
    fn expected_type_code() -> u8 {
        CoreType::Boolean.into()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<bool, DecodeError> {
        let mut buf = [0_u8; 1];
        reader.read_exact(&mut buf)?;

        match buf[0] {
            0x00 => Ok(true),
            0x01 => Ok(false),
            _ => Err(DecodeError::DecodeError(String::from("bool"))),
        }
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
            .ok_or_else(|| GraphSonError::WrongJsonType("bool".to_string()))
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

#[cfg(feature = "graph_binary")]
impl<T: Encode> Encode for Option<T> {
    fn type_code() -> u8 {
        T::type_code()
    }

    fn partial_encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        match self {
            Some(i) => i.partial_encode(writer),
            None => encode_null_object(writer),
        }
    }

    fn encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        match self {
            Some(i) => i.encode(writer),
            None => encode_null_object(writer),
        }
    }

    fn write_partial_nullable_bytes<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), EncodeError> {
        match self {
            Some(val) => {
                writer.write_all(&[0])?;
                val.partial_encode(writer)
            }
            None => Ok(writer.write_all(&[1])?),
        }
    }
}

#[cfg(feature = "graph_binary")]
impl<T: Decode> Decode for Option<T> {
    fn expected_type_code() -> u8 {
        T::expected_type_code()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        Ok(Some(T::partial_decode(reader)?))
    }

    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let mut buf = [255_u8; 2];
        reader.read_exact(&mut buf)?;
        let type_code = Self::expected_type_code();
        match (buf[0], buf[1]) {
            (code, 0) if code == type_code => Self::partial_decode(reader),
            (0xFE, 1) => Ok(None),
            (code, 1) if code == type_code => Ok(None),
            (t, value_byte @ 2..=255) => Err(DecodeError::DecodeError(format!(
                "Type Code and Value Byte does not hold valid value flag, found: TypeCode[{:#X}] expected TypeCode[{:#X}] and ValueFlag[{:#X}]",
                t,
                Self::expected_type_code(),
                value_byte
            ))),
            (t, flag) => Err(DecodeError::DecodeError(format!(
                "Type Code Error, expected type {:#X} or 0xfe, found {:#X} and value_flag {:#X}",
                Self::expected_type_code(),
                t,
                flag
            ))),
        }
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

conversion!(String, String);
conversion!(u8, Byte);
conversion!(i16, Short);
conversion!(i32, Int);
conversion!(i64, Long);
conversion!(f32, Float);
conversion!(f64, Double);
conversion!(bool, Boolean);
conversion!(Uuid, Uuid);

#[test]
fn encode_string_test() {
    let s = String::from("test");

    let mut buf: Vec<u8> = vec![];
    s.encode(&mut buf).unwrap();
    assert_eq!(
        &[0x03, 0x00, 0x00, 0x00, 0x00, 0x04, 0x74, 0x65, 0x73, 0x74][..],
        &buf
    );
}

#[test]
fn encode_empty_string_test() {
    let s = String::new();

    let mut buf: Vec<u8> = vec![];
    s.encode(&mut buf).unwrap();

    assert_eq!(&[0x03, 0x00, 0x00, 0x00, 0x00, 0x00][..], &buf);
}

#[test]
fn decode_fq_empty_string_test() {
    let buf: Vec<u8> = vec![0x03, 0x00, 0x00, 0x00, 0x00, 0x00];

    let s = String::decode(&mut &buf[..]);

    assert_eq!(String::new(), s.unwrap());
}

#[test]
fn decode_string_test() {
    let reader: Vec<u8> = vec![0x0, 0x0, 0x0, 0x04, b'h', b'o', b's', b't'];

    let s = String::partial_decode(&mut &reader[..]);

    assert!(s.is_ok());

    assert_eq!("host", s.unwrap().as_str())
}

#[test]
fn test_string_utf8() {
    let reader = vec![0x3_u8, 0x0, 0x0, 0x0, 0x0, 0x4, 240, 159, 146, 150];

    let s = String::decode(&mut &*reader).unwrap();
    assert_eq!("💖", s);
}

#[test]
fn decode_string_fail_test() {
    let reader: Vec<u8> = vec![0x0, 0x0, 0x0, 0x04, b'h', b'o', b's'];

    let s = String::partial_decode(&mut &reader[..]);

    assert!(s.is_err());
}

#[test]
fn encode_int32_test() {
    let mut buf: Vec<u8> = vec![];
    i32::MAX.encode(&mut buf).unwrap();

    assert_eq!(&[0x01, 0x00, 0x7F, 0xFF, 0xFF, 0xFF][..], buf);

    buf.clear();
    i32::MIN.encode(&mut buf).unwrap();

    assert_eq!(&[0x01, 0x00, 0x80, 0x00, 0x00, 0x00][..], buf);
}

#[test]
fn encode_uuid_test() {
    let mut buf: Vec<u8> = vec![];
    let uuid = uuid::Uuid::from_bytes([
        0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee,
        0xff,
    ]);

    uuid.encode(&mut buf).unwrap();

    assert_eq!(
        [
            0x0c, 0x00, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb,
            0xcc, 0xdd, 0xee, 0xff
        ][..],
        buf
    )
}

#[test]
fn option_decode_test() {
    let reader: Vec<u8> = vec![0x03, 0x0, 0x0, 0x0, 0x0, 0x04, b'h', b'o', b's', b't'];

    let option: Option<String> = Option::decode(&mut &reader[..]).unwrap();

    assert_eq!(option.unwrap(), String::from("host"))
}

#[test]
fn option_none_decode_test() {
    let reader: Vec<u8> = vec![0x03, 0x1];

    let option: Option<String> = Option::decode(&mut &reader[..]).unwrap();

    assert!(option.is_none());

    let reader: Vec<u8> = vec![0xfe, 0x1];

    let option: Option<String> = Option::decode(&mut &reader[..]).unwrap();

    assert!(option.is_none())
}

#[test]
fn option_should_fail_decode_test() {
    let reader: Vec<u8> = vec![0x04, 0x1];

    let option: Result<Option<String>, _> = Option::decode(&mut &reader[..]);

    assert!(option.is_err());

    let reader: Vec<u8> = vec![0xfe, 0x0];

    let option: Result<Option<String>, _> = Option::decode(&mut &reader[..]);

    assert!(option.is_err());

    let reader: Vec<u8> = vec![0xfe, 0x2];

    let option: Result<Option<String>, _> = Option::decode(&mut &reader[..]);

    assert!(option.is_err())
}

#[cfg(feature = "extended")]
#[test]
fn char_decode_utf8() {
    let reader = [0x80_u8, 0x0, 0xe2, 0x99, 0xa5];
    let c = char::decode(&mut &reader[..]).unwrap();

    assert_eq!('♥', c)
}

#[cfg(feature = "extended")]
#[test]
fn char_decode() {
    let reader = [0x80_u8, 0x0, 65];
    let c = char::decode(&mut &reader[..]).unwrap();

    assert_eq!('A', c)
}

#[cfg(feature = "extended")]
#[test]
fn test2() {
    let reader = [0x80_u8, 0x0, 0xc3, 0x9f];
    let c = char::decode(&mut &reader[..]).unwrap();

    assert_eq!('ß', c)
}

#[cfg(feature = "extended")]
#[test]
fn test4() {
    let reader = [0x80_u8, 0x0, 0xf0, 0x9f, 0xa6, 0x80];
    let c = char::decode(&mut &reader[..]).unwrap();

    assert_eq!('🦀', c)
}

#[test]
fn test12() {
    let obj = r#"{"@type" : "g:Int32","@value" : 100}"#;
    let val = serde_json::from_str(obj).expect("a json value");
    assert_eq!(100, i32::decode_v3(&val).unwrap())
}

#[test]
fn f32_inf() {
    let f = r#"{
        "@type" : "g:Float",
        "@value" : "Infinity"
      }"#;
    let v = serde_json::from_str(f).unwrap();
    let a = f32::decode_v3(&v).unwrap();
    assert_eq!(a, f32::INFINITY)
}

#[test]
fn f64_neg_infinity() {
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
