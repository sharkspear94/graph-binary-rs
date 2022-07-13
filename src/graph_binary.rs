use std::collections::{BTreeSet, HashMap};
use std::fmt::{write, Display};
use std::io::{Read, Write};
use std::vec;

use crate::error::{DecodeError, EncodeError};
use crate::macros::{TryBorrowFrom, TryMutBorrowFrom};
use crate::specs;
use crate::structure::binding::Binding;
use crate::structure::bulkset::BulkSet;
use crate::structure::bytecode::ByteCode;
use crate::structure::enums::{
    Barrier, Cardinality, Column, Direction, Merge, Operator, Order, Pick, Pop, Scope, TextP, P, T,
};
use crate::structure::graph::Graph;
use crate::structure::lambda::Lambda;
use crate::structure::metrics::{Metrics, TraversalMetrics};
use crate::structure::path::Path;
use crate::structure::primitivs::UuidDef;
use crate::structure::property::Property;
use crate::structure::traverser::{TraversalStrategy, Traverser};
use crate::structure::vertex::Vertex;
use crate::structure::vertex_property::VertexProperty;
use crate::{specs::CoreType, structure::edge::Edge};
use serde::de::Visitor;
use serde::Deserialize;
use uuid::Uuid;

/// All possible Values supported in the [GraphBinary serialization format](https://tinkerpop.apache.org/docs/current/dev/io/#graphbinary)
#[derive(Debug, PartialEq, Clone)]
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
    BulkSet(BulkSet),
    Tree(BTreeSet<GraphBinary>),
    Metrics(Metrics),
    TraversalMetrics(TraversalMetrics),
    Merge(Merge),
    UnspecifiedNullObject,
    // Custom
    Char(char),
}

pub fn encode_null_object<W: Write>(writer: &mut W) -> Result<(), EncodeError> {
    const BUF: [u8; 2] = [specs::CORE_TYPE_UNSPECIFIED_NULL, specs::VALUE_FLAG_NULL];
    writer.write_all(&BUF)?;
    Ok(())
}

fn encode_byte_buffer<W: Write>(writer: &mut W, buf: &[u8]) -> Result<(), EncodeError> {
    writer.write_all(&[CoreType::ByteBuffer.into(), ValueFlag::Null.into()])?;
    let len = (buf.len() as i32).to_be_bytes();
    writer.write_all(&len)?;
    writer.write_all(buf)?;
    Ok(())
}

impl GraphBinary {
    /// Returns an Option of an owned value if the Type was the GraphBinary variant.
    /// Returns None if GraphBinary enum holds another Type
    ///
    /// ```
    /// # use graph_binary_rs::graph_binary::GraphBinary;
    ///
    /// let gb1 = GraphBinary::Boolean(true);
    /// assert_eq!(Some(true),gb1.get());
    ///
    /// let gb2 = GraphBinary::Boolean(true);
    /// assert_eq!(None, gb2.get::<String>());
    ///
    /// ```
    pub fn get<T: TryFrom<GraphBinary>>(self) -> Option<T> {
        T::try_from(self).ok()
    }

    /// Returns an Option of the borrowed value if the Type was the GraphBinary variant.
    /// Returns None if GraphBinary enum holds another Type
    ///
    /// ```
    /// # use graph_binary_rs::graph_binary::GraphBinary;
    ///
    /// let gb = GraphBinary::String("Janus".to_string());
    ///
    /// assert_eq!(Some("Janus"),gb.get_ref());
    /// assert_eq!(Some(&String::from("Janus")),gb.get_ref());
    /// assert_eq!(None, gb.get_ref::<bool>());
    ///
    /// ```
    pub fn get_ref<T: TryBorrowFrom + ?Sized>(&self) -> Option<&T> {
        T::try_borrow_from(self)
    }

