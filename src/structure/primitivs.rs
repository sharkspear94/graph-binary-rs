use crate::error::DecodeError;
use crate::specs::CoreType;
use crate::{error::EncodeError, graph_binary::Encode};
use serde::Deserialize;
use uuid::Uuid;

use crate::graph_binary::{build_fq_null_bytes, Decode, GraphBinary};

use std::io::Read;
use std::slice;

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

    fn get_partial_len(bytes: &[u8]) -> Result<usize, DecodeError> {
        let t: [u8; 4] = bytes[0..4].try_into()?;
        let mut len = i32::from_be_bytes(t);
        len += 4;
        Ok(len as usize)
    }
}

impl From<String> for GraphBinary {
    fn from(s: String) -> Self {
        GraphBinary::String(s)
    }
}

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

impl From<&str> for GraphBinary {
    fn from(s: &str) -> Self {
        GraphBinary::String(s.to_owned())
    }
}

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

    fn get_partial_len(bytes: &[u8]) -> Result<usize, DecodeError> {
        match bytes[0] {
            one if one < 0b1000_0000 => Ok(1),
            two if (0b1100_0000..0b1110_0000).contains(&two) => Ok(2),
            three if (0b1110_0000..0b1111_0000).contains(&three) => Ok(3),
            four if (0b1111_0000..0b1111_1000).contains(&four) => Ok(4),
            rest => Err(DecodeError::DecodeError(format!(
                "not a valid utf-8 first byte: value {:b}",
                rest
            ))),
        }
    }
}

#[test]
fn test3() {
    let reader = [0x80_u8, 0x0, 0xe2, 0x99, 0xa5];
    let c = char::decode(&mut &reader[..]).unwrap();

    assert_eq!('♥', c)
}

#[test]
fn test1() {
    let reader = [0x80_u8, 0x0, 65];
    let c = char::decode(&mut &reader[..]).unwrap();

    assert_eq!('A', c)
}

#[test]
fn test2() {
    let reader = [0x80_u8, 0x0, 0xc3, 0x9f];
    let c = char::decode(&mut &reader[..]).unwrap();

    assert_eq!('ß', c)
}

#[test]
fn test4() {
    let reader = [0x80_u8, 0x0, 0xf0, 0x9f, 0xa6, 0x80];
    let c = char::decode(&mut &reader[..]).unwrap();

    assert_eq!('🦀', c)
}

impl Encode for u8 {
    fn type_code() -> u8 {
        CoreType::Byte.into()
    }

    fn partial_encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

impl Decode for u8 {
    fn expected_type_code() -> u8 {
        CoreType::Byte.into()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<u8, DecodeError> {
        let mut int = [0_u8; 1];
        reader.read_exact(&mut int)?;

        Ok(u8::from_be_bytes(int))
    }

    fn get_partial_len(_bytes: &[u8]) -> Result<usize, DecodeError> {
        Ok(1)
    }
}

impl From<u8> for GraphBinary {
    fn from(v: u8) -> Self {
        GraphBinary::Byte(v)
    }
}

impl Encode for i16 {
    fn type_code() -> u8 {
        CoreType::Short.into()
    }

    fn partial_encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&self.to_be_bytes())?;

        Ok(())
    }
}

impl Decode for i16 {
    fn expected_type_code() -> u8 {
        CoreType::Short.into()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<i16, DecodeError> {
        let mut int = [0_u8; 2];
        reader.read_exact(&mut int)?;

        Ok(i16::from_be_bytes(int))
    }

    fn get_partial_len(_bytes: &[u8]) -> Result<usize, DecodeError> {
        Ok(2)
    }
}

impl From<i16> for GraphBinary {
    fn from(v: i16) -> Self {
        GraphBinary::Short(v)
    }
}

impl Encode for i32 {
    fn type_code() -> u8 {
        CoreType::Int32.into()
    }

    fn partial_encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

impl Decode for i32 {
    fn expected_type_code() -> u8 {
        CoreType::Int32.into()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<i32, DecodeError> {
        let mut int = [0_u8; 4];
        reader.read_exact(&mut int)?;

        Ok(i32::from_be_bytes(int))
    }

    fn get_partial_len(_bytes: &[u8]) -> Result<usize, DecodeError> {
        Ok(4)
    }
}

impl From<i32> for GraphBinary {
    fn from(v: i32) -> Self {
        GraphBinary::Int(v)
    }
}

impl Encode for i64 {
    fn type_code() -> u8 {
        CoreType::Long.into()
    }

    fn partial_encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

impl Decode for i64 {
    fn expected_type_code() -> u8 {
        CoreType::Long.into()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<i64, DecodeError> {
        let mut int = [0_u8; 8];
        reader.read_exact(&mut int)?;

        Ok(i64::from_be_bytes(int))
    }

    fn get_partial_len(_bytes: &[u8]) -> Result<usize, DecodeError> {
        Ok(8)
    }
}

impl From<i64> for GraphBinary {
    fn from(v: i64) -> Self {
        GraphBinary::Long(v)
    }
}

impl Encode for f32 {
    fn type_code() -> u8 {
        CoreType::Float.into()
    }

    fn partial_encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&self.to_be_bytes())?;

