use crate::{
    error::DecodeError,
    graph_binary::{Decode, Encode, GraphBinary},
    specs::{self, CoreType},
    struct_de_serialize,
};

use super::vertex_property::VertexProperty;

#[derive(Debug, PartialEq, Clone)]
pub struct Vertex {
    pub id: Box<GraphBinary>,
    pub label: String,
    pub properties: Option<Vec<VertexProperty>>,
}

impl Vertex {
    pub fn new<ID: Into<GraphBinary>>(id: ID, label: &str) -> Self {
        Vertex {
            id: Box::new(id.into()),
            label: label.to_owned(),
            properties: None,
        }
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
        let id = Box::new(GraphBinary::decode(reader)?);
        let label = String::partial_decode(reader)?;
        let properties = Option::<Vec<VertexProperty>>::decode(reader)?;

        Ok(Vertex {
            id,
            label,
            properties,
        })
    }

    fn get_partial_len(bytes: &[u8]) -> Result<usize, DecodeError> {
        let mut len = GraphBinary::get_len(bytes)?;
        len += String::get_partial_len(&bytes[len..])?;
        len += GraphBinary::get_len(&bytes[len..])?;
        Ok(len)
    }
}

struct_de_serialize!((Vertex, VertexVisitor, 32));

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