    /// Returns an Option of the mutable borrowed value if the Type was the GraphBinary variant.
    /// Returns None if GraphBinary enum holds another Type
    ///
    /// ```
    /// # use graph_binary_rs::graph_binary::GraphBinary;
    ///
    /// let mut gb = GraphBinary::String("Janus".to_string());
    ///
    /// let s = gb.get_mut_ref::<String>().unwrap();
    /// s.push_str("Graph");
    ///
    /// assert_eq!(Some(&String::from("JanusGraph")),gb.get_ref());
    /// assert_eq!(None, gb.get_ref::<bool>());
    ///
    /// ```
    pub fn get_mut_ref<T: TryMutBorrowFrom + ?Sized>(&mut self) -> Option<&mut T> {
        T::try_mut_borrow_from(self)
    }

    pub fn exceptions(&self) -> Option<&str> {
        if let Some(l) = self.get_ref::<Vec<_>>() {
            l.iter().filter_map(|s| s.get_ref()).next()
        } else {
            None
        }
    }

    pub fn display_results(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Results: ")
    }

    pub fn build_fq_bytes<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        match self {
            GraphBinary::Int(val) => val.encode(writer),
            GraphBinary::Long(val) => val.encode(writer),
            GraphBinary::String(val) => val.encode(writer),
            // CoreType::Date(_) => todo!(),
            // CoreType::Timestamp(_) => todo!(),
            GraphBinary::Class(val) => val.encode(writer),
            GraphBinary::Double(val) => val.encode(writer),
            GraphBinary::Float(val) => val.encode(writer),
            GraphBinary::List(val) => val.encode(writer),
            GraphBinary::Set(val) => val.encode(writer),
            GraphBinary::Map(val) => val.encode(writer),
            GraphBinary::Uuid(val) => val.encode(writer),
            GraphBinary::Edge(val) => val.encode(writer),
            GraphBinary::Path(val) => val.encode(writer),
            GraphBinary::Property(val) => val.encode(writer),
            GraphBinary::Graph(val) => val.encode(writer),
            GraphBinary::Vertex(val) => val.encode(writer),
            GraphBinary::VertexProperty(val) => val.encode(writer),
            GraphBinary::Barrier(val) => val.encode(writer),
            GraphBinary::Binding(val) => val.encode(writer),
            GraphBinary::ByteCode(val) => val.encode(writer),
            GraphBinary::Cardinality(val) => val.encode(writer),
            GraphBinary::Column(val) => val.encode(writer),
            GraphBinary::Direction(val) => val.encode(writer),
            GraphBinary::Operator(val) => val.encode(writer),
            GraphBinary::Order(val) => val.encode(writer),
            GraphBinary::Pick(val) => val.encode(writer),
            GraphBinary::Pop(val) => val.encode(writer),
            GraphBinary::Lambda(val) => val.encode(writer),
            GraphBinary::P(val) => val.encode(writer),
            GraphBinary::Scope(val) => val.encode(writer),
            GraphBinary::T(val) => val.encode(writer),
            GraphBinary::Traverser(val) => val.encode(writer),
            // GraphBinary::BigDecimal(_) => todo!(),
            // GraphBinary::BigInteger(_) => todo!(),
            GraphBinary::Byte(val) => val.encode(writer),
            GraphBinary::ByteBuffer(buf) => encode_byte_buffer(writer, buf),
            GraphBinary::Short(val) => val.encode(writer),
            GraphBinary::Boolean(val) => val.encode(writer),
            GraphBinary::TextP(val) => val.encode(writer),
            GraphBinary::TraversalStrategy(val) => val.encode(writer),
            GraphBinary::BulkSet(_) => todo!(),
            GraphBinary::Tree(_) => todo!(),
            GraphBinary::Metrics(val) => val.encode(writer),
            GraphBinary::TraversalMetrics(val) => val.encode(writer),
            GraphBinary::Merge(val) => val.encode(writer),
            GraphBinary::UnspecifiedNullObject => encode_null_object(writer),
            GraphBinary::Char(val) => val.encode(writer),
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
            GraphBinary::BulkSet(_) => CoreType::BulkSet,
            GraphBinary::UnspecifiedNullObject => CoreType::UnspecifiedNullObject,
            GraphBinary::Merge(_) => CoreType::Merge,
            GraphBinary::Char(_) => todo!(),
        }
    }

    pub fn to_option(self) -> Option<GraphBinary> {
        match self {
            GraphBinary::UnspecifiedNullObject => None,
            graph_binary => Some(graph_binary),
        }
    }
}

