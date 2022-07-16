use std::{collections::HashMap, fmt::Display};

use serde_json::{json, Map};

use crate::{
    conversion,
    error::DecodeError,
    graph_binary::{Decode, Encode, GremlinValue},
    graphson::{DecodeGraphSON, EncodeGraphSON},
    specs::{self, CoreType},
    struct_de_serialize, val_by_key_v2, val_by_key_v3,
};

use super::{
    property::{EitherParent, Property},
    validate_type_entry,
    vertex::Vertex,
};

#[derive(Debug, PartialEq, Clone)]
pub struct VertexProperty {
    pub id: Box<GremlinValue>, // needs refinment
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

    fn get_partial_len(bytes: &[u8]) -> Result<usize, crate::error::DecodeError> {
        let mut len = GremlinValue::get_len(bytes)?;
        len += String::get_partial_len(&bytes[len..])?;
        len += GremlinValue::get_len(&bytes[len..])?;
        len += Option::<Vertex>::get_len(&bytes[len..])?;
        len += Option::<Vec<Property>>::get_len(&bytes[len..])?;

        Ok(len)
    }
}

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

impl DecodeGraphSON for VertexProperty {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let value_object = j_val
            .as_object()
            .filter(|map| validate_type_entry(*map, "g:VertexProperty"))
            .and_then(|m| m.get("@value"))
            .and_then(|m| m.as_object());

        let id = val_by_key_v3!(value_object, "id", GremlinValue, "VertexProperty")?;
        let label = val_by_key_v3!(value_object, "label", String, "VertexProperty")?;
        let value = val_by_key_v3!(value_object, "value", GremlinValue, "VertexProperty")?;

        let properties = value_object
            .and_then(|value_object| value_object.get("properties"))
            .and_then(|prop_obj| prop_obj.as_object())
            .map(|map| map.iter());

        let mut properties_option = None;

        if let Some(iter) = properties {
            let mut v_properties = Vec::<Property>::new();
            for (key, value) in iter {
                v_properties.push(Property {
                    key: key.clone(),
                    value: Box::new(GremlinValue::decode_v3(value)?),
                    parent: EitherParent::None,
                })
            }
            properties_option = Some(v_properties)
        }

        Ok(VertexProperty {
            id: Box::new(id),
            label,
            value: Box::new(value),
            parent: None,
            properties: properties_option,
        })
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let object = j_val
            .as_object()
            .filter(|map| validate_type_entry(*map, "g:VertexProperty"))
            .and_then(|m| m.get("@value"))
            .and_then(|m| m.as_object());

        let id = val_by_key_v2!(object, "id", GremlinValue, "VertexProperty")?;
        let label = val_by_key_v2!(object, "label", String, "VertexProperty")?;
        let value = val_by_key_v2!(object, "value", GremlinValue, "VertexProperty")?;
        let vertex_id = val_by_key_v2!(object, "vertex", GremlinValue, "VertexProperty")?;

        let properties = object
            .and_then(|m| m.get("properties"))
            .and_then(|o| o.as_object())
            .map(|map| map.iter());

        let mut properties_opt = None;
        let mut v_properties = Vec::<Property>::new();

        if let Some(iter) = properties {
            for (k, v) in iter {
                v_properties.push(Property {
                    key: k.clone(),
                    value: Box::new(GremlinValue::decode_v3(v)?),
                    parent: EitherParent::None,
                })
            }
            properties_opt = Some(v_properties)
        }

        Ok(VertexProperty {
            id: Box::new(id),
            label,
            value: Box::new(value),
            parent: Some(Vertex {
                id: Box::new(vertex_id),
                label: Default::default(),
                properties: None,
            }),
            properties: properties_opt,
        })
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

struct_de_serialize!((VertexProperty, VertexVertexProperty, 32));
conversion!(VertexProperty, VertexProperty);
