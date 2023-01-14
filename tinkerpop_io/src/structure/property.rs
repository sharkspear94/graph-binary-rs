use std::fmt::Display;

use super::{edge::Edge, vertex_property::VertexProperty};
use crate::conversion;
use crate::GremlinValue;

#[derive(Debug, PartialEq, Clone)]
pub struct Property {
    pub key: String,
    pub value: Box<GremlinValue>,
    pub parent: EitherParent,
}

impl Property {
    pub fn new(key: &str, value: impl Into<GremlinValue>, parent: EitherParent) -> Self {
        Property {
            key: key.to_string(),
            value: Box::new(value.into()),
            parent,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum EitherParent {
    Edge(Edge),
    VertexProperty(VertexProperty),
    None,
}

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}-{}", self.key, self.value, self.parent)
    }
}

impl Display for EitherParent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EitherParent::Edge(e) => write!(f, "-{e}"),
            EitherParent::VertexProperty(v) => write!(f, "-{v}"),
            EitherParent::None => Ok(()),
        }
    }
}

conversion!(Property, Property);
