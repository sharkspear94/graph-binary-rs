use core::slice;
use std::io::Read;

use uuid::Uuid;

use crate::{
    error::{DecodeError, EncodeError},
    specs::CoreType,
};

use super::{encode_null_object, Decode, Encode};

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
        let mut buf = [0_u8; 4];
        reader.read_exact(&mut buf)?;

        Ok(i32::from_be_bytes(buf))
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
}

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

#[test]
fn encode_string() {
    let s = String::from("test");

    let mut buf: Vec<u8> = vec![];
    s.encode(&mut buf).unwrap();
    assert_eq!(
        &[0x03, 0x00, 0x00, 0x00, 0x00, 0x04, 0x74, 0x65, 0x73, 0x74][..],
        &buf
    );
}

#[test]
fn encode_empty_string() {
    let s = String::new();

    let mut buf: Vec<u8> = vec![];
    s.encode(&mut buf).unwrap();

    assert_eq!(&[0x03, 0x00, 0x00, 0x00, 0x00, 0x00][..], &buf);
}

#[test]
fn decode_empty_string() {
    let buf: Vec<u8> = vec![0x03, 0x00, 0x00, 0x00, 0x00, 0x00];

    let s = String::decode(&mut &buf[..]);

    assert_eq!(String::new(), s.unwrap());
}

#[test]
fn string_decode() {
    let reader: Vec<u8> = vec![0x0, 0x0, 0x0, 0x04, b'h', b'o', b's', b't'];

    let s = String::partial_decode(&mut &reader[..]);

    assert!(s.is_ok());

    assert_eq!("host", s.unwrap().as_str())
}

#[test]
fn string_utf8_decode() {
    let reader = vec![0x3_u8, 0x0, 0x0, 0x0, 0x0, 0x4, 240, 159, 146, 150];

    let s = String::decode(&mut &*reader).unwrap();
    assert_eq!("💖", s);
}

#[test]
fn string_decode_fail() {
    let reader: Vec<u8> = vec![0x0, 0x0, 0x0, 0x04, b'h', b'o', b's'];

    let s = String::partial_decode(&mut &reader[..]);

    assert!(s.is_err());
}

#[test]
fn int32_encode() {
    let mut buf: Vec<u8> = vec![];
    i32::MAX.encode(&mut buf).unwrap();

    assert_eq!(&[0x01, 0x00, 0x7F, 0xFF, 0xFF, 0xFF][..], buf);

    buf.clear();
    i32::MIN.encode(&mut buf).unwrap();

    assert_eq!(&[0x01, 0x00, 0x80, 0x00, 0x00, 0x00][..], buf);
}

#[test]
fn uuid_encode() {
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
fn option_decode() {
    let reader: Vec<u8> = vec![0x03, 0x0, 0x0, 0x0, 0x0, 0x04, b'h', b'o', b's', b't'];

    let option: Option<String> = Option::decode(&mut &reader[..]).unwrap();

    assert_eq!(option.unwrap(), String::from("host"))
}

#[test]
fn option_none_decode() {
    let reader: Vec<u8> = vec![0x03, 0x1];

    let option: Option<String> = Option::decode(&mut &reader[..]).unwrap();

    assert!(option.is_none());

    let reader: Vec<u8> = vec![0xfe, 0x1];

    let option: Option<String> = Option::decode(&mut &reader[..]).unwrap();

    assert!(option.is_none())
}

#[test]
fn option_fail_decode() {
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
