use std::collections::{BTreeMap, BTreeSet};
use std::io::{Read, Write};

use crate::structure::binding::Binding;
use crate::structure::enums::{
    Barrier, Cardinality, Column, Direction, Operator, Order, Pick, Pop, Scope, TextP, P, T,
};
use crate::structure::graph::Graph;
use crate::structure::lambda::Lambda;
use crate::structure::list::List1;
use crate::structure::metrics::{Metrics, TraversalMetrics};
use crate::structure::path::Path;
use crate::structure::property::Property;
use crate::structure::traverser::{TraversalStrategy, Traverser};
use crate::structure::vertex::Vertex;
use crate::structure::vertex_property::VertexProperty;
use crate::{specs::CoreType, structure::edge::Edge};
use uuid::Uuid;

use super::structure::list::List;
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
    Int(Option<i32>),
    Long(i64),
    String(String),
    // Date(Date),
    // Timestamp(Date),
    Class(String),
    Double(f64),
    Float(f32),
    List(List),
    Set(List),
    Map(Map),
    Uuid(Uuid),
    Edge(Edge),
    Path(Path),
    Property(Property),
    Graph(Graph),
    Vertex(Vertex),
    VertexProperty(VertexProperty),
    Barrier(Barrier),
    Binding(Binding),
    ByteCode(Vec<u8>),
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
            GraphBinary::Int(val) => val.fq_gb_bytes(writer),
            GraphBinary::Long(val) => val.fq_gb_bytes(writer),
            GraphBinary::String(s) => s.fq_gb_bytes(writer),
            // CoreType::Date(_) => todo!(),
            // CoreType::Timestamp(_) => todo!(),
            GraphBinary::Class(class) => class.fq_gb_bytes(writer),
            GraphBinary::Double(val) => val.fq_gb_bytes(writer),
            GraphBinary::Float(val) => val.fq_gb_bytes(writer),
            GraphBinary::List(list) => list.fq_gb_bytes(writer),
            GraphBinary::Set(set) => set.fq_gb_bytes(writer),
            GraphBinary::Map(map) => map.fq_gb_bytes(writer),
            GraphBinary::Uuid(uuid) => uuid.fq_gb_bytes(writer),
            GraphBinary::Edge(edge) => edge.fq_gb_bytes(writer),
            GraphBinary::Path(path) => path.fq_gb_bytes(writer),
            GraphBinary::Property(prop) => prop.fq_gb_bytes(writer),
            GraphBinary::Graph(graph) => graph.fq_gb_bytes(writer),
            GraphBinary::Vertex(vertex) => vertex.fq_gb_bytes(writer),
            GraphBinary::VertexProperty(vertex_prop) => vertex_prop.fq_gb_bytes(writer),
            GraphBinary::Barrier(_) => todo!(),
            GraphBinary::Binding(binding) => todo!(),
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
            // GraphBinary::BigDecimal(_) => todo!(),
            // GraphBinary::BigInteger(_) => todo!(),
            GraphBinary::Byte(_) => todo!(),
            GraphBinary::ByteBuffer(buf) => todo!(),
            GraphBinary::Short(_) => todo!(),
            GraphBinary::Boolean(_) => todo!(),
            GraphBinary::TextP(_) => todo!(),
            GraphBinary::TraversalStrategy(_) => todo!(),
            // GraphBinary::BulkSet(_) => todo!(),
            GraphBinary::Tree(_) => todo!(),
            GraphBinary::Metrics(_) => todo!(),
            GraphBinary::TraversalMetrics(_) => todo!(),
            GraphBinary::UnspecifiedNullObject => build_fq_null_bytes(writer),
            // GraphBinary::Custom => todo!(),
            // _ =>  Bytes::new()
        }
    }
    fn build_fq_null<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        match self {
            GraphBinary::Int(_) => todo!(),
            GraphBinary::Long(_) => todo!(),
            GraphBinary::String(_) => todo!(),
            GraphBinary::Class(_) => todo!(),
            GraphBinary::Double(_) => todo!(),
            GraphBinary::Float(_) => todo!(),
            GraphBinary::List(_) => todo!(),
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
            GraphBinary::ByteCode(_) => todo!(),
            GraphBinary::ByteBuffer(buf) => todo!(),
            GraphBinary::UnspecifiedNullObject => todo!(),
            GraphBinary::Cardinality(_) => todo!(),
            GraphBinary::Column(_) => todo!(),
            GraphBinary::Direction(_) => todo!(),
            GraphBinary::Operator(_) => todo!(),
            GraphBinary::Order(_) => todo!(),
            GraphBinary::Pick(_) => todo!(),
            GraphBinary::Pop(_) => todo!(),
            GraphBinary::P(_) => todo!(),
            GraphBinary::Scope(_) => todo!(),
            GraphBinary::T(_) => todo!(),
            GraphBinary::TextP(_) => todo!(),
            GraphBinary::Binding(_) => todo!(),
            GraphBinary::Lambda(_) => todo!(),
            GraphBinary::Traverser(_) => todo!(),
            GraphBinary::Byte(_) => todo!(),
            GraphBinary::Short(_) => todo!(),
            GraphBinary::Boolean(_) => todo!(),
            GraphBinary::TraversalStrategy(_) => todo!(),
            GraphBinary::Tree(_) => todo!(),
            GraphBinary::Metrics(_) => todo!(),
            GraphBinary::TraversalMetrics(_) => todo!(),
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
            GraphBinary::Column(_) => CoreType::Cloumn,
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
        }
    }

    fn decode<R: Read>(reader: R) {}
}
// pub enum Number{
//     Int32(i32),
//     Int64(i64),
//     Byte(u8),
//     Short(i16),
//     BigDecimal(BigDecimal),
//     BigInteger(BigInteger),
//     Uuid(i128),
// }

