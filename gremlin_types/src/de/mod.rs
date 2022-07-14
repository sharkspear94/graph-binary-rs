use std::collections::hash_map::IntoIter;
use std::io::Read;
use std::vec;
mod deser_gb;

mod error;
pub mod graphson;

use serde::de::{DeserializeOwned, MapAccess, SeqAccess, Visitor};
use serde::Deserialize;

use crate::error::DecodeError;
use crate::graph_binary::ValueFlag;

use crate::{
    graph_binary::{Decode, GremlinTypes, MapKeys},
    specs::CoreType,
};

pub fn from_slice<'a, T: Deserialize<'a>>(slice: &'a [u8]) -> Result<T, DecodeError> {
    let mut deserializer = GBDeserializer::from_slice(slice);
    T::deserialize(&mut deserializer)
}

pub fn from_reader<R: Read, T: DeserializeOwned>(reader: &mut R) -> Result<T, DecodeError> {
    let mut buf: Vec<u8> = Vec::new();
    reader.read_to_end(&mut buf)?;
    from_slice(&buf[..])
}

pub fn from_graph_binary<'a, T: Deserialize<'a>>(gb: GremlinTypes) -> Result<T, DecodeError> {
    let de = GraphBinaryDeserializer(gb);
    T::deserialize(de)
}

pub struct GBDeserializer<'de> {
    bytes: &'de [u8],
}

impl<'de> GBDeserializer<'de> {
    fn from_slice(slice: &'de [u8]) -> Self {
        GBDeserializer { bytes: slice }
    }

    fn peek_identifier_tuple(&self) -> Result<(CoreType, ValueFlag), DecodeError> {
        if let Some([core, value_flag]) = self.bytes.get(0..2) {
            Ok((
                CoreType::try_from(*core)?,
                ValueFlag::try_from(*value_flag)?,
            ))
        } else {
            Err(DecodeError::DeserilizationError(
                "peeking type_code and value_flag faild, buffer not long enouth".to_string(),
            ))
        }
    }

    fn pop_identifier_tuple(&mut self) -> Result<(), DecodeError> {
        let mut buf = [255_u8; 2];
        self.bytes.read_exact(&mut buf)?;
        Ok(())
    }
}

struct VertexDeserializer<'a> {
    value_bytes: &'a [u8],
}

impl<'de> serde::Deserializer<'de> for VertexDeserializer<'de> {
    type Error = DecodeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bytes(self.value_bytes)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let mut deser = GBDeserializer::from_slice(self.value_bytes);
        deser.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let mut deser = GBDeserializer::from_slice(self.value_bytes);
        deser.deserialize_map(visitor)
    }

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string
        bytes byte_buf struct option unit newtype_struct
        ignored_any unit_struct tuple_struct tuple enum identifier
    }
}

