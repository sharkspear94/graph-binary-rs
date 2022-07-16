use std::collections::{BTreeSet, HashMap};
use std::fmt::Display;
use std::io::{Read, Write};

use crate::error::{DecodeError, EncodeError};
use crate::graphson::{DecodeGraphSON, EncodeGraphSON};
use crate::macros::{TryBorrowFrom, TryMutBorrowFrom};
use crate::structure::bulkset::BulkSet;
use crate::structure::bytebuffer::ByteBuffer;
use crate::structure::bytecode::Bytecode;
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
use crate::Binding;
use crate::{specs::CoreType, structure::edge::Edge};
use serde::de::Visitor;
use serde::Deserialize;
use uuid::Uuid;

/// All possible Values supported in the [GraphBinary serialization format](https://tinkerpop.apache.org/docs/current/dev/io/#graphbinary)
#[derive(Debug, PartialEq, Clone)]
pub enum GremlinValue {
    Int(i32),
    Long(i64),
    String(String),
    Date(i64),
    Timestamp(i64),
    Class(String),
    Double(f64),
    Float(f32),
    List(Vec<GremlinValue>),
    Set(Vec<GremlinValue>),
    Map(HashMap<MapKeys, GremlinValue>),
    Uuid(Uuid),
    Edge(Edge),
    Path(Path),
    Property(Property),
    Graph(Graph),
    Vertex(Vertex),
    VertexProperty(VertexProperty),
    Barrier(Barrier),
    Binding(Binding),
    Bytecode(Bytecode),
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
    ByteBuffer(ByteBuffer),
    Short(i16),
    Boolean(bool),
    TextP(TextP),
    TraversalStrategy(TraversalStrategy),
    BulkSet(BulkSet),
    Tree(BTreeSet<GremlinValue>),
    Metrics(Metrics),
    TraversalMetrics(TraversalMetrics),
    Merge(Merge),
    UnspecifiedNullObject,
    // Custom
    #[cfg(feature = "extended")]
    Char(char),
    #[cfg(feature = "extended")]
    Duration(),
    #[cfg(feature = "extended")]
    InetAddress(std::net::IpAddr),
    #[cfg(feature = "extended")]
    Instant(),
    #[cfg(feature = "extended")]
    LocalDate(),
    #[cfg(feature = "extended")]
    LocalDateTime(),
    #[cfg(feature = "extended")]
    LocalTime(),
    #[cfg(feature = "extended")]
    MonthDay(),
    #[cfg(feature = "extended")]
    OffsetDateTime(),
    #[cfg(feature = "extended")]
    OffsetTime(),
    #[cfg(feature = "extended")]
    Period(),
    #[cfg(feature = "extended")]
    Year(),
    #[cfg(feature = "extended")]
    YearMonth(),
    #[cfg(feature = "extended")]
    ZonedDateTime(),
    #[cfg(feature = "extended")]
    ZoneOffset(),
}

#[cfg(feature = "graph_binary")]
pub fn encode_null_object<W: Write>(writer: &mut W) -> Result<(), EncodeError> {
    writer.write_all(&[
        CoreType::UnspecifiedNullObject.into(),
        ValueFlag::Null.into(),
    ])?;
    Ok(())
}

#[cfg(feature = "graph_binary")]
fn encode_byte_buffer<W: Write>(writer: &mut W, buf: &[u8]) -> Result<(), EncodeError> {
    writer.write_all(&[CoreType::ByteBuffer.into(), ValueFlag::Null.into()])?;
    let len = (buf.len() as i32).to_be_bytes();
    writer.write_all(&len)?;
    writer.write_all(buf)?;
    Ok(())
}

impl GremlinValue {
    /// Returns an Option of an owned value if the Type was the GremlinValue variant.
    /// Returns None if GremlinValue enum holds another Type
    ///
    /// ```
    /// # use gremlin_types::graph_binary::GremlinValue;
    ///
    /// let gb1 = GremlinValue::Boolean(true);
    /// assert_eq!(Some(true),gb1.get());
    ///
    /// let gb2 = GremlinValue::Boolean(true);
    /// assert_eq!(None, gb2.get::<String>());
    ///
    /// ```
    pub fn get<T: TryFrom<GremlinValue>>(self) -> Option<T> {
        T::try_from(self).ok()
    }

