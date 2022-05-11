use crate::error::DecodeError;
use crate::specs::CoreType;
use crate::{error::EncodeError, graph_binary::Encode};
use uuid::Uuid;

use crate::graph_binary::{
    build_fq_null_bytes, Decode, GraphBinary, INT32_LEN, INT32_TYPE_CODE, INT64_TYPE_CODE,
    STRING_TYPE_CODE, VALUE_PRESENT,
};

use std::io::Read;
use std::slice;

impl Encode for String {
    fn type_code() -> u8 {
        CoreType::String.into()
    }

    fn write_patial_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
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
                .filter_map(|c| c.ok())
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

impl From<String> for GraphBinary {
    fn from(s: String) -> Self {
        GraphBinary::String(s)
    }
}

impl Encode for &str {
    fn type_code() -> u8 {
        CoreType::String.into()
    }

    fn write_patial_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
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

impl Encode for u8 {
    fn type_code() -> u8 {
        CoreType::Byte.into()
    }

    fn write_patial_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
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

impl From<u8> for GraphBinary {
    fn from(v: u8) -> Self {
        GraphBinary::Byte(v)
    }
}

impl Encode for i16 {
    fn type_code() -> u8 {
        CoreType::Short.into()
    }

    fn write_patial_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
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

impl From<i16> for GraphBinary {
    fn from(v: i16) -> Self {
        GraphBinary::Short(v)
    }
}

impl Encode for i32 {
    fn type_code() -> u8 {
        CoreType::Int32.into()
    }

    fn write_patial_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
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

    fn write_patial_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
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

impl From<i64> for GraphBinary {
    fn from(v: i64) -> Self {
        GraphBinary::Long(v)
    }
}

impl Encode for f32 {
    fn type_code() -> u8 {
        CoreType::Float.into()
    }

    fn write_patial_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
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

impl From<f32> for GraphBinary {
    fn from(v: f32) -> Self {
        GraphBinary::Float(v)
    }
}

impl Encode for f64 {
    fn type_code() -> u8 {
        CoreType::Double.into()
    }

    fn write_patial_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
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

impl From<f64> for GraphBinary {
    fn from(v: f64) -> Self {
        GraphBinary::Double(v)
    }
}

impl Encode for Uuid {
    fn type_code() -> u8 {
        CoreType::Uuid.into()
    }

    fn write_patial_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
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

impl From<Uuid> for GraphBinary {
    fn from(v: Uuid) -> Self {
        GraphBinary::Uuid(v)
    }
}

impl Encode for bool {
    fn type_code() -> u8 {
        CoreType::Boolean.into()
    }

    fn write_patial_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
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

impl From<bool> for GraphBinary {
    fn from(v: bool) -> Self {
        GraphBinary::Boolean(v)
    }
}

impl<T: Encode> Encode for Option<T> {
    fn type_code() -> u8 {
        T::type_code()
    }

    fn write_patial_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        match self {
            Some(i) => i.write_patial_bytes(writer),
            None => build_fq_null_bytes(writer),
        }
    }

