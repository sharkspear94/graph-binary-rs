use core::panic;
use std::{
    collections::{hash_map, HashMap},
    io::{Bytes, Write},
};

use serde::{
    ser::{self, Impossible, SerializeMap, SerializeSeq, SerializeStruct, SerializeTuple},
    Serialize,
};

use crate::{
    error::{DecodeError, EncodeError},
    graph_binary::{Encode, GraphBinary},
    specs::CoreType,
    structure::{graph::Graph, list::List},
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

    fn serialize_bool(mut self, v: bool) -> Result<Self::Ok, Self::Error> {
        v.fq_gb_bytes(&mut self.writer)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i32(mut self, v: i32) -> Result<Self::Ok, Self::Error> {
        v.fq_gb_bytes(&mut self.writer)
    }

    fn serialize_i64(mut self, v: i64) -> Result<Self::Ok, Self::Error> {
        v.fq_gb_bytes(&mut self.writer)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        todo!()
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

    fn serialize_f32(mut self, v: f32) -> Result<Self::Ok, Self::Error> {
        v.fq_gb_bytes(&mut self.writer)
    }

    fn serialize_f64(mut self, v: f64) -> Result<Self::Ok, Self::Error> {
        v.fq_gb_bytes(&mut self.writer)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_str(mut self, v: &str) -> Result<Self::Ok, Self::Error> {
        v.fq_gb_bytes(&mut self.writer)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
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
            let len = len as i32; // possible convert error
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
            let len = len as i32; // possible convert error
            let len = len.to_be_bytes();
            let list_start = [CoreType::Map.into(), 0x00, len[0], len[1], len[2], len[3]];
            self.writer.extend_from_slice(&list_start);
            Ok(self)
        } else {
            Err(EncodeError::SerilizationError(
                "unknown map size".to_string(),
            ))
        }
    }

    fn serialize_struct(
        self,
        name: &'static str,
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

// impl serde::ser::SerializeSeq for Serializer {
//     type Ok = () ;

//     type Error = ;

//     fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
//     where
//         T: serde::Serialize {
//         todo!()
//     }

//     fn end(self) -> Result<Self::Ok, Self::Error> {
//         todo!()
//     }
// }

struct GraphBinarySerializer;

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

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;

    type SerializeTuple = Impossible<Self::Ok, Self::Error>;

    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;

    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;

    type SerializeMap = Impossible<Self::Ok, Self::Error>;

    type SerializeStruct = Impossible<Self::Ok, Self::Error>;

    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(GraphBinary::Boolean(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(GraphBinary::Int(Some(v)))
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
        todo!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(GraphBinary::String(v.to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
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
        todo!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
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
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
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
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
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
        todo!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
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
            GraphBinary::Int(_) => todo!(),
            GraphBinary::Long(_) => todo!(),
            GraphBinary::String(_) => todo!(),
            GraphBinary::Class(_) => todo!(),
            GraphBinary::Double(_) => todo!(),
            GraphBinary::Float(_) => todo!(),
            GraphBinary::List(list) => list.serialize(serializer),
            GraphBinary::Set(_) => todo!(),
            GraphBinary::Map(_) => todo!(),
            GraphBinary::Uuid(_) => todo!(),
            GraphBinary::Edge(_) => todo!(),
            GraphBinary::Path(_) => todo!(),
            GraphBinary::Property(_) => todo!(),
            GraphBinary::Graph(_) => todo!(),
            GraphBinary::Vertex(_) => todo!(),
            GraphBinary::VertexProperty(_) => todo!(),
            GraphBinary::Barrier(_) => todo!(),
            GraphBinary::Binding(_) => todo!(),
            GraphBinary::ByteCode(_) => todo!(),
            GraphBinary::Cardinality(_) => todo!(),
            GraphBinary::Column(_) => todo!(),
            GraphBinary::Direction(_) => todo!(),
            GraphBinary::Operator(_) => todo!(),
            GraphBinary::Order(_) => todo!(),
            GraphBinary::Pick(_) => todo!(),
            GraphBinary::Pop(_) => todo!(),
            GraphBinary::Lambda(_) => todo!(),
            GraphBinary::P(_) => todo!(),
            GraphBinary::Scope(_) => todo!(),
            GraphBinary::T(_) => todo!(),
            GraphBinary::Traverser(_) => todo!(),
            GraphBinary::Byte(_) => todo!(),
            GraphBinary::ByteBuffer(_) => todo!(),
            GraphBinary::Short(_) => todo!(),
            GraphBinary::Boolean(_) => todo!(),
            GraphBinary::TextP(_) => todo!(),
            GraphBinary::TraversalStrategy(_) => todo!(),
            GraphBinary::Tree(_) => todo!(),
            GraphBinary::Metrics(_) => todo!(),
            GraphBinary::TraversalMetrics(_) => todo!(),
            GraphBinary::UnspecifiedNullObject => todo!(),
        }
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
fn ser_newtype_test() {
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
