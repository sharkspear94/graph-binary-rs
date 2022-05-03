use crate::{graph_binary::Encode, specs};

use super::{edge::Edge, vertex::Vertex};

#[derive(Debug, PartialEq)]
pub struct Graph {
    vertexes: Vec<Vertex>,
    edges: Vec<Edge>,
}

impl Encode for Graph {
    fn type_code() -> u8 {
        specs::CoreType::Graph.into()
    }

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), crate::error::EncodeError> {
        let v_len = self.vertexes.len() as i32;
        let e_len = self.edges.len() as i32;

        v_len.gb_bytes(writer)?;
        for vertex in &self.vertexes {
            vertex.fq_gb_bytes(writer)?;
        }

        e_len.gb_bytes(writer)?;
        for edge in self.edges.iter() {
            edge.id.fq_gb_bytes(writer)?;
            edge.label.gb_bytes(writer)?;
            edge.in_v_id.fq_gb_bytes(writer)?;
            String::fq_null(writer)?;
            edge.out_v_id.fq_gb_bytes(writer)?;
            String::fq_null(writer)?;
            Vertex::fq_null(writer)?;
            edge.properties.fq_gb_bytes(writer)?; // TODO not sure if prop identifier is needed
        }
        Ok(())
    }
}