impl<'de> serde::Deserializer<'de> for &mut GBDeserializer<'de> {
    type Error = DecodeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.peek_identifier_tuple()? {
            (_, ValueFlag::Null) => {
                // self.pop_identifier_tuple()?;
                visitor.visit_none()
            }
            (CoreType::Int32, _) => visitor.visit_i32(i32::decode(&mut self.bytes)?),
            (CoreType::Long, _) => visitor.visit_i64(i64::decode(&mut self.bytes)?),
            (CoreType::String, _) => visitor.visit_string(String::decode(&mut self.bytes)?),
            (CoreType::Class, _) => visitor.visit_string(String::decode(&mut self.bytes)?),
            (CoreType::Double, _) => visitor.visit_f64(f64::decode(&mut self.bytes)?),
            (CoreType::Float, _) => visitor.visit_f32(f32::decode(&mut self.bytes)?),
            (CoreType::List, _) => self.deserialize_seq(visitor),
            (CoreType::Set, _) => self.deserialize_seq(visitor),
            (CoreType::Map, _) => self.deserialize_map(visitor),
            (CoreType::Byte, _) => visitor.visit_u8(u8::decode(&mut self.bytes)?),
            (CoreType::Short, _) => visitor.visit_i16(i16::decode(&mut self.bytes)?),
            (CoreType::Boolean, _) => visitor.visit_bool(bool::decode(&mut self.bytes)?),
            (CoreType::UnspecifiedNullObject, _) => {
                self.pop_identifier_tuple()?;
                visitor.visit_none()
            }

            (_not_impl, _) => unimplemented!("Coretype is not implemented for Deserializer"), // TODO
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_bool(bool::decode(&mut self.bytes)?)
    }

    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i16(i16::decode(&mut self.bytes)?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i32(i32::decode(&mut self.bytes)?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i64(i64::decode(&mut self.bytes)?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u8(u8::decode(&mut self.bytes)?)
    }

    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_f32(f32::decode(&mut self.bytes)?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_f64(f64::decode(&mut self.bytes)?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_char(char::decode(&mut self.bytes)?)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_string(String::decode(&mut self.bytes)?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_string(String::decode(&mut self.bytes)?)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_bytes(self.bytes)
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.peek_identifier_tuple()? {
            (CoreType::UnspecifiedNullObject, ValueFlag::Null) => {
                self.pop_identifier_tuple()?;
                visitor.visit_none()
            }
            (CoreType::UnspecifiedNullObject, ValueFlag::Set) => Err(
                DecodeError::DeserilizationError("found [0xfe,0x0]".to_string()),
            ),
            (_, ValueFlag::Set) => visitor.visit_some(self),
            (_, ValueFlag::Null) => {
                self.pop_identifier_tuple()?;
                visitor.visit_none()
            }
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_none()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_none()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.pop_identifier_tuple()?;
        let len = i32::partial_decode(&mut self.bytes)? as usize;
        visitor.visit_seq(SeqAccessDeserializer {
            deserializer: self,
            size: len,
        })
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.pop_identifier_tuple()?;
        let len = i32::partial_decode(&mut self.bytes)? as usize;
        visitor.visit_map(SeqAccessDeserializer::new(self, len))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // self.deserialize_any(visitor)
        todo!()
    }
}

struct SeqAccessDeserializer<'a, 'de: 'a> {
    deserializer: &'a mut GBDeserializer<'de>,
    size: usize,
}

impl<'a, 'de> SeqAccessDeserializer<'a, 'de> {
    fn new(deserializer: &'a mut GBDeserializer<'de>, size: usize) -> Self {
        SeqAccessDeserializer { deserializer, size }
    }
}

impl<'de, 'a> SeqAccess<'de> for SeqAccessDeserializer<'a, 'de> {
    type Error = DecodeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        match self.size {
            0 => Ok(None),
            _ => {
                self.size -= 1;
                seed.deserialize(&mut *self.deserializer).map(Some)
            }
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.size)
    }
}

impl<'de, 'a> MapAccess<'de> for SeqAccessDeserializer<'a, 'de> {
    type Error = DecodeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        match self.size {
            0 => Ok(None),
            _ => {
                self.size -= 1;
                seed.deserialize(&mut *self.deserializer).map(Some)
            }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.deserializer)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.size)
    }
}

struct GraphBinaryDeserializer(GremlinTypes);

