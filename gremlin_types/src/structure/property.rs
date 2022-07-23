use std::fmt::Display;

use super::{edge::Edge, vertex_property::VertexProperty};
use crate::error::GraphSonError;
use crate::graphson::{get_val_by_key_v1, get_val_by_key_v2, get_val_by_key_v3, validate_type};
use crate::GremlinValue;
use crate::{
    conversion,
    error::DecodeError,
    specs::{self, CoreType},
};

#[cfg(feature = "graph_binary")]
use crate::graph_binary::{Decode, Encode, ValueFlag};

#[cfg(feature = "graph_son")]
use crate::{
    graphson::{validate_type_entry, DecodeGraphSON, EncodeGraphSON},
    val_by_key_v1, val_by_key_v2, val_by_key_v3,
};
#[cfg(feature = "graph_son")]
use serde_json::json;

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

#[cfg(feature = "graph_binary")]
impl Encode for EitherParent {
    fn type_code() -> u8 {
        unimplemented!()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        _writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        unimplemented!()
    }

    fn encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), crate::error::EncodeError> {
        match self {
            EitherParent::Edge(e) => e.encode(writer),
            EitherParent::VertexProperty(v) => v.encode(writer),
            EitherParent::None => GremlinValue::UnspecifiedNullObject.encode(writer),
        }
    }
}

#[cfg(feature = "graph_binary")]
impl Encode for Property {
    fn type_code() -> u8 {
        specs::CoreType::Property.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.key.partial_encode(writer)?;
        self.value.encode(writer)?;
        self.parent.encode(writer)
    }
}

#[cfg(feature = "graph_binary")]
impl Decode for EitherParent {
    fn expected_type_code() -> u8 {
        unreachable!()
    }

    fn partial_decode<R: std::io::Read>(_reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        unreachable!()
    }

    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let mut buf = [255_u8; 2];
        reader.read_exact(&mut buf)?;

        let identifier = CoreType::try_from(buf[0])?;
        let value_flag = ValueFlag::try_from(buf[1])?;

        match (identifier, value_flag) {
            (CoreType::Edge, ValueFlag::Set) => {
                Ok(EitherParent::Edge(Edge::partial_decode(reader)?))
            }
            (CoreType::VertexProperty, ValueFlag::Set) => Ok(EitherParent::VertexProperty(
                VertexProperty::partial_decode(reader)?,
            )),
            (CoreType::UnspecifiedNullObject, ValueFlag::Null) => Ok(EitherParent::None),
            (c, v) => Err(crate::error::DecodeError::DecodeError(format!(
                "EitherParent decode with Coretype {c:?} and Valueflag {v:?}"
            ))),
        }
    }
}

#[cfg(feature = "graph_binary")]
impl Decode for Property {
    fn expected_type_code() -> u8 {
        CoreType::Property.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let key = String::partial_decode(reader)?;
        let value = Box::new(GremlinValue::decode(reader)?);
        let parent = EitherParent::decode(reader)?;

        Ok(Property { key, value, parent })
    }
}

#[cfg(feature = "graph_son")]
impl EncodeGraphSON for Property {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
          "@type" : "g:Property",
          "@value" : {
            "key" : self.key,
            "value" : self.value.encode_v3()
          }
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        json!({
          "@type" : "g:Property",
          "@value" : {
            "key" : self.key,
            "value" : self.value.encode_v2(),
            "element" : self.parent.encode_v2()
          }
        })
    }

    fn encode_v1(&self) -> serde_json::Value {
        json!({
          "key" : self.key,
          "value" : self.value.encode_v1()
        })
    }
}

#[cfg(feature = "graph_son")]
impl DecodeGraphSON for Property {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:Property")?;

        let key = get_val_by_key_v3(value_object, "key", "Property")?;

        let value = get_val_by_key_v3(value_object, "value", "Property")?;

        Ok(Property {
            key,
            value: Box::new(value),
            parent: EitherParent::None,
        })
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:Property")?;

        let key = get_val_by_key_v2(value_object, "key", "Property")?;

        let value = get_val_by_key_v2(value_object, "value", "Property")?;

        let parent = get_val_by_key_v2(value_object, "element", "Property")?;

        Ok(Property {
            key,
            value: Box::new(value),
            parent,
        })
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let key = get_val_by_key_v1(j_val, "key", "Property")?;

        let value = get_val_by_key_v1(j_val, "value", "Property")?;

        Ok(Property {
            key,
            value: Box::new(value),
            parent: EitherParent::None,
        })
    }
}

#[cfg(feature = "graph_son")]
impl EncodeGraphSON for EitherParent {
    fn encode_v3(&self) -> serde_json::Value {
        unimplemented!()
    }

    fn encode_v2(&self) -> serde_json::Value {
        match self {
            EitherParent::Edge(e) => json!({
              "@type" : "g:Edge",
              "@value" : {
                "id" : e.id.encode_v2(),
                "label" : e.label.encode_v2(),
                "outV" : e.out_v_id.encode_v2(),
                "inV" : e.in_v_id.encode_v2()
              }
            }),
            EitherParent::VertexProperty(v) => v.encode_v2(),
            EitherParent::None => json!(null),
        }
    }

