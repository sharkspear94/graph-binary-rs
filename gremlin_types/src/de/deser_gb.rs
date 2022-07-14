use std::collections::HashMap;

use serde::{
    de::{MapAccess, Visitor},
    Deserialize,
};
use uuid::Uuid;

use crate::{
    graph_binary::{GremlinTypes, MapKeys},
    specs::CoreType,
    structure::{
        // binding::Binding,
        bytecode::Bytecode,
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
    Binding,
};

impl<'de> Deserialize<'de> for GremlinTypes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(GraphBinaryVisitor)
    }
}

pub struct GraphBinaryVisitor;

impl<'de> Visitor<'de> for GraphBinaryVisitor {
    type Value = GremlinTypes;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "enum GraphBinary")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(GremlinTypes::Boolean(v))
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(GremlinTypes::Char(v))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(GremlinTypes::Byte(v))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(GremlinTypes::Short(v))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(GremlinTypes::Int(v))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(GremlinTypes::Long(v))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(GremlinTypes::Float(v))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(GremlinTypes::Double(v))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut vec = if let Some(size) = seq.size_hint() {
            Vec::with_capacity(size)
        } else {
            Vec::new()
        };

        while let Some(item) = seq.next_element()? {
            vec.push(item)
        }

        Ok(GremlinTypes::List(vec))
    }

    fn visit_bytes<E>(self, _v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        todo!()
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
        Ok(GremlinTypes::UnspecifiedNullObject)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(GremlinTypes::UnspecifiedNullObject)
    }

    fn visit_enum<A>(self, _data: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::EnumAccess<'de>,
    {
        todo!()
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let core_type: CoreType = map.next_key()?.unwrap(); //  TODO unwrap

        match core_type {
            CoreType::Edge => Ok(GremlinTypes::Edge(map.next_value::<Edge>()?)),
            CoreType::Vertex => Ok(GremlinTypes::Vertex(map.next_value::<Vertex>()?)),
            CoreType::Barrier => Ok(GremlinTypes::Barrier(map.next_value::<Barrier>()?)),
            CoreType::Cardinality => {
                Ok(GremlinTypes::Cardinality(map.next_value::<Cardinality>()?))
            }
            CoreType::Column => Ok(GremlinTypes::Column(map.next_value::<Column>()?)),
            CoreType::Direction => Ok(GremlinTypes::Direction(map.next_value::<Direction>()?)),
            CoreType::Operator => Ok(GremlinTypes::Operator(map.next_value::<Operator>()?)),
            CoreType::Order => Ok(GremlinTypes::Order(map.next_value::<Order>()?)),
            CoreType::P => Ok(GremlinTypes::P(map.next_value::<P>()?)),
            CoreType::T => Ok(GremlinTypes::T(map.next_value::<T>()?)),
            CoreType::TextP => Ok(GremlinTypes::TextP(map.next_value::<TextP>()?)),
            CoreType::Metrics => Ok(GremlinTypes::Metrics(map.next_value::<Metrics>()?)),
            CoreType::TraversalMetrics => Ok(GremlinTypes::TraversalMetrics(
                map.next_value::<TraversalMetrics>()?,
            )),
            CoreType::Set => Ok(GremlinTypes::Set(map.next_value::<Vec<GremlinTypes>>()?)),
            CoreType::Int32 => todo!(),
            CoreType::Long => todo!(),
            CoreType::String => todo!(),
            CoreType::Class => todo!(),
            CoreType::Double => todo!(),
            CoreType::Float => todo!(),
            CoreType::List => todo!(),
            CoreType::Map => Ok(GremlinTypes::Map(
                map.next_value::<HashMap<MapKeys, GremlinTypes>>()?,
            )),
            CoreType::Uuid => Ok(GremlinTypes::Uuid(Uuid::from(map.next_value::<UuidDef>()?))),
            CoreType::Path => todo!(),
            CoreType::Property => Ok(GremlinTypes::Property(map.next_value::<Property>()?)),
            CoreType::Graph => Ok(GremlinTypes::Graph(map.next_value::<Graph>()?)),
            CoreType::VertexProperty => todo!(),
            CoreType::Binding => Ok(GremlinTypes::Binding(map.next_value::<Binding>()?)),
            CoreType::ByteCode => Ok(GremlinTypes::Bytecode(map.next_value::<Bytecode>()?)),
            CoreType::Pick => Ok(GremlinTypes::Pick(map.next_value::<Pick>()?)),
            CoreType::Pop => Ok(GremlinTypes::Pop(map.next_value::<Pop>()?)),
            CoreType::Lambda => Ok(GremlinTypes::Lambda(map.next_value::<Lambda>()?)),
            CoreType::Scope => Ok(GremlinTypes::Scope(map.next_value::<Scope>()?)),
            CoreType::Traverser => Ok(GremlinTypes::Traverser(map.next_value::<Traverser>()?)),
            CoreType::Byte => todo!(),
            CoreType::ByteBuffer => todo!(),
            CoreType::Short => todo!(),
            CoreType::Boolean => todo!(),
            CoreType::TraversalStrategy => Ok(GremlinTypes::TraversalStrategy(
                map.next_value::<TraversalStrategy>()?,
            )),
            CoreType::BulkSet => todo!(), //Ok(GraphBinary::BulkSet(
            //     map.next_value::<BulkSet>()?,
            // )),
            CoreType::Tree => todo!(),
            CoreType::Merge => Ok(GremlinTypes::Merge(map.next_value::<Merge>()?)),
            CoreType::UnspecifiedNullObject => todo!(),
            CoreType::Char => todo!(), // _ => todo!(),
        }
    }
}

