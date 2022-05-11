use serde::{de::Visitor, Deserialize};

use crate::graph_binary::GraphBinary;

impl<'de> Deserialize<'de> for GraphBinary {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(GraphBinaryVisitor)
    }
}

struct GraphBinaryVisitor;

impl<'de> Visitor<'de> for GraphBinaryVisitor {
    type Value = GraphBinary;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a GraphBinary")
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
            D: serde::Deserializer<'de>, {
        deserializer.deserialize_any(self)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
            E: serde::de::Error, {
        Ok(GraphBinary::UnspecifiedNullObject)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
            E: serde::de::Error, {
        Ok(GraphBinary::UnspecifiedNullObject)
    }
}