    /// Returns an Option of an cloned value if the Type was the GremlinValue variant.
    /// Returns None if GremlinValue enum holds another Type
    ///
    /// ```
    /// # use gremlin_types::graph_binary::GremlinValue;
    ///
    /// let gb1 = GremlinValue::Boolean(true);
    /// assert_eq!(Some(true),gb1.get());
    ///
    /// let gb2 = GremlinValue::Boolean(true);
    /// assert_eq!(None, gb2.get::<String>());
    ///
    /// ```
    pub fn get_cloned<T: TryFrom<GremlinValue>>(&self) -> Option<T> {
        T::try_from(self.clone()).ok()
    }
    /// Returns an Option of the borrowed value if the Type was the GremlinValue variant.
    /// Returns None if GremlinValue enum holds another Type
    ///
    /// ```
    /// # use gremlin_types::graph_binary::GremlinValue;
    ///
    /// let gb = GremlinValue::String("Janus".to_string());
    ///
    /// assert_eq!(Some("Janus"),gb.get_ref());
    /// assert_eq!(Some(&String::from("Janus")),gb.get_ref());
    /// assert_eq!(None, gb.get_ref::<bool>());
    ///
    /// ```
    pub fn get_ref<T: TryBorrowFrom + ?Sized>(&self) -> Option<&T> {
        T::try_borrow_from(self)
    }

    /// Returns an Option of the mutable borrowed value if the Type was the GremlinValue variant.
    /// Returns None if GremlinValue enum holds another Type
    ///
    /// ```
    /// # use gremlin_types::graph_binary::GremlinValue;
    ///
    /// let mut gb = GremlinValue::String("Janus".to_string());
    ///
    /// let s = gb.get_ref_mut::<String>().unwrap();
    /// s.push_str("Graph");
    ///
    /// assert_eq!(Some(&String::from("JanusGraph")),gb.get_ref());
    /// assert_eq!(None, gb.get_ref::<bool>());
    ///
    /// ```
    pub fn get_ref_mut<T: TryMutBorrowFrom + ?Sized>(&mut self) -> Option<&mut T> {
        T::try_mut_borrow_from(self)
    }

