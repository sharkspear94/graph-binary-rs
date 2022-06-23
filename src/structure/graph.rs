use crate::{
    graph_binary::{Decode, Encode, GraphBinary},
    specs::{self, CoreType},
    struct_de_serialize,
};

use super::{property::Property, vertex::Vertex, vertex_property::VertexProperty};

#[derive(Debug, PartialEq, Clone)]
pub struct Graph {
    vertexes: Vec<Vertex>,
    edges: Vec<GraphEdge>,
}

#[derive(Debug, PartialEq, Clone)]
struct GraphEdge {
    id: GraphBinary,
    label: String,
    in_v_id: GraphBinary,
    in_v_label: Option<String>,
    out_v_id: GraphBinary,
    out_v_label: Option<String>,
    parent: Option<Vertex>,
    properties: Vec<Property>,
}

impl Decode for GraphEdge {
    fn expected_type_code() -> u8 {
        unimplemented!("GraphEdge is not a valid GraphBinary Type")
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let id = GraphBinary::decode(reader)?;
        let label = String::partial_decode(reader)?;
        let in_v_id = GraphBinary::decode(reader)?;
        let in_v_label = Option::<String>::decode(reader)?;
        let out_v_id = GraphBinary::decode(reader)?;
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

    fn get_partial_len(bytes: &[u8]) -> Result<usize, crate::error::DecodeError> {
        let mut len = GraphBinary::get_len(bytes)?;
        len += String::get_partial_len(&bytes[len..])?;
        len += GraphBinary::get_len(&bytes[len..])?;
        len += Option::<String>::get_len(&bytes[len..])?; //TODO not sure if correct
        len += GraphBinary::get_len(&bytes[len..])?;
        len += Option::<String>::get_len(&bytes[len..])?; //TODO not sure if correct
        len += Option::<Vertex>::get_len(&bytes[len..])?;
        len += Vec::<Property>::get_partial_len(&bytes[len..])?;

        Ok(len)
    }
}

impl Encode for Graph {
    fn type_code() -> u8 {
        specs::CoreType::Graph.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        let v_len = self.vertexes.len() as i32;
        let e_len = self.edges.len() as i32;

        v_len.partial_encode(writer)?;
        for vertex in &self.vertexes {
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
            let v_id = GraphBinary::decode(reader)?;
            let v_label = String::partial_decode(reader)?;
            let p_len = i32::partial_decode(reader)? as usize;
            let mut p_vec = Vec::with_capacity(p_len);
            for _ in 0..p_len {
                let p_id = GraphBinary::decode(reader)?;
                let p_label = String::partial_decode(reader)?;
                let p_value = GraphBinary::decode(reader)?;
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
            vertexes: v_vec,
            edges: e_vec,
        })
    }

    fn get_partial_len(bytes: &[u8]) -> Result<usize, crate::error::DecodeError> {
        let t: [u8; 4] = bytes[0..4].try_into()?;
        let v_len = i32::from_be_bytes(t);
        let mut len = 4;
        for _ in 0..v_len {
            len += GraphBinary::get_len(&bytes[len..])?;
            len += String::get_partial_len(&bytes[len..])?;

            let t: [u8; 4] = bytes[len..len + 4].try_into()?;
            let p_len = i32::from_be_bytes(t);
            len += 4;

            for _ in 0..p_len {
                len += GraphBinary::get_len(&bytes[len..])?;
                len += String::get_partial_len(&bytes[len..])?;
                len += GraphBinary::get_len(&bytes[len..])?;
                len += 2; //parent is always null
                len += Vec::<Property>::get_partial_len(&bytes[len..])?;
            }
        }

        let t: [u8; 4] = bytes[len..len + 4].try_into()?;
        let e_len = i32::from_be_bytes(t);
        len += 4;

        for _ in 0..e_len {
            len += GraphEdge::get_partial_len(&bytes[len..])?;
        }
        Ok(len)
    }
}

struct_de_serialize!((Graph, GraphVisitor, 254));

#[test]
fn encode_graph_test() {
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
            parent: Box::new(GraphBinary::UnspecifiedNullObject),
        }],
    }];

    let graph = Graph {
        vertexes: v_s,
        edges: edge,
    };

    let mut buf = Vec::new();

    graph.encode(&mut buf).unwrap();

    assert_eq!(expected, *buf);
}

#[test]
fn decode_graph_test() {
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
            parent: Box::new(GraphBinary::UnspecifiedNullObject),
        }],
    }];

    let expected = Graph {
        vertexes: v_s,
        edges: edge,
    };

    let graph = Graph::decode(&mut &reader[..]).unwrap();

    assert_eq!(expected, graph);
}

#[test]
fn consume_graph_test() {
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

    let len = Graph::get_len(&reader).unwrap();

    assert_eq!(reader.len(), len);
}
