use std::fmt::Display;

use crate::{
    conversion,
    error::{DecodeError, GraphSonError},
    graphson::{get_val_by_key_v2, get_val_by_key_v3, validate_type},
    specs::{self, CoreType},
    GremlinValue,
};

use super::{
    property::{EitherParent, Property},
    vertex::Vertex,
};

#[cfg(feature = "graph_binary")]
use crate::graph_binary::{Decode, Encode};

#[cfg(feature = "graph_son")]
use crate::{
    graphson::{validate_type_entry, DecodeGraphSON, EncodeGraphSON},
    val_by_key_v2, val_by_key_v3,
};
#[cfg(feature = "graph_son")]
use serde_json::{json, Map};

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

#[cfg(feature = "graph_binary")]
impl Encode for VertexProperty {
    fn type_code() -> u8 {
        specs::CoreType::VertexProperty.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.id.encode(writer)?;
        self.label.partial_encode(writer)?;
        self.value.encode(writer)?;
        self.parent.encode(writer)?;
        self.properties.encode(writer)?;
        Ok(())
    }
}

#[cfg(feature = "graph_binary")]
impl Decode for VertexProperty {
    fn expected_type_code() -> u8 {
        CoreType::VertexProperty.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let id = GremlinValue::decode(reader)?;
        let label = String::partial_decode(reader)?;
        let value = GremlinValue::decode(reader)?;
        let parent = Option::<Vertex>::decode(reader)?;
        let properties = Option::<Vec<Property>>::decode(reader)?;

        Ok(VertexProperty {
            id: Box::new(id),
            label,
            value: Box::new(value),
            parent,
            properties,
        })
    }
}

#[cfg(feature = "graph_son")]
impl EncodeGraphSON for VertexProperty {
    fn encode_v3(&self) -> serde_json::Value {
        let mut jval_map = Map::new();
        if let Some(props) = &self.properties {
            for property in props {
                jval_map.insert(property.key.clone(), property.value.encode_v3());
            }
            json!(
                {
                    "@type" : "g:VertexProperty",
                    "@value" : {
                      "id" : self.id.encode_v3(),
                      "value" : self.value.encode_v3(),
                      "label" : self.label,
                      "properties" : jval_map
                    }
                }
            )
        } else {
            json!(
                {
                    "@type" : "g:VertexProperty",
                    "@value" : {
                      "id" : self.id.encode_v3(),
                      "value" : self.value.encode_v3(),
                      "label" : self.label
                    }
                }
            )
        }
    }

    fn encode_v2(&self) -> serde_json::Value {
        let mut jval_map = Map::new();
        if let Some(props) = &self.properties {
            for property in props {
                jval_map.insert(property.key.clone(), property.value.encode_v2());
            }
            json!(
                {
                    "@type" : "g:VertexProperty",
                    "@value" : {
                      "id" : self.id.encode_v2(),
                      "value" : self.value.encode_v2(),
                      "label" : self.label,
                      "vertex" : self.parent.as_ref().map(|v| v.id.encode_v2()),
                      "properties" : jval_map
                    }
                }
            )
        } else {
            json!(
                {
                    "@type" : "g:VertexProperty",
                    "@value" : {
                      "id" : self.id.encode_v2(),
                      "value" : self.value.encode_v2(),
                      "vertex" : self.parent.as_ref().map(|v| v.id.encode_v2()),
                      "label" : self.label
                    }
                }
            )
        }
    }

    fn encode_v1(&self) -> serde_json::Value {
        json!({
          "id" : self.id.encode_v1(),
          "value" : self.value.encode_v1(),
          "label" : self.label
        })
    }
}

#[cfg(feature = "graph_son")]
impl DecodeGraphSON for VertexProperty {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:VertexProperty")?;

        let id = get_val_by_key_v3(value_object, "id", "VertexProperty")?;
        let label = get_val_by_key_v3(value_object, "label", "VertexProperty")?;
        let value = get_val_by_key_v3(value_object, "value", "VertexProperty")?;

        let properties = value_object
            .get("properties")
            .and_then(|prop_obj| prop_obj.as_object())
            .map(|map| {
                map.iter()
                    .map(|(k, v)| {
                        GremlinValue::decode_v3(v).map(|g| Property::new(k, g, EitherParent::None))
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;

        Ok(VertexProperty {
            id: Box::new(id),
            label,
            value: Box::new(value),
            parent: None,
            properties,
        })
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:VertexProperty")?;

        let id = get_val_by_key_v2(value_object, "id", "VertexProperty")?;
        let label = get_val_by_key_v2(value_object, "label", "VertexProperty")?;
        let value = get_val_by_key_v2(value_object, "value", "VertexProperty")?;
        let vertex_id = get_val_by_key_v2(value_object, "vertex", "VertexProperty")?;

        let properties = value_object
            .get("properties")
            .and_then(|prop_obj| prop_obj.as_object())
            .map(|map| {
                map.iter()
                    .map(|(k, v)| {
                        GremlinValue::decode_v2(v).map(|g| Property::new(k, g, EitherParent::None))
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;

        Ok(VertexProperty {
            id: Box::new(id),
            label,
            value: Box::new(value),
            parent: Some(Vertex {
                id: Box::new(vertex_id),
                label: Default::default(),
                properties: None,
            }),
            properties,
        })
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

conversion!(VertexProperty, VertexProperty);
