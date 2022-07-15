use std::{collections::HashMap, fmt::Display};

use serde_json::json;

use crate::{
    conversions,
    error::DecodeError,
    graph_binary::{Decode, Encode, GremlinTypes},
    graphson::{DecodeGraphSON, EncodeGraphSON},
    specs::{self, CoreType},
    struct_de_serialize,
    structure::property::{EitherParent, Property},
    val_by_key_v2, val_by_key_v3,
};

use super::{validate_type_entry, vertex_property::VertexProperty};

#[derive(Debug, PartialEq, Clone)]
pub struct Vertex {
    pub id: Box<GremlinTypes>,
    pub label: String,
    pub properties: Option<Vec<VertexProperty>>,
}

impl Vertex {
    pub fn new<ID: Into<GremlinTypes>>(
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

impl Encode for Vertex {
    fn type_code() -> u8 {
        specs::CoreType::Vertex.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.id.encode(writer)?;
        self.label.partial_encode(writer)?;
        self.properties.encode(writer)
    }
}

impl Decode for Vertex {
    fn expected_type_code() -> u8 {
        CoreType::Vertex.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let id = Box::new(GremlinTypes::decode(reader)?);
        let label = String::partial_decode(reader)?;
        let properties = Option::<Vec<VertexProperty>>::decode(reader)?;

        Ok(Vertex {
            id,
            label,
            properties,
        })
    }

    fn get_partial_len(bytes: &[u8]) -> Result<usize, DecodeError> {
        let mut len = GremlinTypes::get_len(bytes)?;
        len += String::get_partial_len(&bytes[len..])?;
        len += GremlinTypes::get_len(&bytes[len..])?;
        Ok(len)
    }
}

impl EncodeGraphSON for Vertex {
    fn encode_v3(&self) -> serde_json::Value {
        if let Some(properties) = &self.properties {
            let mut map = HashMap::<String, Vec<serde_json::Value>>::new();
            for property in properties {
                if map.contains_key(&property.label) {
                    let v = map.get_mut(&property.label).unwrap();
                    v.push(property.encode_v3())
                } else {
                    map.insert(property.label.clone(), vec![property.encode_v3()]);
                }
            }
            json!({
                  "@type" : "g:Vertex",
                  "@value" : {
                    "id" : self.id.encode_v3(),
                    "label" : self.label,
                    "properties" : map
            }})
        } else {
            json!({
                  "@type" : "g:Vertex",
                  "@value" : {
                    "id" : self.id.encode_v3(),
                    "label" : self.label,
            }})
        }
    }

    fn encode_v2(&self) -> serde_json::Value {
        if let Some(properties) = &self.properties {
            let mut map = HashMap::<String, Vec<serde_json::Value>>::new();
            for property in properties {
                if map.contains_key(&property.label) {
                    let v = map.get_mut(&property.label).unwrap();
                    v.push(property.encode_v2())
                } else {
                    map.insert(property.label.clone(), vec![property.encode_v2()]);
                }
            }
            json!({
                  "@type" : "g:Vertex",
                  "@value" : {
                    "id" : self.id.encode_v2(),
                    "label" : self.label,
                    "properties" : map
            }})
        } else {
            json!({
                  "@type" : "g:Vertex",
                  "@value" : {
                    "id" : self.id.encode_v2(),
                    "label" : self.label,
            }})
        }
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

impl DecodeGraphSON for Vertex {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let type_value_object = j_val
            .as_object()
            .filter(|map| validate_type_entry(*map, "g:Vertex"))
            .and_then(|map| map.get("@value"))
            .and_then(|o| o.as_object());

        let id = val_by_key_v3!(type_value_object, "id", GremlinTypes, "Vertex")?;

        let label = val_by_key_v3!(type_value_object, "label", String, "Vertex")?;

        let mut properties = None;
        if let Some(jvalues) = type_value_object
            .and_then(|obj| obj.get("properties"))
            .and_then(|obj| obj.as_object())
            .map(|map| {
                map.values()
                    .flat_map(|val| val.as_array().into_iter())
                    .flatten()
            })
        {
            let mut vertex_properties = if let (_, Some(size)) = jvalues.size_hint() {
                Vec::with_capacity(size)
            } else {
                Vec::new()
            };
            for i in jvalues {
                vertex_properties.push(VertexProperty::decode_v3(i)?);
            }
            properties = Some(vertex_properties);
        }

        Ok(Vertex {
            id: Box::new(id),
            label,
            properties,
        })
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let type_value_object = j_val
            .as_object()
            .filter(|map| validate_type_entry(*map, "g:Vertex"))
            .and_then(|map| map.get("@value"));

        let id = val_by_key_v2!(type_value_object, "id", GremlinTypes, "Vertex")?;
        let label = val_by_key_v2!(type_value_object, "label", String, "Vertex")?;

        let mut properties = None;
        if let Some(jvalues) = type_value_object
            .and_then(|obj| obj.get("properties"))
            .and_then(|obj| obj.as_object())
            .map(|map| {
                map.values()
                    .flat_map(|val| val.as_array().into_iter())
                    .flatten()
            })
        {
            let mut vertex_properties = if let (_, Some(size)) = jvalues.size_hint() {
                Vec::with_capacity(size)
            } else {
                Vec::new()
            };
            for i in jvalues {
                vertex_properties.push(VertexProperty::decode_v2(i)?);
            }
            properties = Some(vertex_properties);
        }

        Ok(Vertex {
            id: Box::new(id),
            label,
            properties,
        })
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

struct_de_serialize!((Vertex, VertexVisitor, 32));
conversions!((Vertex, Vertex));

#[test]
fn test_vertex_none_encode() {
    let expected = [
        0x11_u8, 0x0, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x6, 0x70,
        0x65, 0x72, 0x73, 0x6f, 0x6e, 0xfe, 0x1,
    ];
    let v = Vertex {
        id: Box::new(1_i64.into()),
        label: String::from("person"),
        properties: None,
    };
    let mut buf = Vec::new();
    let v = v.encode(&mut buf);
    assert!(v.is_ok());
    assert_eq!(expected, buf[..])
}

#[test]
fn test_vertex_decode_none() {
    let reader = vec![
        0x11_u8, 0x0, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x6, 0x70,
        0x65, 0x72, 0x73, 0x6f, 0x6e, 0xfe, 0x1,
    ];

    let v = Vertex::decode(&mut &reader[..]);
    assert!(v.is_ok());

    let expected = Vertex {
        id: Box::new(1_i64.into()),
        label: String::from("person"),
        properties: None,
    };

    assert_eq!(expected, v.unwrap())
}

#[test]
fn test_vertex_consume() {
    let reader = vec![
        0x11_u8, 0x0, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x6, 0x70,
        0x65, 0x72, 0x73, 0x6f, 0x6e, 0xfe, 0x01,
    ];

    let size = Vertex::get_len(&reader).unwrap();

    assert_eq!(reader.len(), size)
}

#[test]
fn test_vertex_v3_encode() {
    let v = Vertex {
        id: Box::new(1_i32.into()),
        label: String::from("person"),
        properties: Some(vec![
            VertexProperty {
                id: Box::new(0i64.into()),
                label: "name".into(),
                value: Box::new("marko".into()),
                parent: None,
                properties: None,
            },
            VertexProperty {
                id: Box::new(8i64.into()),
                label: "location".into(),
                value: Box::new("brussels".into()),
                parent: None,
                properties: Some(vec![
                    Property {
                        key: "startTime".into(),
                        value: Box::new(2004.into()),
                        parent: EitherParent::None,
                    },
                    Property {
                        key: "endTime".into(),
                        value: Box::new(2005.into()),
                        parent: EitherParent::None,
                    },
                ]),
            },
            VertexProperty {
                id: Box::new(6i64.into()),
                label: "location".into(),
                value: Box::new("san diego".into()),
                parent: None,
                properties: Some(vec![
                    Property {
                        key: "startTime".into(),
                        value: Box::new(1997.into()),
                        parent: EitherParent::None,
                    },
                    Property {
                        key: "endTime".into(),
                        value: Box::new(2001.into()),
                        parent: EitherParent::None,
                    },
                ]),
            },
        ]),
    };
    let v = v.encode_v3();
    println!("{}", serde_json::to_string_pretty(&v).unwrap());
}

#[test]
fn decode_v3() {
    let str = r#"{
        "@type" : "g:Vertex",
        "@value" : {
          "id" : {
            "@type" : "g:Int32",
            "@value" : 1
          },
          "label" : "person",
          "properties" : {
            "name" : [ {
              "@type" : "g:VertexProperty",
              "@value" : {
                "id" : {
                  "@type" : "g:Int64",
                  "@value" : 0
                },
                "value" : "marko",
                "label" : "name"
              }
            } ],
            "location" : [ {
              "@type" : "g:VertexProperty",
              "@value" : {
                "id" : {
                  "@type" : "g:Int64",
                  "@value" : 6
                },
                "value" : "san diego",
                "label" : "location",
                "properties" : {
                  "startTime" : {
                    "@type" : "g:Int32",
                    "@value" : 1997
                  },
                  "endTime" : {
                    "@type" : "g:Int32",
                    "@value" : 2001
                  }
                }
              }
            }, {
              "@type" : "g:VertexProperty",
              "@value" : {
                "id" : {
                  "@type" : "g:Int64",
                  "@value" : 7
                },
                "value" : "santa cruz",
                "label" : "location",
                "properties" : {
                  "startTime" : {
                    "@type" : "g:Int32",
                    "@value" : 2001
                  },
                  "endTime" : {
                    "@type" : "g:Int32",
                    "@value" : 2004
                  }
                }
              }
            }, {
              "@type" : "g:VertexProperty",
              "@value" : {
                "id" : {
                  "@type" : "g:Int64",
                  "@value" : 8
                },
                "value" : "brussels",
                "label" : "location",
                "properties" : {
                  "startTime" : {
                    "@type" : "g:Int32",
                    "@value" : 2004
                  },
                  "endTime" : {
                    "@type" : "g:Int32",
                    "@value" : 2005
                  }
                }
              }
            }, {
              "@type" : "g:VertexProperty",
              "@value" : {
                "id" : {
                  "@type" : "g:Int64",
                  "@value" : 9
                },
                "value" : "santa fe",
                "label" : "location",
                "properties" : {
                  "startTime" : {
                    "@type" : "g:Int32",
                    "@value" : 2005
                  }
                }
              }
            } ]
          }
        }
      }"#;

    let expected = Vertex {
        id: Box::new(1_i32.into()),
        label: String::from("person"),
        properties: Some(vec![
            VertexProperty::new(0i64, "name", "marko", None, None),
            VertexProperty::new(
                6i64,
                "location",
                "san diego",
                None,
                Some(vec![
                    Property::new("startTime", 1997, EitherParent::None),
                    Property::new("endTime", 2001, EitherParent::None),
                ]),
            ),
            VertexProperty::new(
                7i64,
                "location",
                "santa cruz",
                None,
                Some(vec![
                    Property::new("startTime", 2001, EitherParent::None),
                    Property::new("endTime", 2004, EitherParent::None),
                ]),
            ),
            VertexProperty::new(
                8i64,
                "location",
                "brussels",
                None,
                Some(vec![
                    Property::new("startTime", 2004, EitherParent::None),
                    Property::new("endTime", 2005, EitherParent::None),
                ]),
            ),
            VertexProperty::new(
                9i64,
                "location",
                "santa fe",
                None,
                Some(vec![Property::new("startTime", 2005, EitherParent::None)]),
            ),
        ]),
    };

    let value = serde_json::from_str(str).unwrap();
    let mut v = Vertex::decode_v3(&value).unwrap();
    for p in v.properties.as_mut() {
        for i in p {
            if i.properties.is_some() {
                i.properties
                    .as_mut()
                    .unwrap()
                    .sort_by(|p1, p2| p1.key.cmp(&p2.key).reverse())
            }
        }
    }

    v.properties.as_mut().unwrap().sort_by(|p1, p2| {
        p1.id
            .get_ref::<i64>()
            .unwrap()
            .cmp(p2.id.get_ref::<i64>().unwrap())
    });
    assert_eq!(v, expected)
}

#[test]
fn decode_v3_without_props() {
    let str = r#"{
        "@type" : "g:Vertex",
        "@value" : {
          "id" : {
            "@type" : "g:Int32",
            "@value" : 1
          },
          "label" : "person"
        }
      }"#;

    let expected = Vertex {
        id: Box::new(1_i32.into()),
        label: String::from("person"),
        properties: None,
    };

    let value = serde_json::from_str(str).unwrap();
    let mut v = Vertex::decode_v3(&value).unwrap();
    assert_eq!(v, expected)
}
#[test]
fn test_vertex_v2_encode() {
    let v = Vertex {
        id: Box::new(1_i32.into()),
        label: String::from("person"),
        properties: Some(vec![
            VertexProperty {
                id: Box::new(0i64.into()),
                label: "name".into(),
                value: Box::new("marko".into()),
                parent: None,
                properties: None,
            },
            VertexProperty {
                id: Box::new(8i64.into()),
                label: "location".into(),
                value: Box::new("brussels".into()),
                parent: None,
                properties: Some(vec![
                    Property {
                        key: "startTime".into(),
                        value: Box::new(2004.into()),
                        parent: EitherParent::None,
                    },
                    Property {
                        key: "endTime".into(),
                        value: Box::new(2005.into()),
                        parent: EitherParent::None,
                    },
                ]),
            },
            VertexProperty {
                id: Box::new(6i64.into()),
                label: "location".into(),
                value: Box::new("san diego".into()),
                parent: None,
                properties: Some(vec![
                    Property {
                        key: "startTime".into(),
                        value: Box::new(1997.into()),
                        parent: EitherParent::None,
                    },
                    Property {
                        key: "endTime".into(),
                        value: Box::new(2001.into()),
                        parent: EitherParent::None,
                    },
                ]),
            },
        ]),
    };
    let v = v.encode_v2();
    println!("{}", serde_json::to_string_pretty(&v).unwrap());
}

#[test]
fn decode_v2() {
    let str = r#"{
        "@type" : "g:Vertex",
        "@value" : {
          "id" : {
            "@type" : "g:Int32",
            "@value" : 1
          },
          "label" : "person",
          "properties" : {
            "name" : [ {
              "@type" : "g:VertexProperty",
              "@value" : {
                "id" : {
                  "@type" : "g:Int64",
                  "@value" : 0
                },
                "value" : "marko",
                "vertex" : {
                  "@type" : "g:Int32",
                  "@value" : 1
                },
                "label" : "name"
              }
            } ],
            "location" : [ {
              "@type" : "g:VertexProperty",
              "@value" : {
                "id" : {
                  "@type" : "g:Int64",
                  "@value" : 6
                },
                "value" : "san diego",
                "vertex" : {
                  "@type" : "g:Int32",
                  "@value" : 1
                },
                "label" : "location",
                "properties" : {
                  "startTime" : {
                    "@type" : "g:Int32",
                    "@value" : 1997
                  },
                  "endTime" : {
                    "@type" : "g:Int32",
                    "@value" : 2001
                  }
                }
              }
            }, {
              "@type" : "g:VertexProperty",
              "@value" : {
                "id" : {
                  "@type" : "g:Int64",
                  "@value" : 7
                },
                "value" : "santa cruz",
                "vertex" : {
                  "@type" : "g:Int32",
                  "@value" : 1
                },
                "label" : "location",
                "properties" : {
                  "startTime" : {
                    "@type" : "g:Int32",
                    "@value" : 2001
                  },
                  "endTime" : {
                    "@type" : "g:Int32",
                    "@value" : 2004
                  }
                }
              }
            }, {
              "@type" : "g:VertexProperty",
              "@value" : {
                "id" : {
                  "@type" : "g:Int64",
                  "@value" : 8
                },
                "value" : "brussels",
                "vertex" : {
                  "@type" : "g:Int32",
                  "@value" : 1
                },
                "label" : "location",
                "properties" : {
                  "startTime" : {
                    "@type" : "g:Int32",
                    "@value" : 2004
                  },
                  "endTime" : {
                    "@type" : "g:Int32",
                    "@value" : 2005
                  }
                }
              }
            }, {
              "@type" : "g:VertexProperty",
              "@value" : {
                "id" : {
                  "@type" : "g:Int64",
                  "@value" : 9
                },
                "value" : "santa fe",
                "vertex" : {
                  "@type" : "g:Int32",
                  "@value" : 1
                },
                "label" : "location",
                "properties" : {
                  "startTime" : {
                    "@type" : "g:Int32",
                    "@value" : 2005
                  }
                }
              }
            } ]
          }
        }
      }"#;

    let expected = Vertex {
        id: Box::new(1_i32.into()),
        label: String::from("person"),
        properties: Some(vec![
            VertexProperty::new(0i64, "name", "marko", Some(Vertex::new(1, "", None)), None),
            VertexProperty::new(
                6i64,
                "location",
                "san diego",
                Some(Vertex::new(1, "", None)),
                Some(vec![
                    Property::new("startTime", 1997, EitherParent::None),
                    Property::new("endTime", 2001, EitherParent::None),
                ]),
            ),
            VertexProperty::new(
                7i64,
                "location",
                "santa cruz",
                Some(Vertex::new(1, "", None)),
                Some(vec![
                    Property::new("startTime", 2001, EitherParent::None),
                    Property::new("endTime", 2004, EitherParent::None),
                ]),
            ),
            VertexProperty::new(
                8i64,
                "location",
                "brussels",
                Some(Vertex::new(1, "", None)),
                Some(vec![
                    Property::new("startTime", 2004, EitherParent::None),
                    Property::new("endTime", 2005, EitherParent::None),
                ]),
            ),
            VertexProperty::new(
                9i64,
                "location",
                "santa fe",
                Some(Vertex::new(1, "", None)),
                Some(vec![Property::new("startTime", 2005, EitherParent::None)]),
            ),
        ]),
    };

    let value = serde_json::from_str(str).unwrap();
    let mut v = Vertex::decode_v2(&value).unwrap();
    for p in v.properties.as_mut() {
        for i in p {
            if i.properties.is_some() {
                i.properties
                    .as_mut()
                    .unwrap()
                    .sort_by(|p1, p2| p1.key.cmp(&p2.key).reverse())
            }
        }
    }

    v.properties.as_mut().unwrap().sort_by(|p1, p2| {
        p1.id
            .get_ref::<i64>()
            .unwrap()
            .cmp(p2.id.get_ref::<i64>().unwrap())
    });
    assert_eq!(v, expected)
}

#[test]
fn decode_v2_without_props() {
    let str = r#"{
        "@type" : "g:Vertex",
        "@value" : {
          "id" : {
            "@type" : "g:Int32",
            "@value" : 1
          },
          "label" : "person"
        }
      }"#;

    let expected = Vertex {
        id: Box::new(1_i32.into()),
        label: String::from("person"),
        properties: None,
    };

    let value = serde_json::from_str(str).unwrap();
    let v = Vertex::decode_v2(&value).unwrap();
    assert_eq!(v, expected)
}
