use std::fmt::Display;

use crate::{conversion, GremlinValue};

use super::{property::Property, vertex::Vertex};

#[derive(Debug, PartialEq, Clone)]
pub struct Edge {
    pub id: Box<GremlinValue>,
    pub label: String,
    pub in_v_id: Box<GremlinValue>,
    pub in_v_label: String,
    pub out_v_id: Box<GremlinValue>,
    pub out_v_label: String,
    pub parent: Option<Vertex>,
    pub properties: Option<Vec<Property>>,
}

impl Edge {
    #[must_use]
    pub fn new(label: &str) -> Self {
        Edge {
            id: Box::new(GremlinValue::UnspecifiedNullObject),
            label: label.to_string(),
            in_v_id: Box::new(GremlinValue::UnspecifiedNullObject),
            in_v_label: Default::default(),
            out_v_id: Box::new(GremlinValue::UnspecifiedNullObject),
            out_v_label: Default::default(),
            parent: None,
            properties: None,
        }
    }

    pub fn out_v<T: Into<GremlinValue>>(&mut self, id: T, out_label: &str) -> &mut Self {
        self.out_v_id = Box::new(id.into());
        self.out_v_label = out_label.to_string();
        self
    }

    pub fn out_vertex(&mut self, v: Vertex) -> &mut Self {
        self.out_v_id = v.id;
        self.out_v_label = v.label;
        self
    }
    pub fn in_v<T: Into<GremlinValue>>(&mut self, id: T, in_label: &str) -> &mut Self {
        self.in_v_id = Box::new(id.into());
        self.in_v_label = in_label.to_string();
        self
    }
    pub fn in_vertex(&mut self, v: Vertex) -> &mut Self {
        self.in_v_id = v.id;
        self.in_v_label = v.label;
        self
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id:{},label:{},\ninV_id:{},inV_label:{},\noutV_id:{},outV_label:{}\n",
            self.id, self.label, self.in_v_id, self.in_v_label, self.out_v_id, self.out_v_label
        )?;
        self.parent
            .as_ref()
            .map_or_else(|| Ok(()), |p| write!(f, ",parent:{p}"))?;

        if self.properties.is_some() {
            for property in self.properties.as_ref().unwrap() {
                write!(f, ",properties:{property}")?;
            }
        }
        Ok(())
    }
}

conversion!(Edge, Edge);
