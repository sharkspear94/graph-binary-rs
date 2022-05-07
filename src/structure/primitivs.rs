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

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        let len = self.len() as i32;
        writer.write_all(&len.to_be_bytes())?;
        writer.write_all(self.as_bytes())?;
        Ok(())
    }
}

impl Decode for String {
    fn decode<R: Read>(reader: &mut R) -> Result<String, DecodeError> {
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
            l if l != len as usize => Err(DecodeError::DecodeError("String".to_string())),
            _ => Ok(s),
        }
    }
}

impl Encode for &str {
    fn type_code() -> u8 {
        CoreType::String.into()
    }

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        let len = self.len() as i32;
        writer.write_all(&len.to_be_bytes())?;
        writer.write_all(self.as_bytes())?;
        Ok(())
    }
}

impl Encode for i16 {
    fn type_code() -> u8 {
        CoreType::Short.into()
    }

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&self.to_be_bytes())?;

        Ok(())
    }
}

impl Decode for i16 {
    fn decode<R: Read>(reader: &mut R) -> Result<i16, DecodeError> {
        let mut int = [0_u8; 2];
        reader.read_exact(&mut int)?;

        Ok(i16::from_be_bytes(int))
    }
}

impl Encode for i32 {
    fn type_code() -> u8 {
        CoreType::Int32.into()
    }

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

impl Decode for i32 {
    fn decode<R: Read>(reader: &mut R) -> Result<i32, DecodeError> {
        let mut int = [0_u8; 4];
        reader.read_exact(&mut int)?;

        Ok(i32::from_be_bytes(int))
    }
}

impl Encode for i64 {
    fn type_code() -> u8 {
        CoreType::Long.into()
    }

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

impl Decode for i64 {
    fn decode<R: Read>(reader: &mut R) -> Result<i64, DecodeError> {
        let mut int = [0_u8; 8];
        reader.read_exact(&mut int)?;

        Ok(i64::from_be_bytes(int))
    }
}

impl Encode for f32 {
    fn type_code() -> u8 {
        CoreType::Float.into()
    }

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&self.to_be_bytes())?;

        Ok(())
    }
}

impl Decode for f32 {
    fn decode<R: Read>(reader: &mut R) -> Result<f32, DecodeError> {
        let mut int = [0_u8; 4];
        reader.read_exact(&mut int)?;

        Ok(f32::from_be_bytes(int))
    }
}

impl Encode for f64 {
    fn type_code() -> u8 {
        CoreType::Double.into()
    }

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&self.to_be_bytes())?;

        Ok(())
    }
}

impl Decode for f64 {
    fn decode<R: Read>(reader: &mut R) -> Result<f64, DecodeError> {
        let mut int = [0_u8; 8];
        reader.read_exact(&mut int)?;

        Ok(f64::from_be_bytes(int))
    }
}

impl Encode for Uuid {
    fn type_code() -> u8 {
        CoreType::Uuid.into()
    }

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(self.as_bytes())?;

        Ok(())
    }
}

impl Decode for Uuid {
    fn decode<R: Read>(reader: &mut R) -> Result<Uuid, DecodeError> {
        let mut buf = [0_u8; 16];
        reader.read_exact(&mut buf)?;

        Ok(Uuid::from_bytes(buf))
    }
}

impl Encode for bool {
    fn type_code() -> u8 {
        CoreType::Boolean.into()
    }

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        let b = match self {
            true => 0x00_u8,
            false => 0x01_u8,
        };

        writer.write_all(slice::from_ref(&b))?;

        Ok(())
    }
}

impl Decode for bool {
    fn decode<R: Read>(reader: &mut R) -> Result<bool, DecodeError> {
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

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        match self {
            Some(i) => i.gb_bytes(writer),
            None => build_fq_null_bytes(writer),
        }
    }

    fn fq_gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        match self {
            Some(i) => i.fq_gb_bytes(writer),
            None => build_fq_null_bytes(writer),
        }
    }
}

#[test]
fn encode_string_test() {
    let s = String::from("test");
    let s2 = "test";
    let mut buf: Vec<u8> = vec![];
    s.fq_gb_bytes(&mut buf);
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
    let s2 = "";

    let mut buf: Vec<u8> = vec![];
    s.fq_gb_bytes(&mut buf);

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
fn decode_string_test() {
    let reader: Vec<u8> = vec![0x0, 0x0, 0x0, 0x04, b'h', b'o', b's', b't'];

    let s = String::decode(&mut &reader[..]);

    assert!(s.is_ok());

    assert_eq!("host", s.unwrap().as_str())
}

#[test]
fn decode_string_fail_test() {
    let reader: Vec<u8> = vec![0x0, 0x0, 0x0, 0x04, b'h', b'o', b's'];

    let s = String::decode(&mut &reader[..]);

    assert!(s.is_err());
}

#[test]
fn encode_int32_test() {
    let mut buf: Vec<u8> = vec![];
    i32::MAX.fq_gb_bytes(&mut buf);

    assert_eq!(
        &[INT32_TYPE_CODE, VALUE_PRESENT, 0x7F, 0xFF, 0xFF, 0xFF][..],
        buf
    );

    buf.clear();
    i32::MIN.fq_gb_bytes(&mut buf);

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

    uuid.fq_gb_bytes(&mut buf);

    assert_eq!(
        [
            0x0c, 0x00, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb,
            0xcc, 0xdd, 0xee, 0xff
        ][..],
        buf
    )
}