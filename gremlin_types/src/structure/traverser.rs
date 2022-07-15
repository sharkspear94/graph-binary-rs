use std::{
    collections::HashMap,
    fmt::{write, Display},
};

use serde_json::json;

use super::validate_type_entry;
use crate::{
    conversions,
    graph_binary::{Decode, Encode, GremlinTypes},
    graphson::{DecodeGraphSON, EncodeGraphSON},
    specs::CoreType,
    struct_de_serialize,
    structure::property::{EitherParent, Property},
    val_by_key_v2, val_by_key_v3,
};
use crate::{
    error::DecodeError,
    structure::{vertex::Vertex, vertex_property::VertexProperty},
};

#[derive(Debug, PartialEq, Clone)]
pub struct Traverser {
    pub bulk: i64,
    pub value: Box<GremlinTypes>,
}

impl Display for Traverser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "bulk:{},{}", self.bulk, self.value)
    }
}

pub struct TraverserIter<'a> {
    bulk: usize,
    val: &'a GremlinTypes,
}

impl Traverser {
    pub fn iter(&self) -> TraverserIter {
        TraverserIter {
            bulk: self.bulk as usize,
            val: &self.value,
        }
    }
}

impl<'a> Iterator for TraverserIter<'a> {
    type Item = &'a GremlinTypes;
    fn next(&mut self) -> Option<Self::Item> {
        if self.bulk > 0 {
            self.bulk -= 1;
            Some(self.val)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.bulk, Some(self.bulk))
    }
}

pub struct IntoTraverserIter {
    bulk: usize,
    val: GremlinTypes,
}

impl Iterator for IntoTraverserIter {
    type Item = GremlinTypes;
    fn next(&mut self) -> Option<Self::Item> {
        if self.bulk > 0 {
            self.bulk -= 1;
            Some(self.val.clone())
        } else {
            None
        }
    }
}

impl IntoIterator for Traverser {
    type Item = GremlinTypes;

    type IntoIter = IntoTraverserIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoTraverserIter {
            bulk: self.bulk as usize,
            val: *self.value,
        }
    }
}

impl Encode for Traverser {
    fn type_code() -> u8 {
        CoreType::Traverser.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.bulk.partial_encode(writer)?;
        self.value.encode(writer)
    }
}

impl Decode for Traverser {
    fn expected_type_code() -> u8 {
        CoreType::Traverser.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let bulk = i64::partial_decode(reader)?;
        let value = Box::new(GremlinTypes::decode(reader)?);

        Ok(Traverser { bulk, value })
    }

    fn get_partial_len(bytes: &[u8]) -> Result<usize, crate::error::DecodeError> {
        let mut len = i64::get_partial_len(bytes)?;
        len += GremlinTypes::get_len(&bytes[len..])?;
        Ok(len)
    }
}

impl EncodeGraphSON for Traverser {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
        "@type" : "g:Traverser",
        "@value" : {
          "bulk" : self.bulk.encode_v3(),
          "value": self.value.encode_v3()
        }})
    }

    fn encode_v2(&self) -> serde_json::Value {
        json!({
        "@type" : "g:Traverser",
        "@value" : {
          "bulk" : self.bulk.encode_v2(),
          "value": self.value.encode_v2()
        }})
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

impl DecodeGraphSON for Traverser {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let object = j_val
            .as_object()
            .filter(|map| validate_type_entry(*map, "g:Traverser"))
            .and_then(|map| map.get("@value"))
            .and_then(|v| v.as_object());

        let bulk = val_by_key_v3!(object, "bulk", i64, "Traverser")?;
        let value = val_by_key_v3!(object, "value", GremlinTypes, "Traverser")?;

        Ok(Traverser {
            bulk,
            value: Box::new(value),
        })
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let object = j_val
            .as_object()
            .filter(|map| validate_type_entry(*map, "g:Traverser"))
            .and_then(|map| map.get("@value"))
            .and_then(|v| v.as_object());

        let bulk = val_by_key_v2!(object, "bulk", i64, "Traverser")?;
        let value = val_by_key_v2!(object, "value", GremlinTypes, "Traverser")?;

        Ok(Traverser {
            bulk,
            value: Box::new(value),
        })
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TraversalStrategy {
    pub strategy_class: String,                       // class
    pub configuration: HashMap<String, GremlinTypes>, // not sure if key is correct
}

impl Display for TraversalStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "class:{},config:[", self.strategy_class)?;
        for (key, val) in &self.configuration {
            write!(f, "({key}:{val}),")?;
        }
        write!(f, "]")
    }
}

impl Encode for TraversalStrategy {
    fn type_code() -> u8 {
        CoreType::TraversalStrategy.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.strategy_class.partial_encode(writer)?;
        self.configuration.partial_encode(writer)
    }
}

impl Decode for TraversalStrategy {
    fn expected_type_code() -> u8 {
        CoreType::TraversalStrategy.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let strategy_class = String::partial_decode(reader)?;
        let configuration = HashMap::<String, GremlinTypes>::partial_decode(reader)?;

        Ok(TraversalStrategy {
            strategy_class,
            configuration,
        })
    }

    fn get_partial_len(bytes: &[u8]) -> Result<usize, crate::error::DecodeError> {
        let mut len = String::get_partial_len(bytes)?;
        len += HashMap::<String, GremlinTypes>::get_partial_len(&bytes[len..])?;
        Ok(len)
    }
}

struct_de_serialize!(
    (Traverser, TraverserVisitor, 32),
    (TraversalStrategy, TraversalStrategyVisitor, 32)
);
conversions!(
    (Traverser, Traverser),
    (TraversalStrategy, TraversalStrategy)
);