impl<'de> serde::de::Deserializer<'de> for GraphBinaryDeserializer {
    type Error = DecodeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            GremlinTypes::Int(v) => visitor.visit_i32(v),
            GremlinTypes::Long(v) => visitor.visit_i64(v),
            GremlinTypes::String(v) => visitor.visit_str(&v),
            GremlinTypes::Float(v) => visitor.visit_f32(v),
            GremlinTypes::Double(v) => visitor.visit_f64(v),
            GremlinTypes::Char(v) => visitor.visit_char(v),
            GremlinTypes::List(v) => visitor.visit_seq(SeqDeser {
                iter: v.into_iter(),
            }),
            GremlinTypes::Map(v) => visitor.visit_map(MapDeser {
                size: v.len(),
                iter: v.into_iter(),
                value: None,
            }),
            GremlinTypes::Set(v) => visitor.visit_seq(SeqDeser {
                iter: v.into_iter(),
            }),
            // GraphBinary::Vertex(v) => visitor.visit_map(map),
            GremlinTypes::Byte(v) => visitor.visit_u8(v),
            GremlinTypes::Short(v) => visitor.visit_i16(v),
            GremlinTypes::Boolean(v) => visitor.visit_bool(v),
            // GraphBinary::Traverser(t) =>
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
            GremlinTypes::Map(v) => visitor.visit_map(MapDeser {
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
    iter: vec::IntoIter<GremlinTypes>,
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
    iter: IntoIter<MapKeys, GremlinTypes>,
    value: Option<GremlinTypes>,
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
fn test_bool() {
    let reader = vec![0x27_u8, 0x0, 0x0];

    let bool: Result<bool, DecodeError> = from_slice(&reader);

    assert!(bool.unwrap())
}

#[test]
fn test_bool_option_some() {
    let reader = vec![0x27_u8, 0x0, 0x0];

    let val = from_slice(&reader).unwrap();

    assert_eq!(Some(true), val)
}

#[test]
fn test_i32() {
    let reader = vec![0x1_u8, 0x0, 0x0, 0x0, 0x0, 0x8];

    let val: Result<i32, _> = from_slice(&reader);

    assert_eq!(8, val.unwrap())
}

#[test]
fn test_newtype_struct() {
    let reader = vec![0x1_u8, 0x0, 0x0, 0x0, 0x0, 0x8];

    #[derive(Debug, Deserialize, PartialEq)]
    struct Milli(i32);

    let val = from_slice(&reader).unwrap();

    assert_eq!(Milli(8), val)
}

#[test]
fn test_option_none() {
    let reader = vec![0x1_u8, 0x1, 0x0, 0x0, 0x0, 0x8];

    let val: Option<i32> = from_slice(&reader).unwrap();

    assert_eq!(None, val)
}

#[test]
fn test_i32_option_some() {
    let reader = vec![0x1_u8, 0x0, 0x0, 0x0, 0x0, 0x8];

    let val: Option<i32> = from_slice(&reader).unwrap();

    assert_eq!(Some(8), val)
}

#[test]
fn test_seq() {
    let reader = vec![
        0x9_u8, 0x0, 0x0, 0x0, 0x0, 0x2, 0x1, 0x0, 0x0, 0x0, 0x0, 0xff, 0x1_u8, 0x0, 0x0, 0x0, 0x0,
        0xff,
    ];

    let val: Vec<i32> = from_slice(&reader).unwrap();

    assert_eq!(vec![255, 255], val)
}

#[test]
fn test_seq_array() {
    let reader = vec![
        0x9_u8, 0x0, 0x0, 0x0, 0x0, 0x2, 0x1, 0x0, 0x0, 0x0, 0x0, 0xff, 0x1_u8, 0x0, 0x0, 0x0, 0x0,
        0xff,
    ];

    let val: [i32; 2] = from_slice(&reader).unwrap();

    assert_eq!([255, 255], val)
}

#[test]
fn test_seq_set() {
    use std::collections::HashSet;

    let reader = vec![
        0x0b_u8, 0x0, 0x0, 0x0, 0x0, 0x2, 0x1, 0x0, 0x0, 0x0, 0x0, 0xfe, 0x1_u8, 0x0, 0x0, 0x0,
        0x0, 0xff,
    ];

    let val: HashSet<i32> = from_slice(&reader).unwrap();

    assert_eq!(HashSet::from([254, 255]), val)
}

#[test]
fn test_char() {
    let reader = [0x80_u8, 0x0, 0xf0, 0x9f, 0xa6, 0x80];

    let val: char = from_slice(&reader).unwrap();

    assert_eq!('🦀', val)
}

#[test]
fn test_empty_seq() {
    let reader = vec![0x9_u8, 0x0, 0x0, 0x0, 0x0, 0x0];
    let val: Vec<i32> = from_slice(&reader).unwrap();
    let empty_vec: Vec<i32> = Vec::new();
    assert_eq!(empty_vec, val)
}

#[test]
fn test_tuple() {
    use std::io::Write;

    let mut file = std::fs::File::create("testfile").unwrap();
    file.write_all(&[
        0x09_u8, 0x0, 0x0, 0x0, 0x0, 0x2, 0x1, 0x0, 0x0, 0x0, 0x0, 0xff, 0x1, 0x0, 0x0, 0x0, 0x0,
        0xff,
    ])
    .unwrap();

    let mut file = std::fs::File::open("testfile").unwrap();
    let tuple = from_reader(&mut file).unwrap();

    std::fs::remove_file("testfile").unwrap();

    assert_eq!((255, 255), tuple)
}

#[test]
fn test_map() {
    use std::collections::HashMap;

    let reader = vec![
        0xA_u8, 0x0, 0x0, 0x0, 0x0, 0x2, 0x01, 0x0, 0x0, 0x0, 0x00, 0x01, 0x27_u8, 0x0, 0x0, 0x01,
        0x0, 0x0, 0x0, 0x00, 0xff, 0x27_u8, 0x0, 0x1,
    ];
    let val: HashMap<i32, bool> = from_slice(&reader).unwrap();
    let map = HashMap::from([(255, false), (1, true)]);
    assert_eq!(map, val)
}

#[test]
fn test_empty_map() {
    use std::collections::HashMap;

    let reader = vec![0xA_u8, 0x0, 0x0, 0x0, 0x0, 0x0];
    let val: HashMap<bool, i32> = from_slice(&reader).unwrap();
    let empty_map: HashMap<bool, i32> = HashMap::new();
    assert_eq!(empty_map, val)
}

#[test]
fn test_struct() {
    let reader = vec![
        0xA_u8, 0x0, 0x0, 0x0, 0x0, 0x1, 0x03, 0x0, 0x0, 0x0, 0x0, 0x4, b't', b'e', b's', b't',
        0x01, 0x0, 0x0, 0x0, 0x0, 0x1,
    ];

    #[derive(Debug, Deserialize, PartialEq)]
    struct Milli {
        test: i32,
    }

    let val = from_slice(&reader).unwrap();

    assert_eq!(Milli { test: 1 }, val)
}

#[test]
fn test_struct_option() {
    let reader = vec![
        0xA_u8, 0x0, 0x0, 0x0, 0x0, 0x1, 0x03, 0x0, 0x0, 0x0, 0x0, 0x4, b't', b'e', b's', b't',
        0x01, 0x0, 0x0, 0x0, 0x0, 0x1,
    ];

    #[derive(Debug, Deserialize, PartialEq)]
    struct Milli {
        test: i32,
    }

    let val = from_slice(&reader).unwrap();

    assert_eq!(Some(Milli { test: 1 }), val)
}

#[test]
fn test_struct_inner_option() {
    let reader = vec![
        0xA_u8, 0x0, 0x0, 0x0, 0x0, 0x2, 0x03, 0x0, 0x0, 0x0, 0x0, 0x3, b'a', b'b', b'c', 0xfe,
        0x1, 0x03, 0x0, 0x0, 0x0, 0x0, 0x4, b't', b'e', b's', b't', 0x01, 0x0, 0x0, 0x0, 0x0, 0x1,
    ];

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestStruct {
        abc: Option<i32>,
        test: i32,
    }

    let val = from_slice(&reader).unwrap();

    assert_eq!(TestStruct { abc: None, test: 1 }, val)
}

#[test]
fn test_tuple_struct() {
    let reader = vec![
        0x9_u8, 0x0, 0x0, 0x0, 0x0, 0x2, 0x03, 0x0, 0x0, 0x0, 0x0, 0x3, b'a', b'b', b'c', 0x01,
        0x0, 0x0, 0x0, 0x0, 0x1,
    ];

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestStruct(Option<String>, i32);

    let val = from_slice(&reader).unwrap();

    assert_eq!(TestStruct(Some(String::from("abc")), 1), val)
}

#[test]
fn test_tuple_2() {
    let reader = [
        0x09_u8, 0x0, 0x00, 0x0, 0x0, 0x2, 0x03, 0x00, 0x00, 0x0, 0x0, 0x4, b't', b'e', b's', b't',
        0x01_u8, 0x0, 0x00, 0x0, 0x0, 0x1,
    ];

    let res = from_reader::<&[u8], (String, i32)>(&mut &reader[..]).unwrap();

    assert_eq!(res, ("test".to_string(), 1))
}

#[test]
fn test_ser_deser() {
    use super::ser::to_bytes;
    use serde::Serialize;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestStruct {
        abc: Option<i32>,
        test: i32,
    }

    let test_struct = TestStruct {
        abc: Some(6),
        test: -1,
    };

    let res = to_bytes(test_struct).unwrap();

    let val = from_slice(&res).unwrap();

    assert_eq!(
        TestStruct {
            abc: Some(6),
            test: -1
        },
        val
    )
}

// #[test]
// fn test_ser_deser_gb() {
//     #[derive(Debug, Serialize, Deserialize, PartialEq)]
//     struct TestStruct {
//         abc: Option<i32>,
//         test: GraphBinary,
//     }

//     let test_struct = TestStruct {
//         abc: Some(6),
//         test: GraphBinary::Column(Column::Keys),
//     };

//     let res = to_bytes(test_struct).unwrap();

//     let val = from_slice(&res).unwrap();

//     assert_eq!(
//         TestStruct {
//             abc: Some(6),
//             test: GraphBinary::Column(Column::Keys)
//         },
//         val
//     )
// }

#[test]
fn test_struct_from_gb() {
    use std::collections::HashMap;

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestStruct {
        test: Vec<u8>,
        abc: GremlinTypes,
        milli: i16,
    }

    let gb = GremlinTypes::Map(HashMap::from([
        ("test".into(), vec![0x01_u8, 2, 3].into()),
        ("abc".into(), GremlinTypes::Boolean(true)),
        ("milli".into(), 1_i16.into()),
    ]));

    let expected = TestStruct {
        test: vec![1, 2, 3],
        abc: GremlinTypes::Boolean(true),
        milli: 1,
    };
    let test_struct = crate::de::from_graph_binary(gb).unwrap();
    assert_eq!(expected, test_struct)
}

#[test]
fn test_new_type_struct_from_gb() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct TestStruct(Vec<u8>);

    let gb = vec![0x01_u8, 2, 3].into();

    let expected = TestStruct(vec![1_u8, 2, 3]);

    let test_struct = crate::de::from_graph_binary(gb).unwrap();
    assert_eq!(expected, test_struct)
}