        Ok(())
    }
}

impl Decode for f32 {
    fn expected_type_code() -> u8 {
        CoreType::Float.into()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<f32, DecodeError> {
        let mut int = [0_u8; 4];
        reader.read_exact(&mut int)?;

        Ok(f32::from_be_bytes(int))
    }

    fn get_partial_len(_bytes: &[u8]) -> Result<usize, DecodeError> {
        Ok(4)
    }
}

impl From<f32> for GraphBinary {
    fn from(v: f32) -> Self {
        GraphBinary::Float(v)
    }
}

impl Encode for f64 {
    fn type_code() -> u8 {
        CoreType::Double.into()
    }

    fn partial_encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&self.to_be_bytes())?;

        Ok(())
    }
}

impl Decode for f64 {
    fn expected_type_code() -> u8 {
        CoreType::Double.into()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<f64, DecodeError> {
        let mut int = [0_u8; 8];
        reader.read_exact(&mut int)?;

        Ok(f64::from_be_bytes(int))
    }

    fn get_partial_len(_bytes: &[u8]) -> Result<usize, DecodeError> {
        Ok(8)
    }
}

impl From<f64> for GraphBinary {
    fn from(v: f64) -> Self {
        GraphBinary::Double(v)
    }
}

impl Encode for Uuid {
    fn type_code() -> u8 {
        CoreType::Uuid.into()
    }

    fn partial_encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(self.as_bytes())?;

        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(remote = "Uuid")]
pub struct UuidDef(#[serde(getter = "Uuid::bytes")] [u8; 16]);

impl From<UuidDef> for Uuid {
    fn from(def: UuidDef) -> Uuid {
        Uuid::from_bytes(def.0)
    }
}

impl<'de> Deserialize<'de> for UuidDef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_bytes(UuidDefVisitor)
    }
}

struct UuidDefVisitor;

impl<'de> serde::de::Visitor<'de> for UuidDefVisitor {
    type Value = UuidDef;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a struct UuidDef")
    }

    fn visit_bytes<E>(self, mut v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match UuidDef::decode(&mut v) {
            Ok(val) => Ok(val),
            Err(_) => Err(E::custom(concat!(stringify!($t), " Visitor Decode Error"))),
        }
    }
}

impl Decode for UuidDef {
    fn expected_type_code() -> u8 {
        CoreType::Uuid.into()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<UuidDef, DecodeError> {
        let mut buf = [0_u8; 16];
        reader.read_exact(&mut buf)?;

        Ok(UuidDef(buf))
    }

    fn get_partial_len(_bytes: &[u8]) -> Result<usize, DecodeError> {
        Ok(16)
    }
}

impl Decode for Uuid {
    fn expected_type_code() -> u8 {
        CoreType::Uuid.into()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<Uuid, DecodeError> {
        let mut buf = [0_u8; 16];
        reader.read_exact(&mut buf)?;

        Ok(Uuid::from_bytes(buf))
    }

    fn get_partial_len(_bytes: &[u8]) -> Result<usize, DecodeError> {
        Ok(16)
    }
}

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

    fn get_partial_len(_bytes: &[u8]) -> Result<usize, DecodeError> {
        Ok(16)
    }
}

impl From<Uuid> for GraphBinary {
    fn from(v: Uuid) -> Self {
        GraphBinary::Uuid(v)
    }
}

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

    fn get_partial_len(_bytes: &[u8]) -> Result<usize, DecodeError> {
        Ok(1)
    }
}

impl From<bool> for GraphBinary {
    fn from(v: bool) -> Self {
        GraphBinary::Boolean(v)
    }
}

impl<T: Encode> Encode for Option<T> {
    fn type_code() -> u8 {
        T::type_code()
    }

    fn partial_encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        match self {
            Some(i) => i.partial_encode(writer),
            None => build_fq_null_bytes(writer),
        }
    }

    fn encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        match self {
            Some(i) => i.encode(writer),
            None => build_fq_null_bytes(writer),
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

    fn get_partial_len(_bytes: &[u8]) -> Result<usize, DecodeError> {
        unimplemented!(
            "partial_count_bytes not supported for Option<T> use consumed_bytes or Graphbinray"
        )
    }
    fn get_len(bytes: &[u8]) -> Result<usize, DecodeError> {
        let value = bytes
            .get(1)
            .ok_or_else(|| DecodeError::DecodeError("".to_string()))?;
        match value {
            1 => Ok(2),
            0 => T::get_partial_len(&bytes[2..]),
            rest => Err(DecodeError::DecodeError(format!(
                "ValueFlag in Option<T> consumed bytes not a valid value found: {}",
                rest
            ))),
        }
    }
}

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

macro_rules! tuple_impls {
    ( $( $name:ident )+ ) => {
        impl<$($name: Encode),+> Encode for ($($name,)+)
        {
            fn type_code() -> u8 {
                CoreType::List.into()
            }

            fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
                let len = self.len() as i32;
                len.gb_bytes(writer)?;

                    item.fq_gb_bytes(writer)?;

                Ok(())
            }
        }
    };
}

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
fn test_string_consume() {
    assert_eq!(
        10,
        String::get_len(&[0x3, 0x0, 0x0, 0x0, 0x0, 0x4, 0x1, 0x03, 0x1, 0x4]).unwrap()
    )
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
