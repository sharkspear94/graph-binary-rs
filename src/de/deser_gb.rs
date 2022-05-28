use serde::{
    de::{self, EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor},
    Deserialize,
};

use crate::{
    error::DecodeError,
    graph_binary::{Decode, GraphBinary},
    specs::CoreType,
    structure::{
        binding::Binding,
        edge::Edge,
        enums::{
            Barrier, Cardinality, Column, Direction, Merge, Operator, Order, Pick, Pop, Scope,
            TextP, P, T,
        },
        graph::Graph,
        metrics::{Metrics, TraversalMetrics},
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
        todo!()
    }

    // fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    // where
    //     E: serde::de::Error,
    // {
    //     match v {
    //         "vertex" => GraphBinary::Vertex(self.visit_newtype_struct(deserializer)),
    //     }
    // }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let core_type: CoreType = map.next_key()?.unwrap();
        // let value_flag: ValueFlag = map.next_key()?.unwrap();

        match core_type {
            CoreType::Edge => {
                let edge = map.next_value::<Edge>()?;
                Ok(GraphBinary::Edge(edge))
            }
            CoreType::Vertex => {
                let vertex = map.next_value::<Vertex>()?;
                Ok(GraphBinary::Vertex(vertex))
            }
            CoreType::Barrier => {
                let b = map.next_value::<Barrier>()?;
                Ok(GraphBinary::Barrier(b))
            }
            CoreType::Cardinality => {
                let c = map.next_value::<Cardinality>()?;
                Ok(GraphBinary::Cardinality(c))
            }
            CoreType::Column => {
                let c = map.next_value::<Column>()?;
                Ok(GraphBinary::Column(c))
            }
            CoreType::Direction => {
                let o = map.next_value::<Direction>()?;
                Ok(GraphBinary::Direction(o))
            }
            CoreType::Operator => {
                let o = map.next_value::<Operator>()?;
                Ok(GraphBinary::Operator(o))
            }
            CoreType::Order => {
                let o = map.next_value::<Order>()?;
                Ok(GraphBinary::Order(o))
            }
            CoreType::P => {
                let p = map.next_value::<P>()?;
                Ok(GraphBinary::P(p))
            }
            CoreType::T => {
                let t = map.next_value::<T>()?;
                Ok(GraphBinary::T(t))
            }
            CoreType::TextP => {
                let t = map.next_value::<TextP>()?;
                Ok(GraphBinary::TextP(t))
            }
            CoreType::Metrics => {
                let m = map.next_value::<Metrics>()?;
                Ok(GraphBinary::Metrics(m))
            }
            CoreType::TraversalMetrics => {
                let tm = map.next_value::<TraversalMetrics>()?;
                Ok(GraphBinary::TraversalMetrics(tm))
            }
            CoreType::Set => {
                let set = map.next_value::<Vec<GraphBinary>>()?;
                Ok(GraphBinary::Set(set))
            }
            CoreType::Int32 => todo!(),
            CoreType::Long => todo!(),
            CoreType::String => todo!(),
            CoreType::Class => todo!(),
            CoreType::Double => todo!(),
            CoreType::Float => todo!(),
            CoreType::List => todo!(),
            CoreType::Map => todo!(),
            CoreType::Uuid => todo!(),
            CoreType::Path => todo!(),
            CoreType::Property => todo!(),
            CoreType::Graph => {
                let b = map.next_value::<Graph>()?;
                Ok(GraphBinary::Graph(b))
            }
            CoreType::VertexProperty => todo!(),
            CoreType::Binding => {
                let b = map.next_value::<Binding>()?;
                Ok(GraphBinary::Binding(b))
            }
            CoreType::ByteCode => todo!(),
            CoreType::Pick => {
                let p = map.next_value::<Pick>()?;
                Ok(GraphBinary::Pick(p))
            }
            CoreType::Pop => {
                let p = map.next_value::<Pop>()?;
                Ok(GraphBinary::Pop(p))
            }
            CoreType::Lambda => todo!(),
            CoreType::Scope => {
                let s = map.next_value::<Scope>()?;
                Ok(GraphBinary::Scope(s))
            }
            CoreType::Traverser => todo!(),
            CoreType::Byte => todo!(),
            CoreType::ByteBuffer => todo!(),
            CoreType::Short => todo!(),
            CoreType::Boolean => todo!(),
            CoreType::TraversalStrategy => todo!(),
            CoreType::Tree => todo!(),
            CoreType::Merge => todo!(),
            CoreType::UnspecifiedNullObject => todo!(),
            // _ => todo!(),
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
        GraphBinary::P(P::Between(Box::new(1.into()), Box::new(10.into()))),
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