    pub fn build_fq_bytes<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        match self {
            GremlinValue::Int(val) => val.encode(writer),
            GremlinValue::Long(val) => val.encode(writer),
            GremlinValue::String(val) => val.encode(writer),
            GremlinValue::Date(_) => todo!(),
            GremlinValue::Timestamp(_) => todo!(),
            GremlinValue::Class(val) => val.encode(writer),
            GremlinValue::Double(val) => val.encode(writer),
            GremlinValue::Float(val) => val.encode(writer),
            GremlinValue::List(val) => val.encode(writer),
            GremlinValue::Set(val) => val.encode(writer),
            GremlinValue::Map(val) => val.encode(writer),
            GremlinValue::Uuid(val) => val.encode(writer),
            GremlinValue::Edge(val) => val.encode(writer),
            GremlinValue::Path(val) => val.encode(writer),
            GremlinValue::Property(val) => val.encode(writer),
            GremlinValue::Graph(val) => val.encode(writer),
            GremlinValue::Vertex(val) => val.encode(writer),
            GremlinValue::VertexProperty(val) => val.encode(writer),
            GremlinValue::Barrier(val) => val.encode(writer),
            GremlinValue::Binding(val) => val.encode(writer),
            GremlinValue::Bytecode(val) => val.encode(writer),
            GremlinValue::Cardinality(val) => val.encode(writer),
            GremlinValue::Column(val) => val.encode(writer),
            GremlinValue::Direction(val) => val.encode(writer),
            GremlinValue::Operator(val) => val.encode(writer),
            GremlinValue::Order(val) => val.encode(writer),
            GremlinValue::Pick(val) => val.encode(writer),
            GremlinValue::Pop(val) => val.encode(writer),
            GremlinValue::Lambda(val) => val.encode(writer),
            GremlinValue::P(val) => val.encode(writer),
            GremlinValue::Scope(val) => val.encode(writer),
            GremlinValue::T(val) => val.encode(writer),
            GremlinValue::Traverser(val) => val.encode(writer),
            // GraphBinary::BigDecimal(_) => todo!(),
            // GraphBinary::BigInteger(_) => todo!(),
            GremlinValue::Byte(val) => val.encode(writer),
            GremlinValue::ByteBuffer(val) => val.encode(writer),
            GremlinValue::Short(val) => val.encode(writer),
            GremlinValue::Boolean(val) => val.encode(writer),
            GremlinValue::TextP(val) => val.encode(writer),
            GremlinValue::TraversalStrategy(val) => val.encode(writer),
            GremlinValue::BulkSet(_) => todo!(),
            GremlinValue::Tree(_) => todo!(),
            GremlinValue::Metrics(val) => val.encode(writer),
            GremlinValue::TraversalMetrics(val) => val.encode(writer),
            GremlinValue::Merge(val) => val.encode(writer),
            GremlinValue::UnspecifiedNullObject => encode_null_object(writer),
            #[cfg(feature = "extended")]
            GremlinValue::Char(val) => val.encode(writer),
            #[cfg(feature = "extended")]
            GremlinValue::Char(char) => unimplemented!(),
            #[cfg(feature = "extended")]
            GremlinValue::Duration() => unimplemented!(),
            #[cfg(feature = "extended")]
            GremlinValue::InetAddress(_) => unimplemented!(),
            #[cfg(feature = "extended")]
            GremlinValue::Instant() => unimplemented!(),
            #[cfg(feature = "extended")]
            GremlinValue::LocalDate() => unimplemented!(),
            #[cfg(feature = "extended")]
            GremlinValue::LocalDateTime() => unimplemented!(),
            #[cfg(feature = "extended")]
            GremlinValue::LocalTime() => unimplemented!(),
            #[cfg(feature = "extended")]
            GremlinValue::MonthDay() => unimplemented!(),
            #[cfg(feature = "extended")]
            GremlinValue::OffsetDateTime() => unimplemented!(),
            #[cfg(feature = "extended")]
            GremlinValue::OffsetTime() => unimplemented!(),
            #[cfg(feature = "extended")]
            GremlinValue::Period() => unimplemented!(),
            #[cfg(feature = "extended")]
            GremlinValue::Year() => unimplemented!(),
            #[cfg(feature = "extended")]
            GremlinValue::YearMonth() => unimplemented!(),
            #[cfg(feature = "extended")]
            GremlinValue::ZonedDateTime() => unimplemented!(),
            #[cfg(feature = "extended")]
            GremlinValue::ZoneOffset() => unimplemented!(),
            // GraphBinary::Custom => todo!(),
            // _ =>  Bytes::new()
        }
    }

    pub fn to_option(self) -> Option<GremlinValue> {
        match self {
            GremlinValue::UnspecifiedNullObject => None,
            graph_binary => Some(graph_binary),
        }
    }
}

