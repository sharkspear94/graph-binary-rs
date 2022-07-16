use std::fmt::Display;

use serde_json::json;

use crate::{
    conversion,
    error::DecodeError,
    graph_binary::{Decode, Encode},
    graphson::{DecodeGraphSON, EncodeGraphSON},
    specs::CoreType,
    GremlinValue,
};

use super::validate_type_entry;

#[derive(Debug, PartialEq, Clone)]
pub struct Binding {
    key: String,
    value: Box<GremlinValue>,
}

impl Binding {
    pub fn new(key: &str, value: impl Into<GremlinValue>) -> Self {
        Binding {
            key: key.to_owned(),
            value: Box::new(value.into()),
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &GremlinValue {
        &self.value
    }
}

impl<S: ToString, I: Into<GremlinValue>> From<(S, I)> for Binding {
    fn from(pair: (S, I)) -> Self {
        Binding {
            key: pair.0.to_string(),
            value: Box::new(pair.1.into()),
        }
    }
}

impl Display for Binding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.key, self.value)
    }
}

#[cfg(feature = "graph_binary")]
impl Encode for Binding {
    fn type_code() -> u8 {
        CoreType::Binding.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.key.partial_encode(writer)?;
        self.value.encode(writer)
    }
}

#[cfg(feature = "graph_binary")]
impl Decode for Binding {
    fn expected_type_code() -> u8 {
        CoreType::Binding.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let key = String::partial_decode(reader)?;
        let value = Box::new(GremlinValue::decode(reader)?);

        Ok(Binding { key, value })
    }
}

#[cfg(any(feature = "graph_son_v3", feature = "graph_son_v3"))]
impl EncodeGraphSON for Binding {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
          "@type" : "g:Binding",
          "@value" : {
            "key" : self.key,
            "value" : self.value.encode_v3()
          }
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        self.encode_v3()
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

#[cfg(any(feature = "graph_son_v3", feature = "graph_son_v3"))]
impl DecodeGraphSON for Binding {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let mut key = String::new();
        let value = j_val
            .as_object()
            .filter(|map| validate_type_entry(*map, "g:Binding"))
            .map(|map| {
                if let Some(s) = map.get("key").and_then(|s| s.as_str()) {
                    key = s.to_string()
                }
                map
            })
            .and_then(|m| m.get("value"))
            .and_then(|v| GremlinValue::decode_v3(v).ok())
            .ok_or_else(|| DecodeError::DecodeError("json error f32 v3 in error".to_string()))?;
        Ok(Binding {
            key,
            value: Box::new(value),
        })
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

conversion!(Binding, Binding);

#[test]
fn test_binding_encode() {
    let expected = [
        0x14_u8, 0x0, 0x0, 0x00, 0x00, 0x04, 0x74, 0x65, 0x73, 0x74, 0x01, 0x00, 0x00, 0x0, 0x0,
        0x01,
    ];
    let mut buf: Vec<u8> = vec![];
    let b = Binding {
        key: "test".to_string(),
        value: Box::new(1_i32.into()),
    };
    b.encode(&mut buf).unwrap();
    assert_eq!(expected, &*buf)
}

#[test]
fn test_binding_decode() {
    let buf = vec![
        0x14_u8, 0x0, 0x0, 0x00, 0x00, 0x04, 0x74, 0x65, 0x73, 0x74, 0x01, 0x00, 0x00, 0x0, 0x0,
        0x01,
    ];
    let expected = Binding {
        key: "test".to_string(),
        value: Box::new(1_i32.into()),
    };
    let b = Binding::decode(&mut &buf[..]).unwrap();
    assert_eq!(expected, b)
}
