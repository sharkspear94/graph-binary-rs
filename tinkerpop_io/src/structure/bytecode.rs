use std::fmt::Display;

use crate::{conversion, GremlinValue};

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Bytecode {
    pub(crate) steps: Vec<Step>,
    pub(crate) sources: Vec<Source>,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Step {
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
pub(crate) struct Source {
    pub name: String,
    pub values: Vec<GremlinValue>,
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
