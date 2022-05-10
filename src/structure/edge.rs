use std::io::Bytes;

use serde::Serialize;

use crate::{
    graph_binary::{Encode, GraphBinary},
    specs,
};

use super::{property::Property, vertex::Vertex};

#[derive(Debug, PartialEq)]
pub struct Edge {
    pub id: Box<GraphBinary>,
    pub label: String,
    pub in_v_id: Box<GraphBinary>,
    pub in_v_label: String,
    pub out_v_id: Box<GraphBinary>,
    pub out_v_label: String,
    pub parent: Option<Vertex>,
    pub properties: Option<Vec<Property>>,
}

impl Encode for Edge {
    fn type_code() -> u8 {
        specs::CoreType::Edge.into()
    }

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), crate::error::EncodeError> {
        self.id.fq_gb_bytes(writer)?;
        self.label.gb_bytes(writer)?;
        self.in_v_id.fq_gb_bytes(writer)?;
        self.in_v_label.gb_bytes(writer)?;
        self.out_v_id.fq_gb_bytes(writer)?;
        self.out_v_label.gb_bytes(writer)?;
        self.parent.fq_gb_bytes(writer)?;
        self.properties.fq_gb_bytes(writer)
    }
}

impl Serialize for Edge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut buf: Vec<u8> = Vec::with_capacity(64);
        self.fq_gb_bytes(&mut buf).expect("error during Edge write");
        serializer.serialize_bytes(&buf)
    }
}

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
    let e = e.fq_gb_bytes(&mut buf);
    assert!(e.is_ok());
    assert_eq!(expected, buf[..])
}
#[test]
fn edge_some_encode_test() {
    let expected = [
        0xd_u8, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x9, 0x0, 0x0, 0x0, 0x7, 0x63, 0x72, 0x65, 0x61,
        0x74, 0x65, 0x64, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x3, 0x0, 0x0, 0x0, 0x8,
        0x73, 0x6f, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x1, 0x0, 0x0, 0x0, 0x6, 0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0x11, 0x0, 0x2, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x6, 0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e,
        0x0f, 0x0, 0x0, 0x0, 0x0, 0x4, b'n', b'a', b'm', b'e', 0x3, 0x0, 0x0, 0x0, 0x0, 0x5, b'm',
        b'a', b'r', b'k', b'o', 0xfe, 0x1,
    ];

    let e = Edge {
        id: Box::new(9_i32.into()),
        label: "created".to_string(),
        in_v_id: Box::new(3_i64.into()),
        in_v_label: "software".to_string(),
        out_v_id: Box::new(1_i64.into()),
        out_v_label: "person".to_string(),
        parent: Some(Vertex {
            id: Box::new(1_i64.into()),
            label: String::from("person"),
            properties: Box::new(Some(GraphBinary::Property(Property {
                key: "name".to_string(),
                value: Box::new("marko".into()),
            }))),
        }),
        properties: None,
    };

    let mut buf = Vec::new();
    let e = e.fq_gb_bytes(&mut buf);
    assert!(e.is_ok());
    assert_eq!(expected, buf[..])
}
