use crate::specs::CoreType;

use std::ops::Deref;

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
    pub fn new(buf: &[u8]) -> Self {
        ByteBuffer(Vec::from_iter(buf.iter().copied()))
    }
}

impl Deref for ByteBuffer {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
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
          "@value" : String::from_iter(self.0.iter().map(|byte| *byte as char))
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