// impl Encode for GraphBinary {
//     fn type_code() -> u8 {
//         todo!()
//         // match self {
//         //     GraphBinary::Int(_) => todo!(),
//         //     GraphBinary::Long(_) => todo!(),
//         //     GraphBinary::String(_) => todo!(),
//         //     GraphBinary::Class(_) => todo!(),
//         //     GraphBinary::Double(_) => todo!(),
//         //     GraphBinary::Float(_) => todo!(),
//         //     GraphBinary::List(_) => todo!(),
//         //     GraphBinary::Set(_) => todo!(),
//         //     GraphBinary::Map(_) => todo!(),
//         //     GraphBinary::Uuid(_) => todo!(),
//         //     GraphBinary::Edge(_) => todo!(),
//         //     GraphBinary::Path(_) => todo!(),
//         //     GraphBinary::Property(_) => todo!(),
//         //     GraphBinary::Graph(_) => todo!(),
//         //     GraphBinary::Vertex(_) => todo!(),
//         //     GraphBinary::VertexProperty(_) => todo!(),
//         //     GraphBinary::Barrier(_) => todo!(),
//         //     GraphBinary::ByteCode(_) => todo!(),
//         //     GraphBinary::UnspecifiedNullObject => todo!(),
//         // }
//     }

//     fn gb_bytes<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
//         match self {
//             GraphBinary::Int(_) => todo!(),
//             GraphBinary::Long(_) => todo!(),
//             GraphBinary::String(_) => todo!(),
//             GraphBinary::Class(_) => todo!(),
//             GraphBinary::Double(_) => todo!(),
//             GraphBinary::Float(_) => todo!(),
//             GraphBinary::List(_) => todo!(),
//             GraphBinary::Set(_) => todo!(),
//             GraphBinary::Map(_) => todo!(),
//             GraphBinary::Uuid(_) => todo!(),
//             GraphBinary::Edge(_) => todo!(),
//             GraphBinary::Path(_) => todo!(),
//             GraphBinary::Property(_) => todo!(),
//             GraphBinary::Graph(_) => todo!(),
//             GraphBinary::Vertex(_) => todo!(),
//             GraphBinary::VertexProperty(_) => todo!(),
//             GraphBinary::Barrier(_) => todo!(),
//             GraphBinary::ByteCode(_) => todo!(),
//             GraphBinary::UnspecifiedNullObject => todo!(),
//         }
//     }
// }

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
            MapKeys::Int(val) => GraphBinary::Int(Some(val)),
            MapKeys::String(val) => GraphBinary::String(val),
            MapKeys::Long(val) => GraphBinary::Long(val),
            MapKeys::Uuid(val) => GraphBinary::Uuid(val),
        }
    }
}