impl Display for GremlinValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GremlinValue::Int(val) => write!(f, "{val}_i32"),
            GremlinValue::Long(val) => write!(f, "{val}_i64"),
            GremlinValue::String(val) => write!(f, "\"{val}\""),
            GremlinValue::Class(val) => write!(f, "Class:{val}"),
            GremlinValue::Double(val) => write!(f, "{val}_f64"),
            GremlinValue::Float(val) => write!(f, "{val}_f32"),
            GremlinValue::List(val) => {
                write!(f, "List::[")?;
                for i in val {
                    write!(f, " {i},")?;
                }
                write!(f, "]")
            }
            GremlinValue::Set(val) => {
                write!(f, "Set::[")?;
                for i in val {
                    writeln!(f, " {i},")?;
                }
                write!(f, "]")
            }
            GremlinValue::Map(val) => {
                write!(f, "Map::[")?;
                for (key, value) in val {
                    writeln!(f, "{{{key}:{value}}},")?;
                }
                write!(f, "]")
            }
            GremlinValue::Uuid(val) => write!(f, "Uuid::{val}"),
            GremlinValue::Edge(val) => write!(f, "Edge::{val}"),
            GremlinValue::Path(val) => write!(f, "Path::{val}"),
            GremlinValue::Property(val) => write!(f, "Property::{val}"),
            GremlinValue::Graph(val) => write!(f, "Graph::{val}"),
            GremlinValue::Vertex(val) => write!(f, "Vertex::{val}"),
            GremlinValue::VertexProperty(val) => write!(f, "VertexProperty::{val}"),
            GremlinValue::Barrier(val) => write!(f, "Barrier::{val}"),
            GremlinValue::Binding(val) => write!(f, "Binding::{val}"),
            GremlinValue::Bytecode(val) => write!(f, "Bytecode::{val}"),
            GremlinValue::Cardinality(val) => write!(f, "Cardinality::{val}"),
            GremlinValue::Column(val) => write!(f, "Column::{val}"),
            GremlinValue::Direction(val) => write!(f, "Direction::{val}"),
            GremlinValue::Operator(val) => write!(f, "Operator::{val}"),
            GremlinValue::Order(val) => write!(f, "Order::{val}"),
            GremlinValue::Pick(val) => write!(f, "Pick::{val}"),
            GremlinValue::Pop(val) => write!(f, "Pop::{val}"),
            GremlinValue::Lambda(val) => write!(f, "Lambda::{val}"),
            GremlinValue::P(val) => write!(f, "P::{val}"),
            GremlinValue::Scope(val) => write!(f, "Scope::{val}"),
            GremlinValue::T(val) => write!(f, "T::{val}"),
            GremlinValue::Traverser(val) => write!(f, "Traverser::{val}"),
            GremlinValue::Byte(val) => write!(f, "{val}_u8"),
            GremlinValue::ByteBuffer(val) => todo!(),
            GremlinValue::Short(val) => write!(f, "{val}_i16"),
            GremlinValue::Boolean(val) => write!(f, "{val}"),
            GremlinValue::TextP(val) => write!(f, "TextP::{val}"),
            GremlinValue::TraversalStrategy(val) => write!(f, "TraversalStrategy::{val}"),
            GremlinValue::BulkSet(val) => todo!(),
            GremlinValue::Tree(val) => todo!(),
            GremlinValue::Metrics(val) => write!(f, "{val}"),
            GremlinValue::TraversalMetrics(val) => write!(f, "{val}"),
            GremlinValue::Merge(val) => write!(f, "Merge::{val}"),
            GremlinValue::UnspecifiedNullObject => write!(f, "UnspecifiedNullObject"),
            GremlinValue::Date(_) => todo!(),
            GremlinValue::Timestamp(_) => todo!(),
            #[cfg(feature = "extended")]
            GremlinValue::Char(val) => write!(f, "{val}"),
            #[cfg(feature = "extended")]
            GremlinValue::Char(char) => unimplemented!(),
            #[cfg(feature = "extended")]
            GremlinValue::Duration() => unimplemented!(),
            #[cfg(feature = "extended")]
            GremlinValue::InetAddress(_) => unimplemented!(),
            #[cfg(feature = "extended")]
            GremlinValue::Instant() => unimplemented!(),
            #[cfg(feature = "extended")]
            GremlinValue::LocalDate() => unimplemented!(),
            #[cfg(feature = "extended")]
            GremlinValue::LocalDateTime() => unimplemented!(),
            #[cfg(feature = "extended")]
            GremlinValue::LocalTime() => unimplemented!(),
            #[cfg(feature = "extended")]
            GremlinValue::MonthDay() => unimplemented!(),
            #[cfg(feature = "extended")]
            GremlinValue::OffsetDateTime() => unimplemented!(),
            #[cfg(feature = "extended")]
            GremlinValue::OffsetTime() => unimplemented!(),
            #[cfg(feature = "extended")]
            GremlinValue::Period() => unimplemented!(),
            #[cfg(feature = "extended")]
            GremlinValue::Year() => unimplemented!(),
            #[cfg(feature = "extended")]
            GremlinValue::YearMonth() => unimplemented!(),
            #[cfg(feature = "extended")]
            GremlinValue::ZonedDateTime() => unimplemented!(),
            #[cfg(feature = "extended")]
            GremlinValue::ZoneOffset() => unimplemented!(),
        }
    }
}

impl Default for GremlinValue {
    fn default() -> Self {
        GremlinValue::UnspecifiedNullObject
    }
}