    fn encode_v1(&self) -> serde_json::Value {
        unimplemented!()
    }
}

#[cfg(feature = "graph_son")]
impl DecodeGraphSON for EitherParent {
    fn decode_v3(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        unimplemented!()
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        if let Ok(value_object) = validate_type(j_val, "g:Edge") {
            let id = get_val_by_key_v2(value_object, "id", "EitherParent")?;
            let label = get_val_by_key_v2(value_object, "label", "EitherParent")?;
            let out_v_id = get_val_by_key_v2(value_object, "outV", "EitherParent")?;
            let in_v_id = get_val_by_key_v2(value_object, "inV", "EitherParent")?;

            Ok(EitherParent::Edge(Edge {
                id: Box::new(id),
                label,
                in_v_id: Box::new(in_v_id),
                in_v_label: Default::default(),
                out_v_id: Box::new(out_v_id),
                out_v_label: Default::default(),
                parent: None,
                properties: None,
            }))
        } else if let Ok(value_object) = validate_type(j_val, "g:VertexProperty") {
            // Not sure what VertexProptery looks like
            // let obj = j_val.get("@value");
            // let id = get_val_v2!(obj, "id", GremlinTypes, "EitherParent")?;
            // let label = get_val_v2!(obj, "label", String, "EitherParent")?;
            // let out_v_id = get_val_v2!(obj, "outV", GremlinTypes, "EitherParent")?;
            // let in_v_id = get_val_v2!(obj, "inV", GremlinTypes, "EitherParent")?;
            todo!("")
        } else {
            Err(GraphSonError::KeyNotFound(
                "Edge or VertexProperty".to_string(),
            ))
        }
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        unimplemented!()
    }
}

conversion!(Property, Property);

#[test]
fn decode_v3() {
    let s = r#"{
        "@type" : "g:Property",
        "@value" : {
          "key" : "since",
          "value" : {
            "@type" : "g:Int32",
            "@value" : 2009
          }
        }
      }"#;

    let val = serde_json::from_str(s).unwrap();
    let p = Property::decode_v3(&val).unwrap();
    let expected = Property {
        key: "since".to_string(),
        value: Box::new(2009.into()),
        parent: EitherParent::None,
    };
    assert_eq!(p, expected)
}

#[test]
fn encode_v3() {
    let property = Property {
        key: "since".to_string(),
        value: Box::new(2009.into()),
        parent: EitherParent::None,
    };

    let res_str = serde_json::to_string(&property.encode_v3()).unwrap();
    let expeced = r#"{"@type":"g:Property","@value":{"key":"since","value":{"@type":"g:Int32","@value":2009}}}"#;
    assert_eq!(res_str, expeced)
}

#[test]
fn decode_v2() {
    let s = r#"{
        "@type" : "g:Property",
        "@value" : {
          "key" : "since",
          "value" : {
            "@type" : "g:Int32",
            "@value" : 2009
          },
          "element" : {
            "@type" : "g:Edge",
            "@value" : {
              "id" : {
                "@type" : "g:Int32",
                "@value" : 13
              },
              "label" : "develops",
              "outV" : {
                "@type" : "g:Int32",
                "@value" : 1
              },
              "inV" : {
                "@type" : "g:Int32",
                "@value" : 10
              }
            }
          }
        }
      }"#;

    let val = serde_json::from_str(s).unwrap();
    let p = Property::decode_v2(&val).unwrap();
    let expected = Property {
        key: "since".to_string(),
        value: Box::new(2009.into()),
        parent: EitherParent::Edge(Edge {
            id: Box::new(13.into()),
            label: "develops".to_string(),
            in_v_id: Box::new(10.into()),
            in_v_label: "".to_string(),
            out_v_id: Box::new(1.into()),
            out_v_label: Default::default(),
            parent: None,
            properties: None,
        }),
    };
    assert_eq!(p, expected);
}

#[test]
fn encode_v2() {
    let property = Property {
        key: "since".to_string(),
        value: Box::new(2009.into()),
        parent: EitherParent::Edge(Edge {
            id: Box::new(13.into()),
            label: "develops".to_string(),
            in_v_id: Box::new(10.into()),
            in_v_label: "".to_string(),
            out_v_id: Box::new(1.into()),
            out_v_label: Default::default(),
            parent: None,
            properties: None,
        }),
    };
    let res_str = serde_json::to_string(&property.encode_v2()).unwrap();
    let expected = r#"{"@type":"g:Property","@value":{"element":{"@type":"g:Edge","@value":{"id":{"@type":"g:Int32","@value":13},"inV":{"@type":"g:Int32","@value":10},"label":"develops","outV":{"@type":"g:Int32","@value":1}}},"key":"since","value":{"@type":"g:Int32","@value":2009}}}"#;
    assert_eq!(res_str, expected);
}
