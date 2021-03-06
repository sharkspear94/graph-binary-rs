use std::fmt::Display;

use crate::{
    conversion,
    specs::{self, CoreType},
    GremlinValue,
};

use super::{edge::Edge, property::Property, vertex::Vertex, vertex_property::VertexProperty};

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
pub struct Graph {
    vertices: Vec<Vertex>,
    edges: Vec<GraphEdge>,
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "vertexes:[")?;
        for vertex in &self.vertices {
            write!(f, "{vertex},")?;
        }
        writeln!(f, "]")?;
        write!(f, "edges:[")?;
        for edge in &self.edges {
            write!(f, "{edge}")?;
        }
        write!(f, "]")
    }
}

#[derive(Debug, PartialEq, Clone)]
struct GraphEdge {
    id: GremlinValue,
    label: String,
    in_v_id: GremlinValue,
    in_v_label: Option<String>,
    out_v_id: GremlinValue,
    out_v_label: Option<String>,
    parent: Option<Vertex>,
    properties: Vec<Property>,
}

impl From<Edge> for GraphEdge {
    fn from(e: Edge) -> Self {
        let mut v = Vec::new();
        if e.properties.is_some() {
            v = e.properties.unwrap();
        }
        GraphEdge {
            id: *e.id,
            label: e.label,
            in_v_id: *e.in_v_id,
            in_v_label: Some(e.in_v_label),
            out_v_id: *e.out_v_id,
            out_v_label: Some(e.out_v_label),
            parent: e.parent,
            properties: v,
        }
    }
}

impl Display for GraphEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id:{},label:{},inV_id:{}",
            self.id, self.label, self.in_v_id
        )?;
        self.in_v_label
            .as_ref()
            .map_or_else(|| Ok(()), |label| write!(f, ",inV_label:{label}"))?;

        write!(f, ",outV_id:{}", self.out_v_id)?;

        self.out_v_label
            .as_ref()
            .map_or_else(|| Ok(()), |label| write!(f, ",outV_label:{label}"))?;

        self.parent
            .as_ref()
            .map_or_else(|| Ok(()), |parent| write!(f, ",parent:{parent}"))?;

        write!(f, ",properties:[")?;
        for property in &self.properties {
            write!(f, "{property},")?;
        }
        write!(f, "]")
    }
}

#[cfg(feature = "graph_binary")]
impl Decode for GraphEdge {
    fn expected_type_code() -> u8 {
        unimplemented!("GraphEdge is not a valid GraphBinary Type")
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let id = GremlinValue::decode(reader)?;
        let label = String::partial_decode(reader)?;
        let in_v_id = GremlinValue::decode(reader)?;
        let in_v_label = Option::<String>::decode(reader)?;
        let out_v_id = GremlinValue::decode(reader)?;
        let out_v_label = Option::<String>::decode(reader)?;
        let parent = Option::<Vertex>::decode(reader)?;
        let properties = Vec::<Property>::partial_decode(reader)?;

        Ok(GraphEdge {
            id,
            label,
            in_v_id,
            in_v_label,
            out_v_id,
            out_v_label,
            parent,
            properties,
        })
    }
}

#[cfg(feature = "graph_binary")]
impl Encode for Graph {
    fn type_code() -> u8 {
        specs::CoreType::Graph.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        let v_len = self.vertices.len() as i32;
        let e_len = self.edges.len() as i32;

        v_len.partial_encode(writer)?;
        for vertex in &self.vertices {
            vertex.id.encode(writer)?;
            vertex.label.partial_encode(writer)?;
            if vertex.properties.is_some() {
                let p_len = vertex.properties.as_ref().unwrap().len() as i32;
                p_len.partial_encode(writer)?;
                for prop in vertex.properties.as_ref().unwrap() {
                    prop.id.encode(writer)?;
                    prop.label.partial_encode(writer)?;
                    prop.value.encode(writer)?;
                    prop.parent.encode(writer)?;
                    if prop.properties.is_some() {
                        prop.properties.as_ref().unwrap().partial_encode(writer)?;
                    } else {
                        prop.properties.encode(writer)?;
                    }
                }
            } else {
                None::<i32>.encode(writer)?;
            }
            // vertex.properties.write_patial_bytes(writer)?;
        }

        e_len.partial_encode(writer)?;
        for edge in &self.edges {
            edge.id.encode(writer)?;
            edge.label.partial_encode(writer)?;
            edge.in_v_id.encode(writer)?;
            edge.in_v_label.encode(writer)?;
            edge.out_v_id.encode(writer)?;
            edge.out_v_label.encode(writer)?;
            edge.parent.encode(writer)?;
            edge.properties.partial_encode(writer)?; // TODO not sure if prop identifier is needed
        }
        Ok(())
    }
}

