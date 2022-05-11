use std::collections::HashMap;
use std::io::Read;
mod deser_gb;

use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::Deserialize;

use crate::error::DecodeError;
use crate::graph_binary::{decode, ValueFlag};
use crate::structure::primitivs;
use crate::{
    error::EncodeError,
    graph_binary::{Decode, GraphBinary, MapKeys},
    specs::CoreType,
    structure::enums::T,
    // structure::{enums::T, list::List},
};

// pub fn from_reader<'a, R: Read, T: Deserialize<'a>>(reader: R) -> Result<T, DecodeError> {
//     let deserializer = Deserializer::new(reader);
//     T::deserialize(deserializer)
// }

pub fn from_slice<'a, T: Deserialize<'a>>(slice: &'a [u8]) -> Result<T, DecodeError> {
    let mut deserializer = Deserializer::from_slice(slice);
    T::deserialize(&mut deserializer)
}

pub struct Deserializer<'de> {
    bytes: &'de [u8],

    current_type: CoreType,
}

impl<'de> Deserializer<'de> {
    // fn from_reader<R: Read>(reader: R) -> Self {
    //     Deserializer { bytes: reader.read_to_end(buf) }
    // }

    fn from_slice(slice: &'de [u8]) -> Self {
        Deserializer {
            bytes: slice,
            current_type: CoreType::None,
        }
    }

    fn get_type_info(&mut self) -> Result<(CoreType, ValueFlag), DecodeError> {
        let mut buf = [255_u8; 2];
        self.bytes.read_exact(&mut buf)?;

        Ok((CoreType::try_from(buf[0])?, ValueFlag::try_from(buf[1])?))
    }

    fn peek_type_value_flag(&self) -> Result<(CoreType, ValueFlag), DecodeError> {
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

    fn next_core_type(&mut self) {
        let coretype = self.get_type_info().unwrap(); //TODO cleanup
        self.current_type = coretype.0
    }

    fn visit_distribution<V: Visitor<'de>>(
        &mut self,
        core_type: CoreType,
        visitor: V,
    ) -> Result<V::Value, DecodeError> {
        match core_type {
            CoreType::Int32 => visitor.visit_i32(i32::partial_decode(&mut self.bytes)?),
            CoreType::Long => visitor.visit_i64(i64::partial_decode(&mut self.bytes)?),
            CoreType::String => visitor.visit_string(String::partial_decode(&mut self.bytes)?),
            CoreType::Class => visitor.visit_string(String::partial_decode(&mut self.bytes)?),
            CoreType::Double => visitor.visit_f64(f64::partial_decode(&mut self.bytes)?),
            CoreType::Float => visitor.visit_f32(f32::partial_decode(&mut self.bytes)?),
            CoreType::List => todo!(),
            CoreType::Set => todo!(),
            CoreType::Map => todo!(),
            CoreType::Uuid => todo!(),
            CoreType::Edge => todo!(),
            CoreType::Path => todo!(),
            CoreType::Property => todo!(),
            CoreType::Graph => todo!(),
            CoreType::Vertex => todo!(),
            CoreType::VertexProperty => todo!(),
            CoreType::Barrier => todo!(),
            CoreType::Binding => todo!(),
            CoreType::ByteCode => todo!(),
            CoreType::Cardinality => todo!(),
            CoreType::Column => todo!(),
            CoreType::Direction => todo!(),
            CoreType::Operator => todo!(),
            CoreType::Order => todo!(),
            CoreType::Pick => todo!(),
            CoreType::Pop => todo!(),
            CoreType::Lambda => todo!(),
            CoreType::P => todo!(),
            CoreType::Scope => todo!(),
            CoreType::T => todo!(),
            CoreType::Traverser => todo!(),
            CoreType::Byte => todo!(),
            CoreType::ByteBuffer => todo!(),
            CoreType::Short => todo!(),
            CoreType::Boolean => todo!(),
            CoreType::TextP => todo!(),
            CoreType::TraversalStrategy => todo!(),
            CoreType::Tree => todo!(),
            CoreType::Metrics => todo!(),
            CoreType::TraversalMetrics => todo!(),
            CoreType::Merge => todo!(),
            CoreType::UnspecifiedNullObject => todo!(),
            CoreType::None => todo!(),
        }
    }
}

// impl<R: Read> Deserializer<R> {
//     fn parse_bool(&mut self) -> Result<>{

//     }
// }

