use std::str::FromStr;

use bigdecimal::BigDecimal;
use num::BigInt;
use serde_json::json;

use crate::{
    graph_binary::{Decode, Encode},
    graphson::{validate_type, DecodeGraphSON, EncodeGraphSON},
    specs::CoreType,
};

impl Encode for BigInt {
    fn type_code() -> u8 {
        CoreType::BigInteger.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        let bytes = self.to_signed_bytes_be();
        let len = bytes.len() as i32;
        len.partial_encode(writer)?;
        writer.write_all(&bytes)?;
        Ok(())
    }
}

impl Decode for BigInt {
    fn expected_type_code() -> u8 {
        CoreType::BigInteger.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let len = i32::partial_decode(reader)?;
        let mut buf = vec![0; len as usize];
        reader.read_exact(&mut buf)?;
        Ok(BigInt::from_signed_bytes_be(&buf))
    }
}

impl EncodeGraphSON for BigInt {
    fn encode_v3(&self) -> serde_json::Value {
        let num =
            serde_json::Value::Number(serde_json::Number::from_str(&self.to_string()).unwrap());
        json!({
          "@type" : "gx:BigInteger",
          "@value" : num
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        self.encode_v3()
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

impl DecodeGraphSON for BigInt {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, crate::error::GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "gx:BigInteger")?;
        match value_object {
            serde_json::Value::Number(val) => BigInt::from_str(&val.to_string()).map_err(|err| {
                crate::error::GraphSonError::Parse(format!("cannot parse BigInt: {err}"))
            }),
            _ => Err(crate::error::GraphSonError::WrongJsonType(
                "number".to_string(),
            )),
        }
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, crate::error::GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, crate::error::GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

impl Encode for BigDecimal {
    fn type_code() -> u8 {
        CoreType::BigDecimal.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        let (big_int, scale) = self.as_bigint_and_exponent();
        (scale as i32).partial_encode(writer)?;
        big_int.partial_encode(writer)
    }
}

impl Decode for BigDecimal {
    fn expected_type_code() -> u8 {
        CoreType::BigDecimal.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let scale = i32::partial_decode(reader)?;
        let big_int = BigInt::partial_decode(reader)?;

        Ok(BigDecimal::new(big_int, scale as i64))
    }
}

impl EncodeGraphSON for BigDecimal {
    fn encode_v3(&self) -> serde_json::Value {
        let num =
            serde_json::Value::Number(serde_json::Number::from_str(&self.to_string()).unwrap());
        json!({
          "@type" : "gx:BigDecimal",
          "@value" : num
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        self.encode_v3()
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

impl DecodeGraphSON for BigDecimal {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, crate::error::GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "gx:BigDecimal")?;
        match value_object {
            serde_json::Value::Number(val) => {
                BigDecimal::from_str(&val.to_string()).map_err(|err| {
                    crate::error::GraphSonError::Parse(format!("cannot parse BigDecimal: {err}"))
                })
            }
            _ => Err(crate::error::GraphSonError::WrongJsonType(
                "number".to_string(),
            )),
        }
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, crate::error::GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, crate::error::GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

#[test]
fn big_int_encode_v3() {
    let s = BigInt::from_str("123456789987654321123456789987654321").unwrap();
    let expected = r#"{"@type":"gx:BigInteger","@value":123456789987654321123456789987654321}"#;
    let res = serde_json::to_string(&s.encode_v3()).unwrap();
    assert_eq!(res, expected)
}

#[test]
fn big_int_decode_v3() {
    let expected = BigInt::from_str("123456789987654321123456789987654321").unwrap();
    let s = r#"{"@type":"gx:BigInteger","@value":123456789987654321123456789987654321}"#;
    let val = serde_json::from_str(s).unwrap();
    let res = BigInt::decode_v3(&val).unwrap();
    assert_eq!(res, expected)
}

#[test]
fn big_dec_encode_v3() {
    let s = BigDecimal::from_str("123456789987654321123456789987654321").unwrap();
    let expected = r#"{"@type":"gx:BigDecimal","@value":123456789987654321123456789987654321}"#;
    let res = serde_json::to_string(&s.encode_v3()).unwrap();
    assert_eq!(res, expected)
}

#[test]
fn big_dec_scale_encode_v3() {
    let s = BigDecimal::from_str("123456789987654321.123456789987654321").unwrap();
    let expected = r#"{"@type":"gx:BigDecimal","@value":123456789987654321.123456789987654321}"#;
    let res = serde_json::to_string(&s.encode_v3()).unwrap();
    assert_eq!(res, expected)
}

#[test]
fn big_dec_decode_v3() {
    let expected = BigDecimal::from_str("123456789987654321123456789987654321").unwrap();
    let s = r#"{"@type":"gx:BigDecimal","@value":123456789987654321123456789987654321}"#;
    let val = serde_json::from_str(s).unwrap();
    let res = BigDecimal::decode_v3(&val).unwrap();
    assert_eq!(res, expected)
}

#[test]
fn big_dec_scale_decode_v3() {
    let expected = BigDecimal::from_str("123456789987654321.123456789987654321").unwrap();
    let s = r#"{"@type":"gx:BigDecimal","@value":123456789987654321.123456789987654321}"#;
    let val = serde_json::from_str(s).unwrap();
    let res = BigDecimal::decode_v3(&val).unwrap();
    assert_eq!(res, expected)
}

// #[test]
// fn asdasd12() {
//     let j = json!({
//       "@type" : "gx:BigInteger",
//       "@value" : 123123123123123123123123123123123123
//     });
//     println!("{s}");
// }
