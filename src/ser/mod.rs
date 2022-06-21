use std::{collections::HashMap, io::Write};

use serde::{
    ser::{self, Impossible, SerializeMap, SerializeSeq, SerializeStruct, SerializeTuple},
    Serialize,
};

use crate::{
    error::EncodeError,
    graph_binary::{Encode, GraphBinary, MapKeys},
    specs::CoreType,
    structure::enums::T,
    // structure::{enums::T, list::List},
};

pub fn to_bytes<T>(value: T) -> Result<Vec<u8>, EncodeError>
where
    T: Serialize,
{
    let mut serializer = Serializer { writer: Vec::new() };
    value.serialize(&mut serializer)?;
    Ok(serializer.writer)
}

struct Serializer {
    writer: Vec<u8>,
}

impl Serializer {
    fn new() -> Self {
        Serializer { writer: Vec::new() }
    }
}

impl ser::Serializer for &mut Serializer {
    type Ok = ();

    type Error = EncodeError;

    type SerializeSeq = Self;

    type SerializeTuple = Self;

    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;

    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;

    type SerializeMap = Self;

    type SerializeStruct = Self;

    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        v.write_full_qualified_bytes(&mut self.writer)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        v.write_full_qualified_bytes(&mut self.writer)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        v.write_full_qualified_bytes(&mut self.writer)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        v.write_full_qualified_bytes(&mut self.writer)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        v.write_full_qualified_bytes(&mut self.writer)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        v.write_full_qualified_bytes(&mut self.writer)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        v.write_full_qualified_bytes(&mut self.writer)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(v)?;
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        GraphBinary::UnspecifiedNullObject.build_fq_bytes(&mut self.writer)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        GraphBinary::UnspecifiedNullObject.build_fq_bytes(&mut self.writer)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        GraphBinary::UnspecifiedNullObject.build_fq_bytes(&mut self.writer) // not sure if correct
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
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
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        if let Some(len) = len {
            let len = i32::try_from(len)?;
            let len = len.to_be_bytes();
            let list_start = [0x09_u8, 0x00, len[0], len[1], len[2], len[3]];
            self.writer.extend_from_slice(&list_start);
            Ok(self)
        } else {
            Err(EncodeError::SerilizationError(
                "unknown seq size".to_string(),
            ))
        }
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        if let Some(len) = len {
            let len = i32::try_from(len)?;
            let len = len.to_be_bytes();
            let map_start = [CoreType::Map.into(), 0x00, len[0], len[1], len[2], len[3]];
            self.writer.extend_from_slice(&map_start);
            Ok(self)
        } else {
            Err(EncodeError::SerilizationError(
                "unknown map size".to_string(),
            ))
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
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

// struct SeqSerializer  {
//     inner: Vec<E>,
// }

impl SerializeSeq for &mut Serializer {
    type Ok = ();

    type Error = EncodeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        // self.inner
        //     .push(value.serialize(GraphBinarySerializer::new())?);
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl SerializeTuple for &mut Serializer {
    type Ok = ();

    type Error = EncodeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl SerializeMap for &mut Serializer {
    type Ok = ();

    type Error = EncodeError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        key.serialize(&mut **self)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl SerializeStruct for &mut Serializer {
    type Ok = ();

    type Error = EncodeError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

struct GraphBinarySerializer;

impl SerializeStruct for GraphBinarySerializer {
    type Ok = GraphBinary;

    type Error = EncodeError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl GraphBinarySerializer {
    fn new() -> GraphBinarySerializer {
        GraphBinarySerializer {}
    }
}

pub fn to_graph_binary<T>(value: &T) -> Result<GraphBinary, EncodeError>
where
    T: Serialize,
{
    let serializer = GraphBinarySerializer {};
    value.serialize(serializer)
}

impl serde::Serializer for GraphBinarySerializer {
    type Ok = GraphBinary;
    type Error = EncodeError;

    type SerializeSeq = GraphBinarySerializerSeq;

    type SerializeTuple = GraphBinarySerializerSeq;

    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;

    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;

    type SerializeMap = GraphBinarySerializerMap;

    type SerializeStruct = GraphBinarySerializerMap;

    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(GraphBinary::Boolean(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(GraphBinary::Short(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(GraphBinary::Int(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(GraphBinary::Long(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(GraphBinary::Byte(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(GraphBinary::Float(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(GraphBinary::Double(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(GraphBinary::Char(v))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(GraphBinary::String(v.to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(GraphBinary::ByteBuffer(Vec::from_iter(v.to_owned())))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(GraphBinary::UnspecifiedNullObject)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(GraphBinary::UnspecifiedNullObject)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(GraphBinary::UnspecifiedNullObject)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
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
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
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
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        match len {
            Some(capacity) => Ok(GraphBinarySerializerMap(HashMap::with_capacity(capacity))),
            None => Ok(GraphBinarySerializerMap(HashMap::new())),
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
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

impl Serialize for GraphBinary {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            GraphBinary::Int(value) => value.serialize(serializer),
            GraphBinary::Long(value) => value.serialize(serializer),
            GraphBinary::String(value) => value.serialize(serializer),
            GraphBinary::Class(value) => value.serialize(serializer),
            GraphBinary::Double(value) => value.serialize(serializer),
            GraphBinary::Float(value) => value.serialize(serializer),
            GraphBinary::List(value) => value.serialize(serializer),
            GraphBinary::Set(value) => value.serialize(serializer),
            GraphBinary::Map(value) => value.serialize(serializer),
            GraphBinary::Uuid(value) => todo!(),
            GraphBinary::Edge(value) => value.serialize(serializer),
            GraphBinary::Path(value) => todo!(),
            GraphBinary::Property(value) => value.serialize(serializer),
            GraphBinary::Graph(value) => value.serialize(serializer),
            GraphBinary::Vertex(value) => value.serialize(serializer),
            GraphBinary::VertexProperty(value) => value.serialize(serializer),
            GraphBinary::Barrier(value) => value.serialize(serializer),
            GraphBinary::Binding(value) => value.serialize(serializer),
            GraphBinary::ByteCode(value) => value.serialize(serializer),
            GraphBinary::Cardinality(value) => value.serialize(serializer),
            GraphBinary::Column(value) => value.serialize(serializer),
            GraphBinary::Direction(value) => value.serialize(serializer),
            GraphBinary::Operator(value) => value.serialize(serializer),
            GraphBinary::Order(value) => value.serialize(serializer),
            GraphBinary::Pick(value) => value.serialize(serializer),
            GraphBinary::Pop(value) => value.serialize(serializer),
            GraphBinary::Lambda(value) => value.serialize(serializer),
            GraphBinary::P(value) => value.serialize(serializer),
            GraphBinary::Scope(value) => value.serialize(serializer),
            GraphBinary::T(value) => value.serialize(serializer),
            GraphBinary::Traverser(value) => value.serialize(serializer),
            GraphBinary::Byte(value) => value.serialize(serializer),
            GraphBinary::ByteBuffer(value) => value.serialize(serializer),
            GraphBinary::Short(value) => value.serialize(serializer),
            GraphBinary::Boolean(value) => value.serialize(serializer),
            GraphBinary::TextP(value) => value.serialize(serializer),
            GraphBinary::TraversalStrategy(value) => value.serialize(serializer),
            GraphBinary::Tree(value) => todo!(),
            GraphBinary::Metrics(value) => value.serialize(serializer),
            GraphBinary::TraversalMetrics(value) => value.serialize(serializer),
            GraphBinary::BulkSet(value) => todo!(),
            GraphBinary::UnspecifiedNullObject => serializer.serialize_none(),
            GraphBinary::Merge(value) => value.serialize(serializer),
            GraphBinary::Char(value) => value.serialize(serializer),
        }
    }
}
struct GraphBinarySerializerSeq(Vec<GraphBinary>);

impl SerializeSeq for GraphBinarySerializerSeq {
    type Ok = GraphBinary;

    type Error = EncodeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.0.push(value.serialize(GraphBinarySerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(GraphBinary::List(self.0))
    }
}

impl SerializeTuple for GraphBinarySerializerSeq {
    type Ok = GraphBinary;

    type Error = EncodeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.0.push(value.serialize(GraphBinarySerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(GraphBinary::List(self.0))
    }
}
struct GraphBinarySerializerMap(HashMap<MapKeys, GraphBinary>);

impl SerializeMap for GraphBinarySerializerMap {
    type Ok = GraphBinary;

    type Error = EncodeError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
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
        let key = MapKeys::try_from(key.serialize(GraphBinarySerializer)?).unwrap(); // TODO tryFrom needs own Error
        self.0.insert(key, value.serialize(GraphBinarySerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(GraphBinary::Map(self.0))
    }
}

impl SerializeStruct for GraphBinarySerializerMap {
    type Ok = GraphBinary;

    type Error = EncodeError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.0
            .insert(key.into(), value.serialize(GraphBinarySerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(GraphBinary::Map(self.0))
    }
}

#[test]
fn ser_test() {
    let bytes = to_bytes(3);
    assert_eq!([0x01_u8, 0x0, 0x00, 0x0, 0x0, 0x3][..], bytes.unwrap());
}

#[test]
fn ser_seq_test() {
    let bytes = to_bytes(vec![1, 2]);
    assert_eq!(
        [
            0x09_u8, 0x0, 0x00, 0x0, 0x0, 0x2, 0x01_u8, 0x0, 0x00, 0x0, 0x0, 0x1, 0x01_u8, 0x0,
            0x00, 0x0, 0x0, 0x2
        ][..],
        bytes.unwrap()
    );

    let array = [1, 2, 3, 4];
    let bytes = to_bytes(&array[..=1]);
    assert_eq!(
        [
            0x09_u8, 0x0, 0x00, 0x0, 0x0, 0x2, 0x01_u8, 0x0, 0x00, 0x0, 0x0, 0x1, 0x01_u8, 0x0,
            0x00, 0x0, 0x0, 0x2
        ][..],
        bytes.unwrap()
    );
    let bytes = to_bytes(&array);
    assert_eq!(
        [
            0x09_u8, 0x0, 0x00, 0x0, 0x0, 0x4, 0x01_u8, 0x0, 0x00, 0x0, 0x0, 0x1, 0x01_u8, 0x0,
            0x00, 0x0, 0x0, 0x2, 0x01_u8, 0x0, 0x00, 0x0, 0x0, 0x3, 0x01_u8, 0x0, 0x00, 0x0, 0x0,
            0x4
        ][..],
        bytes.unwrap()
    );

    let bytes = ("test", 1);
    assert_eq!(
        [
            0x09_u8, 0x0, 0x00, 0x0, 0x0, 0x2, 0x03, 0x00, 0x00, 0x0, 0x0, 0x4, b't', b'e', b's',
            b't', 0x01_u8, 0x0, 0x00, 0x0, 0x0, 0x1,
        ][..],
        to_bytes(bytes).unwrap()
    );
}

#[test]
fn ser_newtype1_test() {
    #[derive(Debug, Serialize)]
    struct Millimeters(i32);

    let bytes = to_bytes(Millimeters(3));
    assert_eq!([0x01_u8, 0x0, 0x00, 0x0, 0x0, 0x3][..], bytes.unwrap());
}

#[test]
fn ser_map_test() {
    let mut hash_map = HashMap::new();

    hash_map.insert("test", 1_i32);

    let bytes = to_bytes(hash_map);
    let map_bytes = [
        0x0a_u8, 0x0, 0x00, 0x0, 0x0, 0x1, 0x03, 0x00, 0x00, 0x0, 0x0, 0x4, b't', b'e', b's', b't',
        0x01, 0x00, 0x00, 0x0, 0x0, 0x1,
    ];
    assert_eq!(map_bytes[..], bytes.unwrap());
}

#[test]
fn ser_struct_test() {
    #[derive(Debug, Serialize)]
    struct Millimeters(i32);

    #[derive(Debug, Serialize)]
    struct TestStruct {
        test: i32,
        abc: bool,
        milli: Millimeters,
    }
    let test = TestStruct {
        test: 1,
        abc: true,
        milli: Millimeters(1),
    };

    let bytes = to_bytes(test);
    let map_bytes = [
        0x0a_u8, 0x0, 0x00, 0x0, 0x0, 0x3, 0x03, 0x00, 0x00, 0x0, 0x0, 0x4, b't', b'e', b's', b't',
        0x01, 0x00, 0x00, 0x0, 0x0, 0x1, 0x03, 0x00, 0x00, 0x0, 0x0, 0x3, b'a', b'b', b'c', 0x27,
        0x00, 0x00, 0x03, 0x00, 0x00, 0x0, 0x0, 0x5, b'm', b'i', b'l', b'l', b'i', 0x01, 0x00,
        0x00, 0x0, 0x0, 0x1,
    ];
    assert_eq!(map_bytes[..], bytes.unwrap());
}

#[test]
fn ser_struct_gb_test() {
    #[derive(Debug, Serialize)]
    struct Millimeters(i32);

    #[derive(Debug, Serialize)]
    struct TestStruct {
        test: i32,
        abc: GraphBinary,
        milli: Millimeters,
    }
    let test = TestStruct {
        test: 1,
        abc: GraphBinary::Boolean(true),
        milli: Millimeters(1),
    };

    let bytes = to_bytes(test);
    let map_bytes = [
        0x0a_u8, 0x0, 0x00, 0x0, 0x0, 0x3, 0x03, 0x00, 0x00, 0x0, 0x0, 0x4, b't', b'e', b's', b't',
        0x01, 0x00, 0x00, 0x0, 0x0, 0x1, 0x03, 0x00, 0x00, 0x0, 0x0, 0x3, b'a', b'b', b'c', 0x27,
        0x00, 0x00, 0x03, 0x00, 0x00, 0x0, 0x0, 0x5, b'm', b'i', b'l', b'l', b'i', 0x01, 0x00,
        0x00, 0x0, 0x0, 0x1,
    ];
    assert_eq!(map_bytes[..], bytes.unwrap());
}

#[test]
fn ser_struct_option_gb_test() {
    #[derive(Debug, Serialize)]
    struct Millimeters(i32);

    #[derive(Debug, Serialize)]
    struct TestStruct {
        test: i32,
        abc: Option<GraphBinary>,
        milli: Millimeters,
    }
    let test = TestStruct {
        test: 1,
        abc: None,
        milli: Millimeters(1),
    };

    let bytes = to_bytes(test);
    let map_bytes = [
        0x0a_u8, 0x0, 0x00, 0x0, 0x0, 0x3, 0x03, 0x00, 0x00, 0x0, 0x0, 0x4, b't', b'e', b's', b't',
        0x01, 0x00, 0x00, 0x0, 0x0, 0x1, 0x03, 0x00, 0x00, 0x0, 0x0, 0x3, b'a', b'b', b'c', 0xFE,
        0x01, 0x03, 0x00, 0x00, 0x0, 0x0, 0x5, b'm', b'i', b'l', b'l', b'i', 0x01, 0x00, 0x00, 0x0,
        0x0, 0x1,
    ];
    assert_eq!(map_bytes[..], bytes.unwrap());
}

#[test]
fn ser_struct_t_test() {
    #[derive(Debug, Serialize)]
    struct Millimeters(i32);

    #[derive(Debug, Serialize)]
    struct TestStruct {
        test: i32,
        abc: Option<GraphBinary>,
        milli: Millimeters,
    }
    let test = TestStruct {
        test: 1,
        abc: Some(GraphBinary::T(T::Id)),
        milli: Millimeters(1),
    };

    let bytes = to_bytes(test);
    let map_bytes = [
        0x0a_u8, 0x0, 0x00, 0x0, 0x0, 0x3, 0x03, 0x00, 0x00, 0x0, 0x0, 0x4, b't', b'e', b's', b't',
        0x01, 0x00, 0x00, 0x0, 0x0, 0x1, 0x03, 0x00, 0x00, 0x0, 0x0, 0x3, b'a', b'b', b'c', 0x20,
        0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x02, b'i', b'd', 0x03, 0x00, 0x00, 0x0, 0x0, 0x5,
        b'm', b'i', b'l', b'l', b'i', 0x01, 0x00, 0x00, 0x0, 0x0, 0x1,
    ];
    assert_eq!(map_bytes[..], bytes.unwrap());
}

#[test]
fn struct_to_gb() {
    #[derive(Debug, Serialize)]
    struct TestStruct {
        test: i32,
        abc: GraphBinary,
    }

    let test = TestStruct {
        test: 1,
        abc: GraphBinary::Boolean(true),
    };

    let gb = to_graph_binary(&test).unwrap();

    let map = HashMap::from([("test".into(), 1.into()), ("abc".into(), true.into())]);

    let expected = GraphBinary::Map(map);

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

    let expected = GraphBinary::Map(map);

    assert_eq!(expected, gb);
}
