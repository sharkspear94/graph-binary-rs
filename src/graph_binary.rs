use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::io::{Read, Write};

use crate::structure::binding::Binding;
use crate::structure::enums::{
    Barrier, Cardinality, Column, Direction, Merge, Operator, Order, Pick, Pop, Scope, TextP, P, T,
};
use crate::structure::graph::Graph;
use crate::structure::lambda::Lambda;
// use crate::structure::list::List1;
use crate::structure::bytecode::ByteCode;
use crate::structure::metrics::{Metrics, TraversalMetrics};
use crate::structure::path::Path;
use crate::structure::property::Property;
use crate::structure::traverser::{TraversalStrategy, Traverser};
use crate::structure::vertex::Vertex;
use crate::structure::vertex_property::VertexProperty;
use crate::{specs::CoreType, structure::edge::Edge};
use serde::de::Visitor;
use serde::Deserialize;
use uuid::Uuid;

// use super::structure::list::List;
use super::structure::map::Map;

pub const VALUE_PRESENT: u8 = 0x00;
pub const VALUE_NULL: u8 = 0x01;

pub const VALUE_PRESENT_BYTE_LEN: usize = 2;
pub const INT32_LEN: usize = 4;
pub const LONG64_LEN: usize = 8;

pub const INT32_TYPE_CODE: u8 = 0x01;
pub const INT64_TYPE_CODE: u8 = 0x02;
pub const STRING_TYPE_CODE: u8 = 0x03;
pub const DATE_TYPE_CODE: u8 = 0x04;

pub const LIST_TYPE_CODE: u8 = 0x09;
pub const MAP_TYPE_CODE: u8 = 0x0a;

#[derive(Debug, PartialEq)]
pub enum GraphBinary {
    Int(i32),
    Long(i64),
    String(String),
    // Date(Date),
    // Timestamp(Date),
    Class(String),
    Double(f64),
    Float(f32),
    List(Vec<GraphBinary>),
    Set(Vec<GraphBinary>),
    Map(HashMap<MapKeys, GraphBinary>),
    Uuid(Uuid),
    Edge(Edge),
    Path(Path),
    Property(Property),
    Graph(Graph),
    Vertex(Vertex),
    VertexProperty(VertexProperty),
    Barrier(Barrier),
    Binding(Binding),
    ByteCode(ByteCode),
    Cardinality(Cardinality),
    Column(Column),
    Direction(Direction),
    Operator(Operator),
    Order(Order),
    Pick(Pick),
    Pop(Pop),
    Lambda(Lambda),
    P(P),
    Scope(Scope),
    T(T),
    Traverser(Traverser),
    // BigDecimal(BigDecimal),
    // BigInteger(BigInteger),
    Byte(u8),
    ByteBuffer(Vec<u8>),
    Short(i16),
    Boolean(bool),
    TextP(TextP),
    TraversalStrategy(TraversalStrategy),
    // BulkSet(BulkSet),
    Tree(BTreeSet<GraphBinary>),
    Metrics(Metrics),
    TraversalMetrics(TraversalMetrics),
    Merge(Merge),
    UnspecifiedNullObject,
    // Custom
}

pub fn build_fq_null_bytes<W: Write>(writer: &mut W) -> Result<(), EncodeError> {
    writer.write_all(&[CoreType::UnspecifiedNullObject.into(), 0x01])?;
    Ok(())
}