#[cfg(feature = "graph_binary")]
impl Decode for Graph {
    fn expected_type_code() -> u8 {
        CoreType::Graph.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let v_len = i32::partial_decode(reader)? as usize;
        let mut v_vec = Vec::with_capacity(v_len);
        for _ in 0..v_len {
            let v_id = GremlinValue::decode(reader)?;
            let v_label = String::partial_decode(reader)?;
            let p_len = i32::partial_decode(reader)? as usize;
            let mut p_vec = Vec::with_capacity(p_len);
            for _ in 0..p_len {
                let p_id = GremlinValue::decode(reader)?;
                let p_label = String::partial_decode(reader)?;
                let p_value = GremlinValue::decode(reader)?;
                let p_parent = Option::<Vertex>::decode(reader)?;
                let p_properties = Option::<Vec<Property>>::partial_decode(reader)?;
                p_vec.push(VertexProperty {
                    id: Box::new(p_id),
                    label: p_label,
                    value: Box::new(p_value),
                    parent: p_parent,
                    properties: p_properties,
                });
            }
            v_vec.push(Vertex {
                id: Box::new(v_id),
                label: v_label,
                properties: Some(p_vec),
            });
        }
        let e_len = i32::partial_decode(reader)? as usize;
        let mut e_vec = Vec::with_capacity(v_len);
        for _ in 0..e_len {
            e_vec.push(GraphEdge::partial_decode(reader)?);
        }
        Ok(Graph {
            vertices: v_vec,
            edges: e_vec,
        })
    }
}

#[cfg(feature = "graph_son")]
impl EncodeGraphSON for GraphEdge {
    fn encode_v3(&self) -> serde_json::Value {
        let properties_map = self
            .properties
            .iter()
            .map(|prop| (prop.key.clone(), prop.encode_v3()))
            .collect::<Map<String, serde_json::Value>>();

        let mut json_value = json!({
          "@type" : "g:Edge",
          "@value" : {
            "id" : self.id.encode_v3(),
            "label" : self.label.encode_v3(),
            "inVLabel" : self.in_v_label.encode_v3(),
            "outVLabel" : self.out_v_label.encode_v3(),
            "inV" : self.in_v_id.encode_v3(),
            "outV" : self.out_v_id.encode_v3(),
          }
        });
        if !properties_map.is_empty() {
            json_value["@value"]
                .as_object_mut()
                .unwrap()
                .insert("properties".to_string(), json! {properties_map});
        }
        json_value
    }

