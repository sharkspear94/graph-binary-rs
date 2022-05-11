use crate::{
    graph_binary::{Encode, GraphBinary},
    specs,
};

use super::{edge::Edge, vertex::Vertex};

#[derive(Debug, PartialEq)]
pub struct Graph {
    vertexes: Vec<Vertex>,
    edges: Vec<Edge>,
}

impl From<Graph> for GraphBinary {
    fn from(g: Graph) -> Self {
        GraphBinary::Graph(g)
    }
}

impl Encode for Graph {
    fn type_code() -> u8 {
        specs::CoreType::Graph.into()
    }

    fn write_patial_bytes<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        let v_len = self.vertexes.len() as i32;
        let e_len = self.edges.len() as i32;

        v_len.write_patial_bytes(writer)?;
        for vertex in &self.vertexes {
            vertex.write_full_qualified_bytes(writer)?;
        }

        e_len.write_patial_bytes(writer)?;
        for edge in self.edges.iter() {
            edge.id.write_full_qualified_bytes(writer)?;
            edge.label.write_patial_bytes(writer)?;
            edge.in_v_id.write_full_qualified_bytes(writer)?;
            String::write_full_qualified_null_bytes(writer)?;
            edge.out_v_id.write_full_qualified_bytes(writer)?;
            String::write_full_qualified_null_bytes(writer)?;
            Vertex::write_full_qualified_null_bytes(writer)?;
            edge.properties.write_full_qualified_bytes(writer)?; // TODO not sure if prop identifier is needed
        }
        Ok(())
    }
}