impl From<&MapKeys> for GraphBinary {
    fn from(keys: &MapKeys) -> GraphBinary {
        match keys {
            MapKeys::Int(val) => GraphBinary::Int(Some(*val)),
            MapKeys::String(val) => GraphBinary::String(val.clone()),
            MapKeys::Long(val) => GraphBinary::Long(*val),
            MapKeys::Uuid(val) => GraphBinary::Uuid(*val),
        }
    }
}

pub fn decode<R: Read>(reader: &mut R) -> Result<GraphBinary, DecodeError> {
    let mut buf = [255_u8; 2];
    reader.read_exact(&mut buf)?;

    let identifier = CoreType::try_from(buf[0])?;
    let value_flag = ValueFlag::try_from(buf[1])?;

    let gb = match (identifier, value_flag) {
        (CoreType::Int32, ValueFlag::Set) => GraphBinary::Int(Some(i32::decode(reader)?)),
        (CoreType::Int32, ValueFlag::Null) => GraphBinary::Int(None),
        (CoreType::Long, ValueFlag::Set) => GraphBinary::Long(i64::decode(reader)?),
        (CoreType::Long, ValueFlag::Null) => todo!(),
        (CoreType::String, ValueFlag::Set) => GraphBinary::String(String::decode(reader)?),
        (CoreType::String, ValueFlag::Null) => todo!(),
        (CoreType::Class, ValueFlag::Set) => GraphBinary::Class(String::decode(reader)?),
        (CoreType::Class, ValueFlag::Null) => todo!(),
        (CoreType::Double, ValueFlag::Set) => GraphBinary::Double(f64::decode(reader)?),
        (CoreType::Double, ValueFlag::Null) => todo!(),
        (CoreType::Float, ValueFlag::Set) => GraphBinary::Float(f32::decode(reader)?),
        (CoreType::Float, ValueFlag::Null) => todo!(),
        (CoreType::List, ValueFlag::Set) => GraphBinary::List(List::decode(reader)?),
        (CoreType::List, ValueFlag::Null) => todo!(),
        (CoreType::Set, ValueFlag::Set) => GraphBinary::Set(List::decode(reader)?),
        (CoreType::Set, ValueFlag::Null) => todo!(),
        (CoreType::Map, ValueFlag::Set) => todo!(),
        (CoreType::Map, ValueFlag::Null) => todo!(),
        (CoreType::Uuid, ValueFlag::Set) => todo!(),
        (CoreType::Uuid, ValueFlag::Null) => todo!(),
        (CoreType::Edge, ValueFlag::Set) => todo!(),
        (CoreType::Edge, ValueFlag::Null) => todo!(),
        (CoreType::Path, ValueFlag::Set) => todo!(),
        (CoreType::Path, ValueFlag::Null) => todo!(),
        (CoreType::Property, ValueFlag::Set) => todo!(),
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
        (CoreType::Cloumn, ValueFlag::Set) => todo!(),
        (CoreType::Cloumn, ValueFlag::Null) => todo!(),
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
        (CoreType::UnspecifiedNullObject, ValueFlag::Null) => todo!(),
        // (CoreType::Int32,0x00) => GraphBinary::Int(i32::decode(reader)?),
        // (0x02,0x00) => GraphBinary::Long(i64::decode(reader)?),
        // (LIST_TYPE_CODE,0x0) => GraphBinary::List(List::decode(reader)?),
        // (_,_) => return Err(DecodeError::DecodeError("qualifier not known".to_string())),
    };

    Ok(gb)
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
            _ => Err(DecodeError::ConvertError("value_flag")),
        }
    }
}

pub trait Decode {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized;
}

use crate::error::{DecodeError, EncodeError};

pub trait Encode {
    fn type_code() -> u8;

    fn gb_bytes<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError>;

    fn fq_null<W: Write>(writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&[Self::type_code(), VALUE_NULL])?;
        Ok(())
    }

    fn fq_gb_bytes<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&[Self::type_code(), VALUE_PRESENT])?;
        self.gb_bytes(writer)
    }

    fn nullable_gb_bytes<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_all(&[VALUE_PRESENT])?;
        self.gb_bytes(writer)
    }
}

#[test]
fn testing() {
    let mut buf: Vec<u8> = vec![];
    15_i32.gb_bytes(&mut buf);
    assert_eq!([0x00, 0x00, 0x00, 0x0F], buf.as_slice());

    buf.clear();
    15_i32.fq_gb_bytes(&mut buf);
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