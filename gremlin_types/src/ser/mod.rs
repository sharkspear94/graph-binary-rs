use std::collections::HashMap;

use serde::{
    ser::{Impossible, SerializeMap, SerializeSeq, SerializeStruct, SerializeTuple},
    Serialize,
};

use crate::{
    error::EncodeError,
    structure::{bytebuffer::ByteBuffer, map::MapKeys},
    GremlinValue,
};

struct GremlinValueSerializer;

pub fn to_graph_binary<T>(value: &T) -> Result<GremlinValue, EncodeError>
where
    T: Serialize,
{
    let serializer = GremlinValueSerializer {};
    value.serialize(serializer)
}

impl serde::Serializer for GremlinValueSerializer {
    type Ok = GremlinValue;
    type Error = EncodeError;

    type SerializeSeq = GraphBinarySerializerSeq;

    type SerializeTuple = GraphBinarySerializerSeq;

    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;

    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;

    type SerializeMap = GraphBinarySerializerMap;

    type SerializeStruct = GraphBinarySerializerMap;

    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(GremlinValue::Boolean(v))
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(GremlinValue::Short(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(GremlinValue::Int(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(GremlinValue::Long(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(GremlinValue::Byte(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(GremlinValue::Int(v as i32))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(GremlinValue::Long(v as i64))
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(GremlinValue::Float(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(GremlinValue::Double(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        #[cfg(not(feature = "extended"))]
        unimplemented!("serializing char needs the extended flag");
        #[cfg(feature = "extended")]
        Ok(GremlinValue::Char(v))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(GremlinValue::String(v.to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(GremlinValue::ByteBuffer(ByteBuffer::new(v)))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(GremlinValue::UnspecifiedNullObject)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(GremlinValue::UnspecifiedNullObject)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(GremlinValue::UnspecifiedNullObject) // TODO maybe string instead of null object
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(GremlinValue::String(variant.to_owned()))
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        match len {
            Some(l) => Ok(GraphBinarySerializerSeq(Vec::with_capacity(l))),
            None => Ok(GraphBinarySerializerSeq(Vec::new())),
        }
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        match len {
            Some(capacity) => Ok(GraphBinarySerializerMap {
                map: HashMap::with_capacity(capacity),
                key: None,
            }),
            None => Ok(GraphBinarySerializerMap {
                map: HashMap::new(),
                key: None,
            }),
        }
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

struct GraphBinarySerializerSeq(Vec<GremlinValue>);

impl SerializeSeq for GraphBinarySerializerSeq {
    type Ok = GremlinValue;

    type Error = EncodeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.0.push(value.serialize(GremlinValueSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(GremlinValue::List(self.0))
    }
}

impl SerializeTuple for GraphBinarySerializerSeq {
    type Ok = GremlinValue;

    type Error = EncodeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.0.push(value.serialize(GremlinValueSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(GremlinValue::List(self.0))
    }
}
struct GraphBinarySerializerMap {
    map: HashMap<MapKeys, GremlinValue>,
    key: Option<MapKeys>,
}

impl SerializeMap for GraphBinarySerializerMap {
    type Ok = GremlinValue;

    type Error = EncodeError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.key = Some(
            MapKeys::try_from(key.serialize(GremlinValueSerializer)?).map_err(|err| {
                EncodeError::Serilization(format!("cannot convert to MapKeys: {err}"))
            })?,
        );
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if let Some(key) = self.key.take() {
            let val = value.serialize(GremlinValueSerializer)?;
            self.map.insert(key, val);
            Ok(())
        } else {
            Err(EncodeError::Serilization(
                "serialiezed value befor key".to_string(),
            ))
        }
    }

    fn serialize_entry<K: ?Sized, V: ?Sized>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<(), Self::Error>
    where
        K: Serialize,
        V: Serialize,
    {
        let key = MapKeys::try_from(key.serialize(GremlinValueSerializer)?).map_err(|err| {
            EncodeError::Serilization(format!("cannot convert to MapKeys: {err}"))
        })?;
        self.map
            .insert(key, value.serialize(GremlinValueSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(GremlinValue::Map(self.map))
    }
}

impl SerializeStruct for GraphBinarySerializerMap {
    type Ok = GremlinValue;

    type Error = EncodeError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.map
            .insert(key.into(), value.serialize(GremlinValueSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(GremlinValue::Map(self.map))
    }
}

#[test]
fn struct_to_gb() {
    #[derive(Debug, Serialize)]
    struct TestStruct {
        test: i32,
        abc: bool,
    }

    let test = TestStruct { test: 1, abc: true };

    let gb = to_graph_binary(&test).unwrap();

    let map = HashMap::from([("test".into(), 1.into()), ("abc".into(), true.into())]);

    let expected = GremlinValue::Map(map);

    assert_eq!(expected, gb);
}

#[test]
fn struct_to_gb2() {
    #[derive(Debug, Serialize)]
    struct TestStruct {
        test: i32,
        abc: bool,
    }

    let test = TestStruct { test: 1, abc: true };

    let gb = to_graph_binary(&test).unwrap();

    let map = HashMap::from([("test".into(), 1.into()), ("abc".into(), true.into())]);

    let expected = GremlinValue::Map(map);

    assert_eq!(expected, gb);
}
