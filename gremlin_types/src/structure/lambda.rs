use std::fmt::Display;

use crate::{conversion, specs::CoreType};

#[cfg(feature = "graph_binary")]
use crate::graph_binary::{Decode, Encode};

#[cfg(feature = "graph_son")]
use crate::error::GraphSonError;
#[cfg(feature = "graph_son")]
use crate::graphson::{get_val_by_key_v3, validate_type, DecodeGraphSON, EncodeGraphSON};
#[cfg(feature = "graph_son")]
use serde_json::json;

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
}

#[cfg(feature = "graph_son")]
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

#[cfg(feature = "graph_son")]
impl DecodeGraphSON for Lambda {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:Lambda")?;

        let script = get_val_by_key_v3(value_object, "script", "Lambda")?;
        let language = get_val_by_key_v3(value_object, "language", "Lambda")?;

        let arguments_length = value_object
            .get("arguments")
            .ok_or_else(|| GraphSonError::KeyNotFound("arguments".to_string()))?
            .as_i64()
            .ok_or_else(|| GraphSonError::WrongJsonType("i64".to_string()))
            .map(|len| len as i32)?;
        Ok(Lambda {
            language,
            script,
            arguments_length,
        })
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Lambda::decode_v3(j_val)
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
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
