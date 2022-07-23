use std::fmt::Display;

use crate::error::{DecodeError, GraphSonError};

use crate::graphson::{get_val_by_key_v2, get_val_by_key_v3, validate_type};
use crate::GremlinValue;
use crate::{conversion, specs::CoreType};

use super::list::Set;

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
pub struct Path {
    labels: Vec<Set<String>>,   // List<Set<String>>
    objects: Vec<GremlinValue>, // List<T>
}

#[cfg(feature = "graph_binary")]
impl Encode for Path {
    fn type_code() -> u8 {
        CoreType::Path.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        writer.write_all(&[CoreType::List.into(), 0x0])?;
        let len = i32::try_from(self.labels.len())?;
        len.partial_encode(writer)?;
        for set in &self.labels {
            writer.write_all(&[CoreType::Set.into(), 0x0])?;
            set.partial_encode(writer)?;
        }
        self.objects.encode(writer)
    }
}

#[cfg(feature = "graph_binary")]
impl Decode for Path {
    fn expected_type_code() -> u8 {
        CoreType::Path.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        reader.read_exact(&mut [0_u8, 0])?;
        let len = i32::partial_decode(reader)? as usize;
        let mut labels = Vec::with_capacity(len);
        for _ in 0..len {
            reader.read_exact(&mut [0_u8, 0])?;
            let set = Set::<String>::partial_decode(reader)?;
            labels.push(set);
        }
        let objects = Vec::<GremlinValue>::decode(reader)?;

        Ok(Path { labels, objects })
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (labels, object) in self.labels.iter().zip(&self.objects) {
            write!(f, "labels:[")?;
            if !labels.is_empty() {
                for label in &labels[..labels.len() - 1] {
                    write!(f, "{label},")?;
                }
                write!(f, "{}", labels.last().unwrap())?;
            }
            writeln!(f, "],object[{object}]")?;
        }
        Ok(())
    }
}

#[cfg(feature = "graph_son")]
impl EncodeGraphSON for Path {
    fn encode_v3(&self) -> serde_json::Value {
        json!(
            {
                "@type" : "g:Path",
                "@value" : {
                  "labels" : self.labels.encode_v3(),
                  "objects" : self.objects.encode_v3()
                }
            }
        )
    }
    fn encode_v2(&self) -> serde_json::Value {
        json!(
            {
                "@type" : "g:Path",
                "@value" : {
                  "labels" : self.labels.encode_v2(),
                  "objects" : self.objects.encode_v2()
                }
            }
        )
    }

    fn encode_v1(&self) -> serde_json::Value {
        json!({
            "labels": self.labels.encode_v1(),
            "objects" : self.objects.encode_v1(),
        })
    }
}

#[cfg(feature = "graph_son")]
impl DecodeGraphSON for Path {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:Path")?;

        let labels = get_val_by_key_v3(value_object, "labels", "Path")?;
        let objects = get_val_by_key_v3(value_object, "objects", "Path")?;

        Ok(Path { labels, objects })
    }
    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:Path")?;

        let labels = get_val_by_key_v2(value_object, "labels", "Path")?;
        let objects = get_val_by_key_v2(value_object, "objects", "Path")?;

        Ok(Path { labels, objects })
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

conversion!(Path, Path);

#[test]
fn test_encode() {
    let expected = [
        0xe_u8, 0x0, 0x9, 0x0, 0x0, 0x0, 0x0, 0x3, 0xb, 0x0, 0x0, 0x0, 0x0, 0x0, 0xb, 0x0, 0x0,
        0x0, 0x0, 0x0, 0xb, 0x0, 0x0, 0x0, 0x0, 0x0, 0x9, 0x0, 0x0, 0x0, 0x0, 0x3, 0x3, 0x0, 0x0,
        0x0, 0x0, 0x5, 0x6d, 0x61, 0x72, 0x6b, 0x6f, 0x1, 0x0, 0x0, 0x0, 0x0, 0x20, 0x3, 0x0, 0x0,
        0x0, 0x0, 0x6, 0x72, 0x69, 0x70, 0x70, 0x6c, 0x65,
    ];

    let path = Path {
        labels: vec![Set::new(vec![]), Set::new(vec![]), Set::new(vec![])],
        objects: vec!["marko".into(), 32_i32.into(), "ripple".into()],
    };
    let mut buf = Vec::new();
    path.encode(&mut buf).unwrap();

    assert_eq!(&expected[..], &buf)
}

#[test]
fn test_decode() {
    let expecetd = Path {
        labels: vec![Set::new(vec![]), Set::new(vec![]), Set::new(vec![])],
        objects: vec!["marko".into(), 32_i32.into(), "ripple".into()],
    };

    let buf = vec![
        0xe_u8, 0x0, 0x9, 0x0, 0x0, 0x0, 0x0, 0x3, 0xb, 0x0, 0x0, 0x0, 0x0, 0x0, 0xb, 0x0, 0x0,
        0x0, 0x0, 0x0, 0xb, 0x0, 0x0, 0x0, 0x0, 0x0, 0x9, 0x0, 0x0, 0x0, 0x0, 0x3, 0x3, 0x0, 0x0,
        0x0, 0x0, 0x5, 0x6d, 0x61, 0x72, 0x6b, 0x6f, 0x1, 0x0, 0x0, 0x0, 0x0, 0x20, 0x3, 0x0, 0x0,
        0x0, 0x0, 0x6, 0x72, 0x69, 0x70, 0x70, 0x6c, 0x65,
    ];

    let path = Path::decode(&mut &buf[..]).unwrap();

    assert_eq!(expecetd, path)
}

