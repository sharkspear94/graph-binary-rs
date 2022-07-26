use std::fmt::Display;

use crate::{conversion, specs::CoreType, GremlinValue};

#[cfg(feature = "graph_binary")]
use crate::graph_binary::{Decode, Encode};

#[cfg(feature = "graph_son")]
use crate::error::GraphSonError;
#[cfg(feature = "graph_son")]
use crate::graphson::{validate_type, DecodeGraphSON, EncodeGraphSON};
#[cfg(feature = "graph_son")]
use serde_json::json;

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Bytecode {
    steps: Vec<Step>,
    sources: Vec<Source>,
}

#[derive(Debug, PartialEq, Clone)]
struct Step {
    pub name: String,
    pub values: Vec<GremlinValue>,
}

impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"["{}""#, self.name)?;
        for step in &self.values {
            write!(f, ", {step}")?;
        }
        write!(f, "]")
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Source {
    name: String,
    values: Vec<GremlinValue>,
}

impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"["{}""#, self.name)?;
        for source in &self.values {
            write!(f, ", {source}")?;
        }
        write!(f, "]")
    }
}

impl Bytecode {
    #[must_use]
    pub fn new() -> Self {
        Bytecode::default()
    }
    pub fn push_new_step(&mut self, name: &str, values: Vec<GremlinValue>) {
        self.steps.push(Step {
            name: name.to_string(),
            values,
        });
    }
    pub fn push_new_source(&mut self, name: &str, values: Vec<GremlinValue>) {
        self.sources.push(Source {
            name: name.to_string(),
            values,
        });
    }

    pub fn extend_last_step(&mut self, values: impl Iterator<Item = impl Into<GremlinValue>>) {
        let last = self
            .steps
            .last_mut()
            .expect("Bytecode step cannot be extended without prior step");
        last.values.extend(values.map(Into::into));
    }

    pub fn add_to_last_step(&mut self, value: impl Into<GremlinValue>) {
        let last = self
            .steps
            .last_mut()
            .expect("Bytecode step cannot be extended without prior step");
        last.values.push(value.into());
    }

    pub fn extend_last_source(&mut self, values: impl Iterator<Item = impl Into<GremlinValue>>) {
        let last = self
            .sources
            .last_mut()
            .expect("Bytecode source cannot be extended without prior step");
        last.values.extend(values.map(Into::into));
    }

    pub fn add_to_last_source(&mut self, value: impl Into<GremlinValue>) {
        let last = self
            .sources
            .last_mut()
            .expect("Bytecode source cannot be extended without prior step");
        last.values.push(value.into());
    }
}

impl Display for Bytecode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "sources: [")?;
        if !self.sources.is_empty() {
            for source in &self.sources[..self.sources.len() - 1] {
                write!(f, "{source},")?;
            }
            write!(f, "{}", self.sources.last().unwrap())?;
        }
        writeln!(f, "]")?;
        write!(f, "steps: [")?;
        if !self.steps.is_empty() {
            for step in &self.steps[..self.steps.len() - 1] {
                write!(f, "{step},")?;
            }
            write!(f, "{}", self.steps.last().unwrap())?;
        }
        write!(f, "]")
    }
}

#[cfg(feature = "graph_binary")]
impl Encode for Bytecode {
    fn type_code() -> u8 {
        CoreType::ByteCode.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        let len = self.steps.len() as i32;
        len.partial_encode(writer)?;
        for step in &self.steps {
            step.name.partial_encode(writer)?;
            step.values.partial_encode(writer)?;
        }
        let len = self.sources.len() as i32;
        len.partial_encode(writer)?;
        for source in &self.sources {
            source.name.partial_encode(writer)?;
            source.values.partial_encode(writer)?;
        }
        Ok(())
    }
}
#[cfg(feature = "graph_binary")]
impl Decode for Bytecode {
    fn expected_type_code() -> u8 {
        CoreType::ByteCode.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let len = i32::partial_decode(reader)? as usize;
        let mut steps = Vec::with_capacity(len);
        for _ in 0..len {
            let name = String::partial_decode(reader)?;
            let values = Vec::<GremlinValue>::partial_decode(reader)?;
            steps.push(Step { name, values });
        }

        let len = i32::partial_decode(reader)? as usize;

        let mut sources = Vec::with_capacity(len);
        for _ in 0..len {
            let name = String::partial_decode(reader)?;
            let values = Vec::<GremlinValue>::partial_decode(reader)?;
            sources.push(Source { name, values });
        }

        Ok(Bytecode { steps, sources })
    }
}