impl<'de, 'a> serde::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = DecodeError;

    fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut buf = [255_u8; 2];
        self.bytes.read_exact(&mut buf)?;

        match self.get_type_info()? {
            (_, ValueFlag::Null) => visitor.visit_none(),

            (CoreType::Int32, _) => visitor.visit_i32(i32::partial_decode(&mut self.bytes)?),
            (CoreType::Long, _) => visitor.visit_i64(i64::partial_decode(&mut self.bytes)?),
            (CoreType::String, _) => visitor.visit_string(String::partial_decode(&mut self.bytes)?),

            (CoreType::Class, _) => visitor.visit_string(String::partial_decode(&mut self.bytes)?),
            (CoreType::Double, _) => visitor.visit_f64(f64::partial_decode(&mut self.bytes)?),
            (CoreType::Float, _) => visitor.visit_f32(f32::partial_decode(&mut self.bytes)?),
            // (CoreType::List, _) => visitor.visit_seq(todo!()),
            // (CoreType::Map, _) => visitor.visit_map(todo!()),
            // (CoreType::Set, _) => visitor.visit_seq(todo!()),
            // (CoreType::Uuid, _) => visitor.visit_seq(todo!()),
            // (CoreType::Edge, _) => visitor.vis(todo!()),
            (CoreType::Byte, _) => visitor.visit_u8(u8::partial_decode(&mut self.bytes)?),
            (CoreType::ByteBuffer, _) => visitor.visit_u8(u8::partial_decode(&mut self.bytes)?),
            (CoreType::Short, _) => visitor.visit_i16(i16::partial_decode(&mut self.bytes)?),
            (CoreType::Boolean, _) => visitor.visit_bool(bool::partial_decode(&mut self.bytes)?),

            (CoreType::UnspecifiedNullObject, _) => visitor.visit_none(),

            (_, _) => Err(DecodeError::DeserilizationError("coretype error".into())), // TODO
        }
    }

    fn deserialize_bool<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_bool(bool::fully_self_decode(&mut self.bytes)?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i32(i32::fully_self_decode(&mut self.bytes)?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i64(i64::fully_self_decode(&mut self.bytes)?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u8(u8::fully_self_decode(&mut self.bytes)?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f32<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_f32(f32::fully_self_decode(&mut self.bytes)?)
    }

    fn deserialize_f64<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_f64(f64::fully_self_decode(&mut self.bytes)?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_string(String::fully_self_decode(&mut self.bytes)?)
    }

    fn deserialize_string<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_string(String::fully_self_decode(&mut self.bytes)?)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.peek_type_value_flag()? {
            (CoreType::UnspecifiedNullObject, ValueFlag::Null) => visitor.visit_none(),
            (CoreType::UnspecifiedNullObject, ValueFlag::Set) => Err(
                DecodeError::DeserilizationError("found [0xfe,0x0]".to_string()),
            ),
            (_, ValueFlag::Set) => visitor.visit_some(self),
            (_, ValueFlag::Null) => visitor.visit_none(),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
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
        match self.get_type_info()? {
            (CoreType::List, ValueFlag::Set) => {
                let len = i32::partial_decode(&mut self.bytes)? as usize;
                visitor.visit_seq(SeqAccessDe {
                    deserializer: self,
                    size: len,
                })
            }
            (type_code, _) => Err(DecodeError::DeserilizationError(format!(
                "deserialize seq with typecode {}",
                u8::from(type_code)
            ))),
        }
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.get_type_info()? {
            (CoreType::Map, ValueFlag::Set) => {
                let len = i32::partial_decode(&mut self.bytes)? as usize;
                visitor.visit_map(SeqAccessDe {
                    deserializer: self,
                    size: len,
                })
            }
            (core_type, _) => Err(DecodeError::DeserilizationError(format!(
                "deserialize map with typecode {:#X}",
                u8::from(core_type)
            ))),
        }
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
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

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }
}

struct SeqAccessDe<'a, 'de: 'a> {
    deserializer: &'a mut Deserializer<'de>,
    size: usize,
}

impl<'a, 'de> SeqAccessDe<'a, 'de> {
    fn new(deserializer: &'a mut Deserializer<'de>, size: usize) -> Self {
        SeqAccessDe { deserializer, size }
    }
}

impl<'de, 'a> SeqAccess<'de> for SeqAccessDe<'a, 'de> {
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

impl<'de, 'a> MapAccess<'de> for SeqAccessDe<'a, 'de> {
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
fn test_empty_seq() {
    let reader = vec![0x9_u8, 0x0, 0x0, 0x0, 0x0, 0x0];
    let val: Vec<i32> = from_slice(&reader).unwrap();
    let empty_vec: Vec<i32> = Vec::new();
    assert_eq!(empty_vec, val)
}

#[test]
fn test_tuple() {
    let reader = vec![
        0x9_u8, 0x0, 0x0, 0x0, 0x0, 0x2, 0x1, 0x0, 0x0, 0x0, 0x0, 0xff, 0x1_u8, 0x0, 0x0, 0x0, 0x0,
        0xff,
    ];

    let tuple: (i32, i32) = from_slice(&reader).unwrap();

    assert_eq!((255, 255), tuple)
}

#[test]
fn test_map() {
    let reader = vec![
        0xA_u8, 0x0, 0x0, 0x0, 0x0, 0x2, 0x01, 0x0, 0x0, 0x0, 0x00, 0xff, 0x27_u8, 0x0, 0x1, 0x01,
        0x0, 0x0, 0x0, 0x00, 0x01, 0x27_u8, 0x0, 0x0,
    ];
    let val: HashMap<i32, bool> = from_slice(&reader).unwrap();
    let map = HashMap::from([(255, false), (1, true)]);
    assert_eq!(map, val)
}

#[test]
fn test_empty_map() {
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
fn test_struct_nested() {
    let reader = vec![
        0xA_u8, 0x0, 0x0, 0x0, 0x0, 0x1, 0x03, 0x0, 0x0, 0x0, 0x0, 0x4, b't', b'e', b's', b't',
        0xA_u8, 0x0, 0x0, 0x0, 0x0, 0x1, 0x03, 0x0, 0x0, 0x0, 0x0, 0x4, b't', b'e', b's', b't',
        0x01, 0x0, 0x0, 0x0, 0x0, 0x1,
    ];

    #[derive(Debug, Deserialize, PartialEq)]
    struct Milli {
        test: i32,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct MilliNested {
        test: Milli,
    }

    let val = from_slice(&reader).unwrap();

    assert_eq!(
        MilliNested {
            test: Milli { test: 1 }
        },
        val
    )
}
