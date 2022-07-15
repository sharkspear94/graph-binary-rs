use std::collections::{BTreeSet, HashMap};
use std::fmt::Display;
use std::io::{Read, Write};
use std::str::FromStr;

use crate::error::{DecodeError, EncodeError};
use crate::graphson::{DecodeGraphSON, EncodeGraphSON};
use crate::macros::{TryBorrowFrom, TryMutBorrowFrom};
use crate::structure::bulkset::BulkSet;
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
pub enum GremlinTypes {
    Int(i32),
    Long(i64),
    String(String),
    // Date(Date),
    // Timestamp(Date),
    Class(String),
    Double(f64),
    Float(f32),
    List(Vec<GremlinTypes>),
    Set(Vec<GremlinTypes>),
    Map(HashMap<MapKeys, GremlinTypes>),
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
    ByteBuffer(Vec<u8>),
    Short(i16),
    Boolean(bool),
    TextP(TextP),
    TraversalStrategy(TraversalStrategy),
    BulkSet(BulkSet),
    Tree(BTreeSet<GremlinTypes>),
    Metrics(Metrics),
    TraversalMetrics(TraversalMetrics),
    Merge(Merge),
    UnspecifiedNullObject,
    // Custom
    Char(char),
    // Duration(),
    // InetAddress(std::net::IpAddr),
    // Instant(),
    // LocalDate(),
    // LocalDateTime(),
    // LocalTime(),
    // MonthDay(),
    // OffsetDateTime(),
    // OffsetTime(),
    // Period(),
    // Year(),
    // YearMonth(),
    // ZonedDateTime(),
    // ZoneOffset,

}

pub fn encode_null_object<W: Write>(writer: &mut W) -> Result<(), EncodeError> {
    writer.write_all(&[
        CoreType::UnspecifiedNullObject.into(),
        ValueFlag::Null.into(),
    ])?;
    Ok(())
}

fn encode_byte_buffer<W: Write>(writer: &mut W, buf: &[u8]) -> Result<(), EncodeError> {
    writer.write_all(&[CoreType::ByteBuffer.into(), ValueFlag::Null.into()])?;
    let len = (buf.len() as i32).to_be_bytes();
    writer.write_all(&len)?;
    writer.write_all(buf)?;
    Ok(())
}

impl GremlinTypes {
    /// Returns an Option of an owned value if the Type was the GremlinTypes variant.
    /// Returns None if GremlinTypes enum holds another Type
    ///
    /// ```
    /// # use gremlin_types::graph_binary::GremlinTypes;
    ///
    /// let gb1 = GremlinTypes::Boolean(true);
    /// assert_eq!(Some(true),gb1.get());
    ///
    /// let gb2 = GremlinTypes::Boolean(true);
    /// assert_eq!(None, gb2.get::<String>());
    ///
    /// ```
    pub fn get<T: TryFrom<GremlinTypes>>(self) -> Option<T> {
        T::try_from(self).ok()
    }

    /// Returns an Option of an cloned value if the Type was the GremlinTypes variant.
    /// Returns None if GremlinTypes enum holds another Type
    ///
    /// ```
    /// # use gremlin_types::graph_binary::GremlinTypes;
    ///
    /// let gb1 = GremlinTypes::Boolean(true);
    /// assert_eq!(Some(true),gb1.get());
    ///
    /// let gb2 = GremlinTypes::Boolean(true);
    /// assert_eq!(None, gb2.get::<String>());
    ///
    /// ```
    pub fn get_cloned<T: TryFrom<GremlinTypes>>(&self) -> Option<T> {
        T::try_from(self.clone()).ok()
    }
    /// Returns an Option of the borrowed value if the Type was the GremlinTypes variant.
    /// Returns None if GremlinTypes enum holds another Type
    ///
    /// ```
    /// # use gremlin_types::graph_binary::GremlinTypes;
    ///
    /// let gb = GremlinTypes::String("Janus".to_string());
    ///
    /// assert_eq!(Some("Janus"),gb.get_ref());
    /// assert_eq!(Some(&String::from("Janus")),gb.get_ref());
    /// assert_eq!(None, gb.get_ref::<bool>());
    ///
    /// ```
    pub fn get_ref<T: TryBorrowFrom + ?Sized>(&self) -> Option<&T> {
        T::try_borrow_from(self)
    }

