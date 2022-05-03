use crate::{graph_binary::Encode, specs};

use super::vertex_property::VertexProperty;

#[derive(Debug, PartialEq)]
pub struct Vertex {
    id: i32,
    label: String,
    properties: Vec<VertexProperty>,
}

impl Encode for Vertex {
    fn type_code() -> u8 {
        specs::CoreType::Vertex.into()
    }

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), crate::error::EncodeError> {
        self.id.fq_gb_bytes(writer)?;
        self.label.gb_bytes(writer)?;
        for vertex_prop in &self.properties {
            vertex_prop.fq_gb_bytes(writer)?;
        }
        Ok(())
    }
}
