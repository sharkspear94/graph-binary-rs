use std::collections::HashMap;

use serde::{
    de::{MapAccess, Visitor},
    Deserialize,
};
use uuid::Uuid;

use crate::{
    graph_binary::{GraphBinary, MapKeys},
    specs::CoreType,
    structure::{
        binding::Binding,
        bulkset::BulkSet,
        bytecode::ByteCode,
        edge::Edge,
        enums::{
            Barrier, Cardinality, Column, Direction, Merge, Operator, Order, Pick, Pop, Scope,
            TextP, P, T,
        },
        graph::Graph,
        lambda::Lambda,
        metrics::{Metrics, TraversalMetrics},
        primitivs::UuidDef,
        property::Property,
        traverser::{TraversalStrategy, Traverser},
        vertex::Vertex,
    },
};

use super::{from_slice, Deserializer, U8Deser};

impl<'de> Deserialize<'de> for GraphBinary {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(GraphBinaryVisitor)
    }
}

pub struct GraphBinaryVisitor;

impl<'de> Visitor<'de> for GraphBinaryVisitor {
    type Value = GraphBinary;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "enum GraphBinary")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(GraphBinary::Boolean(v))
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(GraphBinary::Char(v))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(GraphBinary::Byte(v))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(GraphBinary::Short(v))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(GraphBinary::Int(v))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(GraphBinary::Long(v))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(GraphBinary::Float(v))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(GraphBinary::Double(v))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        // let variant = seq.next_element::<CoreType>()?.unwrap();
        let mut vec = if let Some(size) = seq.size_hint() {
            Vec::with_capacity(size)
        } else {
            Vec::new()
        };

        while let Some(item) = seq.next_element()? {
            vec.push(item)
        }

        Ok(GraphBinary::List(vec))
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(GraphBinary::UnspecifiedNullObject)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(GraphBinary::UnspecifiedNullObject)
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let core_type: CoreType = map.next_key()?.unwrap(); //  TODO unwrap

        match core_type {
            CoreType::Edge => Ok(GraphBinary::Edge(map.next_value::<Edge>()?)),
            CoreType::Vertex => Ok(GraphBinary::Vertex(map.next_value::<Vertex>()?)),
            CoreType::Barrier => Ok(GraphBinary::Barrier(map.next_value::<Barrier>()?)),
            CoreType::Cardinality => Ok(GraphBinary::Cardinality(map.next_value::<Cardinality>()?)),
            CoreType::Column => Ok(GraphBinary::Column(map.next_value::<Column>()?)),
            CoreType::Direction => Ok(GraphBinary::Direction(map.next_value::<Direction>()?)),
            CoreType::Operator => Ok(GraphBinary::Operator(map.next_value::<Operator>()?)),
            CoreType::Order => Ok(GraphBinary::Order(map.next_value::<Order>()?)),
            CoreType::P => Ok(GraphBinary::P(map.next_value::<P>()?)),
            CoreType::T => Ok(GraphBinary::T(map.next_value::<T>()?)),
            CoreType::TextP => Ok(GraphBinary::TextP(map.next_value::<TextP>()?)),
            CoreType::Metrics => Ok(GraphBinary::Metrics(map.next_value::<Metrics>()?)),
            CoreType::TraversalMetrics => Ok(GraphBinary::TraversalMetrics(
                map.next_value::<TraversalMetrics>()?,
            )),
            CoreType::Set => Ok(GraphBinary::Set(map.next_value::<Vec<GraphBinary>>()?)),
            CoreType::Int32 => todo!(),
            CoreType::Long => todo!(),
            CoreType::String => todo!(),
            CoreType::Class => todo!(),
            CoreType::Double => todo!(),
            CoreType::Float => todo!(),
            CoreType::List => todo!(),
            CoreType::Map => Ok(GraphBinary::Map(
                map.next_value::<HashMap<MapKeys, GraphBinary>>()?,
            )),
            CoreType::Uuid => Ok(GraphBinary::Uuid(Uuid::from(map.next_value::<UuidDef>()?))),
            CoreType::Path => todo!(),
            CoreType::Property => Ok(GraphBinary::Property(map.next_value::<Property>()?)),
            CoreType::Graph => Ok(GraphBinary::Graph(map.next_value::<Graph>()?)),
            CoreType::VertexProperty => todo!(),
            CoreType::Binding => Ok(GraphBinary::Binding(map.next_value::<Binding>()?)),
            CoreType::ByteCode => Ok(GraphBinary::ByteCode(map.next_value::<ByteCode>()?)),
            CoreType::Pick => Ok(GraphBinary::Pick(map.next_value::<Pick>()?)),
            CoreType::Pop => Ok(GraphBinary::Pop(map.next_value::<Pop>()?)),
            CoreType::Lambda => Ok(GraphBinary::Lambda(map.next_value::<Lambda>()?)),
            CoreType::Scope => Ok(GraphBinary::Scope(map.next_value::<Scope>()?)),
            CoreType::Traverser => Ok(GraphBinary::Traverser(map.next_value::<Traverser>()?)),
            CoreType::Byte => todo!(),
            CoreType::ByteBuffer => todo!(),
            CoreType::Short => todo!(),
            CoreType::Boolean => todo!(),
            CoreType::TraversalStrategy => Ok(GraphBinary::TraversalStrategy(
                map.next_value::<TraversalStrategy>()?,
            )),
            CoreType::BulkSet => todo!(), //Ok(GraphBinary::BulkSet(
            //     map.next_value::<BulkSet>()?,
            // )),
            CoreType::Tree => todo!(),
            CoreType::Merge => Ok(GraphBinary::Merge(map.next_value::<Merge>()?)),
            CoreType::UnspecifiedNullObject => todo!(),
            CoreType::Char => todo!(), // _ => todo!(),
        }
    }
}

