use crate::specs::CoreType;

use std::fmt::Display;

#[cfg(feature = "graph_binary")]
use crate::graph_binary::{Decode, Encode};

#[cfg(feature = "graph_son")]
use crate::error::GraphSonError;
#[cfg(feature = "graph_son")]
use crate::graphson::{validate_type, DecodeGraphSON, EncodeGraphSON};
#[cfg(feature = "graph_son")]
use serde_json::json;

#[derive(Debug, Clone, PartialEq)]
pub struct ByteBuffer(Vec<u8>);

impl ByteBuffer {
    #[must_use]
    pub fn new(buf: &[u8]) -> Self {
        ByteBuffer(buf.to_vec())
    }
    #[must_use]
    pub fn bytes(&self) -> &Vec<u8> {
        &self.0
    }
    #[must_use]
    pub fn bytes_mut(&mut self) -> &mut Vec<u8> {
        &mut self.0
    }
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Display for ByteBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for i in &self.0 {
            write!(f, "{i},")?;
        }
        write!(f, "]")
    }
}

#[cfg(feature = "graph_binary")]
impl Encode for ByteBuffer {
    fn type_code() -> u8 {
        CoreType::ByteBuffer.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        let len = self.0.len() as i32;
        len.partial_encode(writer)?;
        writer.write_all(&self.0)?;
        Ok(())
    }
}

#[cfg(feature = "graph_binary")]
impl Decode for ByteBuffer {
    fn expected_type_code() -> u8 {
        CoreType::ByteBuffer.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let len = i32::partial_decode(reader)? as usize;
        let mut buffer = vec![0; len];
        reader.read_exact(&mut buffer)?;
        Ok(ByteBuffer(buffer))
    }
}

#[cfg(feature = "graph_son")]
impl EncodeGraphSON for ByteBuffer {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
          "@type" : "gx:ByteBuffer",
          "@value" : self.0.iter().map(|byte| *byte as char).collect::<String>()
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
impl DecodeGraphSON for ByteBuffer {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let inner = validate_type(j_val, "gx:ByteBuffer")?
            .as_str()
            .map(|s| s.chars().map(|c| c as u8).collect::<Vec<u8>>())
            .ok_or_else(|| GraphSonError::WrongJsonType("str".to_string()))?;
        Ok(ByteBuffer(inner))
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

#[test]
fn encode_gb() {
    let expected = [0x25, 0x0, 0x0, 0x0, 0x0, 0x4, b'a', b'b', b'c', b'd'];
    let byte_buffer = ByteBuffer(vec![b'a', b'b', b'c', b'd']);

    let mut writer = Vec::new();
    byte_buffer.encode(&mut writer).unwrap();
    assert_eq!(writer, expected)
}

#[test]
fn decode_gb() {
    let buf = vec![0x25, 0x0, 0x0, 0x0, 0x0, 0x4, b'a', b'b', b'c', b'd'];
    let expected = ByteBuffer(vec![b'a', b'b', b'c', b'd']);

    let res = ByteBuffer::decode(&mut &buf[..]).unwrap();
    assert_eq!(res, expected)
}

#[test]
fn encode_v3() {
    let byte_buffer = ByteBuffer(vec![b'a', b'b', b'c', b'd', 255, 128, 129, 130]);

    let v = byte_buffer.encode_v3();
    assert_eq!(
        v.to_string(),
        "{\"@type\":\"gx:ByteBuffer\",\"@value\":\"abcdÿ\u{80}\u{81}\u{82}\"}"
    )
}

#[test]
fn decode_v3() {
    let jstr = "{\"@type\":\"gx:ByteBuffer\",\"@value\":\"abcdÿ\u{80}\u{81}\u{82}\"}";
    let expected = ByteBuffer(vec![b'a', b'b', b'c', b'd', 255, 128, 129, 130]);

    let v: serde_json::Value = serde_json::from_str(jstr).unwrap();
    let res = ByteBuffer::decode_v3(&v).unwrap();
    assert_eq!(res, expected)
}
