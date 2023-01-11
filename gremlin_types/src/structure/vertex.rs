use std::fmt::Display;

use crate::{conversion, GremlinValue};

use super::vertex_property::VertexProperty;

#[derive(Debug, PartialEq, Clone)]
pub struct Vertex {
    pub id: Box<GremlinValue>,
    pub label: String,
    pub properties: Option<Vec<VertexProperty>>,
}

impl Vertex {
    #[must_use]
    pub fn new<ID: Into<GremlinValue>>(
        id: ID,
        label: &str,
        properties: Option<Vec<VertexProperty>>,
    ) -> Self {
        Vertex {
            id: Box::new(id.into()),
            label: label.to_owned(),
            properties,
        }
    }
    #[must_use]
    pub fn id(&self) -> &GremlinValue {
        &self.id
    }

    #[must_use]
    pub fn label(&self) -> &String {
        &self.label
    }
}

impl Display for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id:{},label:{}", self.id, self.label)?;
        if self.properties.is_some() {
            for property in self.properties.as_ref().unwrap() {
                write!(f, "property:{property}")?;
            }
        }
        Ok(())
    }
}

conversion!(Vertex, Vertex);