    fn encode_v2(&self) -> serde_json::Value {
        let properties_map = self
            .properties
            .iter()
            .map(|prop| (prop.key.clone(), prop.encode_v2()))
            .collect::<Map<String, serde_json::Value>>();

        let mut json_value = json!({
          "@type" : "g:Edge",
          "@value" : {
            "id" : self.id.encode_v2(),
            "label" : self.label.encode_v2(),
            "inVLabel" : self.in_v_label.encode_v2(),
            "outVLabel" : self.out_v_label.encode_v2(),
            "inV" : self.in_v_id.encode_v2(),
            "outV" : self.out_v_id.encode_v2(),
          }
        });
        if !properties_map.is_empty() {
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
impl DecodeGraphSON for GraphEdge {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Ok(Edge::decode_v3(j_val)?.into())
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Ok(Edge::decode_v2(j_val)?.into())
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

#[cfg(feature = "graph_son")]
impl EncodeGraphSON for Graph {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
            "@type" : "tinker:graph",
            "@value" : {
                "vertices": self.vertices.iter().map(EncodeGraphSON::encode_v3).collect::<Vec<serde_json::Value>>(),
                "edges": self.edges.iter().map(EncodeGraphSON::encode_v3).collect::<Vec<serde_json::Value>>()
             }
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        json!({
            "@type" : "tinker:graph",
            "@value" : {
                "vertices": self.vertices.iter().map(EncodeGraphSON::encode_v2).collect::<Vec<serde_json::Value>>(),
                "edges": self.edges.iter().map(EncodeGraphSON::encode_v2).collect::<Vec<serde_json::Value>>()
             }
        })
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

#[cfg(feature = "graph_son")]
impl DecodeGraphSON for Graph {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:TinkerGraph")?;

        let vertices = get_val_by_key_v3(value_object, "vertices", "TinkerGraph")?;
        let edges = get_val_by_key_v3(value_object, "edges", "TinkerGraph")?;

        Ok(Graph { vertices, edges })
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:TinkerGraph")?;

        let vertices = get_val_by_key_v2(value_object, "vertices", "TinkerGraph")?;
        let edges = get_val_by_key_v2(value_object, "edges", "TinkerGraph")?;

        Ok(Graph { vertices, edges })
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

conversion!(Graph, Graph);

#[test]
fn encode_gb() {
    use super::property::EitherParent;

    let expected = [
        0x10_u8, 0x0, 0x0, 0x0, 0x0, 0x2, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0,
        0x0, 0x0, 0x6, 0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0x0, 0x0, 0x0, 0x2, 0x2, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x4, 0x6e, 0x61, 0x6d, 0x65, 0x3, 0x0, 0x0,
        0x0, 0x0, 0x5, 0x6d, 0x61, 0x72, 0x6b, 0x6f, 0xfe, 0x1, 0x0, 0x0, 0x0, 0x0, 0x2, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x2, 0x0, 0x0, 0x0, 0x3, 0x61, 0x67, 0x65, 0x1, 0x0, 0x0,
        0x0, 0x0, 0x1d, 0xfe, 0x1, 0x0, 0x0, 0x0, 0x0, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x2, 0x0, 0x0, 0x0, 0x6, 0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0x0, 0x0, 0x0, 0x2, 0x2, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x3, 0x0, 0x0, 0x0, 0x4, 0x6e, 0x61, 0x6d, 0x65, 0x3,
        0x0, 0x0, 0x0, 0x0, 0x5, 0x76, 0x61, 0x64, 0x61, 0x73, 0xfe, 0x1, 0x0, 0x0, 0x0, 0x0, 0x2,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x4, 0x0, 0x0, 0x0, 0x3, 0x61, 0x67, 0x65, 0x1,
        0x0, 0x0, 0x0, 0x0, 0x1b, 0xfe, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x2, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xd, 0x0, 0x0, 0x0, 0x4, 0x74, 0x65, 0x73, 0x74, 0x2, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x2, 0xfe, 0x1, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x1, 0xfe, 0x1, 0xfe, 0x1, 0x0, 0x0, 0x0, 0x1, 0xf, 0x0, 0x0, 0x0, 0x0, 0x5, 0x73,
        0x69, 0x6e, 0x63, 0x65, 0x1, 0x0, 0x0, 0x0, 0x0, 0x7b, 0xfe, 0x1,
    ];

    let v_s = vec![
        Vertex {
            id: Box::new(1_i64.into()),
            label: "person".to_string(),
            properties: Some(vec![
                VertexProperty {
                    id: Box::new(0i64.into()),
                    label: "name".to_string(),
                    value: Box::new("marko".into()),
                    parent: None,
                    properties: Some(Vec::new()),
                },
                VertexProperty {
                    id: Box::new(2i64.into()),
                    label: "age".to_string(),
                    value: Box::new(29_i32.into()),
                    parent: None,
                    properties: Some(Vec::new()),
                },
            ]),
        },
        Vertex {
            id: Box::new(2_i64.into()),
            label: "person".to_string(),
            properties: Some(vec![
                VertexProperty {
                    id: Box::new(3i64.into()),
                    label: "name".to_string(),
                    value: Box::new("vadas".into()),
                    parent: None,
                    properties: Some(Vec::new()),
                },
                VertexProperty {
                    id: Box::new(4i64.into()),
                    label: "age".to_string(),
                    value: Box::new(27_i32.into()),
                    parent: None,
                    properties: Some(Vec::new()),
                },
            ]),
        },
    ];

    let edge = vec![GraphEdge {
        id: 13_i64.into(),
        label: "test".to_string(),
        in_v_id: 2_i64.into(),
        in_v_label: None,
        out_v_id: 1_i64.into(),
        out_v_label: None,
        parent: None,
        properties: vec![Property {
            key: "since".to_string(),
            value: Box::new(123_i32.into()),
            parent: EitherParent::None,
        }],
    }];

    let graph = Graph {
        vertices: v_s,
        edges: edge,
    };

    let mut buf = Vec::new();

    graph.encode(&mut buf).unwrap();

    assert_eq!(expected, *buf);
}

#[test]
fn decode_gb() {
    use super::property::EitherParent;

    let reader = vec![
        0x10_u8, 0x0, 0x0, 0x0, 0x0, 0x2, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0,
        0x0, 0x0, 0x6, 0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0x0, 0x0, 0x0, 0x2, 0x2, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x4, 0x6e, 0x61, 0x6d, 0x65, 0x3, 0x0, 0x0,
        0x0, 0x0, 0x5, 0x6d, 0x61, 0x72, 0x6b, 0x6f, 0xfe, 0x1, 0x0, 0x0, 0x0, 0x0, 0x2, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x2, 0x0, 0x0, 0x0, 0x3, 0x61, 0x67, 0x65, 0x1, 0x0, 0x0,
        0x0, 0x0, 0x1d, 0xfe, 0x1, 0x0, 0x0, 0x0, 0x0, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x2, 0x0, 0x0, 0x0, 0x6, 0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0x0, 0x0, 0x0, 0x2, 0x2, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x3, 0x0, 0x0, 0x0, 0x4, 0x6e, 0x61, 0x6d, 0x65, 0x3,
        0x0, 0x0, 0x0, 0x0, 0x5, 0x76, 0x61, 0x64, 0x61, 0x73, 0xfe, 0x1, 0x0, 0x0, 0x0, 0x0, 0x2,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x4, 0x0, 0x0, 0x0, 0x3, 0x61, 0x67, 0x65, 0x1,
        0x0, 0x0, 0x0, 0x0, 0x1b, 0xfe, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x2, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xd, 0x0, 0x0, 0x0, 0x4, 0x74, 0x65, 0x73, 0x74, 0x2, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x2, 0xfe, 0x1, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x1, 0xfe, 0x1, 0xfe, 0x1, 0x0, 0x0, 0x0, 0x1, 0xf, 0x0, 0x0, 0x0, 0x0, 0x5, 0x73,
        0x69, 0x6e, 0x63, 0x65, 0x1, 0x0, 0x0, 0x0, 0x0, 0x7b, 0xfe, 0x1,
    ];

    let v_s = vec![
        Vertex {
            id: Box::new(1_i64.into()),
            label: "person".to_string(),
            properties: Some(vec![
                VertexProperty {
                    id: Box::new(0i64.into()),
                    label: "name".to_string(),
                    value: Box::new("marko".into()),
                    parent: None,
                    properties: Some(Vec::new()),
                },
                VertexProperty {
                    id: Box::new(2i64.into()),
                    label: "age".to_string(),
                    value: Box::new(29_i32.into()),
                    parent: None,
                    properties: Some(Vec::new()),
                },
            ]),
        },
        Vertex {
            id: Box::new(2_i64.into()),
            label: "person".to_string(),
            properties: Some(vec![
                VertexProperty {
                    id: Box::new(3i64.into()),
                    label: "name".to_string(),
                    value: Box::new("vadas".into()),
                    parent: None,
                    properties: Some(Vec::new()),
                },
                VertexProperty {
                    id: Box::new(4i64.into()),
                    label: "age".to_string(),
                    value: Box::new(27_i32.into()),
                    parent: None,
                    properties: Some(Vec::new()),
                },
            ]),
        },
    ];

    let edge = vec![GraphEdge {
        id: 13_i64.into(),
        label: "test".to_string(),
        in_v_id: 2_i64.into(),
        in_v_label: None,
        out_v_id: 1_i64.into(),
        out_v_label: None,
        parent: None,
        properties: vec![Property {
            key: "since".to_string(),
            value: Box::new(123_i32.into()),
            parent: EitherParent::None,
        }],
    }];

    let expected = Graph {
        vertices: v_s,
        edges: edge,
    };

    let graph = Graph::decode(&mut &reader[..]).unwrap();

    assert_eq!(expected, graph);
}