//TODO impl sources in encoding Bytecode
#[cfg(feature = "graph_son")]
impl EncodeGraphSON for Bytecode {
    fn encode_v3(&self) -> serde_json::Value {
        let v: Vec<Vec<serde_json::Value>> = self
            .steps
            .iter()
            .map(|s| {
                let mut inner = vec![s.name.encode_v3()];
                inner.extend(s.values.iter().map(EncodeGraphSON::encode_v3));
                inner
            })
            .collect();
        json!({
          "@type" : "g:Bytecode",
          "@value" : {
            "step" : v
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
impl DecodeGraphSON for Bytecode {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let mut steps = Vec::<Step>::new();
        let mut sources = Vec::<Source>::new();

        let value_object = validate_type(j_val, "g:Bytecode")?;

        let steps_iter = value_object.get("step").and_then(|v| v.as_array());

        if let Some(iter) = steps_iter {
            for inner in iter.iter().flat_map(|v| v.as_array()) {
                let mut step_args = Vec::<GremlinValue>::new();
                let name = inner
                    .first()
                    .and_then(|v| String::decode_v3(v).ok())
                    .ok_or_else(|| GraphSonError::KeyNotFound("first".to_string()))?;
                for i in &inner[1..] {
                    step_args.push(GremlinValue::decode_v3(i)?);
                }
                steps.push(Step {
                    name,
                    values: step_args,
                });
            }
        };

        let source_iter = value_object.get("source").and_then(|v| v.as_array());

        if let Some(iter) = source_iter {
            for inner in iter.iter().filter_map(|v| v.as_array()) {
                let mut source_args = Vec::<GremlinValue>::new();
                let name = inner
                    .first()
                    .and_then(|v| String::decode_v3(v).ok())
                    .ok_or_else(|| GraphSonError::KeyNotFound("first".to_string()))?;
                for i in &inner[1..] {
                    source_args.push(GremlinValue::decode_v3(i)?);
                }
                sources.push(Source {
                    name,
                    values: source_args,
                });
            }
        };
        Ok(Bytecode { steps, sources })
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

conversion!(Bytecode, Bytecode);

#[test]
fn test_display() {
    use crate::structure::enums::T;

    let mut bytecode = Bytecode::default();
    bytecode.push_new_source("withComputer", vec![]);
    bytecode.push_new_step("V", vec![]);
    bytecode.push_new_step("has", vec!["Person".into(), T::Id.into(), 500.into()]);
    bytecode.push_new_step("out", vec!["Person".into()]);

    let expected = "sources: [[\"withComputer\"]]\nsteps: [[\"V\"],[\"has\", \"Person\", T::id, 500_i32],[\"out\", \"Person\"]]";
    assert_eq!(bytecode.to_string(), expected)
}

#[test]
fn decode_v3() {
    let string = r#"{
        "@type" : "g:Bytecode",
        "@value" : {
          "step" : [ [ "V" ], [ "hasLabel", "person" ], [ "out" ], [ "in" ], [ "tree" ] ],
          "source": [["inject",{"@type" : "g:Int32","@value" : 29}]]
        }
      }"#;

    let mut expected = Bytecode::default();
    expected.push_new_step("V", vec![]);
    expected.push_new_step("hasLabel", vec!["person".into()]);
    expected.push_new_step("out", vec![]);
    expected.push_new_step("in", vec![]);
    expected.push_new_step("tree", vec![]);
    expected.push_new_source("inject", vec![29.into()]);

    let j_val = serde_json::from_str(string).unwrap();
    let bc = Bytecode::decode_v3(&j_val).unwrap();
    assert_eq!(bc, expected)
}

#[test]
fn fail_decode_v3() {
    let string = r#"{
        "@type" : "g:Bytecode",
        "@value" : {
          "step" : [ [], [ "hasLabel", "person" ], [], [ "in" ], [ "tree" ] ]
        }
      }"#;

    let j_val = serde_json::from_str(string).unwrap();
    let bc = Bytecode::decode_v3(&j_val);
    assert!(bc.is_err())
}

#[test]
fn fail2_decode_v3() {
    let string = r#"{
        "@type" : "g:Bytecode",
        "@value" : {
          "step" : [ [true], [ "hasLabel", "person" ], [ "in" ], [ "tree" ] ]
        }
      }"#;

    let j_val = serde_json::from_str(string).unwrap();
    let bc = Bytecode::decode_v3(&j_val);
    assert!(bc.is_err())
}

#[test]
fn decode_int_parameter_v3() {
    let string = r#"{
        "@type" : "g:Bytecode",
        "@value" : {
          "step" : [ [ "V" ], [ "has", "person","age",{"@type" : "g:Int32","@value" : 29} ], [ "out" ], [ "in" ], [ "tree" ] ]
        }
      }"#;

    let mut expected = Bytecode::default();
    expected.push_new_step("V", vec![]);
    expected.push_new_step("has", vec!["person".into(), "age".into(), 29.into()]);
    expected.push_new_step("out", vec![]);
    expected.push_new_step("in", vec![]);
    expected.push_new_step("tree", vec![]);

    let j_val = serde_json::from_str(string).unwrap();
    let bc = Bytecode::decode_v3(&j_val).unwrap();
    assert_eq!(bc, expected)
}
