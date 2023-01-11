use std::fmt::Display;

use crate::{conversion, GremlinValue};

use super::{edge::Edge, property::Property, vertex::Vertex};

#[derive(Debug, PartialEq, Clone)]
pub struct Graph {
    pub(crate) vertices: Vec<Vertex>,
    pub(crate) edges: Vec<GraphEdge>,
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "vertexes:[")?;
        for vertex in &self.vertices {
            write!(f, "{vertex},")?;
        }
        writeln!(f, "]")?;
        write!(f, "edges:[")?;
        for edge in &self.edges {
            write!(f, "{edge}")?;
        }
        write!(f, "]")
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct GraphEdge {
    pub id: GremlinValue,
    pub label: String,
    pub in_v_id: GremlinValue,
    pub in_v_label: Option<String>,
    pub out_v_id: GremlinValue,
    pub out_v_label: Option<String>,
    pub parent: Option<Vertex>,
    pub properties: Vec<Property>,
}

impl From<Edge> for GraphEdge {
    fn from(e: Edge) -> Self {
        let mut v = Vec::new();
        if e.properties.is_some() {
            v = e.properties.unwrap();
        }
        GraphEdge {
            id: *e.id,
            label: e.label,
            in_v_id: *e.in_v_id,
            in_v_label: Some(e.in_v_label),
            out_v_id: *e.out_v_id,
            out_v_label: Some(e.out_v_label),
            parent: e.parent,
            properties: v,
        }
    }
}

impl Display for GraphEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id:{},label:{},inV_id:{}",
            self.id, self.label, self.in_v_id
        )?;
        self.in_v_label
            .as_ref()
            .map_or_else(|| Ok(()), |label| write!(f, ",inV_label:{label}"))?;

        write!(f, ",outV_id:{}", self.out_v_id)?;

        self.out_v_label
            .as_ref()
            .map_or_else(|| Ok(()), |label| write!(f, ",outV_label:{label}"))?;

        self.parent
            .as_ref()
            .map_or_else(|| Ok(()), |parent| write!(f, ",parent:{parent}"))?;

        write!(f, ",properties:[")?;
        for property in &self.properties {
            write!(f, "{property},")?;
        }
        write!(f, "]")
    }
}



conversion!(Graph, Graph);