#[cfg(feature = "graph_binary")]
impl Decode for GremlinValue {
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
            CoreType::List => Vec::<GremlinValue>::get_len(bytes),
            CoreType::Set => Vec::<GremlinValue>::get_len(bytes),
            CoreType::Map => HashMap::<MapKeys, GremlinValue>::get_len(bytes),
            CoreType::Uuid => Uuid::get_len(bytes),
            CoreType::Edge => Edge::get_len(bytes),
            CoreType::Path => Path::get_len(bytes),
            CoreType::Property => Property::get_len(bytes),
            CoreType::Graph => Graph::get_len(bytes),
            CoreType::Vertex => Vertex::get_len(bytes),
            CoreType::VertexProperty => VertexProperty::get_len(bytes),
            CoreType::Barrier => Barrier::get_len(bytes),
            CoreType::Binding => Binding::get_len(bytes),
            CoreType::ByteCode => Bytecode::get_len(bytes),
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
            #[cfg(feature = "extended")]
            CoreType::Char => char::get_len(bytes),
        }
    }
}

#[cfg(feature = "graph_binary")]
impl Encode for GremlinValue {
    fn type_code() -> u8 {
        unimplemented!("")
    }

    fn partial_encode<W: Write>(&self, _writer: &mut W) -> Result<(), EncodeError> {
        unimplemented!("partial decode is not supported for GraphBinary")
    }

    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        self.build_fq_bytes(writer)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum MapKeys {
    Int(i32),
    String(String),
    Long(i64),
    Uuid(Uuid),
    T(T),
    Direction(Direction),
}

impl Display for MapKeys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapKeys::Int(val) => write!(f, "{val}"),
            MapKeys::String(val) => write!(f, "{val}"),
            MapKeys::Long(val) => write!(f, "{val}"),
            MapKeys::Uuid(val) => write!(f, "{val}"),
            MapKeys::T(val) => write!(f, "{val}"),
            MapKeys::Direction(val) => write!(f, "{val}"),
        }
    }
}

impl From<MapKeys> for GremlinValue {
    fn from(keys: MapKeys) -> GremlinValue {
        match keys {
            MapKeys::Int(val) => GremlinValue::Int(val),
            MapKeys::String(val) => GremlinValue::String(val),
            MapKeys::Long(val) => GremlinValue::Long(val),
            MapKeys::Uuid(val) => GremlinValue::Uuid(val),
            MapKeys::T(val) => GremlinValue::T(val),
            MapKeys::Direction(val) => GremlinValue::Direction(val),
        }
    }
}

impl<T: Into<GremlinValue> + Clone> From<&T> for GremlinValue {
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}

impl<T: Into<GremlinValue>, const N: usize> From<[T; N]> for GremlinValue {
    fn from(array: [T; N]) -> Self {
        GremlinValue::List(array.into_iter().map(Into::into).collect())
    }
}

impl TryFrom<GremlinValue> for MapKeys {
    type Error = DecodeError;

    fn try_from(value: GremlinValue) -> Result<Self, Self::Error> {
        match value {
            GremlinValue::Int(val) => Ok(MapKeys::Int(val)),
            GremlinValue::Long(val) => Ok(MapKeys::Long(val)),
            GremlinValue::String(val) => Ok(MapKeys::String(val)),
            GremlinValue::Uuid(val) => Ok(MapKeys::Uuid(val)),
            GremlinValue::T(val) => Ok(MapKeys::T(val)),
            GremlinValue::Direction(val) => Ok(MapKeys::Direction(val)),
            rest => Err(DecodeError::ConvertError(format!(
                "cannot convert from {:?} to MapKeys",
                rest
            ))),
        }
    }
}

impl TryFrom<MapKeys> for String {
    type Error = DecodeError;

    fn try_from(value: MapKeys) -> Result<Self, Self::Error> {
        match value {
            MapKeys::Int(_) => Err(DecodeError::ConvertError(
                "cannot convert from MapKeys::Int to String".to_string(),
            )),
            MapKeys::String(s) => Ok(s),
            MapKeys::Long(_) => Err(DecodeError::ConvertError(
                "cannot convert from MapKeys::Long to String".to_string(),
            )),
            MapKeys::Uuid(u) => Ok(u.to_string()),
            MapKeys::T(t) => Ok(t.to_string()),
            MapKeys::Direction(d) => Ok(d.to_string()),
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

#[cfg(feature = "graph_binary")]
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

#[cfg(feature = "graph_binary")]
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
        let key = GremlinValue::decode(reader)?;
        MapKeys::try_from(key)
    }

    fn get_partial_len(_bytes: &[u8]) -> Result<usize, DecodeError> {
        unimplemented!("use consume_bytes insted")
    }

    fn get_len(bytes: &[u8]) -> Result<usize, DecodeError> {
        GremlinValue::get_len(bytes)
    }
}

#[cfg(feature = "graph_son")]
impl EncodeGraphSON for MapKeys {
    fn encode_v3(&self) -> serde_json::Value {
        match self {
            MapKeys::Int(val) => val.encode_v3(),
            MapKeys::String(val) => val.encode_v3(),
            MapKeys::Long(val) => val.encode_v3(),
            MapKeys::Uuid(val) => val.encode_v3(),
            MapKeys::T(val) => val.encode_v3(),
            MapKeys::Direction(val) => val.encode_v3(),
        }
    }