#[test]
fn test_int() {
    use super::GBDeserializer;

    let buf = [0x01, 0x0, 0x0, 0x0, 0x0, 0x01];

    let gb = GremlinTypes::deserialize(&mut GBDeserializer { bytes: &buf });

    assert_eq!(GremlinTypes::Int(1), gb.unwrap())
}

#[test]
fn test_int_some() {
    use super::GBDeserializer;

    let buf = [0x0fe, 0x1, 0x0, 0x0, 0x0, 0x01];

    let gb = GremlinTypes::deserialize(&mut GBDeserializer { bytes: &buf });

    assert_eq!(GremlinTypes::UnspecifiedNullObject, gb.unwrap())
}

// #[test]
// fn test_t() {
//     let buf = [0x20, 0x0, 0x03, 0x0, 0x0, 0x0, 0x0, 0x02, b'i', b'd'];

//     let gb = from_slice::<GraphBinary>(&buf).unwrap();

//     assert_eq!(GraphBinary::T(T::Id), gb);
// }

// #[test]
// fn test_barrier() {
//     let buf = [
//         0x13, 0x0, 0x03, 0x0, 0x0, 0x0, 0x0, 0x08, b'n', b'o', b'r', b'm', b'S', b'a', b'c', b'k',
//     ];

//     let gb = from_slice::<GraphBinary>(&buf).unwrap();

//     assert_eq!(GraphBinary::Barrier(Barrier::NormSack), gb);
// }

// #[test]
// fn test_p() {
//     let buf = [
//         0x1e, 0x0, 0x03, 0x0, 0x0, 0x0, 0x0, 0x07, b'b', b'e', b't', b'w', b'e', b'e', b'n', 0x00,
//         0x00, 0x0, 0x2, 0x1, 0x0, 0x0, 0x0, 0x0, 0x1, 0x1, 0x0, 0x0, 0x0, 0x0, 0xa,
//     ];

//     let gb = from_slice::<GraphBinary>(&buf).unwrap();

//     assert_eq!(GraphBinary::P(P::between(1, 10)), gb);
// }

// #[test]
// fn test_order() {
//     let buf = [0x1a, 0x0, 0x03, 0x0, 0x0, 0x0, 0x0, 0x03, b'a', b's', b'c'];

//     let gb = from_slice::<GraphBinary>(&buf).unwrap();

//     assert_eq!(GraphBinary::Order(Order::Asc), gb);
// }

// #[test]
// fn text_p_deser() {
//     let reader = vec![
//         0x28, 0x00, 0x03, 0x0, 0x0, 0x0, 0x0, 0x0c, b's', b't', b'a', b'r', b't', b'i', b'n', b'g',
//         b'W', b'i', b't', b'h', 0x0, 0x0, 0x0, 0x01, 0x3, 0x0, 0x0, 0x0, 0x0, 0x04, b't', b'e',
//         b's', b't',
//     ];

//     let p = from_slice(&reader);

//     // assert!(p.is_ok());

//     assert_eq!(
//         GraphBinary::TextP(TextP::StartingWith(vec!["test".into()])),
//         p.unwrap()
//     );
// }

// #[test]
// fn test_vertex() {
//     let buf = [
//         0x11_u8, 0x0, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x6, 0x70,
//         0x65, 0x72, 0x73, 0x6f, 0x6e, 0xfe, 0x1,
//     ];

//     let gb2 = from_slice::<GraphBinary>(&buf).unwrap();

//     assert_eq!(
//         GraphBinary::Vertex(Vertex {
//             id: Box::new(1_i64.into()),
//             label: String::from("person"),
//             properties: None,
//         }),
//         gb2
//     );
// }

// #[test]
// fn test_vertex_struct() {
//     #[derive(Debug, Deserialize, PartialEq)]
//     struct TestStruct {
//         test: i32,
//         milli: i16,
//         x: GraphBinary,
//     }

//     let test = TestStruct {
//         test: 1,
//         milli: 1,
//         x: GraphBinary::Vertex(Vertex {
//             id: Box::new(1_i64.into()),
//             label: String::from("person"),
//             properties: None,
//         }),
//     };

//     let buf = [
//         0x0a_u8, 0x0, 0x00, 0x0, 0x0, 0x3, 0x03, 0x00, 0x00, 0x0, 0x0, 0x4, b't', b'e', b's', b't',
//         0x01, 0x00, 0x00, 0x0, 0x0, 0x1, 0x03, 0x00, 0x00, 0x0, 0x0, 0x1, b'x', 0x011_u8, 0x0, 0x2,
//         0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x6, 0x70, 0x65, 0x72, 0x73,
//         0x6f, 0x6e, 0xfe, 0x1, 0x03, 0x00, 0x00, 0x0, 0x0, 0x5, b'm', b'i', b'l', b'l', b'i', 0x26,
//         0x00, 0x00, 0x1,
//     ];

