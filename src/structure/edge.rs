use crate::{graph_binary::Encode, specs};

use super::{property::Property, vertex::Vertex};

#[derive(Debug, PartialEq)]
pub struct Edge {
    pub id: i64,
    pub label: String,
    pub in_v_id: i64,
    pub in_v_label: String,
    pub out_v_id: i64,
    pub out_v_label: String,
    pub parent: Vertex,
    pub properties: Vec<Property>,
}

impl Encode for Edge {
    fn type_code() -> u8 {
        specs::CoreType::Edge.into()
    }

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), crate::error::EncodeError> {
        self.id.fq_gb_bytes(writer)?;
        self.label.gb_bytes(writer)?;
        self.in_v_id.fq_gb_bytes(writer)?;
        self.in_v_label.gb_bytes(writer)?;
        self.out_v_id.fq_gb_bytes(writer)?;
        self.out_v_label.gb_bytes(writer)?;
        self.parent.fq_gb_bytes(writer)?;
        for property in &self.properties {
            // TODO
            property.fq_gb_bytes(writer)?;
        }
        Ok(())
    }
}
