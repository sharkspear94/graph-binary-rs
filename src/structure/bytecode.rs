use crate::{
    graph_binary::{Decode, Encode, GraphBinary},
    specs::CoreType,
    struct_de_serialize,
};

#[derive(Debug, PartialEq, Default, Clone)]
pub struct ByteCode {
    pub steps: Vec<Step>,
    pub sources: Vec<Source>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Step {
    pub name: String,
    pub values: Vec<GraphBinary>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Source {
    pub name: String,
    pub values: Vec<GraphBinary>,
}

impl ByteCode {
    pub fn new() -> Self {
        ByteCode::default()
    }
    pub fn add_step(&mut self, name: &str, values: Vec<GraphBinary>) {
        self.steps.push(Step {
            name: name.to_string(),
            values,
        });
    }
    pub fn add_source(&mut self, name: &str, values: Vec<GraphBinary>) {
        self.sources.push(Source {
            name: name.to_string(),
            values,
        });
    }

    pub fn extend_last_step(&mut self, values: impl Iterator<Item = impl Into<GraphBinary>>) {
        let last = self
            .steps
            .last_mut()
            .expect("Bytecode step cannot be extended without prior step");
        last.values.extend(values.map(Into::into))
    }

    pub fn add_to_last_step(&mut self, value: impl Into<GraphBinary>) {
        let last = self
            .steps
            .last_mut()
            .expect("Bytecode step cannot be extended without prior step");
        last.values.push(value.into())
    }
}

impl Encode for ByteCode {
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

impl Decode for ByteCode {
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
            let values = Vec::<GraphBinary>::partial_decode(reader)?;
            steps.push(Step { name, values });
        }

        let len = i32::partial_decode(reader)? as usize;
        let mut sources = Vec::with_capacity(len);
        for _ in 0..len {
            let name = String::partial_decode(reader)?;
            let values = Vec::<GraphBinary>::partial_decode(reader)?;
            sources.push(Source { name, values });
        }

        Ok(ByteCode { steps, sources })
    }

    fn get_partial_len(bytes: &[u8]) -> Result<usize, crate::error::DecodeError> {
        let t: [u8; 4] = bytes[0..4].try_into()?;
        let steps_len = i32::from_be_bytes(t);
        let mut len = 4;
        for _ in 0..steps_len {
            len += String::get_partial_len(&bytes[len..])?;
            len += Vec::<GraphBinary>::get_partial_len(&bytes[len..])?;
        }
        let t: [u8; 4] = bytes[len..len + 4].try_into()?;
        let sources_len = i32::from_be_bytes(t);
        len += 4;
        for _ in 0..sources_len {
            len += String::get_partial_len(&bytes[len..])?;
            len += Vec::<GraphBinary>::get_partial_len(&bytes[len..])?;
        }
        Ok(len)
    }
}

struct_de_serialize!((ByteCode, ByteCodeVisitor, 32));