impl Decode for GraphBinary {
    fn partial_decode<R: Read>(_reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        unimplemented!("partial decode is not supported for GraphBinary")
    }

    fn expected_type_code() -> u8 {
        unimplemented!("expected type code is not supported for GraphBinary")
    }

    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        decode(reader)
    }

    fn get_partial_len(_bytes: &[u8]) -> Result<usize, DecodeError> {
        unimplemented!("partial_count_bytes is not supported for GraphBinary")
    }

    fn get_len(bytes: &[u8]) -> Result<usize, DecodeError> {
        match CoreType::try_from(bytes[0])? {
            CoreType::Int32 => i32::get_len(bytes),
            CoreType::Long => i64::get_len(bytes),
            CoreType::String => String::get_len(bytes),
            CoreType::Class => String::get_len(bytes),
            CoreType::Double => f64::get_len(bytes),
            CoreType::Float => f32::get_len(bytes),
            CoreType::List => Vec::<GraphBinary>::get_len(bytes),
            CoreType::Set => Vec::<GraphBinary>::get_len(bytes),
            CoreType::Map => HashMap::<MapKeys, GraphBinary>::get_len(bytes),
            CoreType::Uuid => Uuid::get_len(bytes),
            CoreType::Edge => Edge::get_len(bytes),
            CoreType::Path => Path::get_len(bytes),
            CoreType::Property => Property::get_len(bytes),
            CoreType::Graph => Graph::get_len(bytes),
            CoreType::Vertex => Vertex::get_len(bytes),
            CoreType::VertexProperty => VertexProperty::get_len(bytes),
            CoreType::Barrier => Barrier::get_len(bytes),
            CoreType::Binding => Binding::get_len(bytes),
            CoreType::ByteCode => ByteCode::get_len(bytes),
            CoreType::Cardinality => Cardinality::get_len(bytes),
            CoreType::Column => Column::get_len(bytes),
            CoreType::Direction => Direction::get_len(bytes),
            CoreType::Operator => Operator::get_len(bytes),
            CoreType::Order => Order::get_len(bytes),
            CoreType::Pick => Pick::get_len(bytes),
            CoreType::Pop => Pop::get_len(bytes),
            CoreType::Lambda => todo!(),
            CoreType::P => P::get_len(bytes),
            CoreType::Scope => Scope::get_len(bytes),
            CoreType::T => T::get_len(bytes),
            CoreType::Traverser => Traverser::get_len(bytes),
            CoreType::Byte => u8::get_len(bytes),
            CoreType::ByteBuffer => todo!(),
            CoreType::Short => i16::get_len(bytes),
            CoreType::Boolean => bool::get_len(bytes),
            CoreType::TextP => TextP::get_len(bytes),
            CoreType::TraversalStrategy => TraversalStrategy::get_len(bytes),
            CoreType::Tree => todo!(), //Tree::consumed_bytes(bytes),
            CoreType::Metrics => Metrics::get_len(bytes),
            CoreType::TraversalMetrics => TraversalMetrics::get_len(bytes),
            CoreType::BulkSet => todo!(),
            CoreType::Merge => Merge::get_len(bytes),
            CoreType::UnspecifiedNullObject => Ok(2),
            CoreType::Char => char::get_len(bytes),
        }
    }
}

impl Encode for GraphBinary {
    fn type_code() -> u8 {
        todo!()
    }

    fn partial_encode<W: Write>(&self, _writer: &mut W) -> Result<(), EncodeError> {
        todo!()
    }

    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        self.build_fq_bytes(writer)
    }
}

pub struct BigDecimal {}

pub struct BigInteger {}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum MapKeys {
    Int(i32),
    String(String),
    Long(i64),
    Uuid(Uuid),
    T(T),
    Direction(Direction),
}

