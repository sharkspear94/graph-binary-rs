use serde::{de::Visitor, Deserialize};

use crate::{
    error::DecodeError,
    graph_binary::{Decode, Encode, GraphBinary},
    specs::{self, CoreType},
    struct_deserialize,
    structure::property::Property,
};

use super::vertex_property::VertexProperty;

#[derive(Debug, PartialEq)]
pub struct Vertex {
    pub id: Box<GraphBinary>,
    pub label: String,
    pub properties: Option<Vec<VertexProperty>>,
}

impl Vertex {}

impl Encode for Vertex {
    fn type_code() -> u8 {
        specs::CoreType::Vertex.into()
    }

    fn write_patial_bytes<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.id.write_full_qualified_bytes(writer)?;
        self.label.write_patial_bytes(writer)?;
        self.properties.write_full_qualified_bytes(writer)
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
        let id = Box::new(GraphBinary::fully_self_decode(reader)?);
        let label = String::partial_decode(reader)?;
        let properties = Option::<Vec<VertexProperty>>::fully_self_decode(reader)?;

        Ok(Vertex {
            id,
            label,
            properties,
        })
    }

    fn partial_count_bytes(bytes: &[u8]) -> Result<usize, DecodeError> {
        let mut len = GraphBinary::consumed_bytes(bytes)?;
        len += String::partial_count_bytes(&bytes[len..])?;
        len += GraphBinary::consumed_bytes(&bytes[len..])?;
        Ok(len)
    }
}

struct_deserialize!((Vertex, VertexVisitor));

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
    let v = v.write_full_qualified_bytes(&mut buf);
    assert!(v.is_ok());
    assert_eq!(expected, buf[..])
}

// #[test]
// fn test_vertex_some_encode() {
//     let expected = [
//         0x11_u8, 0x0, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x6, 0x70,
//         0x65, 0x72, 0x73, 0x6f, 0x6e, 0x0f, 0x0, 0x0, 0x0, 0x0, 0x4, b'n', b'a', b'm', b'e', 0x3,
//         0x0, 0x0, 0x0, 0x0, 0x5, b'm', b'a', b'r', b'k', b'o',
//     ];
//     let v = Vertex {
//         id: Box::new(1_i64.into()),
//         label: String::from("person"),
//         properties: Box::new(Some(VertexProperty { id: todo!(), label: todo!(), value: todo!(), parent: todo!(), properties: todo!() }
//             key: "name".to_string(),
//             value: Box::new("marko".into()),
//         }))),
//     };
//     let mut buf = Vec::new();
//     let v = v.write_full_qualified_bytes(&mut buf);
//     assert!(v.is_ok());
//     assert_eq!(expected, buf[..])
// }

#[test]
fn test_vertex_decode_none() {
    let reader = vec![
        0x11_u8, 0x0, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x6, 0x70,
        0x65, 0x72, 0x73, 0x6f, 0x6e, 0xfe, 0x1,
    ];

    let v = Vertex::fully_self_decode(&mut &reader[..]);
    assert!(v.is_ok());

    let expected = Vertex {
        id: Box::new(1_i64.into()),
        label: String::from("person"),
        properties: None,
    };

    assert_eq!(expected, v.unwrap())
}

// #[test]
// fn test_vertex_decode_some() {
//     let reader = vec![
//         0x11_u8, 0x0, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x6, 0x70,
//         0x65, 0x72, 0x73, 0x6f, 0x6e, 0x0f, 0x0, 0x0, 0x0, 0x0, 0x4, b'n', b'a', b'm', b'e', 0x3,
//         0x0, 0x0, 0x0, 0x0, 0x5, b'm', b'a', b'r', b'k', b'o',
//     ];

//     let v = Vertex::fully_self_decode(&mut &reader[..]);
//     assert!(v.is_ok());

//     let expected = Vertex {
//         id: Box::new(1_i64.into()),
//         label: String::from("person"),
//         properties: Box::new(Some(GraphBinary::Property(Property {
//             key: "name".to_string(),
//             value: Box::new("marko".into()),
//         }))),
//     };

//     assert_eq!(expected, v.unwrap())
// }

#[test]
fn test_vertex_consume() {
    let reader = vec![
        0x11_u8, 0x0, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x6, 0x70,
        0x65, 0x72, 0x73, 0x6f, 0x6e, 0xfe, 0x01,
    ];

    let size = Vertex::consumed_bytes(&reader).unwrap();

    assert_eq!(reader.len(), size)
}