//     let res: TestStruct = crate::de::from_slice(&buf).unwrap();

//     assert_eq!(test, res)
// }

// #[test]
// fn test_seq() {
//     #[derive(Debug, Deserialize, PartialEq)]
//     struct TestStruct {
//         test: i32,
//         abc: GraphBinary,
//         milli: i16,
//     }

//     let test = TestStruct {
//         test: 1,
//         abc: GraphBinary::List(vec![1.into()]),
//         milli: 1,
//     };

//     let buf = [
//         0x0a_u8, 0x0, 0x00, 0x0, 0x0, 0x3, 0x03, 0x00, 0x00, 0x0, 0x0, 0x4, b't', b'e', b's', b't',
//         0x01, 0x00, 0x00, 0x0, 0x0, 0x1, 0x03, 0x00, 0x00, 0x0, 0x0, 0x3, b'a', b'b', b'c', 0x09,
//         0x0, 0x00, 0x00, 0x00, 0x01, 0x01, 0x0, 0x0, 0x0, 0x0, 0x1, 0x03, 0x00, 0x00, 0x0, 0x0,
//         0x5, b'm', b'i', b'l', b'l', b'i', 0x26, 0x00, 0x00, 0x1,
//     ];

//     let res: TestStruct = crate::de::from_slice(&buf).unwrap();

//     assert_eq!(test, res)
// }

// #[test]
// fn test_seq_set() {
//     #[derive(Debug, Deserialize, PartialEq)]
//     struct TestStruct {
//         test: i32,
//         abc: GraphBinary,
//         milli: i16,
//     }

//     let test = TestStruct {
//         test: 1,
//         abc: GraphBinary::Set(vec![1.into()]),
//         milli: 1,
//     };

//     let buf = [
//         0x0a_u8, 0x0, 0x00, 0x0, 0x0, 0x3, 0x03, 0x00, 0x00, 0x0, 0x0, 0x4, b't', b'e', b's', b't',
//         0x01, 0x00, 0x00, 0x0, 0x0, 0x1, 0x03, 0x00, 0x00, 0x0, 0x0, 0x3, b'a', b'b', b'c', 0x0b,
//         0x0, 0x00, 0x00, 0x00, 0x01, 0x01, 0x0, 0x0, 0x0, 0x0, 0x1, 0x03, 0x00, 0x00, 0x0, 0x0,
//         0x5, b'm', b'i', b'l', b'l', b'i', 0x26, 0x00, 0x00, 0x1,
//     ];

//     let res: TestStruct = crate::de::from_slice(&buf).unwrap();

//     assert_eq!(test, res)
// }

// #[test]
// fn test_map() {
//     let reader = vec![
//         0xA_u8, 0x0, 0x0, 0x0, 0x0, 0x1, 0x03, 0x0, 0x0, 0x0, 0x0, 0x4, b't', b'e', b's', b't',
//         0x01, 0x0, 0x0, 0x0, 0x0, 0x1,
//     ];

//     let map = HashMap::from([(MapKeys::String("test".into()), 1.into())]);

//     assert_eq!(GraphBinary::Map(map), from_slice(&reader).unwrap())
// }

// #[test]
// fn test_map_test() {
//     let reader = vec![
//         0xA_u8, 0x0, 0x0, 0x0, 0x0, 0x1, 0xc, 0x0, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
//         0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x27, 0x0, 0x0,
//     ];

//     let map = HashMap::from([
//         // (MapKeys::String("test".into()), 1.into()),
//         (
//             MapKeys::Uuid(uuid::Uuid::from_bytes([
//                 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
//                 0xee, 0xff,
//             ])),
//             true.into(),
//         ),
//     ]);

//     assert_eq!(GraphBinary::Map(map), from_slice(&reader).unwrap())
// }

// #[test]
// fn test_uuid() {
//     let reader = vec![
//         0xc, 0x0, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc,
//         0xdd, 0xee, 0xff,
//     ];

//     assert_eq!(
//         GraphBinary::Uuid(Uuid::from_bytes([
//             0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
//             0xee, 0xff,
//         ])),
//         from_slice(&reader).unwrap()
//     )
// }

#[test]
fn test_struct_from_gb() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct TestStruct {
        test: i32,
        abc: GremlinTypes,
        milli: i16,
    }

    let gb = GremlinTypes::Map(HashMap::from([
        ("test".into(), 1_i32.into()),
        ("abc".into(), GremlinTypes::Boolean(true)),
        ("milli".into(), 1_i16.into()),
    ]));

    let expected = TestStruct {
        test: 1,
        abc: GremlinTypes::Boolean(true),
        milli: 1,
    };
    let test_struct = crate::de::from_graph_binary(gb).unwrap();
    assert_eq!(expected, test_struct)
}