impl From<MapKeys> for GraphBinary {
    fn from(keys: MapKeys) -> GraphBinary {
        match keys {
            MapKeys::Int(val) => GraphBinary::Int(val),
            MapKeys::String(val) => GraphBinary::String(val),
            MapKeys::Long(val) => GraphBinary::Long(val),
            MapKeys::Uuid(val) => GraphBinary::Uuid(val),
            MapKeys::T(val) => GraphBinary::T(val),
            MapKeys::Direction(val) => GraphBinary::Direction(val),
        }
    }
}

impl<T: Into<GraphBinary> + Clone> From<&T> for GraphBinary {
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}

impl<T: Into<GraphBinary>, const N: usize> From<[T; N]> for GraphBinary {
    fn from(array: [T; N]) -> Self {
        GraphBinary::List(array.into_iter().map(Into::into).collect())
    }
}

impl TryFrom<GraphBinary> for MapKeys {
    type Error = DecodeError;

    fn try_from(value: GraphBinary) -> Result<Self, Self::Error> {
        match value {
            GraphBinary::Int(val) => Ok(MapKeys::Int(val)),
            GraphBinary::Long(val) => Ok(MapKeys::Long(val)),
            GraphBinary::String(val) => Ok(MapKeys::String(val)),
            GraphBinary::Uuid(val) => Ok(MapKeys::Uuid(val)),
            GraphBinary::T(val) => Ok(MapKeys::T(val)),
            GraphBinary::Direction(val) => Ok(MapKeys::Direction(val)),
            rest => Err(DecodeError::ConvertError(format!(
                "cannot convert from {:?} to MapKeys",
                rest
            ))),
        }
    }
}

impl From<&str> for MapKeys {
    fn from(s: &str) -> Self {
        MapKeys::String(s.to_owned())
    }
}

impl From<String> for MapKeys {
    fn from(s: String) -> Self {
        MapKeys::String(s)
    }
}

impl From<i32> for MapKeys {
    fn from(val: i32) -> Self {
        MapKeys::Int(val)
    }
}

impl From<i64> for MapKeys {
    fn from(val: i64) -> Self {
        MapKeys::Long(val)
    }
}

impl From<Uuid> for MapKeys {
    fn from(val: Uuid) -> Self {
        MapKeys::Uuid(val)
    }
}

impl Encode for MapKeys {
    fn type_code() -> u8 {
        unimplemented!()
    }

    fn partial_encode<W: Write>(&self, _writer: &mut W) -> Result<(), EncodeError> {
        todo!()
    }

    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        match self {
            MapKeys::Int(val) => val.encode(writer),
            MapKeys::String(val) => val.encode(writer),
            MapKeys::Long(val) => val.encode(writer),
            MapKeys::Uuid(val) => val.encode(writer),
            MapKeys::T(val) => val.encode(writer),
            MapKeys::Direction(val) => val.encode(writer),
        }
    }
}

impl Decode for MapKeys {
    fn expected_type_code() -> u8 {
        unimplemented!("MapKeys is a collection of different GrapBinary Keys")
    }

    fn partial_decode<R: Read>(_reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        unimplemented!()
    }

    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let key = GraphBinary::decode(reader)?;
        MapKeys::try_from(key)
    }

    fn get_partial_len(_bytes: &[u8]) -> Result<usize, DecodeError> {
        unimplemented!("use consume_bytes insted")
    }

    fn get_len(bytes: &[u8]) -> Result<usize, DecodeError> {
        GraphBinary::get_len(bytes)
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

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let core_type: CoreType = map.next_key()?.unwrap();

        match core_type {
            CoreType::Uuid => Ok(MapKeys::Uuid(Uuid::from(map.next_value::<UuidDef>()?))), // can be implemented on visit_u128
            CoreType::T => Ok(MapKeys::T(map.next_value::<T>()?)),
            CoreType::Direction => Ok(MapKeys::Direction(map.next_value::<Direction>()?)),
            _ => unimplemented!("Mapkeys Error"),
        }
    }
}