#[test]
fn encode_traverser() {
    let expected = [
        0x21, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x3, 0x03, 0x0, 0x0, 0x0, 0x0, 0x3, b'a',
        b'b', b'c',
    ];

    let t = Traverser {
        bulk: 3,
        value: Box::new("abc".into()),
    };
    let mut writer = Vec::<u8>::new();
    t.encode(&mut writer).unwrap();
    assert_eq!(expected, &writer[..])
}

#[test]
fn decode_traverser() {
    let reader = vec![
        0x21, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x3, 0x03, 0x0, 0x0, 0x0, 0x0, 0x3, b'a',
        b'b', b'c',
    ];

    let expected = Traverser {
        bulk: 3,
        value: Box::new("abc".into()),
    };

    assert_eq!(expected, Traverser::decode(&mut &reader[..]).unwrap())
}

#[test]
fn test_iter() {
    let t = Traverser {
        bulk: 3,
        value: Box::new(1.into()),
    };
    let mut iter = t.iter();
    assert_eq!(iter.next(), Some(&1.into()));
    assert_eq!(iter.next(), Some(&1.into()));
    assert_eq!(iter.next(), Some(&1.into()));
    assert_eq!(iter.next(), None)
}
#[test]
fn encode_v3() {
    let expected = r#"{"@type":"g:Traverser","@value":{"bulk":{"@type":"g:Int64","@value":1},"value":{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":1},"label":"person","properties":{"name":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":0},"value":"marko","label":"name"}}],"location":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":6},"value":"san diego","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":1997},"endTime":{"@type":"g:Int32","@value":2001}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":7},"value":"santa cruz","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2001},"endTime":{"@type":"g:Int32","@value":2004}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":8},"value":"brussels","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2004},"endTime":{"@type":"g:Int32","@value":2005}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":9},"value":"santa fe","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2005}}}}]}}}}}"#;
    let t = Traverser {
        bulk: 1,
        value: Box::new(
            Vertex::new(
                1,
                "person",
                Some(vec![
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
            )
            .into(),
        ),
    };
    let s = t.encode_v3();
    let res: serde_json::Value = serde_json::from_str(expected).unwrap();
    assert_eq!(s, res)
}

#[test]
fn decode_v3() {
    let s = r#"{"@type":"g:Traverser","@value":{"bulk":{"@type":"g:Int64","@value":1},"value":{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":1},"label":"person","properties":{"name":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":0},"value":"marko","label":"name"}}],"location":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":6},"value":"san diego","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":1997},"endTime":{"@type":"g:Int32","@value":2001}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":7},"value":"santa cruz","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2001},"endTime":{"@type":"g:Int32","@value":2004}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":8},"value":"brussels","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2004},"endTime":{"@type":"g:Int32","@value":2005}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":9},"value":"santa fe","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2005}}}}]}}}}}"#;
    let expected = Traverser {
        bulk: 1,
        value: Box::new(
            Vertex::new(
                1,
                "person",
                Some(vec![
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
            )
            .into(),
        ),
    };

    let res: serde_json::Value = serde_json::from_str(s).unwrap();
    let mut res = Traverser::decode_v3(&res).unwrap();

    res.value
        .get_mut_ref::<Vertex>()
        .unwrap()
        .properties
        .as_mut()
        .into_iter()
        .for_each(|p| {
            for i in p {
                if i.properties.is_some() {
                    i.properties
                        .as_mut()
                        .unwrap()
                        .sort_by(|p1, p2| p1.key.cmp(&p2.key).reverse())
                }
            }
        });
    res.value
        .get_mut_ref::<Vertex>()
        .unwrap()
        .properties
        .as_mut()
        .unwrap()
        .sort_by(|p1, p2| {
            p1.id
                .get_ref::<i64>()
                .unwrap()
                .cmp(p2.id.get_ref::<i64>().unwrap())
        });
    assert_eq!(res, expected)
}

#[test]
fn decode_v2() {
    let s = r#"{"@type":"g:Traverser","@value":{"bulk":{"@type":"g:Int64","@value":1},"value":{
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
      }}}"#;
    let expected = Traverser {
        bulk: 1,
        value: Box::new(
            Vertex {
                id: Box::new(1_i32.into()),
                label: String::from("person"),
                properties: Some(vec![
                    VertexProperty::new(
                        0i64,
                        "name",
                        "marko",
                        Some(Vertex::new(1, "", None)),
                        None,
                    ),
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
            }
            .into(),
        ),
    };

    let res: serde_json::Value = serde_json::from_str(s).unwrap();
    let mut res = Traverser::decode_v2(&res).unwrap();

    res.value
        .get_mut_ref::<Vertex>()
        .unwrap()
        .properties
        .as_mut()
        .into_iter()
        .for_each(|p| {
            for i in p {
                if i.properties.is_some() {
                    i.properties
                        .as_mut()
                        .unwrap()
                        .sort_by(|p1, p2| p1.key.cmp(&p2.key).reverse())
                }
            }
        });
    res.value
        .get_mut_ref::<Vertex>()
        .unwrap()
        .properties
        .as_mut()
        .unwrap()
        .sort_by(|p1, p2| {
            p1.id
                .get_ref::<i64>()
                .unwrap()
                .cmp(p2.id.get_ref::<i64>().unwrap())
        });
    assert_eq!(res, expected)
}

#[test]
fn iter() {
    let t = Traverser {
        bulk: 3,
        value: Box::new(1.into()),
    };
    let mut iter = t.iter();
    assert_eq!(iter.next(), Some(&1.into()));
    assert_eq!(iter.next(), Some(&1.into()));
    assert_eq!(iter.next(), Some(&1.into()));
    assert_eq!(iter.next(), None)
}
