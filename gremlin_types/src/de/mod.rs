use std::collections::hash_map::IntoIter;
use std::io::Read;
use std::vec;

mod error;

use serde::de::{DeserializeOwned, MapAccess, SeqAccess, Visitor};
use serde::Deserialize;

use crate::error::DecodeError;
use crate::graph_binary::ValueFlag;
use crate::GremlinValue;

use crate::structure::map::MapKeys;
use crate::{graph_binary::Decode, specs::CoreType};

fn from_gremlin<'de, T: Deserialize<'de>>(g: GremlinValue) -> Result<T, DecodeError> {
    let de = GraphBinaryDeserializer(g);
    T::deserialize(de)
}

struct GraphBinaryDeserializer(GremlinValue);

impl<'de> serde::de::Deserializer<'de> for GraphBinaryDeserializer {
    type Error = DecodeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            GremlinValue::Int(v) => visitor.visit_i32(v),
            GremlinValue::Long(v) => visitor.visit_i64(v),
            GremlinValue::String(v) => visitor.visit_str(&v),
            GremlinValue::Float(v) => visitor.visit_f32(v),
            GremlinValue::Double(v) => visitor.visit_f64(v),
            GremlinValue::List(v) => visitor.visit_seq(SeqDeser {
                iter: v.into_iter(),
            }),
            GremlinValue::Map(v) => visitor.visit_map(MapDeser {
                size: v.len(),
                iter: v.into_iter(),
                value: None,
            }),
            GremlinValue::Set(v) => visitor.visit_seq(SeqDeser {
                iter: v.into_iter(),
            }),
            GremlinValue::Byte(v) => visitor.visit_u8(v),
            GremlinValue::Short(v) => visitor.visit_i16(v),
            GremlinValue::Boolean(v) => visitor.visit_bool(v),
            #[cfg(feature = "extended")]
            GremlinValue::Char(v) => visitor.visit_char(v),
            _ => Err(DecodeError::DecodeError(
                "Graphbinary not supported in deserialize_any".to_string(),
            )),
        }
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            GremlinValue::Map(v) => visitor.visit_map(MapDeser {
                size: v.len(),
                iter: v.into_iter(),
                value: None,
            }),
            _ => Err(DecodeError::DecodeError(
                "Graphbinary Deserializer only supports GraphBinary::Map in deserialize_struct"
                    .to_string(),
            )),
        }
    }

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq
        bytes byte_buf map option unit
        ignored_any unit_struct tuple_struct tuple enum identifier
    }
}

struct SeqDeser {
    iter: vec::IntoIter<GremlinValue>,
}

impl<'de> SeqAccess<'de> for SeqDeser {
    type Error = DecodeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if let Some(element) = self.iter.next() {
            let de = GraphBinaryDeserializer(element);
            seed.deserialize(de).map(Some)
        } else {
            Ok(None)
        }
    }
}

struct MapDeser {
    iter: IntoIter<MapKeys, GremlinValue>,
    value: Option<GremlinValue>,
    size: usize,
}

impl<'de> MapAccess<'de> for MapDeser {
    type Error = DecodeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if let Some((key, value)) = self.iter.next() {
            self.value = Some(value);
            self.size -= 1;

            let de = GraphBinaryDeserializer(key.into());
            seed.deserialize(de).map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let val = self
            .value
            .take()
            .ok_or_else(|| DecodeError::DecodeError("value without key".to_string()))?;
        let de = GraphBinaryDeserializer(val);
        seed.deserialize(de)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.size)
    }
}

#[test]
fn test_struct_from_gb() {
    use std::collections::HashMap;

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestStruct {
        test: Vec<u8>,
        abc: bool,
        milli: i16,
    }

    let gb = GremlinValue::Map(HashMap::from([
        ("test".into(), vec![0x01_u8, 2, 3].into()),
        ("abc".into(), true.into()),
        ("milli".into(), 1_i16.into()),
    ]));

    let expected = TestStruct {
        test: vec![1, 2, 3],
        abc: true,
        milli: 1,
    };
    let test_struct = crate::de::from_gremlin(gb).unwrap();
    assert_eq!(expected, test_struct)
}

#[test]
fn test_new_type_struct_from_gb() {
    use std::collections::HashMap;
    #[derive(Debug, Deserialize, PartialEq)]
    struct TestStruct {
        test: Vec<u8>,
        abc: Vec<i64>,
        milli: i16,
    }

    let gb = GremlinValue::Map(HashMap::from([
        ("test".into(), vec![0x01_u8, 2, 3].into()),
        ("abc".into(), vec![123, 321].into()),
        ("milli".into(), 123_i16.into()),
        ("other_field".into(), false.into()),
    ]));

    let expected = TestStruct {
        test: vec![1_u8, 2, 3],
        abc: vec![123, 321],
        milli: 123,
    };

    let test_struct = crate::de::from_gremlin(gb).unwrap();
    assert_eq!(expected, test_struct)
}

#[test]
fn struct_from_gb() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct TestStruct(Vec<u8>);

    let gb = vec![0x01_u8, 2, 3].into();

    let expected = TestStruct(vec![1_u8, 2, 3]);

    let test_struct = crate::de::from_gremlin(gb).unwrap();
    assert_eq!(expected, test_struct)
}
