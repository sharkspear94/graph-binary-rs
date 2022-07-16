use super::validate_type_entry;
use crate::error::DecodeError;
use crate::graph_binary::{encode_null_object, Decode};
use crate::graphson::{DecodeGraphSON, EncodeGraphSON};
use crate::macros::{TryBorrowFrom, TryMutBorrowFrom};
use crate::specs::CoreType;
use crate::{conversion, GremlinValue};
use crate::{error::EncodeError, graph_binary::Encode};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use std::io::Read;
use std::slice;
use std::str::FromStr;

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
        let mut buf = [0_u8; 4];
        reader.read_exact(&mut buf)?;
        let len = i32::from_be_bytes(buf);

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
#[cfg(any(feature = "graph_son_v3", feature = "graph_son_v2"))]
impl EncodeGraphSON for String {
    fn encode_v3(&self) -> serde_json::Value {
        json!(self)
    }

    #[cfg(feature = "graph_son_v2")]
    fn encode_v2(&self) -> serde_json::Value {
        self.encode_v3()
    }

    fn encode_v1(&self) -> serde_json::Value {
        self.encode_v3()
    }
}
#[cfg(any(feature = "graph_son_v3", feature = "graph_son_v2"))]
impl DecodeGraphSON for String {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        j_val
            .as_str()
            .ok_or_else(|| DecodeError::DecodeError("json error in String".to_string()))
            .map(ToString::to_string)
    }
    #[cfg(feature = "graph_son_v2")]
    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, DecodeError>
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
#[cfg(any(feature = "graph_son_v3", feature = "graph_son_v2"))]
impl EncodeGraphSON for &str {
    fn encode_v3(&self) -> serde_json::Value {
        json!(self)
    }
    #[cfg(feature = "graph_son_v2")]
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

#[cfg(all(feature = "graph_binary", feature = "extended"))]
impl Encode for char {
    fn type_code() -> u8 {
        CoreType::Char.into()
    }

    fn partial_encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        let mut buf = [0; 4];
        let slice = self.encode_utf8(&mut buf);
        writer.write_all(slice.as_bytes())?;
        Ok(())
    }
}

#[cfg(all(feature = "graph_binary", feature = "extended"))]
impl Decode for char {
    fn expected_type_code() -> u8 {
        CoreType::Char.into()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let mut first_byte = [0_u8; 1];
        reader.read_exact(&mut first_byte)?;

        match first_byte[0] {
            one if one < 0b1000_0000 => Ok(char::from(one)),
            two if (0b1100_0000..0b1110_0000).contains(&two) => {
                let mut second_byte = [0_u8; 1];
                reader.read_exact(&mut second_byte)?;
                std::str::from_utf8(&[first_byte[0], second_byte[0]])?
                    .chars()
                    .next()
                    .ok_or_else(|| {
                        DecodeError::DecodeError("error converting u32 to char".to_string())
                    })
            }
            three if (0b1110_0000..0b1111_0000).contains(&three) => {
                let mut rest = [0_u8; 2];
                reader.read_exact(&mut rest)?;
                std::str::from_utf8(&[first_byte[0], rest[0], rest[1]])?
                    .chars()
                    .next()
                    .ok_or_else(|| {
                        DecodeError::DecodeError("error converting u32 to char".to_string())
                    })
            }
            four if (0b1111_0000..0b1111_1000).contains(&four) => {
                let mut rest = [0_u8; 3];
                reader.read_exact(&mut rest)?;
                std::str::from_utf8(&[first_byte[0], rest[0], rest[1], rest[2]])?
                    .chars()
                    .next()
                    .ok_or_else(|| {
                        DecodeError::DecodeError("error converting u32 to char".to_string())
                    })
            }
            rest => Err(DecodeError::DecodeError(format!(
                "not a valid utf-8 first byte: value {:b}",
                rest
            ))),
        }
    }
}
#[cfg(feature = "extended")]
impl DecodeGraphSON for char {
    #[cfg(any(feature = "graph_son_v3", feature = "graph_son_v2"))]
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        j_val
            .as_object()
            .filter(|map| validate_type_entry(*map, "gx:Char"))
            .and_then(|map| map.get("@value"))
            .and_then(|value| value.as_str())
            .and_then(|s| s.chars().next()) //FIXME more than 1 char is not evaluated
            .ok_or_else(|| DecodeError::DecodeError("json error in char".to_string()))
    }

    #[cfg(feature = "graph_son_v2")]
    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        j_val
            .as_str()
            .and_then(|s| s.chars().next())
            .ok_or_else(|| DecodeError::DecodeError("json error in char".to_string()))
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

