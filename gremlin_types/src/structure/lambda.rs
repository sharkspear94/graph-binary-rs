use std::fmt::Display;

use serde_json::json;

use super::validate_type_entry;
use crate::error::DecodeError;
use crate::{
    conversion,
    graph_binary::{Decode, Encode, GremlinValue},
    graphson::{DecodeGraphSON, EncodeGraphSON},
    specs::CoreType,
    struct_de_serialize, val_by_key_v3,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Lambda {
    pub language: String,
    pub script: String,
    pub arguments_length: i32,
}

impl Lambda {
    pub fn new(script: &str) -> Self {
        Lambda {
            language: "gremlin-groovy".to_string(),
            script: script.to_string(),
            arguments_length: 1,
        }
    }
}

#[cfg(feature = "graph_binary")]
impl Encode for Lambda {
    fn type_code() -> u8 {
        CoreType::Lambda.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.language.partial_encode(writer)?;
        self.script.partial_encode(writer)?;
        self.arguments_length.partial_encode(writer)
    }
}

#[cfg(feature = "graph_binary")]
impl Decode for Lambda {
    fn expected_type_code() -> u8 {
        CoreType::Lambda.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let language = String::partial_decode(reader)?;
        let script = String::partial_decode(reader)?;
        let arguments_length = i32::partial_decode(reader)?;

        Ok(Lambda {
            language,
            script,
            arguments_length,
        })
    }

    fn get_partial_len(bytes: &[u8]) -> Result<usize, crate::error::DecodeError> {
        let mut len = String::get_partial_len(bytes)?;
        len += String::get_partial_len(&bytes[len..])?;
        len += i32::get_partial_len(&bytes[len..])?;
        Ok(len)
    }
}

impl EncodeGraphSON for Lambda {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
          "@type" : "g:Lambda",
          "@value" : {
            "script" : self.script,
            "language" : self.language,
            "arguments" : self.arguments_length // TODO test on server if io Doc is right
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

impl DecodeGraphSON for Lambda {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let object = j_val
            .as_object()
            .filter(|map| validate_type_entry(*map, "g:Lambda"))
            .and_then(|map| map.get("@value"))
            .and_then(|v| v.as_object());

        let script = val_by_key_v3!(object, "script", String, "Lambda")?;
        let language = val_by_key_v3!(object, "language", String, "Lambda")?;
        let arguments_length = object
            .and_then(|len| len.get("arguments"))
            .and_then(|arguments| arguments.as_i64())
            .ok_or_else(|| {
                DecodeError::DecodeError("could not deserialize args len in Lambda".to_string())
            })
            .map(|len| len as i32)?;
        Ok(Lambda {
            language,
            script,
            arguments_length,
        })
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        Lambda::decode_v3(j_val)
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

impl Display for Lambda {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}),({}),args_len:{}",
            self.language, self.script, self.arguments_length
        )
    }
}

struct_de_serialize!((Lambda, LambdaVisitor, 254));
conversion!(Lambda, Lambda);

#[test]
fn encode_v3() {
    let l = Lambda {
        language: "gremlin-groovy".to_string(),
        script: "{ it.get() }".to_string(),
        arguments_length: 1,
    };

    let s = l.encode_v3();
    let res = serde_json::to_string(&s).unwrap();
    let v: serde_json::Value = serde_json::from_str(&res).unwrap();
    let expected = r#"{"@type":"g:Lambda","@value":{"script":"{ it.get()}","language":"gremlin-groovy","arguments":1}}"#;

    assert_eq!(s, v);
}

#[test]
fn decode_v3() {
    let expected = Lambda {
        language: "gremlin-groovy".to_string(),
        script: "{ it.get() }".to_string(),
        arguments_length: 1,
    };
    let s = r#"{
        "@type" : "g:Lambda",
        "@value" : {
          "script" : "{ it.get() }",
          "language" : "gremlin-groovy",
          "arguments" : 1
        }
      }"#;
    let v: serde_json::Value = serde_json::from_str(s).unwrap();
    let res = Lambda::decode_v3(&v).unwrap();

    assert_eq!(res, expected);
}