    fn write_full_qualified_bytes<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), EncodeError> {
        match self {
            Some(i) => i.write_full_qualified_bytes(writer),
            None => build_fq_null_bytes(writer),
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

    fn fully_self_decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let mut buf = [255_u8; 2];
        reader.read_exact(&mut buf)?;
        let type_code = Self::expected_type_code();
        match (buf[0], buf[1]) {
            (code, 0) if code == type_code => Self::partial_decode(reader),
            (code, 1) if code == type_code => Ok(None),
            (0xFE, 1) => Ok(None),
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

impl<T: Encode> Encode for &[T] {
    fn type_code() -> u8 {
        CoreType::List.into()
    }

    fn write_patial_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        let len = self.len() as i32;
        len.write_patial_bytes(writer)?;

        for item in *self {
            item.write_full_qualified_bytes(writer)?;
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
    s.write_full_qualified_bytes(&mut buf).unwrap();
    assert_eq!(
        &[
            STRING_TYPE_CODE,
            VALUE_PRESENT,
            0x00,
            0x00,
            0x00,
            0x04,
            0x74,
            0x65,
            0x73,
            0x74
        ][..],
        &buf
    );
    // assert_eq!(
    //     Bytes::from_static(&[
    //         STRING_TYPE_CODE,
    //         VALUE_PRESENT,
    //         0x00,
    //         0x00,
    //         0x00,
    //         0x04,
    //         0x74,
    //         0x65,
    //         0x73,
    //         0x74
    //     ]),
    //     s2.generate_fully_qualiffied_bytes()
    // );
}

#[test]
fn encode_empty_string_test() {
    let s = String::new();

    let mut buf: Vec<u8> = vec![];
    s.write_full_qualified_bytes(&mut buf).unwrap();

    assert_eq!(
        &[STRING_TYPE_CODE, VALUE_PRESENT, 0x00, 0x00, 0x00, 0x00][..],
        &buf
    );
    // assert_eq!(
    //     Bytes::from_static(&[STRING_TYPE_CODE, VALUE_PRESENT, 0x00, 0x00, 0x00, 0x00]),
    //     s2.generate_fully_qualiffied_bytes()
    // );
}

#[test]
fn decode_fq_empty_string_test() {
    let buf: Vec<u8> = vec![0x03, VALUE_PRESENT, 0x00, 0x00, 0x00, 0x00];

    let s = String::fully_self_decode(&mut &buf[..]);

    assert_eq!(String::new(), s.unwrap());
    // assert_eq!(
    //     Bytes::from_static(&[STRING_TYPE_CODE, VALUE_PRESENT, 0x00, 0x00, 0x00, 0x00]),
    //     s2.generate_fully_qualiffied_bytes()
    // );
}

#[test]
fn decode_string_test() {
    let reader: Vec<u8> = vec![0x0, 0x0, 0x0, 0x04, b'h', b'o', b's', b't'];

    let s = String::partial_decode(&mut &reader[..]);

    assert!(s.is_ok());

    assert_eq!("host", s.unwrap().as_str())
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
    i32::MAX.write_full_qualified_bytes(&mut buf).unwrap();

    assert_eq!(
        &[INT32_TYPE_CODE, VALUE_PRESENT, 0x7F, 0xFF, 0xFF, 0xFF][..],
        buf
    );

    buf.clear();
    i32::MIN.write_full_qualified_bytes(&mut buf).unwrap();

    assert_eq!(
        &[INT32_TYPE_CODE, VALUE_PRESENT, 0x80, 0x00, 0x00, 0x00][..],
        buf
    );
}

#[test]
fn encode_uuid_test() {
    let mut buf: Vec<u8> = vec![];
    let uuid = uuid::Uuid::from_bytes([
        0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee,
        0xff,
    ]);

    uuid.write_full_qualified_bytes(&mut buf).unwrap();

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

    let option: Option<String> = Option::fully_self_decode(&mut &reader[..]).unwrap();

    assert_eq!(option.unwrap(), String::from("host"))
}

#[test]
fn option_none_decode_test() {
    let reader: Vec<u8> = vec![0x03, 0x1];

    let option: Option<String> = Option::fully_self_decode(&mut &reader[..]).unwrap();

    assert!(option.is_none());

    let reader: Vec<u8> = vec![0xfe, 0x1];

    let option: Option<String> = Option::fully_self_decode(&mut &reader[..]).unwrap();

    assert!(option.is_none())
}

#[test]
fn option_should_fail_decode_test() {
    let reader: Vec<u8> = vec![0x04, 0x1];

    let option: Result<Option<String>, _> = Option::fully_self_decode(&mut &reader[..]);

    assert!(option.is_err());

    let reader: Vec<u8> = vec![0xfe, 0x0];

    let option: Result<Option<String>, _> = Option::fully_self_decode(&mut &reader[..]);

    assert!(option.is_err());

    let reader: Vec<u8> = vec![0xfe, 0x2];

    let option: Result<Option<String>, _> = Option::fully_self_decode(&mut &reader[..]);

    assert!(option.is_err())
}
