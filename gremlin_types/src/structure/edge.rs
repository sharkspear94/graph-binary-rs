use std::fmt::Display;

#[cfg(any(feature = "graph_binary", feature = "graph_son"))]
use crate::error::DecodeError;
#[cfg(feature = "graph_binary")]
use crate::specs::{self, CoreType};
use crate::{conversion, GremlinValue};

use super::{
    property::{self, Property},
    vertex::Vertex,
};

#[cfg(feature = "graph_binary")]
use crate::graph_binary::{Decode, Encode};

#[cfg(feature = "graph_son")]
use crate::error::GraphSonError;
#[cfg(feature = "graph_son")]
use crate::graphson::{
    get_val_by_key_v2, get_val_by_key_v3, validate_type, DecodeGraphSON, EncodeGraphSON,
};

#[cfg(feature = "graph_son")]
use serde_json::{json, Map};

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

#[cfg(feature = "graph_binary")]
impl Encode for Edge {
    fn type_code() -> u8 {
        specs::CoreType::Edge.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.id.encode(writer)?;
        self.label.partial_encode(writer)?;
        self.in_v_id.encode(writer)?;
        self.in_v_label.partial_encode(writer)?;
        self.out_v_id.encode(writer)?;
        self.out_v_label.partial_encode(writer)?;
        self.parent.encode(writer)?;
        self.properties.encode(writer)
    }
}

#[cfg(feature = "graph_binary")]
impl Decode for Edge {
    fn expected_type_code() -> u8 {
        CoreType::Edge.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let id = GremlinValue::decode(reader)?;
        let label = String::partial_decode(reader)?;
        let in_v_id = GremlinValue::decode(reader)?;
        let in_v_label = String::partial_decode(reader)?;
        let out_v_id = GremlinValue::decode(reader)?;
        let out_v_label = String::partial_decode(reader)?;
        let parent = Option::<Vertex>::decode(reader)?;
        let properties = Option::<Vec<Property>>::decode(reader)?;

        Ok(Edge {
            id: Box::new(id),
            label,
            in_v_id: Box::new(in_v_id),
            in_v_label,
            out_v_id: Box::new(out_v_id),
            out_v_label,
            parent,
            properties,
        })
    }
}

#[cfg(feature = "graph_son")]
impl EncodeGraphSON for Edge {
    fn encode_v3(&self) -> serde_json::Value {
        let properties_map = self.properties.as_ref().map(|vec| {
            vec.iter()
                .map(|prop| (prop.key.clone(), prop.encode_v3()))
                .collect::<Map<String, serde_json::Value>>()
        });
        // needs testing

        let mut json_value = json!({
          "@type" : "g:Edge",
          "@value" : {
            "id" : self.id.encode_v3(),
            "label" : self.label,
            "inVLabel" : self.in_v_label,
            "outVLabel" : self.out_v_label,
            "inV" : self.in_v_id.encode_v3(),
            "outV" : self.out_v_id.encode_v3(),
          }
        });
        if let Some(properties_map) = properties_map {
            json_value["@value"]
                .as_object_mut()
                .unwrap()
                .insert("properties".to_string(), json! {properties_map});
        }
        json_value
    }