#[test]
fn test_int() {
    let buf = [0x01, 0x0, 0x0, 0x0, 0x0, 0x01];

    let gb = GraphBinary::deserialize(&mut Deserializer { bytes: &buf });

    assert_eq!(GraphBinary::Int(1), gb.unwrap())
}

#[test]
fn test_int_some() {
    let buf = [0x0fe, 0x1, 0x0, 0x0, 0x0, 0x01];

    let gb = GraphBinary::deserialize(&mut Deserializer { bytes: &buf });

    assert_eq!(GraphBinary::UnspecifiedNullObject, gb.unwrap())
}

#[test]
fn test_t() {
    let buf = [0x20, 0x0, 0x03, 0x0, 0x0, 0x0, 0x0, 0x02, b'i', b'd'];

    let gb = from_slice::<GraphBinary>(&buf).unwrap();

    assert_eq!(GraphBinary::T(T::Id), gb);
}

#[test]
fn test_barrier() {
    let buf = [
        0x13, 0x0, 0x03, 0x0, 0x0, 0x0, 0x0, 0x08, b'n', b'o', b'r', b'm', b'S', b'a', b'c', b'k',
    ];

    let gb = from_slice::<GraphBinary>(&buf).unwrap();

    assert_eq!(GraphBinary::Barrier(Barrier::NormSack), gb);
}

#[test]
fn test_p() {
    let buf = [
        0x1e, 0x0, 0x03, 0x0, 0x0, 0x0, 0x0, 0x07, b'b', b'e', b't', b'w', b'e', b'e', b'n', 0x00,
        0x00, 0x0, 0x2, 0x1, 0x0, 0x0, 0x0, 0x0, 0x1, 0x1, 0x0, 0x0, 0x0, 0x0, 0xa,
    ];

    let gb = from_slice::<GraphBinary>(&buf).unwrap();

    assert_eq!(
        GraphBinary::P(P::between(1,10)),
        gb
    );
}

#[test]
fn test_order() {
    let buf = [0x1a, 0x0, 0x03, 0x0, 0x0, 0x0, 0x0, 0x03, b'a', b's', b'c'];

    let gb = from_slice::<GraphBinary>(&buf).unwrap();

    assert_eq!(GraphBinary::Order(Order::Asc), gb);
}

#[test]
fn text_p_deser() {
    let reader = vec![
        0x28, 0x00, 0x03, 0x0, 0x0, 0x0, 0x0, 0x0c, b's', b't', b'a', b'r', b't', b'i', b'n', b'g',
        b'W', b'i', b't', b'h', 0x0, 0x0, 0x0, 0x01, 0x3, 0x0, 0x0, 0x0, 0x0, 0x04, b't', b'e',
        b's', b't',
    ];

    let p = from_slice(&reader);

    // assert!(p.is_ok());

    assert_eq!(
        GraphBinary::TextP(TextP::StartingWith(vec!["test".into()])),
        p.unwrap()
    );
}

#[test]
fn test_vertex() {
    let buf = [
        0x11_u8, 0x0, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x6, 0x70,
        0x65, 0x72, 0x73, 0x6f, 0x6e, 0xfe, 0x1,
    ];

    let gb2 = from_slice::<GraphBinary>(&buf).unwrap();

    assert_eq!(
        GraphBinary::Vertex(Vertex {
            id: Box::new(1_i64.into()),
            label: String::from("person"),
            properties: None,
        }),
        gb2
    );
}

