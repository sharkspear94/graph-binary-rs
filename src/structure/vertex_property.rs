use crate::{
    graph_binary::{Encode, GraphBinary},
    specs,
};

use super::{property::Property, vertex::Vertex};

#[derive(Debug, PartialEq)]
pub struct VertexProperty {
    id: i32, // needs refinment
    label: String,
    value: Box<GraphBinary>,
    parent: Vertex,
    properties: Vec<Property>,
}

impl Encode for VertexProperty {
    fn type_code() -> u8 {
        specs::CoreType::VertexProperty.into()
    }

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), crate::error::EncodeError> {
        self.id.fq_gb_bytes(writer)?;
        self.label.gb_bytes(writer)?;
        self.value.build_fq_bytes(writer)?;
        //self.parent.fq_null(writer)?; // TODO: not sure if correct impl
        for property in &self.properties {
            property.fq_gb_bytes(writer)?;
        }
        Ok(())
    }
}
