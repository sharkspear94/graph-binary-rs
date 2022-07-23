use std::{
    io::Read,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

#[cfg(feature = "graph_son")]
use crate::graphson::{validate_type_entry, DecodeGraphSON, EncodeGraphSON};
use crate::{
    conversion,
    error::{DecodeError, EncodeError, GraphSonError},
    graphson::validate_type,
    specs::CoreType,
};
#[cfg(feature = "graph_son")]
use serde_json::json;

#[cfg(feature = "graph_binary")]
use crate::graph_binary::{Decode, Encode};

#[cfg(feature = "graph_binary")]
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

#[cfg(feature = "graph_binary")]
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
#[cfg(feature = "graph_son")]
impl DecodeGraphSON for char {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        j_val
            .as_object()
            .filter(|map| validate_type_entry(*map, "gx:Char"))
            .and_then(|map| map.get("@value"))
            .and_then(|value| value.as_str())
            .and_then(|s| s.chars().next()) //FIXME more than 1 char is not evaluated
            .ok_or_else(|| GraphSonError::WrongJsonType("str".to_string()))
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
            .as_str()
            .and_then(|s| s.chars().next())
            .ok_or_else(|| GraphSonError::WrongJsonType("str".to_string()))
    }
}

#[cfg(feature = "graph_son")]
impl EncodeGraphSON for char {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
            "@type": "gx:Char",
            "@value": self
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        json!({
            "@type": "gx:Char",
            "@value": self
        })
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

#[cfg(feature = "graph_binary")]
impl Encode for IpAddr {
    fn type_code() -> u8 {
        CoreType::InetAddress.into()
    }

    fn partial_encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        match self {
            IpAddr::V4(v4) => {
                writer.write_all(&4i32.to_be_bytes())?;
                writer.write_all(&v4.octets())?;
                Ok(())
            }
            IpAddr::V6(v6) => {
                writer.write_all(&16i32.to_be_bytes())?;
                writer.write_all(&v6.octets())?;
                Ok(())
            }
        }
    }
}

#[cfg(feature = "graph_binary")]
impl Decode for IpAddr {
    fn expected_type_code() -> u8 {
        CoreType::InetAddress.into()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        match i32::partial_decode(reader)? {
            4 => {
                let mut buf = [0u8; 4];
                reader.read_exact(&mut buf)?;
                Ok(IpAddr::V4(Ipv4Addr::from(buf)))
            }
            16 => {
                let mut buf = [0u8; 16];
                reader.read_exact(&mut buf)?;
                Ok(IpAddr::V6(Ipv6Addr::from(buf)))
            }
            rest => Err(DecodeError::DecodeError(format!(
                "lenght of ip is not valid. Found length {rest}"
            ))),
        }
    }
}

#[cfg(feature = "graph_son")]
impl EncodeGraphSON for IpAddr {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
            "@type" : "gx:InetAddress",
            "@value" : self.to_string()
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        json!({
            "@type" : "gx:InetAddress",
            "@value" : self.to_string()
        })
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

#[cfg(feature = "graph_son")]
impl DecodeGraphSON for IpAddr {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "gx:InetAddress")?;

        match value_object
            .as_str()
            .ok_or_else(|| GraphSonError::WrongJsonType("str".to_string()))?
        {
            "localhost" => Ok(IpAddr::V4(Ipv4Addr::LOCALHOST)),
            other => IpAddr::from_str(other).map_err(|e| GraphSonError::Parse(e.to_string())),
        }
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

conversion!(char, Char);
conversion!(IpAddr, InetAddress);

#[test]
fn ip_v4_encode() {
    let expected = [0x82, 0x0, 0x0, 0x0, 0x0, 0x4, 167, 123, 5, 1];

    let mut buf = vec![];
    IpAddr::from_str("167.123.5.1")
        .unwrap()
        .encode(&mut buf)
        .unwrap();

    assert_eq!(buf, expected)
}

#[test]
fn ip_v6_encode() {
    let expected = [
        0x82, 0x0, 0x0, 0x0, 0x0, 0x10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ];

    let mut buf = vec![];
    IpAddr::from_str("0:0:0:0:0:0:0:1")
        .unwrap()
        .encode(&mut buf)
        .unwrap();

    assert_eq!(buf, expected)
}

#[test]
fn ip_v4_decode() {
    let buf = vec![0x82_u8, 0x0, 0x0, 0x0, 0x0, 0x4, 167, 123, 5, 1];

    let expected = IpAddr::from_str("167.123.5.1").unwrap();
    let res = IpAddr::decode(&mut &buf[..]).unwrap();
    assert_eq!(res, expected)
}

#[test]
fn ip_v6_decode() {
    let buf = [
        0x82, 0x0, 0x0, 0x0, 0x0, 0x10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ];

    let expected = IpAddr::from_str("0:0:0:0:0:0:0:1").unwrap();
    let res = IpAddr::decode(&mut &buf[..]).unwrap();

    assert_eq!(res, expected)
}

#[test]
fn ip_encode_v3() {
    let expected = r#"{"@type":"gx:InetAddress","@value":"167.123.5.1"}"#;

    let v = IpAddr::from_str("167.123.5.1").unwrap().encode_v3();
    let res = serde_json::to_string(&v).unwrap();
    assert_eq!(res, expected);
}

#[test]
fn ip_decode_v3() {
    let s = r#"{
        "@type" : "gx:InetAddress",
        "@value" : "localhost"
      }"#;

    let v = serde_json::from_str(s).unwrap();
    let res = IpAddr::decode_v3(&v).unwrap();
    assert_eq!(res, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
}

#[test]
fn ip_v6_decode_v3() {
    let s = r#"{
        "@type" : "gx:InetAddress",
        "@value" : "2001:0db8:85a3:08d3:1319:8a2e:0370:7347"
      }"#;

    let v = serde_json::from_str(s).unwrap();
    let res = IpAddr::decode_v3(&v).unwrap();
    assert_eq!(
        res,
        IpAddr::V6(Ipv6Addr::from_str("2001:0db8:85a3:08d3:1319:8a2e:0370:7347").unwrap())
    )
}