    /// Returns an Option of the mutable borrowed value if the Type was the GremlinTypes variant.
    /// Returns None if GremlinTypes enum holds another Type
    ///
    /// ```
    /// # use gremlin_types::graph_binary::GremlinTypes;
    ///
    /// let mut gb = GremlinTypes::String("Janus".to_string());
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
            GremlinTypes::Int(val) => val.encode(writer),
            GremlinTypes::Long(val) => val.encode(writer),
            GremlinTypes::String(val) => val.encode(writer),
            // CoreType::Date(_) => todo!(),
            // CoreType::Timestamp(_) => todo!(),
            GremlinTypes::Class(val) => val.encode(writer),
            GremlinTypes::Double(val) => val.encode(writer),
            GremlinTypes::Float(val) => val.encode(writer),
            GremlinTypes::List(val) => val.encode(writer),
            GremlinTypes::Set(val) => val.encode(writer),
            GremlinTypes::Map(val) => val.encode(writer),
            GremlinTypes::Uuid(val) => val.encode(writer),
            GremlinTypes::Edge(val) => val.encode(writer),
            GremlinTypes::Path(val) => val.encode(writer),
            GremlinTypes::Property(val) => val.encode(writer),
            GremlinTypes::Graph(val) => val.encode(writer),
            GremlinTypes::Vertex(val) => val.encode(writer),
            GremlinTypes::VertexProperty(val) => val.encode(writer),
            GremlinTypes::Barrier(val) => val.encode(writer),
            GremlinTypes::Binding(val) => val.encode(writer),
            GremlinTypes::Bytecode(val) => val.encode(writer),
            GremlinTypes::Cardinality(val) => val.encode(writer),
            GremlinTypes::Column(val) => val.encode(writer),
            GremlinTypes::Direction(val) => val.encode(writer),
            GremlinTypes::Operator(val) => val.encode(writer),
            GremlinTypes::Order(val) => val.encode(writer),
            GremlinTypes::Pick(val) => val.encode(writer),
            GremlinTypes::Pop(val) => val.encode(writer),
            GremlinTypes::Lambda(val) => val.encode(writer),
            GremlinTypes::P(val) => val.encode(writer),
            GremlinTypes::Scope(val) => val.encode(writer),
            GremlinTypes::T(val) => val.encode(writer),
            GremlinTypes::Traverser(val) => val.encode(writer),
            // GraphBinary::BigDecimal(_) => todo!(),
            // GraphBinary::BigInteger(_) => todo!(),
            GremlinTypes::Byte(val) => val.encode(writer),
            GremlinTypes::ByteBuffer(buf) => encode_byte_buffer(writer, buf),
            GremlinTypes::Short(val) => val.encode(writer),
            GremlinTypes::Boolean(val) => val.encode(writer),
            GremlinTypes::TextP(val) => val.encode(writer),
            GremlinTypes::TraversalStrategy(val) => val.encode(writer),
            GremlinTypes::BulkSet(_) => todo!(),
            GremlinTypes::Tree(_) => todo!(),
            GremlinTypes::Metrics(val) => val.encode(writer),
            GremlinTypes::TraversalMetrics(val) => val.encode(writer),
            GremlinTypes::Merge(val) => val.encode(writer),
            GremlinTypes::UnspecifiedNullObject => encode_null_object(writer),
            GremlinTypes::Char(val) => val.encode(writer),
            // GraphBinary::Custom => todo!(),
            // _ =>  Bytes::new()
        }
    }

    pub fn type_info(&self) -> CoreType {
        match self {
            GremlinTypes::Int(_) => CoreType::Int32,
            GremlinTypes::Long(_) => CoreType::Long,
            GremlinTypes::String(_) => CoreType::String,
            GremlinTypes::Class(_) => CoreType::Class,
            GremlinTypes::Double(_) => CoreType::Double,
            GremlinTypes::Float(_) => CoreType::Float,
            GremlinTypes::List(_) => CoreType::List,
            GremlinTypes::Set(_) => CoreType::Set,
            GremlinTypes::Map(_) => CoreType::Map,
            GremlinTypes::Uuid(_) => CoreType::Uuid,
            GremlinTypes::Edge(_) => CoreType::Edge,
            GremlinTypes::Path(_) => CoreType::Path,
            GremlinTypes::Property(_) => CoreType::Property,
            GremlinTypes::Graph(_) => CoreType::Graph,
            GremlinTypes::Vertex(_) => CoreType::Vertex,
            GremlinTypes::VertexProperty(_) => CoreType::VertexProperty,
            GremlinTypes::Barrier(_) => CoreType::Barrier,
            GremlinTypes::Binding(_) => CoreType::Binding,
            GremlinTypes::Bytecode(_) => CoreType::ByteCode,
            GremlinTypes::Cardinality(_) => CoreType::Cardinality,
            GremlinTypes::Column(_) => CoreType::Column,
            GremlinTypes::Direction(_) => CoreType::Direction,
            GremlinTypes::Operator(_) => CoreType::Operator,
            GremlinTypes::Order(_) => CoreType::Order,
            GremlinTypes::Pick(_) => CoreType::Pick,
            GremlinTypes::Pop(_) => CoreType::Pop,
            GremlinTypes::Lambda(_) => CoreType::Lambda,
            GremlinTypes::P(_) => CoreType::P,
            GremlinTypes::Scope(_) => CoreType::Scope,
            GremlinTypes::T(_) => CoreType::T,
            GremlinTypes::Traverser(_) => CoreType::Traverser,
            GremlinTypes::Byte(_) => CoreType::Byte,
            GremlinTypes::ByteBuffer(_) => CoreType::ByteBuffer,
            GremlinTypes::Short(_) => CoreType::Short,
            GremlinTypes::Boolean(_) => CoreType::Boolean,
            GremlinTypes::TextP(_) => CoreType::TextP,
            GremlinTypes::TraversalStrategy(_) => CoreType::TraversalStrategy,
            GremlinTypes::Tree(_) => CoreType::Tree,
            GremlinTypes::Metrics(_) => CoreType::Metrics,
            GremlinTypes::TraversalMetrics(_) => CoreType::TraversalMetrics,
            GremlinTypes::BulkSet(_) => CoreType::BulkSet,
            GremlinTypes::UnspecifiedNullObject => CoreType::UnspecifiedNullObject,
            GremlinTypes::Merge(_) => CoreType::Merge,
            GremlinTypes::Char(_) => todo!(),
        }
    }

    pub fn to_option(self) -> Option<GremlinTypes> {
        match self {
            GremlinTypes::UnspecifiedNullObject => None,
            graph_binary => Some(graph_binary),
        }
    }
}