#[cfg(any(feature = "graph_son_v3", feature = "graph_son_v2"))]
impl DecodeGraphSON for u8 {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        j_val
            .as_object()
            .filter(|map| validate_type_entry(*map, "gx:Byte"))
            .and_then(|map| map.get("@value"))
            .and_then(|value| value.as_u64())
            .ok_or_else(|| DecodeError::DecodeError("json error in Byte".to_string()))
            .map(|val| val as u8)
    }

    #[cfg(feature = "graph_son_v2")]
    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        j_val
            .as_u64()
            .ok_or_else(|| DecodeError::DecodeError("json error in Byte".to_string()))
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

#[cfg(any(feature = "graph_son_v3", feature = "graph_son_v2"))]
impl DecodeGraphSON for i16 {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        j_val
            .as_object()
            .filter(|map| validate_type_entry(*map, "gx:Short"))
            .and_then(|map| map.get("@value"))
            .and_then(|value| value.as_i64())
            .ok_or_else(|| DecodeError::DecodeError("json error in i16".to_string()))
            .map(|val| val as i16)
    }

    #[cfg(feature = "graph_son_v2")]
    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        j_val
            .as_i64()
            .ok_or_else(|| DecodeError::DecodeError("json error in i16".to_string()))
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
        let mut int = [0_u8; 4];
        reader.read_exact(&mut int)?;

        Ok(i32::from_be_bytes(int))
    }
}

#[cfg(any(feature = "graph_son_v3", feature = "graph_son_v2"))]
impl DecodeGraphSON for i32 {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        j_val
            .as_object()
            .filter(|map| validate_type_entry(*map, "g:Int32"))
            .and_then(|map| map.get("@value"))
            .and_then(|value| value.as_i64())
            .ok_or_else(|| DecodeError::DecodeError("json error i32 v3 in error".to_string()))
            .map(|t| t as i32)
    }
    #[cfg(feature = "graph_son_v2")]
    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        j_val
            .as_i64()
            .ok_or_else(|| DecodeError::DecodeError("json error i32 v1 in error".to_string()))
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

#[cfg(any(feature = "graph_son_v3", feature = "graph_son_v2"))]
impl DecodeGraphSON for i64 {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        j_val
            .as_object()
            .filter(|map| validate_type_entry(*map, "g:Int64"))
            .and_then(|map| map.get("@value"))
            .and_then(|value| value.as_i64())
            .ok_or_else(|| DecodeError::DecodeError("json error in i64".to_string()))
    }

    #[cfg(feature = "graph_son_v2")]
    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        j_val
            .as_i64()
            .ok_or_else(|| DecodeError::DecodeError("json error in i64 v1".to_string()))
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

#[cfg(any(feature = "graph_son_v3", feature = "graph_son_v2"))]
impl DecodeGraphSON for f32 {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let val = j_val
            .as_object()
            .filter(|map| validate_type_entry(*map, "g:Float"))
            .and_then(|map| map.get("@value"))
            .ok_or_else(|| {
                DecodeError::DecodeError("type identifier for f32 v3 failed".to_string())
            })?;

        if let Some(res) = val.as_f64().map(|f| f as f32) {
            return Ok(res);
        }
        if let Some(res) = val.as_str().and_then(|s| match s {
            "NaN" => Some(f32::NAN),
            "Infinity" => Some(f32::INFINITY),
            "-Infinity" => Some(f32::NEG_INFINITY),
            _ => None,
        }) {
            Ok(res)
        } else {
            Err(DecodeError::DecodeError(
                "json error f32 v3 in error".to_string(),
            ))
        }
    }

    #[cfg(feature = "graph_son_v2")]
    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        j_val
            .as_f64()
            .ok_or_else(|| DecodeError::DecodeError("json error f32 v1 in error".to_string()))
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