impl serde::ser::Serialize for MapKeys {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut buf: Vec<u8> = Vec::with_capacity(64);
        match self.encode(&mut buf) {
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
        (_, ValueFlag::Null) => Ok(GraphBinary::UnspecifiedNullObject),
        (CoreType::Int32, _) => Ok(GraphBinary::Int(i32::partial_decode(reader)?)),
        (CoreType::Long, _) => Ok(GraphBinary::Long(i64::partial_decode(reader)?)),
        (CoreType::String, _) => Ok(GraphBinary::String(String::partial_decode(reader)?)),
        (CoreType::Class, _) => Ok(GraphBinary::Class(String::partial_decode(reader)?)),
        (CoreType::Double, _) => Ok(GraphBinary::Double(f64::partial_decode(reader)?)),
        (CoreType::Float, _) => Ok(GraphBinary::Float(f32::partial_decode(reader)?)),
        (CoreType::List, _) => Ok(GraphBinary::List(Vec::partial_decode(reader)?)),
        (CoreType::Set, _) => Ok(GraphBinary::Set(Vec::partial_decode(reader)?)),
        (CoreType::Map, _) => Ok(GraphBinary::Map(
            HashMap::<MapKeys, GraphBinary>::partial_decode(reader)?,
        )),
        (CoreType::Uuid, _) => Ok(GraphBinary::Uuid(Uuid::partial_decode(reader)?)),
        (CoreType::Edge, _) => Ok(GraphBinary::Edge(Edge::partial_decode(reader)?)),
        (CoreType::Path, _) => Ok(GraphBinary::Path(Path::partial_decode(reader)?)),
        (CoreType::Property, _) => Ok(GraphBinary::Property(Property::partial_decode(reader)?)),
        (CoreType::Graph, _) => Ok(GraphBinary::Graph(Graph::partial_decode(reader)?)),
        (CoreType::Vertex, _) => Ok(GraphBinary::Vertex(Vertex::partial_decode(reader)?)),
        (CoreType::VertexProperty, _) => Ok(GraphBinary::VertexProperty(
            VertexProperty::partial_decode(reader)?,
        )),
        (CoreType::Short, _) => Ok(GraphBinary::Short(i16::partial_decode(reader)?)),
        (CoreType::Boolean, _) => Ok(GraphBinary::Boolean(bool::partial_decode(reader)?)),

        (CoreType::Cardinality, _) => Ok(GraphBinary::Cardinality(Cardinality::partial_decode(
            reader,
        )?)),
        (CoreType::Column, _) => Ok(GraphBinary::Column(Column::partial_decode(reader)?)),
        (CoreType::Direction, _) => Ok(GraphBinary::Direction(Direction::partial_decode(reader)?)),
        (CoreType::Operator, _) => Ok(GraphBinary::Operator(Operator::partial_decode(reader)?)),
        (CoreType::Order, _) => Ok(GraphBinary::Order(Order::partial_decode(reader)?)),
        (CoreType::Pick, _) => Ok(GraphBinary::Pick(Pick::partial_decode(reader)?)),
        (CoreType::Pop, _) => Ok(GraphBinary::Pop(Pop::partial_decode(reader)?)),
        (CoreType::P, _) => Ok(GraphBinary::P(P::partial_decode(reader)?)),
        (CoreType::Scope, _) => Ok(GraphBinary::Scope(Scope::partial_decode(reader)?)),
        (CoreType::T, _) => Ok(GraphBinary::T(T::partial_decode(reader)?)),
        (CoreType::Barrier, _) => Ok(GraphBinary::Barrier(Barrier::partial_decode(reader)?)),
        (CoreType::Binding, _) => Ok(GraphBinary::Binding(Binding::partial_decode(reader)?)),
        (CoreType::ByteCode, _) => Ok(GraphBinary::ByteCode(ByteCode::partial_decode(reader)?)),
        (CoreType::Lambda, _) => Ok(GraphBinary::Lambda(Lambda::partial_decode(reader)?)),
        (CoreType::Traverser, _) => Ok(GraphBinary::Traverser(Traverser::partial_decode(reader)?)),
        (CoreType::Byte, _) => Ok(GraphBinary::Byte(u8::partial_decode(reader)?)),
        (CoreType::ByteBuffer, _) => partial_decode_byte_buffer(reader),
        (CoreType::TextP, _) => Ok(GraphBinary::TextP(TextP::partial_decode(reader)?)),
        (CoreType::TraversalStrategy, _) => Ok(GraphBinary::TraversalStrategy(
            TraversalStrategy::partial_decode(reader)?,
        )),
        (CoreType::Tree, _) => todo!(),
        (CoreType::Metrics, _) => Ok(GraphBinary::Metrics(Metrics::partial_decode(reader)?)),

        (CoreType::TraversalMetrics, _) => Ok(GraphBinary::TraversalMetrics(
            TraversalMetrics::partial_decode(reader)?,
        )),
        (CoreType::Merge, _) => Ok(GraphBinary::Merge(Merge::partial_decode(reader)?)),
        (CoreType::BulkSet, _) => Ok(GraphBinary::BulkSet(BulkSet::partial_decode(reader)?)),
        (CoreType::UnspecifiedNullObject, _) => Err(DecodeError::DecodeError(
            "UnspecifiedNullObject wrong valueflag".to_string(),
        )),
        (CoreType::Char, _) => Ok(GraphBinary::Char(char::partial_decode(reader)?)),
    }
}