impl Display for GremlinTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GremlinTypes::Int(val) => write!(f, "{val}_i32"),
            GremlinTypes::Long(val) => write!(f, "{val}_i64"),
            GremlinTypes::String(val) => write!(f, "\"{val}\""),
            GremlinTypes::Class(val) => write!(f, "Class:{val}"),
            GremlinTypes::Double(val) => write!(f, "{val}_f64"),
            GremlinTypes::Float(val) => write!(f, "{val}_f32"),
            GremlinTypes::List(val) => {
                write!(f, "List::[")?;
                for i in val {
                    write!(f, " {i},")?;
                }
                write!(f, "]")
            }
            GremlinTypes::Set(val) => {
                write!(f, "Set::[")?;
                for i in val {
                    writeln!(f, " {i},")?;
                }
                write!(f, "]")
            }
            GremlinTypes::Map(val) => {
                write!(f, "Map::[")?;
                for (key, value) in val {
                    writeln!(f, "{{{key}:{value}}},")?;
                }
                write!(f, "]")
            }
            GremlinTypes::Uuid(val) => write!(f, "Uuid::{val}"),
            GremlinTypes::Edge(val) => write!(f, "Edge::{val}"),
            GremlinTypes::Path(val) => write!(f, "Path::{val}"),
            GremlinTypes::Property(val) => write!(f, "Property::{val}"),
            GremlinTypes::Graph(val) => write!(f, "Graph::{val}"),
            GremlinTypes::Vertex(val) => write!(f, "Vertex::{val}"),
            GremlinTypes::VertexProperty(val) => write!(f, "VertexProperty::{val}"),
            GremlinTypes::Barrier(val) => write!(f, "Barrier::{val}"),
            GremlinTypes::Binding(val) => write!(f, "Binding::{val}"),
            GremlinTypes::Bytecode(val) => write!(f, "Bytecode::{val}"),
            GremlinTypes::Cardinality(val) => write!(f, "Cardinality::{val}"),
            GremlinTypes::Column(val) => write!(f, "Column::{val}"),
            GremlinTypes::Direction(val) => write!(f, "Direction::{val}"),
            GremlinTypes::Operator(val) => write!(f, "Operator::{val}"),
            GremlinTypes::Order(val) => write!(f, "Order::{val}"),
            GremlinTypes::Pick(val) => write!(f, "Pick::{val}"),
            GremlinTypes::Pop(val) => write!(f, "Pop::{val}"),
            GremlinTypes::Lambda(val) => write!(f, "Lambda::{val}"),
            GremlinTypes::P(val) => write!(f, "P::{val}"),
            GremlinTypes::Scope(val) => write!(f, "Scope::{val}"),
            GremlinTypes::T(val) => write!(f, "T::{val}"),
            GremlinTypes::Traverser(val) => write!(f, "Traverser::{val}"),
            GremlinTypes::Byte(val) => write!(f, "{val}_u8"),
            GremlinTypes::ByteBuffer(val) => todo!(),
            GremlinTypes::Short(val) => write!(f, "{val}_i16"),
            GremlinTypes::Boolean(val) => write!(f, "{val}"),
            GremlinTypes::TextP(val) => write!(f, "TextP::{val}"),
            GremlinTypes::TraversalStrategy(val) => write!(f, "TraversalStrategy::{val}"),
            GremlinTypes::BulkSet(val) => todo!(),
            GremlinTypes::Tree(val) => todo!(),
            GremlinTypes::Metrics(val) => write!(f, "{val}"),
            GremlinTypes::TraversalMetrics(val) => write!(f, "{val}"),
            GremlinTypes::Merge(val) => write!(f, "Merge::{val}"),
            GremlinTypes::UnspecifiedNullObject => write!(f, "UnspecifiedNullObject"),
            GremlinTypes::Char(val) => write!(f, "\'{val}\'"),
        }
    }
}

