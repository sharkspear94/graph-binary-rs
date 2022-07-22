use std::{collections::HashMap, fmt::Display};

use crate::error::DecodeError;
use crate::{conversion, specs::CoreType, GremlinValue};

#[cfg(feature = "graph_binary")]
use crate::graph_binary::{Decode, Encode};

#[cfg(feature = "graph_son")]
use crate::{
    graphson::{validate_type_entry, DecodeGraphSON, EncodeGraphSON},
    val_by_key_v2, val_by_key_v3,
};
#[cfg(feature = "graph_son")]
use serde_json::json;

#[derive(Debug, PartialEq, Clone)]
pub struct Traverser {
    pub bulk: i64,
    pub value: Box<GremlinValue>,
}

impl Display for Traverser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "bulk:{},{}", self.bulk, self.value)
    }
}

pub struct TraverserIter<'a> {
    bulk: usize,
    val: &'a GremlinValue,
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
    type Item = &'a GremlinValue;
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
    val: GremlinValue,
}

impl Iterator for IntoTraverserIter {
    type Item = GremlinValue;
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
    type Item = GremlinValue;

    type IntoIter = IntoTraverserIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoTraverserIter {
            bulk: self.bulk as usize,
            val: *self.value,
        }
    }
}

#[cfg(feature = "graph_binary")]
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

#[cfg(feature = "graph_binary")]
impl Decode for Traverser {
    fn expected_type_code() -> u8 {
        CoreType::Traverser.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let bulk = i64::partial_decode(reader)?;
        let value = Box::new(GremlinValue::decode(reader)?);

        Ok(Traverser { bulk, value })
    }
}

#[cfg(feature = "graph_son")]
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

#[cfg(feature = "graph_son")]
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
        let value = val_by_key_v3!(object, "value", GremlinValue, "Traverser")?;

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
        let value = val_by_key_v2!(object, "value", GremlinValue, "Traverser")?;

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
    pub configuration: HashMap<String, GremlinValue>, // not sure if key is correct
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

#[cfg(feature = "graph_binary")]
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

#[cfg(feature = "graph_binary")]
impl Decode for TraversalStrategy {
    fn expected_type_code() -> u8 {
        CoreType::TraversalStrategy.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let strategy_class = String::partial_decode(reader)?;
        let configuration = HashMap::<String, GremlinValue>::partial_decode(reader)?;

        Ok(TraversalStrategy {
            strategy_class,
            configuration,
        })
    }
}

conversion!(Traverser, Traverser);
conversion!(TraversalStrategy, TraversalStrategy);

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
    use super::property::{EitherParent, Property};
    use super::{vertex::Vertex, vertex_property::VertexProperty};

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
    use super::property::{EitherParent, Property};
    use super::{vertex::Vertex, vertex_property::VertexProperty};

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
        .get_ref_mut::<Vertex>()
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
        .get_ref_mut::<Vertex>()
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
    use super::property::{EitherParent, Property};
    use super::{vertex::Vertex, vertex_property::VertexProperty};

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
        .get_ref_mut::<Vertex>()
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
        .get_ref_mut::<Vertex>()
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
