use crate::{
    graph_binary::{Decode, Encode, GraphBinary, decode},
    specs,
};

use super::{vertex_property::VertexProperty, property};

#[derive(Debug, PartialEq)]
pub struct Vertex {
    id: i32,
    label: String,
    properties: Option<Box<GraphBinary>>,
}

impl Encode for Vertex {
    fn type_code() -> u8 {
        specs::CoreType::Vertex.into()
    }

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), crate::error::EncodeError> {
        self.id.fq_gb_bytes(writer)?;
        self.label.gb_bytes(writer)?;
        // self.properties.fq_gb_bytes(writer)?;
        todo!()
    }
}

impl Decode for Vertex {
    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let id = decode(reader)?;
        let label = String::decode(reader)?;        
        todo!()
    }
}