impl Default for GremlinTypes {
    fn default() -> Self {
        GremlinTypes::UnspecifiedNullObject
    }
}

impl Decode for GremlinTypes {
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
            CoreType::List => Vec::<GremlinTypes>::get_len(bytes),
            CoreType::Set => Vec::<GremlinTypes>::get_len(bytes),
            CoreType::Map => HashMap::<MapKeys, GremlinTypes>::get_len(bytes),
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
            CoreType::Char => char::get_len(bytes),
        }
    }
}

impl Encode for GremlinTypes {
    fn type_code() -> u8 {
        unimplemented!()
    }

    fn partial_encode<W: Write>(&self, _writer: &mut W) -> Result<(), EncodeError> {
        unimplemented!()
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

impl From<MapKeys> for GremlinTypes {
    fn from(keys: MapKeys) -> GremlinTypes {
        match keys {
            MapKeys::Int(val) => GremlinTypes::Int(val),
            MapKeys::String(val) => GremlinTypes::String(val),
            MapKeys::Long(val) => GremlinTypes::Long(val),
            MapKeys::Uuid(val) => GremlinTypes::Uuid(val),
            MapKeys::T(val) => GremlinTypes::T(val),
            MapKeys::Direction(val) => GremlinTypes::Direction(val),
        }
    }
}

impl<T: Into<GremlinTypes> + Clone> From<&T> for GremlinTypes {
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}

impl<T: Into<GremlinTypes>, const N: usize> From<[T; N]> for GremlinTypes {
    fn from(array: [T; N]) -> Self {
        GremlinTypes::List(array.into_iter().map(Into::into).collect())
    }
}

impl TryFrom<GremlinTypes> for MapKeys {
    type Error = DecodeError;

    fn try_from(value: GremlinTypes) -> Result<Self, Self::Error> {
        match value {
            GremlinTypes::Int(val) => Ok(MapKeys::Int(val)),
            GremlinTypes::Long(val) => Ok(MapKeys::Long(val)),
            GremlinTypes::String(val) => Ok(MapKeys::String(val)),
            GremlinTypes::Uuid(val) => Ok(MapKeys::Uuid(val)),
            GremlinTypes::T(val) => Ok(MapKeys::T(val)),
            GremlinTypes::Direction(val) => Ok(MapKeys::Direction(val)),
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
        let key = GremlinTypes::decode(reader)?;
        MapKeys::try_from(key)
    }

    fn get_partial_len(_bytes: &[u8]) -> Result<usize, DecodeError> {
        unimplemented!("use consume_bytes insted")
    }

    fn get_len(bytes: &[u8]) -> Result<usize, DecodeError> {
        GremlinTypes::get_len(bytes)
    }
}

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

impl DecodeGraphSON for MapKeys {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let g_key = GremlinTypes::decode_v3(j_val)?;
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

pub fn decode<R: Read>(reader: &mut R) -> Result<GremlinTypes, DecodeError> {
    let mut buf = [255_u8; 2];
    reader.read_exact(&mut buf)?;

    let identifier = CoreType::try_from(buf[0])?;
    let value_flag = ValueFlag::try_from(buf[1])?;

    match (identifier, value_flag) {
        (_, ValueFlag::Null) => Ok(GremlinTypes::UnspecifiedNullObject),
        (CoreType::Int32, _) => Ok(GremlinTypes::Int(i32::partial_decode(reader)?)),
        (CoreType::Long, _) => Ok(GremlinTypes::Long(i64::partial_decode(reader)?)),
        (CoreType::String, _) => Ok(GremlinTypes::String(String::partial_decode(reader)?)),
        (CoreType::Class, _) => Ok(GremlinTypes::Class(String::partial_decode(reader)?)),
        (CoreType::Double, _) => Ok(GremlinTypes::Double(f64::partial_decode(reader)?)),
        (CoreType::Float, _) => Ok(GremlinTypes::Float(f32::partial_decode(reader)?)),
        (CoreType::List, _) => Ok(GremlinTypes::List(Vec::partial_decode(reader)?)),
        (CoreType::Set, _) => Ok(GremlinTypes::Set(Vec::partial_decode(reader)?)),
        (CoreType::Map, _) => Ok(GremlinTypes::Map(
            HashMap::<MapKeys, GremlinTypes>::partial_decode(reader)?,
        )),
        (CoreType::Uuid, _) => Ok(GremlinTypes::Uuid(Uuid::partial_decode(reader)?)),
        (CoreType::Edge, _) => Ok(GremlinTypes::Edge(Edge::partial_decode(reader)?)),
        (CoreType::Path, _) => Ok(GremlinTypes::Path(Path::partial_decode(reader)?)),
        (CoreType::Property, _) => Ok(GremlinTypes::Property(Property::partial_decode(reader)?)),
        (CoreType::Graph, _) => Ok(GremlinTypes::Graph(Graph::partial_decode(reader)?)),
        (CoreType::Vertex, _) => Ok(GremlinTypes::Vertex(Vertex::partial_decode(reader)?)),
        (CoreType::VertexProperty, _) => Ok(GremlinTypes::VertexProperty(
            VertexProperty::partial_decode(reader)?,
        )),
        (CoreType::Short, _) => Ok(GremlinTypes::Short(i16::partial_decode(reader)?)),
        (CoreType::Boolean, _) => Ok(GremlinTypes::Boolean(bool::partial_decode(reader)?)),

        (CoreType::Cardinality, _) => Ok(GremlinTypes::Cardinality(Cardinality::partial_decode(
            reader,
        )?)),
        (CoreType::Column, _) => Ok(GremlinTypes::Column(Column::partial_decode(reader)?)),
        (CoreType::Direction, _) => Ok(GremlinTypes::Direction(Direction::partial_decode(reader)?)),
        (CoreType::Operator, _) => Ok(GremlinTypes::Operator(Operator::partial_decode(reader)?)),
        (CoreType::Order, _) => Ok(GremlinTypes::Order(Order::partial_decode(reader)?)),
        (CoreType::Pick, _) => Ok(GremlinTypes::Pick(Pick::partial_decode(reader)?)),
        (CoreType::Pop, _) => Ok(GremlinTypes::Pop(Pop::partial_decode(reader)?)),
        (CoreType::P, _) => Ok(GremlinTypes::P(P::partial_decode(reader)?)),
        (CoreType::Scope, _) => Ok(GremlinTypes::Scope(Scope::partial_decode(reader)?)),
        (CoreType::T, _) => Ok(GremlinTypes::T(T::partial_decode(reader)?)),
        (CoreType::Barrier, _) => Ok(GremlinTypes::Barrier(Barrier::partial_decode(reader)?)),
        (CoreType::Binding, _) => Ok(GremlinTypes::Binding(Binding::partial_decode(reader)?)),
        (CoreType::ByteCode, _) => Ok(GremlinTypes::Bytecode(Bytecode::partial_decode(reader)?)),
        (CoreType::Lambda, _) => Ok(GremlinTypes::Lambda(Lambda::partial_decode(reader)?)),
        (CoreType::Traverser, _) => Ok(GremlinTypes::Traverser(Traverser::partial_decode(reader)?)),
        (CoreType::Byte, _) => Ok(GremlinTypes::Byte(u8::partial_decode(reader)?)),
        (CoreType::ByteBuffer, _) => partial_decode_byte_buffer(reader),
        (CoreType::TextP, _) => Ok(GremlinTypes::TextP(TextP::partial_decode(reader)?)),
        (CoreType::TraversalStrategy, _) => Ok(GremlinTypes::TraversalStrategy(
            TraversalStrategy::partial_decode(reader)?,
        )),
        (CoreType::Tree, _) => todo!(),
        (CoreType::Metrics, _) => Ok(GremlinTypes::Metrics(Metrics::partial_decode(reader)?)),

        (CoreType::TraversalMetrics, _) => Ok(GremlinTypes::TraversalMetrics(
            TraversalMetrics::partial_decode(reader)?,
        )),
        (CoreType::Merge, _) => Ok(GremlinTypes::Merge(Merge::partial_decode(reader)?)),
        (CoreType::BulkSet, _) => Ok(GremlinTypes::BulkSet(BulkSet::partial_decode(reader)?)),
        (CoreType::UnspecifiedNullObject, _) => Err(DecodeError::DecodeError(
            "UnspecifiedNullObject wrong valueflag".to_string(),
        )),
        (CoreType::Char, _) => Ok(GremlinTypes::Char(char::partial_decode(reader)?)),
    }
}

fn partial_decode_byte_buffer<R: Read>(reader: &mut R) -> Result<GremlinTypes, DecodeError> {
    let len = i32::partial_decode(reader)?;
    let mut buf = vec![0; len as usize];
    reader.read_exact(&mut buf)?;
    Ok(GremlinTypes::ByteBuffer(buf))
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

    let gb = GremlinTypes::decode(&mut &buf[..]);
    assert_eq!(
        GremlinTypes::ByteBuffer(vec![100, 101, 102, 103, 104, 105]),
        gb.unwrap()
    );

    let buf_0 = [CoreType::ByteBuffer.into(), 0x0, 0x0, 0x0, 0x0, 0x0];

    let gb = GremlinTypes::decode(&mut &buf_0[..]);
    assert_eq!(GremlinTypes::ByteBuffer(vec![]), gb.unwrap())
}