impl GraphBinary {
    pub fn build_fq_bytes<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        match self {
            GraphBinary::Int(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::Long(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::String(val) => val.write_full_qualified_bytes(writer),
            // CoreType::Date(_) => todo!(),
            // CoreType::Timestamp(_) => todo!(),
            GraphBinary::Class(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::Double(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::Float(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::List(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::Set(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::Map(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::Uuid(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::Edge(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::Path(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::Property(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::Graph(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::Vertex(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::VertexProperty(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::Barrier(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::Binding(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::ByteCode(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::Cardinality(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::Column(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::Direction(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::Operator(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::Order(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::Pick(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::Pop(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::Lambda(val) => todo!(),
            GraphBinary::P(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::Scope(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::T(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::Traverser(val) => todo!(),
            // GraphBinary::BigDecimal(_) => todo!(),
            // GraphBinary::BigInteger(_) => todo!(),
            GraphBinary::Byte(_) => todo!(),
            GraphBinary::ByteBuffer(buf) => todo!(),
            GraphBinary::Short(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::Boolean(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::TextP(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::TraversalStrategy(_) => todo!(),
            // GraphBinary::BulkSet(_) => todo!(),
            GraphBinary::Tree(_) => todo!(),
            GraphBinary::Metrics(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::TraversalMetrics(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::Merge(val) => val.write_full_qualified_bytes(writer),
            GraphBinary::UnspecifiedNullObject => build_fq_null_bytes(writer),
            // GraphBinary::Custom => todo!(),
            // _ =>  Bytes::new()
        }
    }

    pub fn type_info(&self) -> CoreType {
        match self {
            GraphBinary::Int(_) => CoreType::Int32,
            GraphBinary::Long(_) => CoreType::Long,
            GraphBinary::String(_) => CoreType::String,
            GraphBinary::Class(_) => CoreType::Class,
            GraphBinary::Double(_) => CoreType::Double,
            GraphBinary::Float(_) => CoreType::Float,
            GraphBinary::List(_) => CoreType::List,
            GraphBinary::Set(_) => CoreType::Set,
            GraphBinary::Map(_) => CoreType::Map,
            GraphBinary::Uuid(_) => CoreType::Uuid,
            GraphBinary::Edge(_) => CoreType::Edge,
            GraphBinary::Path(_) => CoreType::Path,
            GraphBinary::Property(_) => CoreType::Property,
            GraphBinary::Graph(_) => CoreType::Graph,
            GraphBinary::Vertex(_) => CoreType::Vertex,
            GraphBinary::VertexProperty(_) => CoreType::VertexProperty,
            GraphBinary::Barrier(_) => CoreType::Barrier,
            GraphBinary::Binding(_) => CoreType::Binding,
            GraphBinary::ByteCode(_) => CoreType::ByteCode,
            GraphBinary::Cardinality(_) => CoreType::Cardinality,
            GraphBinary::Column(_) => CoreType::Column,
            GraphBinary::Direction(_) => CoreType::Direction,
            GraphBinary::Operator(_) => CoreType::Operator,
            GraphBinary::Order(_) => CoreType::Order,
            GraphBinary::Pick(_) => CoreType::Pick,
            GraphBinary::Pop(_) => CoreType::Pop,
            GraphBinary::Lambda(_) => CoreType::Lambda,
            GraphBinary::P(_) => CoreType::P,
            GraphBinary::Scope(_) => CoreType::Scope,
            GraphBinary::T(_) => CoreType::T,
            GraphBinary::Traverser(_) => CoreType::Traverser,
            GraphBinary::Byte(_) => CoreType::Byte,
            GraphBinary::ByteBuffer(_) => CoreType::ByteBuffer,
            GraphBinary::Short(_) => CoreType::Short,
            GraphBinary::Boolean(_) => CoreType::Boolean,
            GraphBinary::TextP(_) => CoreType::TextP,
            GraphBinary::TraversalStrategy(_) => CoreType::TraversalStrategy,
            GraphBinary::Tree(_) => CoreType::Tree,
            GraphBinary::Metrics(_) => CoreType::Metrics,
            GraphBinary::TraversalMetrics(_) => CoreType::TraversalMetrics,
            GraphBinary::UnspecifiedNullObject => CoreType::UnspecifiedNullObject,
            GraphBinary::Merge(_) => CoreType::Merge,
        }
    }

    pub fn to_option(self) -> Option<GraphBinary> {
        match self {
            GraphBinary::UnspecifiedNullObject => None,
            graph_binary => Some(graph_binary),
        }
    }

    fn decode<R: Read>(reader: R) {}
}

impl Decode for GraphBinary {
    fn partial_decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        unimplemented!("partial decode is not supported for GraphBinary")
    }

    fn expected_type_code() -> u8 {
        unimplemented!()
    }

    fn fully_self_decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        decode(reader)
    }

    fn partial_count_bytes(bytes: &[u8]) -> Result<usize, DecodeError> {
        unimplemented!("partial_count_bytes is not supported for GraphBinary")
    }

    fn consumed_bytes(bytes: &[u8]) -> Result<usize, DecodeError> {
        match CoreType::try_from(bytes[0])? {
            CoreType::Int32 => i32::consumed_bytes(bytes),
            CoreType::Long => i64::consumed_bytes(bytes),
            CoreType::String => String::consumed_bytes(bytes),
            CoreType::Class => String::consumed_bytes(bytes),
            CoreType::Double => f64::consumed_bytes(bytes),
            CoreType::Float => f32::consumed_bytes(bytes),
            CoreType::List => Vec::<GraphBinary>::consumed_bytes(bytes),
            CoreType::Set => Vec::<GraphBinary>::consumed_bytes(bytes),
            CoreType::Map => todo!(),
            CoreType::Uuid => Uuid::consumed_bytes(bytes),
            CoreType::Edge => Edge::consumed_bytes(bytes),
            CoreType::Path => Path::consumed_bytes(bytes),
            CoreType::Property => Property::consumed_bytes(bytes),
            CoreType::Graph => Graph::consumed_bytes(bytes),
            CoreType::Vertex => Vertex::consumed_bytes(bytes),
            CoreType::VertexProperty => VertexProperty::consumed_bytes(bytes),
            CoreType::Barrier => Barrier::consumed_bytes(bytes),
            CoreType::Binding => Binding::consumed_bytes(bytes),
            CoreType::ByteCode => ByteCode::consumed_bytes(bytes),
            CoreType::Cardinality => Cardinality::consumed_bytes(bytes),
            CoreType::Column => Column::consumed_bytes(bytes),
            CoreType::Direction => Direction::consumed_bytes(bytes),
            CoreType::Operator => Operator::consumed_bytes(bytes),
            CoreType::Order => Order::consumed_bytes(bytes),
            CoreType::Pick => Pick::consumed_bytes(bytes),
            CoreType::Pop => Pop::consumed_bytes(bytes),
            CoreType::Lambda => todo!(),
            CoreType::P => P::consumed_bytes(bytes),
            CoreType::Scope => Scope::consumed_bytes(bytes),
            CoreType::T => T::consumed_bytes(bytes),
            CoreType::Traverser => Traverser::consumed_bytes(bytes),
            CoreType::Byte => u8::consumed_bytes(bytes),
            CoreType::ByteBuffer => todo!(),
            CoreType::Short => i16::consumed_bytes(bytes),
            CoreType::Boolean => bool::consumed_bytes(bytes),
            CoreType::TextP => TextP::consumed_bytes(bytes),
            CoreType::TraversalStrategy => todo!(), // TraversalStrategy::consumed_bytes(bytes),
            CoreType::Tree => todo!(),              //Tree::consumed_bytes(bytes),
            CoreType::Metrics => Metrics::consumed_bytes(bytes),
            CoreType::TraversalMetrics => TraversalMetrics::consumed_bytes(bytes),
            CoreType::Merge => Merge::consumed_bytes(bytes),
            CoreType::UnspecifiedNullObject => Ok(2),
        }
    }
}

impl Encode for GraphBinary {
    fn type_code() -> u8 {
        todo!()
    }

    fn write_patial_bytes<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        todo!()
    }

    fn write_full_qualified_bytes<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        self.build_fq_bytes(writer)
    }
}

pub struct BigDecimal {}

pub struct BigInteger {}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum MapKeys {
    Int(i32),
    String(String),
    Long(i64),
    Uuid(Uuid),
}

impl From<MapKeys> for GraphBinary {
    fn from(keys: MapKeys) -> GraphBinary {
        match keys {
            MapKeys::Int(val) => GraphBinary::Int(val),
            MapKeys::String(val) => GraphBinary::String(val),
            MapKeys::Long(val) => GraphBinary::Long(val),
            MapKeys::Uuid(val) => GraphBinary::Uuid(val),
        }
    }
}

impl From<&MapKeys> for GraphBinary {
    fn from(keys: &MapKeys) -> GraphBinary {
        match keys {
            MapKeys::Int(val) => GraphBinary::Int(*val),
            MapKeys::String(val) => GraphBinary::String(val.clone()),
            MapKeys::Long(val) => GraphBinary::Long(*val),
            MapKeys::Uuid(val) => GraphBinary::Uuid(*val),
        }
    }
}

impl Encode for MapKeys {
    fn type_code() -> u8 {
        unimplemented!()
    }

    fn write_patial_bytes<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        todo!()
    }

    fn write_full_qualified_bytes<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        match self {
            MapKeys::Int(val) => val.write_full_qualified_bytes(writer),
            MapKeys::String(val) => val.write_full_qualified_bytes(writer),
            MapKeys::Long(val) => val.write_full_qualified_bytes(writer),
            MapKeys::Uuid(val) => val.write_full_qualified_bytes(writer),
        }
    }
}

impl Decode for MapKeys {
    fn expected_type_code() -> u8 {
        unimplemented!()
    }

    fn partial_decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        unimplemented!()
    }

    fn fully_self_decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let mut buf = [255_u8; 2];
        reader.read_exact(&mut buf)?;
        match (buf[0], buf[1]) {
            (0x01, 0) => Ok(MapKeys::Int(i32::partial_decode(reader)?)),
            (0x02, 0) => Ok(MapKeys::Long(i64::partial_decode(reader)?)),
            (0x03, 0) => Ok(MapKeys::String(String::partial_decode(reader)?)),
            (0x0c, 0) => Ok(MapKeys::Uuid(Uuid::partial_decode(reader)?)),
            (code, _) => Err(DecodeError::DecodeError(format!(
                "{} CoreType found: MapKey not Supported in Rust must implement Eq and Hash",
                code
            ))),
        }
    }

    fn partial_count_bytes(bytes: &[u8]) -> Result<usize, DecodeError> {
        todo!()
    }

    fn consumed_bytes(bytes: &[u8]) -> Result<usize, DecodeError> {
        match (bytes[0], bytes[1]) {
            (0x01, 0) => i32::consumed_bytes(bytes),
            (0x02, 0) => i64::consumed_bytes(bytes),
            (0x03, 0) => String::consumed_bytes(bytes),
            (0x0c, 0) => Uuid::consumed_bytes(bytes),
            (code, _) => Err(DecodeError::DecodeError(format!(
                "{} CoreType found: MapKey not Supported in Rust must implement Eq and Hash",
                code
            ))),
        }
    }
}

impl<'de> serde::Deserialize<'de> for MapKeys {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(MapKeysVisitor)
    }
}

struct MapKeysVisitor;

impl<'de> serde::de::Visitor<'de> for MapKeysVisitor {
    type Value = MapKeys;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, concat!("a enum  MapKeys"))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(MapKeys::String(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(MapKeys::String(v.to_owned()))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(MapKeys::Int(v))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(MapKeys::Long(v))
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(MapKeys::Uuid(Uuid::from_u128(v)))
    }
}

impl serde::ser::Serialize for MapKeys {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut buf: Vec<u8> = Vec::with_capacity(64);
        match self.write_full_qualified_bytes(&mut buf) {
            Ok(_) => serializer.serialize_bytes(&buf),
            Err(e) => Err(serde::ser::Error::custom(format!(
                "serilization Error of MapKeys: reason: {}",
                e
            ))),
        }
    }
}

pub fn decode<R: Read>(reader: &mut R) -> Result<GraphBinary, DecodeError> {
    let mut buf = [255_u8; 2];
    reader.read_exact(&mut buf)?;

    let identifier = CoreType::try_from(buf[0])?;
    let value_flag = ValueFlag::try_from(buf[1])?;

    match (identifier, value_flag) {
        (CoreType::Int32, ValueFlag::Set) => Ok(GraphBinary::Int(i32::partial_decode(reader)?)),
        (CoreType::Long, ValueFlag::Set) => Ok(GraphBinary::Long(i64::partial_decode(reader)?)),
        (CoreType::Long, ValueFlag::Null) => todo!(),
        (CoreType::String, ValueFlag::Set) => {
            Ok(GraphBinary::String(String::partial_decode(reader)?))
        }
        (CoreType::String, ValueFlag::Null) => todo!(),
        (CoreType::Class, ValueFlag::Set) => {
            Ok(GraphBinary::Class(String::partial_decode(reader)?))
        }
        (CoreType::Class, ValueFlag::Null) => todo!(),
        (CoreType::Double, ValueFlag::Set) => Ok(GraphBinary::Double(f64::partial_decode(reader)?)),
        (CoreType::Double, ValueFlag::Null) => todo!(),
        (CoreType::Float, ValueFlag::Set) => Ok(GraphBinary::Float(f32::partial_decode(reader)?)),
        (CoreType::Float, ValueFlag::Null) => todo!(),
        (CoreType::List, ValueFlag::Set) => Ok(GraphBinary::List(Vec::partial_decode(reader)?)),
        (CoreType::List, ValueFlag::Null) => todo!(),
        (CoreType::Set, ValueFlag::Set) => Ok(GraphBinary::Set(Vec::partial_decode(reader)?)),
        (CoreType::Set, ValueFlag::Null) => todo!(),
        (CoreType::Map, ValueFlag::Set) => todo!(),
        (CoreType::Map, ValueFlag::Null) => todo!(),
        (CoreType::Uuid, ValueFlag::Set) => todo!(),
        (CoreType::Uuid, ValueFlag::Null) => todo!(),
        (CoreType::Edge, ValueFlag::Set) => todo!(),
        (CoreType::Edge, ValueFlag::Null) => todo!(),
        (CoreType::Path, ValueFlag::Set) => todo!(),
        (CoreType::Path, ValueFlag::Null) => todo!(),
        (CoreType::Property, ValueFlag::Set) => {
            Ok(GraphBinary::Property(Property::partial_decode(reader)?))
        }
        (CoreType::Property, ValueFlag::Null) => todo!(),
        (CoreType::Graph, ValueFlag::Set) => todo!(),
        (CoreType::Graph, ValueFlag::Null) => todo!(),
        (CoreType::Vertex, ValueFlag::Set) => todo!(),
        (CoreType::Vertex, ValueFlag::Null) => todo!(),
        (CoreType::VertexProperty, ValueFlag::Set) => todo!(),
        (CoreType::VertexProperty, ValueFlag::Null) => todo!(),
        (CoreType::Short, ValueFlag::Set) => todo!(),
        (CoreType::Short, ValueFlag::Null) => todo!(),
        (CoreType::Boolean, ValueFlag::Set) => todo!(),
        (CoreType::Boolean, ValueFlag::Null) => todo!(),
        (CoreType::Cardinality, ValueFlag::Set) => todo!(),
        (CoreType::Cardinality, ValueFlag::Null) => todo!(),
        (CoreType::Column, ValueFlag::Set) => todo!(),
        (CoreType::Column, ValueFlag::Null) => todo!(),
        (CoreType::Direction, ValueFlag::Set) => todo!(),
        (CoreType::Direction, ValueFlag::Null) => todo!(),
        (CoreType::Operator, ValueFlag::Set) => todo!(),
        (CoreType::Operator, ValueFlag::Null) => todo!(),
        (CoreType::Order, ValueFlag::Set) => todo!(),
        (CoreType::Order, ValueFlag::Null) => todo!(),
        (CoreType::Pick, ValueFlag::Set) => todo!(),
        (CoreType::Pick, ValueFlag::Null) => todo!(),
        (CoreType::Pop, ValueFlag::Set) => todo!(),
        (CoreType::Pop, ValueFlag::Null) => todo!(),
        (CoreType::P, ValueFlag::Set) => todo!(),
        (CoreType::P, ValueFlag::Null) => todo!(),
        (CoreType::Scope, ValueFlag::Set) => todo!(),
        (CoreType::Scope, ValueFlag::Null) => todo!(),
        (CoreType::T, ValueFlag::Set) => todo!(),
        (CoreType::T, ValueFlag::Null) => todo!(),
        (CoreType::Barrier, ValueFlag::Set) => todo!(),
        (CoreType::Barrier, ValueFlag::Null) => todo!(),
        (CoreType::Binding, ValueFlag::Set) => todo!(),
        (CoreType::Binding, ValueFlag::Null) => todo!(),
        (CoreType::ByteCode, ValueFlag::Set) => todo!(),
        (CoreType::ByteCode, ValueFlag::Null) => todo!(),
        (CoreType::Lambda, ValueFlag::Set) => todo!(),
        (CoreType::Lambda, ValueFlag::Null) => todo!(),
        (CoreType::Traverser, ValueFlag::Set) => todo!(),
        (CoreType::Traverser, ValueFlag::Null) => todo!(),
        (CoreType::Byte, ValueFlag::Set) => todo!(),
        (CoreType::Byte, ValueFlag::Null) => todo!(),
        (CoreType::ByteBuffer, ValueFlag::Set) => todo!(),
        (CoreType::ByteBuffer, ValueFlag::Null) => todo!(),
        (CoreType::TextP, ValueFlag::Set) => todo!(),
        (CoreType::TextP, ValueFlag::Null) => todo!(),
        (CoreType::TraversalStrategy, ValueFlag::Set) => todo!(),
        (CoreType::TraversalStrategy, ValueFlag::Null) => todo!(),
        (CoreType::Tree, ValueFlag::Set) => todo!(),
        (CoreType::Tree, ValueFlag::Null) => todo!(),
        (CoreType::Metrics, ValueFlag::Set) => todo!(),
        (CoreType::Metrics, ValueFlag::Null) => todo!(),
        (CoreType::TraversalMetrics, ValueFlag::Set) => todo!(),
        (CoreType::TraversalMetrics, ValueFlag::Null) => todo!(),
        (CoreType::Merge, ValueFlag::Set) => todo!(),
        (CoreType::Merge, ValueFlag::Null) => todo!(),
        (CoreType::UnspecifiedNullObject, ValueFlag::Set) => todo!(),
        (CoreType::UnspecifiedNullObject, ValueFlag::Null) => {
            Ok(GraphBinary::UnspecifiedNullObject)
        }
        (CoreType::Int32, ValueFlag::Null) => todo!(),
        // (CoreType::Int32,0x00) => GraphBinary::Int(i32::decode(reader)?),
        // (0x02,0x00) => GraphBinary::Long(i64::decode(reader)?),
        // (LIST_TYPE_CODE,0x0) => GraphBinary::List(List::decode(reader)?),
        // (_,_) => return Err(DecodeError::DecodeError("qualifier not known".to_string())),
        (_, _) => Err(DecodeError::DecodeError(
            "Coretype not Implemented".to_string(),
        )),
    }
}

#[repr(u8)]
pub enum ValueFlag {
    Set = 0x00,
    Null = 0x01,
}

impl TryFrom<u8> for ValueFlag {
    type Error = DecodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(ValueFlag::Set),
            0x01 => Ok(ValueFlag::Null),
            rest => Err(DecodeError::ConvertError(format!(
                "Expected ValueFlag found {rest}"
            ))),
        }
    }
}

impl From<ValueFlag> for u8 {
    fn from(v: ValueFlag) -> Self {
        match v {
            ValueFlag::Set => 0,
            ValueFlag::Null => 1,
        }
    }
}

impl<'de> Deserialize<'de> for ValueFlag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ValueFlagVisitor;

        impl<'de> Visitor<'de> for ValueFlagVisitor {
            type Value = ValueFlag;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a enum ValueFlag")
            }

            fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match ValueFlag::try_from(v) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(E::custom(format!(
                        "conversion of ValueFlag in Deserialize failed: Error Message: {}",
                        e
                    ))),
                }
            }
        }

        deserializer.deserialize_u8(ValueFlagVisitor)
    }
}

pub trait Decode {
    fn expected_type_code() -> u8;

    fn partial_decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized;

    fn fully_self_decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let mut buf = [255_u8; 2];
        reader.read_exact(&mut buf)?;
        match (buf[0], buf[1]) {
            (code, 0) if code == Self::expected_type_code() => Self::partial_decode(reader),
            (t, value_flag) => Err(DecodeError::DecodeError(format!(
                "Type Code Error, expected type {:#X}, found {:#X} and value_flag {:#X}",
                Self::expected_type_code(),
                t,
                value_flag
            ))),
        }
    }

    fn partial_count_bytes(bytes: &[u8]) -> Result<usize, DecodeError>;

    fn consumed_bytes(bytes: &[u8]) -> Result<usize, DecodeError> {
        let partial_bytes = Self::partial_count_bytes(&bytes[2..])?;
        Ok(partial_bytes + 2)
    }
}

use crate::error::{DecodeError, EncodeError};

pub trait Encode {
    fn type_code() -> u8;

    fn write_patial_bytes<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError>;

    fn write_full_qualified_null_bytes<W: Write>(writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&[Self::type_code(), VALUE_NULL])?;
        Ok(())
    }

    fn write_full_qualified_bytes<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&[Self::type_code(), VALUE_PRESENT])?;
        self.write_patial_bytes(writer)
    }

    fn write_nullable_bytes<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&[VALUE_PRESENT])?;
        self.write_patial_bytes(writer)
    }
}

#[test]
fn testing() {
    let mut buf: Vec<u8> = vec![];
    15_i32.write_patial_bytes(&mut buf).unwrap();
    assert_eq!([0x00, 0x00, 0x00, 0x0F], buf.as_slice());

    buf.clear();
    15_i32.write_full_qualified_bytes(&mut buf).unwrap();
    assert_eq!([0x01, 0x00, 0x00, 0x00, 0x00, 0x0F], buf.as_slice());
}

// pub enum Parent {
//     Edge(Edge),
//     VertexProperty(VertexProperty),
// }

// impl FullyQualifiedBytes for Binding {
//     fn get_type_code(&self) -> Bytes {
//         Bytes::from_static(&[CORE_TYPE_BINDING])
//     }

//     fn generate_byte_representation(&self) -> Bytes {
//         let mut ret = bytes::BytesMut::with_capacity(64);
//         ret.extend(self.key.generate_byte_representation());
//         ret.extend(self.val.build_fq_bytes());
//         ret.freeze()
//     }
// }