#[cfg(any(feature = "graph_son_v3", feature = "graph_son_v2"))]
impl DecodeGraphSON for f64 {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let val = j_val
            .as_object()
            .filter(|map| validate_type_entry(*map, "g:Double"))
            .and_then(|map| map.get("@value"))
            .ok_or_else(|| {
                DecodeError::DecodeError("type identifier for f64 v3 failed".to_string())
            })?;

        if let Some(res) = val.as_f64() {
            return Ok(res);
        }
        if let Some(res) = val.as_str().and_then(|s| match s {
            "NaN" => Some(f64::NAN),
            "Infinity" => Some(f64::INFINITY),
            "-Infinity" => Some(f64::NEG_INFINITY),
            _ => None,
        }) {
            Ok(res)
        } else {
            Err(DecodeError::DecodeError(
                "json error f64 v3 in error".to_string(),
            ))
        }
    }

    #[cfg(feature = "graph_son_v2")]
    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        j_val
            .as_f64()
            .ok_or_else(|| DecodeError::DecodeError("json error f64 v1 in error".to_string()))
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

#[cfg(any(feature = "graph_son_v3", feature = "graph_son_v2"))]
impl DecodeGraphSON for Uuid {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        j_val
            .as_object()
            .filter(|map| validate_type_entry(*map, "g:UUID"))
            .and_then(|map| map.get("@value"))
            .and_then(|value| value.as_str())
            .and_then(|s| Uuid::from_str(s).ok())
            .ok_or_else(|| DecodeError::DecodeError("json error Uuid v3 in error".to_string()))
    }

    #[cfg(feature = "graph_son_v2")]
    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        j_val
            .as_str()
            .and_then(|s| Uuid::from_str(s).ok())
            .ok_or_else(|| DecodeError::DecodeError("json error Uuid ".to_string()))
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

#[cfg(any(feature = "graph_son_v3", feature = "graph_son_v2"))]
impl EncodeGraphSON for bool {
    fn encode_v3(&self) -> serde_json::Value {
        json!(self)
    }

    #[cfg(feature = "graph_son_v2")]
    fn encode_v2(&self) -> serde_json::Value {
        self.encode_v3()
    }

    fn encode_v1(&self) -> serde_json::Value {
        self.encode_v3()
    }
}

#[cfg(any(feature = "graph_son_v3", feature = "graph_son_v2"))]
impl DecodeGraphSON for bool {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        j_val
            .as_bool()
            .ok_or_else(|| DecodeError::DecodeError("json error bool".to_string()))
    }

    #[cfg(feature = "graph_son_v2")]
    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, DecodeError>
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

#[cfg(any(feature = "graph_son_v3", feature = "graph_son_v2"))]
impl<T: EncodeGraphSON> EncodeGraphSON for Option<T> {
    #[cfg(feature = "graph_son_v3")]
    fn encode_v3(&self) -> serde_json::Value {
        match self {
            Some(val) => val.encode_v3(), // not sure if correct
            None => json!(null),
        }
    }

    #[cfg(feature = "graph_son_v2")]
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

#[cfg(any(feature = "graph_son_v3", feature = "graph_son_v2"))]
impl<T: DecodeGraphSON> DecodeGraphSON for Option<T> {
    #[cfg(feature = "graph_son_v3")]
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        match j_val {
            serde_json::Value::Null => Ok(None),
            _ => Ok(Some(T::decode_v3(j_val)?)),
        }
    }

    #[cfg(feature = "graph_son_v2")]
    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        match j_val {
            serde_json::Value::Null => Ok(None),
            _ => Ok(Some(T::decode_v2(j_val)?)),
        }
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, DecodeError>
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
        #[cfg(any(feature = "graph_son_v3", feature = "graph_son_v2"))]
        impl EncodeGraphSON for $t {
            #[cfg(feature = "graph_son_v3")]
            fn encode_v3(&self) -> serde_json::Value {
                json!({
                    "@type" : $type_sig,
                    "@value" : self
                })
            }
            #[cfg(feature = "graph_son_v2")]
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
    (char, "gx:Char"),
    (u8, "gx:Byte"),
    (i16, "gx:Int16"),
    (i32, "g:Int32"),
    (i64, "g:Int64"),
    (f32, "g:Float"),
    (f64, "g:Double"),
    (Uuid, "g:UUID"),
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
#[cfg(feature = "extended")]
conversion!(char, Char);

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