#[test]
fn encode_v3() {
    use super::vertex::Vertex;
    let p = Path {
        labels: vec![Set::new(vec![]), Set::new(vec![]), Set::new(vec![])],
        objects: vec![
            Vertex::new(1, "person", None).into(),
            Vertex::new(10, "sofware", None).into(),
            Vertex::new(11, "software", None).into(),
        ],
    };

    let s = serde_json::to_string(&p.encode_v3()).unwrap();
    let expected = r#"{"@type":"g:Path","@value":{"labels":{"@type":"g:List","@value":[{"@type":"g:Set","@value":[]},{"@type":"g:Set","@value":[]},{"@type":"g:Set","@value":[]}]},"objects":{"@type":"g:List","@value":[{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":1},"label":"person"}},{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":10},"label":"sofware"}},{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":11},"label":"software"}}]}}}"#;
    assert_eq!(s, expected)
}

#[test]
fn decode_v3() {
    use super::vertex::Vertex;
    let s = r#"{"@type":"g:Path","@value":{"labels":{"@type":"g:List","@value":[{"@type":"g:Set","@value":[]},{"@type":"g:Set","@value":[]},{"@type":"g:Set","@value":[]}]},"objects":{"@type":"g:List","@value":[{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":1},"label":"person"}},{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":10},"label":"sofware"}},{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":11},"label":"software"}}]}}}"#;
    let expected = Path {
        labels: vec![Set::new(vec![]), Set::new(vec![]), Set::new(vec![])],
        objects: vec![
            Vertex::new(1, "person", None).into(),
            Vertex::new(10, "sofware", None).into(),
            Vertex::new(11, "software", None).into(),
        ],
    };

    let jval = serde_json::from_str(s).unwrap();
    let path = Path::decode_v3(&jval).unwrap();
    assert_eq!(path, expected)
}

#[test]
fn encode_v2() {
    use super::vertex::Vertex;
    use crate::structure::vertex_property::VertexProperty;
    let p = Path {
        labels: vec![Set::new(vec![]), Set::new(vec![]), Set::new(vec![])],
        objects: vec![
            Vertex::new(1, "person", None).into(),
            Vertex::new(
                10,
                "software",
                Some(vec![VertexProperty::new(
                    4i64,
                    "name",
                    "gremlin",
                    Some(Vertex::new(10, "software", None)),
                    None,
                )]),
            )
            .into(),
            Vertex::new(
                11,
                "software",
                Some(vec![VertexProperty::new(
                    5i64,
                    "name",
                    "tinkergraph",
                    Some(Vertex::new(11, "software", None)),
                    None,
                )]),
            )
            .into(),
        ],
    };

    let s = serde_json::to_string(&p.encode_v2()).unwrap();
    let expected = r#"{"@type":"g:Path","@value":{"labels":[[],[],[]],"objects":[{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":1},"label":"person"}},{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":10},"label":"software","properties":{"name":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":4},"value":"gremlin","vertex":{"@type":"g:Int32","@value":10},"label":"name"}}]}}},{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":11},"label":"software","properties":{"name":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":5},"value":"tinkergraph","vertex":{"@type":"g:Int32","@value":11},"label":"name"}}]}}}]}}"#;
    let value: serde_json::Value = serde_json::from_str(expected).unwrap();
    let own_value: serde_json::Value = serde_json::from_str(&s).unwrap();
    assert_eq!(own_value, value);
}

#[test]
fn decode_v2() {
    use super::vertex::Vertex;
    use crate::structure::vertex_property::VertexProperty;
    let s = r#"{"@type":"g:Path","@value":{"labels":[[],[],[]],"objects":[{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":1},"label":"person"}},{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":10},"label":"software","properties":{"name":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":4},"value":"gremlin","vertex":{"@type":"g:Int32","@value":10},"label":"name"}}]}}},{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":11},"label":"software","properties":{"name":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":5},"value":"tinkergraph","vertex":{"@type":"g:Int32","@value":11},"label":"name"}}]}}}]}}"#;
    let expected = Path {
        labels: vec![Set::new(vec![]), Set::new(vec![]), Set::new(vec![])],
        objects: vec![
            Vertex::new(1, "person", None).into(),
            Vertex::new(
                10,
                "software",
                Some(vec![VertexProperty::new(
                    4i64,
                    "name",
                    "gremlin",
                    Some(Vertex::new(10, "", None)),
                    None,
                )]),
            )
            .into(),
            Vertex::new(
                11,
                "software",
                Some(vec![VertexProperty::new(
                    5i64,
                    "name",
                    "tinkergraph",
                    Some(Vertex::new(11, "", None)),
                    None,
                )]),
            )
            .into(),
        ],
    };

    let jval = serde_json::from_str(s).unwrap();
    let path = Path::decode_v2(&jval).unwrap();
    assert_eq!(path, expected)
}