    fn encode_v2(&self) -> serde_json::Value {
        match self {
            MapKeys::Int(_) => panic!(
                "non String Mapkeys are not suppoted in GraphSONV2, tried Serilization with i32"
            ),
            MapKeys::String(val) => val.encode_v2(),
            MapKeys::Long(_) => panic!(
                "non String Mapkeys are not suppoted in GraphSONV2, tried Serilization with i64"
            ),
            MapKeys::Uuid(val) => val.to_string().encode_v2(),
            MapKeys::T(val) => val.to_string().encode_v2(),
            MapKeys::Direction(val) => val.to_string().encode_v2(),
        }
    }

    fn encode_v1(&self) -> serde_json::Value {
        match self {
            MapKeys::Int(_) => panic!(
                "non String Mapkeys are not suppoted in GraphSONV2, tried Serilization with i32"
            ),
            MapKeys::String(val) => val.encode_v2(),
            MapKeys::Long(_) => panic!(
                "non String Mapkeys are not suppoted in GraphSONV2, tried Serilization with i64"
            ),
            MapKeys::Uuid(val) => val.to_string().encode_v2(),
            MapKeys::T(val) => val.to_string().encode_v2(),
            MapKeys::Direction(val) => val.to_string().encode_v2(),
        }
    }
}

#[cfg(feature = "graph_son")]
impl DecodeGraphSON for MapKeys {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let g_key = GremlinValue::decode_v3(j_val)?;
        MapKeys::try_from(g_key)
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let s = String::decode_v2(j_val)?;
        Ok(MapKeys::String(s))
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        todo!()
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

#[cfg(feature = "graph_binary")]
pub fn decode<R: Read>(reader: &mut R) -> Result<GremlinValue, DecodeError> {
    let mut buf = [255_u8; 2];
    reader.read_exact(&mut buf)?;

    let identifier = CoreType::try_from(buf[0])?;
    let value_flag = ValueFlag::try_from(buf[1])?;

    match (identifier, value_flag) {
        (_, ValueFlag::Null) => Ok(GremlinValue::UnspecifiedNullObject),
        (CoreType::Int32, _) => Ok(GremlinValue::Int(i32::partial_decode(reader)?)),
        (CoreType::Long, _) => Ok(GremlinValue::Long(i64::partial_decode(reader)?)),
        (CoreType::String, _) => Ok(GremlinValue::String(String::partial_decode(reader)?)),
        (CoreType::Class, _) => Ok(GremlinValue::Class(String::partial_decode(reader)?)),
        (CoreType::Double, _) => Ok(GremlinValue::Double(f64::partial_decode(reader)?)),
        (CoreType::Float, _) => Ok(GremlinValue::Float(f32::partial_decode(reader)?)),
        (CoreType::List, _) => Ok(GremlinValue::List(Vec::partial_decode(reader)?)),
        (CoreType::Set, _) => Ok(GremlinValue::Set(Vec::partial_decode(reader)?)),
        (CoreType::Map, _) => Ok(GremlinValue::Map(
            HashMap::<MapKeys, GremlinValue>::partial_decode(reader)?,
        )),
        (CoreType::Uuid, _) => Ok(GremlinValue::Uuid(Uuid::partial_decode(reader)?)),
        (CoreType::Edge, _) => Ok(GremlinValue::Edge(Edge::partial_decode(reader)?)),
        (CoreType::Path, _) => Ok(GremlinValue::Path(Path::partial_decode(reader)?)),
        (CoreType::Property, _) => Ok(GremlinValue::Property(Property::partial_decode(reader)?)),
        (CoreType::Graph, _) => Ok(GremlinValue::Graph(Graph::partial_decode(reader)?)),
        (CoreType::Vertex, _) => Ok(GremlinValue::Vertex(Vertex::partial_decode(reader)?)),
        (CoreType::VertexProperty, _) => Ok(GremlinValue::VertexProperty(
            VertexProperty::partial_decode(reader)?,
        )),
        (CoreType::Short, _) => Ok(GremlinValue::Short(i16::partial_decode(reader)?)),
        (CoreType::Boolean, _) => Ok(GremlinValue::Boolean(bool::partial_decode(reader)?)),

        (CoreType::Cardinality, _) => Ok(GremlinValue::Cardinality(Cardinality::partial_decode(
            reader,
        )?)),
        (CoreType::Column, _) => Ok(GremlinValue::Column(Column::partial_decode(reader)?)),
        (CoreType::Direction, _) => Ok(GremlinValue::Direction(Direction::partial_decode(reader)?)),
        (CoreType::Operator, _) => Ok(GremlinValue::Operator(Operator::partial_decode(reader)?)),
        (CoreType::Order, _) => Ok(GremlinValue::Order(Order::partial_decode(reader)?)),
        (CoreType::Pick, _) => Ok(GremlinValue::Pick(Pick::partial_decode(reader)?)),
        (CoreType::Pop, _) => Ok(GremlinValue::Pop(Pop::partial_decode(reader)?)),
        (CoreType::P, _) => Ok(GremlinValue::P(P::partial_decode(reader)?)),
        (CoreType::Scope, _) => Ok(GremlinValue::Scope(Scope::partial_decode(reader)?)),
        (CoreType::T, _) => Ok(GremlinValue::T(T::partial_decode(reader)?)),
        (CoreType::Barrier, _) => Ok(GremlinValue::Barrier(Barrier::partial_decode(reader)?)),
        (CoreType::Binding, _) => Ok(GremlinValue::Binding(Binding::partial_decode(reader)?)),
        (CoreType::ByteCode, _) => Ok(GremlinValue::Bytecode(Bytecode::partial_decode(reader)?)),
        (CoreType::Lambda, _) => Ok(GremlinValue::Lambda(Lambda::partial_decode(reader)?)),
        (CoreType::Traverser, _) => Ok(GremlinValue::Traverser(Traverser::partial_decode(reader)?)),
        (CoreType::Byte, _) => Ok(GremlinValue::Byte(u8::partial_decode(reader)?)),
        (CoreType::ByteBuffer, _) => Ok(GremlinValue::ByteBuffer(ByteBuffer::partial_decode(
            reader,
        )?)),
        (CoreType::TextP, _) => Ok(GremlinValue::TextP(TextP::partial_decode(reader)?)),
        (CoreType::TraversalStrategy, _) => Ok(GremlinValue::TraversalStrategy(
            TraversalStrategy::partial_decode(reader)?,
        )),
        (CoreType::Tree, _) => todo!(),
        (CoreType::Metrics, _) => Ok(GremlinValue::Metrics(Metrics::partial_decode(reader)?)),

        (CoreType::TraversalMetrics, _) => Ok(GremlinValue::TraversalMetrics(
            TraversalMetrics::partial_decode(reader)?,
        )),
        (CoreType::Merge, _) => Ok(GremlinValue::Merge(Merge::partial_decode(reader)?)),
        (CoreType::BulkSet, _) => Ok(GremlinValue::BulkSet(BulkSet::partial_decode(reader)?)),
        (CoreType::UnspecifiedNullObject, _) => Err(DecodeError::DecodeError(
            "UnspecifiedNullObject wrong valueflag".to_string(),
        )),
        #[cfg(feature = "extended")]
        (CoreType::Char, _) => Ok(GremlinValue::Char(char::partial_decode(reader)?)),
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
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

#[cfg(feature = "graph_binary")]
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

#[cfg(feature = "graph_binary")]
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
fn testqdeasd() {
    let mut buf: Vec<u8> = vec![];
    15_i32.partial_encode(&mut buf).unwrap();
    assert_eq!([0x00, 0x00, 0x00, 0x0F], buf.as_slice());

    buf.clear();
    15_i32.encode(&mut buf).unwrap();
    assert_eq!([0x01, 0x00, 0x00, 0x00, 0x00, 0x0F], buf.as_slice());
}