fn partial_decode_byte_buffer<R: Read>(reader: &mut R) -> Result<GraphBinary, DecodeError> {
    let len = i32::partial_decode(reader)?;
    let mut buf = vec![0; len as usize];
    reader.read_exact(&mut buf)?;
    Ok(GraphBinary::ByteBuffer(buf))
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

    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError>
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

    fn get_partial_len(bytes: &[u8]) -> Result<usize, DecodeError>;

    fn get_len(bytes: &[u8]) -> Result<usize, DecodeError> {
        let partial_bytes = Self::get_partial_len(&bytes[2..])?;
        Ok(partial_bytes + 2)
    }

    fn nullable_decode<R: Read>(reader: &mut R) -> Result<Option<Self>, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let mut buf = [255_u8; 1];
        reader.read_exact(&mut buf)?;
        match buf[0] {
            0 => Ok(Self::partial_decode(reader).ok()),
            1 => Ok(None),
            err => Err(DecodeError::DecodeError(format!(
                "found {} expected 0 or 1",
                err
            ))),
        }
    }
}

pub trait Encode {
    fn type_code() -> u8;

    fn partial_encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError>;

    fn null_encode<W: Write>(writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&[Self::type_code(), ValueFlag::Null.into()])?;
        Ok(())
    }

    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&[Self::type_code(), ValueFlag::Set.into()])?;
        self.partial_encode(writer)
    }

    fn nullable_encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&[ValueFlag::Set.into()])?;
        self.partial_encode(writer)
    }
    fn write_partial_nullable_bytes<W: Write>(&self, _writer: &mut W) -> Result<(), EncodeError> {
        unimplemented!("this Method should only be called from Option<T>")
    }
}

#[test]
fn testing() {
    let mut buf: Vec<u8> = vec![];
    15_i32.partial_encode(&mut buf).unwrap();
    assert_eq!([0x00, 0x00, 0x00, 0x0F], buf.as_slice());

    buf.clear();
    15_i32.encode(&mut buf).unwrap();
    assert_eq!([0x01, 0x00, 0x00, 0x00, 0x00, 0x0F], buf.as_slice());
}

#[test]
fn test_byte_buffer_decode() {
    let buf = [
        CoreType::ByteBuffer.into(),
        0x0,
        0x0,
        0x0,
        0x0,
        0x6,
        100,
        101,
        102,
        103,
        104,
        105,
    ];

    let gb = GraphBinary::decode(&mut &buf[..]);
    assert_eq!(
        GraphBinary::ByteBuffer(vec![100, 101, 102, 103, 104, 105]),
        gb.unwrap()
    );

    let buf_0 = [CoreType::ByteBuffer.into(), 0x0, 0x0, 0x0, 0x0, 0x0];

    let gb = GraphBinary::decode(&mut &buf_0[..]);
    assert_eq!(GraphBinary::ByteBuffer(vec![]), gb.unwrap())
}