    fn encode_v2(&self) -> serde_json::Value {
        let properties_map = self.properties.as_ref().map(|vec| {
            vec.iter()
                .map(|prop| (prop.key.clone(), prop.value.encode_v2()))
                .collect::<Map<String, serde_json::Value>>()
        });

        let mut json_value = json!({
          "@type" : "g:Edge",
          "@value" : {
            "id" : self.id.encode_v2(),
            "label" : self.label,
            "inVLabel" : self.in_v_label,
            "outVLabel" : self.out_v_label,
            "inV" : self.in_v_id.encode_v2(),
            "outV" : self.out_v_id.encode_v2(),
          }
        });
        if let Some(properties_map) = properties_map {
            json_value["@value"]
                .as_object_mut()
                .unwrap()
                .insert("properties".to_string(), json! {properties_map});
        }
        json_value
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

#[cfg(feature = "graph_son")]
impl DecodeGraphSON for Edge {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:Edge")?;

        let id = get_val_by_key_v3(value_object, "id", "Edge")?;
        let label = get_val_by_key_v3(value_object, "label", "Edge")?;
        let in_v_id = get_val_by_key_v3(value_object, "inV", "Edge")?;
        let in_v_label = get_val_by_key_v3(value_object, "inVLabel", "Edge")?;
        let out_v_id = get_val_by_key_v3(value_object, "outV", "Edge")?;
        let out_v_label = get_val_by_key_v3(value_object, "outVLabel", "Edge")?;

        let properties = value_object
            .get("properties")
            .and_then(|map| map.as_object())
            .map(|map| {
                map.values()
                    .map(Property::decode_v3)
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;

        Ok(Edge {
            id: Box::new(id),
            label,
            in_v_id: Box::new(in_v_id),
            in_v_label,
            out_v_id: Box::new(out_v_id),
            out_v_label,
            parent: None,
            properties,
        })
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:Edge")?;

        let id = get_val_by_key_v2(value_object, "id", "Edge")?;
        let label = get_val_by_key_v2(value_object, "label", "Edge")?;
        let in_v_id = get_val_by_key_v2(value_object, "inV", "Edge")?;
        let in_v_label = get_val_by_key_v2(value_object, "inVLabel", "Edge")?;
        let out_v_id = get_val_by_key_v2(value_object, "outV", "Edge")?;
        let out_v_label = get_val_by_key_v2(value_object, "outVLabel", "Edge")?;

        let properties = value_object
            .get("properties")
            .and_then(|map| map.as_object())
            .map(|map| {
                map.iter()
                    .map(|(k, v)| {
                        GremlinValue::decode_v2(v)
                            .map(|g| Property::new(k, g, property::EitherParent::None))
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;

        Ok(Edge {
            id: Box::new(id),
            label,
            in_v_id: Box::new(in_v_id),
            in_v_label,
            out_v_id: Box::new(out_v_id),
            out_v_label,
            parent: None,
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

conversion!(Edge, Edge);

#[test]
fn edge_none_encode_gb() {
    let expected = [
        0xd_u8, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x9, 0x0, 0x0, 0x0, 0x7, 0x63, 0x72, 0x65, 0x61,
        0x74, 0x65, 0x64, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x3, 0x0, 0x0, 0x0, 0x8,
        0x73, 0x6f, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x1, 0x0, 0x0, 0x0, 0x6, 0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0xfe, 0x1, 0xfe, 0x1,
    ];

    let e = Edge {
        id: Box::new(9_i32.into()),
        label: "created".to_string(),
        in_v_id: Box::new(3_i64.into()),
        in_v_label: "software".to_string(),
        out_v_id: Box::new(1_i64.into()),
        out_v_label: "person".to_string(),
        parent: None,
        properties: None,
    };

    print!("{e}");

    let mut buf = Vec::new();
    let e = e.encode(&mut buf);
    assert!(e.is_ok());
    assert_eq!(expected, buf[..])
}

#[test]
fn edge_decode_gb() {
    let reader = vec![
        0xd, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x9, 0x0, 0x0, 0x0, 0x7, 0x63, 0x72, 0x65, 0x61, 0x74,
        0x65, 0x64, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x3, 0x0, 0x0, 0x0, 0x8, 0x73,
        0x6f, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1,
        0x0, 0x0, 0x0, 0x6, 0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0xfe, 0x1, 0xfe, 0x1,
    ];

    let p = Edge::decode(&mut &reader[..]);

    // assert!(p.is_ok());
    let expected = Edge {
        id: Box::new(9_i32.into()),
        label: "created".to_string(),
        in_v_id: Box::new(3_i64.into()),
        in_v_label: "software".to_string(),
        out_v_id: Box::new(1_i64.into()),
        out_v_label: "person".to_string(),
        parent: None,
        properties: None,
    };

    assert_eq!(expected, p.unwrap());
}

#[test]
fn encode_v3() {
    let e = Edge {
        id: Box::new(13.into()),
        label: "develops".to_string(),
        in_v_id: Box::new(10.into()),
        in_v_label: "software".to_string(),
        out_v_id: Box::new(1.into()),
        out_v_label: "person".to_string(),
        parent: None,
        properties: Some(vec![Property {
            key: "since".to_string(),
            value: Box::new(2009.into()),
            parent: property::EitherParent::None,
        }]),
    };

    let s = serde_json::to_string(&e.encode_v3()).unwrap();

    let expected = r#"{"@type":"g:Edge","@value":{"id":{"@type":"g:Int32","@value":13},"inV":{"@type":"g:Int32","@value":10},"inVLabel":"software","label":"develops","outV":{"@type":"g:Int32","@value":1},"outVLabel":"person","properties":{"since":{"@type":"g:Property","@value":{"key":"since","value":{"@type":"g:Int32","@value":2009}}}}}}"#;
    assert_eq!(s, expected)
}

#[test]
fn encode_v3_without_props() {
    let e = Edge {
        id: Box::new(13.into()),
        label: "develops".to_string(),
        in_v_id: Box::new(10.into()),
        in_v_label: "software".to_string(),
        out_v_id: Box::new(1.into()),
        out_v_label: "person".to_string(),
        parent: None,
        properties: None,
    };

    let s = serde_json::to_string(&e.encode_v3()).unwrap();

    let expected = r#"{"@type":"g:Edge","@value":{"id":{"@type":"g:Int32","@value":13},"inV":{"@type":"g:Int32","@value":10},"inVLabel":"software","label":"develops","outV":{"@type":"g:Int32","@value":1},"outVLabel":"person"}}"#;
    assert_eq!(s, expected)
}

#[test]
fn decode_v3() {
    let expected = Edge {
        id: Box::new(13.into()),
        label: "develops".to_string(),
        in_v_id: Box::new(10.into()),
        in_v_label: "software".to_string(),
        out_v_id: Box::new(1.into()),
        out_v_label: "person".to_string(),
        parent: None,
        properties: Some(vec![Property {
            key: "since".to_string(),
            value: Box::new(2009.into()),
            parent: property::EitherParent::None,
        }]),
    };

    let input = r#"{
        "@type" : "g:Edge",
        "@value" : {
          "id" : {
            "@type" : "g:Int32",
            "@value" : 13
          },
          "label" : "develops",
          "inVLabel" : "software",
          "outVLabel" : "person",
          "inV" : {
            "@type" : "g:Int32",
            "@value" : 10
          },
          "outV" : {
            "@type" : "g:Int32",
            "@value" : 1
          },
          "properties" : {
            "since" : {
              "@type" : "g:Property",
              "@value" : {
                "key" : "since",
                "value" : {
                  "@type" : "g:Int32",
                  "@value" : 2009
                }
              }
            }
          }
        }
      }"#;

    let v = serde_json::from_str(input).unwrap();
    let e = Edge::decode_v3(&v).unwrap();
    assert_eq!(e, expected)
}

#[test]
fn decode_v3_without_props() {
    let expected = Edge {
        id: Box::new(13.into()),
        label: "develops".to_string(),
        in_v_id: Box::new(10.into()),
        in_v_label: "software".to_string(),
        out_v_id: Box::new(1.into()),
        out_v_label: "person".to_string(),
        parent: None,
        properties: None,
    };

    let input = r#"{
        "@type" : "g:Edge",
        "@value" : {
          "id" : {
            "@type" : "g:Int32",
            "@value" : 13
          },
          "label" : "develops",
          "inVLabel" : "software",
          "outVLabel" : "person",
          "inV" : {
            "@type" : "g:Int32",
            "@value" : 10
          },
          "outV" : {
            "@type" : "g:Int32",
            "@value" : 1
          }
        }
      }"#;

    let v = serde_json::from_str(input).unwrap();
    let e = Edge::decode_v3(&v).unwrap();
    assert_eq!(e, expected)
}

#[test]
fn encode_v2() {
    let e = Edge {
        id: Box::new(13.into()),
        label: "develops".to_string(),
        in_v_id: Box::new(10.into()),
        in_v_label: "software".to_string(),
        out_v_id: Box::new(1.into()),
        out_v_label: "person".to_string(),
        parent: None,
        properties: Some(vec![Property {
            key: "since".to_string(),
            value: Box::new(2009.into()),
            parent: property::EitherParent::None,
        }]),
    };

    let s = serde_json::to_string(&e.encode_v2()).unwrap();

    let expected = r#"{"@type":"g:Edge","@value":{"id":{"@type":"g:Int32","@value":13},"inV":{"@type":"g:Int32","@value":10},"inVLabel":"software","label":"develops","outV":{"@type":"g:Int32","@value":1},"outVLabel":"person","properties":{"since":{"@type":"g:Int32","@value":2009}}}}"#;
    assert_eq!(s, expected)
}

#[test]
fn encode_v2_without_props() {
    let e = Edge {
        id: Box::new(13.into()),
        label: "develops".to_string(),
        in_v_id: Box::new(10.into()),
        in_v_label: "software".to_string(),
        out_v_id: Box::new(1.into()),
        out_v_label: "person".to_string(),
        parent: None,
        properties: None,
    };

    let s = serde_json::to_string(&e.encode_v2()).unwrap();

    let expected = r#"{"@type":"g:Edge","@value":{"id":{"@type":"g:Int32","@value":13},"inV":{"@type":"g:Int32","@value":10},"inVLabel":"software","label":"develops","outV":{"@type":"g:Int32","@value":1},"outVLabel":"person"}}"#;
    assert_eq!(s, expected)
}

#[test]
fn decode_v2() {
    let expected = Edge {
        id: Box::new(13.into()),
        label: "develops".to_string(),
        in_v_id: Box::new(10.into()),
        in_v_label: "software".to_string(),
        out_v_id: Box::new(1.into()),
        out_v_label: "person".to_string(),
        parent: None,
        properties: Some(vec![Property {
            key: "since".to_string(),
            value: Box::new(2009.into()),
            parent: property::EitherParent::None,
        }]),
    };

    let input = r#"{
        "@type" : "g:Edge",
        "@value" : {
          "id" : {
            "@type" : "g:Int32",
            "@value" : 13
          },
          "label" : "develops",
          "inVLabel" : "software",
          "outVLabel" : "person",
          "inV" : {
            "@type" : "g:Int32",
            "@value" : 10
          },
          "outV" : {
            "@type" : "g:Int32",
            "@value" : 1
          },
          "properties" : {
            "since" : {
              "@type" : "g:Int32",
              "@value" : 2009
            }
          }
        }
      }"#;

    let v = serde_json::from_str(input).unwrap();
    let e = Edge::decode_v2(&v).unwrap();
    assert_eq!(e, expected)
}

#[test]
fn decode_v2_without_props() {
    let expected = Edge {
        id: Box::new(13.into()),
        label: "develops".to_string(),
        in_v_id: Box::new(10.into()),
        in_v_label: "software".to_string(),
        out_v_id: Box::new(1.into()),
        out_v_label: "person".to_string(),
        parent: None,
        properties: None,
    };

    let input = r#"{
        "@type" : "g:Edge",
        "@value" : {
          "id" : {
            "@type" : "g:Int32",
            "@value" : 13
          },
          "label" : "develops",
          "inVLabel" : "software",
          "outVLabel" : "person",
          "inV" : {
            "@type" : "g:Int32",
            "@value" : 10
          },
          "outV" : {
            "@type" : "g:Int32",
            "@value" : 1
          }
        }
      }"#;

    let v = serde_json::from_str(input).unwrap();
    let e = Edge::decode_v2(&v).unwrap();
    assert_eq!(e, expected)
}
