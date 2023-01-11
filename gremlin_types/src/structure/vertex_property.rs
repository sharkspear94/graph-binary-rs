use std::fmt::Display;

use crate::{conversion, GremlinValue};

use super::{property::Property, vertex::Vertex};

#[derive(Debug, PartialEq, Clone)]
pub struct VertexProperty {
    pub id: Box<GremlinValue>, // TODO needs refinment
    pub label: String,
    pub value: Box<GremlinValue>,
    pub parent: Option<Vertex>,
    pub properties: Option<Vec<Property>>,
}

impl VertexProperty {
    pub fn new(
        id: impl Into<GremlinValue>,
        label: &str,
        value: impl Into<GremlinValue>,
        parent: Option<Vertex>,
        properties: Option<Vec<Property>>,
    ) -> Self {
        VertexProperty {
            id: Box::new(id.into()),
            label: label.to_string(),
            value: Box::new(value.into()),
            parent,
            properties,
        }
    }
}

impl Display for VertexProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id:{},label:{},value:{}",
            self.id, self.label, self.value
        )?;
        self.parent
            .as_ref()
            .map_or_else(|| Ok(()), |p| write!(f, ",parent:{p}"))?;

        if self.properties.is_some() {
            for property in self.properties.as_ref().unwrap() {
                write!(f, ",properties:{property}",)?;
            }
        }
        Ok(())
    }
}

conversion!(VertexProperty, VertexProperty);
