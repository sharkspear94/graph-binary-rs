use crate::{
    graph_binary::{Decode, Encode, GraphBinary},
    specs::{self, CoreType},
    struct_de_serialize,
};

use super::vertex::Vertex;

#[derive(Debug, PartialEq, Clone)]
pub struct Edge {
    pub id: Box<GraphBinary>,
    pub label: String,
    pub in_v_id: Box<GraphBinary>,
    pub in_v_label: String,
    pub out_v_id: Box<GraphBinary>,
    pub out_v_label: String,
    pub parent: Option<Vertex>,
    pub properties: Option<Vec<GraphBinary>>,
}

impl Encode for Edge {
    fn type_code() -> u8 {
        specs::CoreType::Edge.into()
    }

    fn write_patial_bytes<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.id.write_full_qualified_bytes(writer)?;
        self.label.write_patial_bytes(writer)?;
        self.in_v_id.write_full_qualified_bytes(writer)?;
        self.in_v_label.write_patial_bytes(writer)?;
        self.out_v_id.write_full_qualified_bytes(writer)?;
        self.out_v_label.write_patial_bytes(writer)?;
        self.parent.write_full_qualified_bytes(writer)?;
        self.properties.write_full_qualified_bytes(writer)
    }
}

impl Decode for Edge {
    fn expected_type_code() -> u8 {
        CoreType::Edge.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let id = GraphBinary::fully_self_decode(reader)?;
        let label = String::partial_decode(reader)?;
        let in_v_id = GraphBinary::fully_self_decode(reader)?;
        let in_v_label = String::partial_decode(reader)?;
        let out_v_id = GraphBinary::fully_self_decode(reader)?;
        let out_v_label = String::partial_decode(reader)?;
        let parent = Option::<Vertex>::fully_self_decode(reader)?;
        let properties = Option::<Vec<GraphBinary>>::fully_self_decode(reader)?;

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

    fn partial_count_bytes(bytes: &[u8]) -> Result<usize, crate::error::DecodeError> {
        let mut len = GraphBinary::consumed_bytes(bytes)?;
        len += String::partial_count_bytes(&bytes[len..])?;
        len += GraphBinary::consumed_bytes(&bytes[len..])?;
        len += String::partial_count_bytes(&bytes[len..])?;
        len += GraphBinary::consumed_bytes(&bytes[len..])?;
        len += String::partial_count_bytes(&bytes[len..])?;
        len += Option::<Vertex>::consumed_bytes(&bytes[len..])?;
        len += Option::<Vec<GraphBinary>>::consumed_bytes(&bytes[len..])?;
        Ok(len)
    }
}

struct_de_serialize!((Edge, EdgeVisitor, 64));

#[test]
fn edge_none_encode_test() {
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

    let mut buf = Vec::new();
    let e = e.write_full_qualified_bytes(&mut buf);
    assert!(e.is_ok());
    assert_eq!(expected, buf[..])
}

// #[test]
// fn edge_some_encode_test() {
//     let expected = [
//         0xd_u8, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x9, 0x0, 0x0, 0x0, 0x7, 0x63, 0x72, 0x65, 0x61,
//         0x74, 0x65, 0x64, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x3, 0x0, 0x0, 0x0, 0x8,
//         0x73, 0x6f, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
//         0x0, 0x1, 0x0, 0x0, 0x0, 0x6, 0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0x11, 0x0, 0x2, 0x0, 0x0,
//         0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x6, 0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e,
//         0x0f, 0x0, 0x0, 0x0, 0x0, 0x4, b'n', b'a', b'm', b'e', 0x3, 0x0, 0x0, 0x0, 0x0, 0x5, b'm',
//         b'a', b'r', b'k', b'o', 0xfe, 0x1,
//     ];

//     let e = Edge {
//         id: Box::new(9_i32.into()),
//         label: "created".to_string(),
//         in_v_id: Box::new(3_i64.into()),
//         in_v_label: "software".to_string(),
//         out_v_id: Box::new(1_i64.into()),
//         out_v_label: "person".to_string(),
//         parent: Some(Vertex {
//             id: Box::new(1_i64.into()),
//             label: String::from("person"),
//             properties: Box::new(Some(GraphBinary::Property(Property {
//                 key: "name".to_string(),
//                 value: Box::new("marko".into()),
//                 parent: None,
//             }))),
//         }),
//         properties: None,
//     };

//     let mut buf = Vec::new();
//     let e = e.write_full_qualified_bytes(&mut buf);
//     assert!(e.is_ok());
//     assert_eq!(expected, buf[..])
// }

#[test]
fn edge_decode_test() {
    let reader = vec![
        0xd, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x9, 0x0, 0x0, 0x0, 0x7, 0x63, 0x72, 0x65, 0x61, 0x74,
        0x65, 0x64, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x3, 0x0, 0x0, 0x0, 0x8, 0x73,
        0x6f, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1,
        0x0, 0x0, 0x0, 0x6, 0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0xfe, 0x1, 0xfe, 0x1,
    ];

    let p = Edge::fully_self_decode(&mut &reader[..]);

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
fn edge_deser_test() {
    use crate::de::from_slice;

    let reader = vec![
        0xd, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x9, 0x0, 0x0, 0x0, 0x7, 0x63, 0x72, 0x65, 0x61, 0x74,
        0x65, 0x64, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x3, 0x0, 0x0, 0x0, 0x8, 0x73,
        0x6f, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1,
        0x0, 0x0, 0x0, 0x6, 0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0xfe, 0x1, 0xfe, 0x1,
    ];

    let p = from_slice(&reader);

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

    let p = from_slice(&reader);

    assert_eq!(GraphBinary::Edge(expected), p.unwrap());
}
