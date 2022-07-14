use std::fmt::Display;

use serde_json::json;

use super::validate_type_entry;
use super::{edge::Edge, vertex_property::VertexProperty};
use crate::{
    conversions,
    error::DecodeError,
    get_val_v1, get_val_v2, get_val_v3,
    graph_binary::{Decode, Encode, GremlinTypes, ValueFlag},
    graphson::{DecodeGraphSON, EncodeGraphSON},
    specs::{self, CoreType},
    struct_de_serialize,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Property {
    pub key: String,
    pub value: Box<GremlinTypes>,
    pub parent: EitherParent,
}

impl Property {
    pub fn new(key: &str, value: impl Into<GremlinTypes>, parent: EitherParent) -> Self {
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
            EitherParent::None => GremlinTypes::UnspecifiedNullObject.encode(writer),
        }
    }
}

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

impl Decode for EitherParent {
    fn expected_type_code() -> u8 {
        todo!()
    }

    fn partial_decode<R: std::io::Read>(_reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        unimplemented!()
    }

    fn get_partial_len(_bytes: &[u8]) -> Result<usize, crate::error::DecodeError> {
        unimplemented!()
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

impl Decode for Property {
    fn expected_type_code() -> u8 {
        CoreType::Property.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let key = String::partial_decode(reader)?;
        let value = Box::new(GremlinTypes::decode(reader)?);
        let parent = EitherParent::decode(reader)?;

        Ok(Property { key, value, parent })
    }

    fn get_partial_len(bytes: &[u8]) -> Result<usize, crate::error::DecodeError> {
        let mut len = String::get_partial_len(bytes)?;
        len += GremlinTypes::get_len(&bytes[len..])?;
        len += GremlinTypes::get_len(&bytes[len..])?; //FIXME
        Ok(len)
    }
}

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
            "value" : self.value.encode_v3(),
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

impl DecodeGraphSON for Property {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let object = j_val
            .as_object()
            .filter(|map| validate_type_entry(*map, "g:Property"))
            .and_then(|map| map.get("@value"))
            .and_then(|v| v.as_object());

        let key = get_val_v3!(object, "key", String, "Property")?;
        let value = get_val_v3!(object, "value", GremlinTypes, "Property")?;

        Ok(Property {
            key,
            value: Box::new(value),
            parent: EitherParent::None,
        })
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let object = j_val
            .as_object()
            .filter(|map| validate_type_entry(*map, "g:Property"))
            .and_then(|map| map.get("@value"))
            .and_then(|v| v.as_object());

        let key = get_val_v2!(object, "key", String, "Property")?;
        let value = get_val_v2!(object, "value", GremlinTypes, "Property")?;

        let parent = get_val_v2!(object, "element", EitherParent, "Property")?;

        Ok(Property {
            key,
            value: Box::new(value),
            parent,
        })
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let object = j_val.as_object();
        let key = get_val_v1!(object, "key", String, "Property")?;
        let value = get_val_v1!(object, "value", GremlinTypes, "Property")?;

        Ok(Property {
            key,
            value: Box::new(value),
            parent: EitherParent::None,
        })
    }
}

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

impl DecodeGraphSON for EitherParent {
    fn decode_v3(_j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        unimplemented!()
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        if j_val
            .as_object()
            .filter(|map| validate_type_entry(*map, "g:Edge"))
            .is_some()
        {
            let obj = j_val.get("@value");
            let id = get_val_v2!(obj, "id", GremlinTypes, "EitherParent")?;
            let label = get_val_v2!(obj, "label", String, "EitherParent")?;
            let out_v_id = get_val_v2!(obj, "outV", GremlinTypes, "EitherParent")?;
            let in_v_id = get_val_v2!(obj, "inV", GremlinTypes, "EitherParent")?;

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
        } else if j_val
            .as_object()
            .filter(|map| validate_type_entry(*map, "g:VertexProperty"))
            .is_some()
        {
            // Not sure what VertexProptery looks like
            // let obj = j_val.get("@value");
            // let id = get_val_v2!(obj, "id", GremlinTypes, "EitherParent")?;
            // let label = get_val_v2!(obj, "label", String, "EitherParent")?;
            // let out_v_id = get_val_v2!(obj, "outV", GremlinTypes, "EitherParent")?;
            // let in_v_id = get_val_v2!(obj, "inV", GremlinTypes, "EitherParent")?;
            todo!("")
        } else {
            Err(DecodeError::DecodeError(
                "Property Parent/Element not found in decode Property v2".to_string(),
            ))
        }
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        unimplemented!()
    }
}

struct_de_serialize!((Property, PropertyVisitor, 32));
conversions!((Property, Property));

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