#[test]
fn test_vertex_struct() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct TestStruct {
        test: i32,
        milli: i16,
        x: GraphBinary,
    }

    let test = TestStruct {
        test: 1,
        milli: 1,
        x: GraphBinary::Vertex(Vertex {
            id: Box::new(1_i64.into()),
            label: String::from("person"),
            properties: None,
        }),
    };

    let buf = [
        0x0a_u8, 0x0, 0x00, 0x0, 0x0, 0x3, 0x03, 0x00, 0x00, 0x0, 0x0, 0x4, b't', b'e', b's', b't',
        0x01, 0x00, 0x00, 0x0, 0x0, 0x1, 0x03, 0x00, 0x00, 0x0, 0x0, 0x1, b'x', 0x011_u8, 0x0, 0x2,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x6, 0x70, 0x65, 0x72, 0x73,
        0x6f, 0x6e, 0xfe, 0x1, 0x03, 0x00, 0x00, 0x0, 0x0, 0x5, b'm', b'i', b'l', b'l', b'i', 0x26,
        0x00, 0x00, 0x1,
    ];

    let res: TestStruct = crate::de::from_slice(&buf).unwrap();

    assert_eq!(test, res)
}

#[test]
fn test_seq() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct TestStruct {
        test: i32,
        abc: GraphBinary,
        milli: i16,
    }

    let test = TestStruct {
        test: 1,
        abc: GraphBinary::List(vec![1.into()]),
        milli: 1,
    };

    let buf = [
        0x0a_u8, 0x0, 0x00, 0x0, 0x0, 0x3, 0x03, 0x00, 0x00, 0x0, 0x0, 0x4, b't', b'e', b's', b't',
        0x01, 0x00, 0x00, 0x0, 0x0, 0x1, 0x03, 0x00, 0x00, 0x0, 0x0, 0x3, b'a', b'b', b'c', 0x09,
        0x0, 0x00, 0x00, 0x00, 0x01, 0x01, 0x0, 0x0, 0x0, 0x0, 0x1, 0x03, 0x00, 0x00, 0x0, 0x0,
        0x5, b'm', b'i', b'l', b'l', b'i', 0x26, 0x00, 0x00, 0x1,
    ];

    let res: TestStruct = crate::de::from_slice(&buf).unwrap();

    assert_eq!(test, res)
}

#[test]
fn test_seq_set() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct TestStruct {
        test: i32,
        abc: GraphBinary,
        milli: i16,
    }

    let test = TestStruct {
        test: 1,
        abc: GraphBinary::Set(vec![1.into()]),
        milli: 1,
    };

    let buf = [
        0x0a_u8, 0x0, 0x00, 0x0, 0x0, 0x3, 0x03, 0x00, 0x00, 0x0, 0x0, 0x4, b't', b'e', b's', b't',
        0x01, 0x00, 0x00, 0x0, 0x0, 0x1, 0x03, 0x00, 0x00, 0x0, 0x0, 0x3, b'a', b'b', b'c', 0x0b,
        0x0, 0x00, 0x00, 0x00, 0x01, 0x01, 0x0, 0x0, 0x0, 0x0, 0x1, 0x03, 0x00, 0x00, 0x0, 0x0,
        0x5, b'm', b'i', b'l', b'l', b'i', 0x26, 0x00, 0x00, 0x1,
    ];

    let res: TestStruct = crate::de::from_slice(&buf).unwrap();

    assert_eq!(test, res)
}

#[test]
fn test_map() {
    let reader = vec![
        0xA_u8, 0x0, 0x0, 0x0, 0x0, 0x1, 0x03, 0x0, 0x0, 0x0, 0x0, 0x4, b't', b'e', b's', b't',
        0x01, 0x0, 0x0, 0x0, 0x0, 0x1,
    ];

    let map = HashMap::from([(MapKeys::String("test".into()), 1.into())]);

    assert_eq!(GraphBinary::Map(map), from_slice(&reader).unwrap())
}

#[test]
fn test_map_test() {
    let reader = vec![
        0xA_u8, 0x0, 0x0, 0x0, 0x0, 0x1, 0xc, 0x0, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
        0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x27, 0x0, 0x0,
    ];

    let map = HashMap::from([
        // (MapKeys::String("test".into()), 1.into()),
        (
            MapKeys::Uuid(uuid::Uuid::from_bytes([
                0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
                0xee, 0xff,
            ])),
            true.into(),
        ),
    ]);

    assert_eq!(GraphBinary::Map(map), from_slice(&reader).unwrap())
}

#[test]
fn test_uuid() {
    let reader = vec![
        0xc, 0x0, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc,
        0xdd, 0xee, 0xff,
    ];

    assert_eq!(
        GraphBinary::Uuid(Uuid::from_bytes([
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ])),
        from_slice(&reader).unwrap()
    )
}

#[test]
fn test_struct_from_gb() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct TestStruct {
        test: i32,
        abc: GraphBinary,
        milli: i16,
    }

    let gb = GraphBinary::Map(HashMap::from([
        ("test".into(), 1_i32.into()),
        ("abc".into(), GraphBinary::Boolean(true)),
        ("milli".into(), 1_i16.into()),
    ]));

    let expected = TestStruct {
        test: 1,
        abc: GraphBinary::Boolean(true),
        milli: 1,
    };
    let test_struct = crate::de::from_graph_binary(gb).unwrap();
    assert_eq!(expected, test_struct)
}
