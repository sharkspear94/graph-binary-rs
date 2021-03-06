use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

use crate::error::{DecodeError, EncodeError};

use crate::structure::bulkset::BulkSet;
use crate::structure::bytebuffer::ByteBuffer;
use crate::structure::bytecode::Bytecode;
use crate::structure::enums::{
    Barrier, Cardinality, Column, Direction, Merge, Operator, Order, Pick, Pop, Scope, TextP, P, T,
};
use crate::structure::graph::Graph;
use crate::structure::lambda::Lambda;
use crate::structure::list::Set;
use crate::structure::map::MapKeys;
use crate::structure::metrics::{Metrics, TraversalMetrics};
use crate::structure::path::Path;
use crate::structure::property::Property;
use crate::structure::traverser::{TraversalStrategy, Traverser};
use crate::structure::vertex::Vertex;
use crate::structure::vertex_property::VertexProperty;
use crate::Binding;
use crate::GremlinValue;
use crate::{specs::CoreType, structure::edge::Edge};
use bigdecimal::BigDecimal;
use num::BigInt;
use uuid::Uuid;

#[cfg(feature = "extended")]
use crate::extended::chrono::{
    Instant, MonthDay, OffsetTime, Period, Year, YearMonth, ZonedDateTime,
};
#[cfg(feature = "extended")]
use chrono::{DateTime, Duration, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime};

pub fn from_file(path: &std::path::Path) -> Result<GremlinValue, DecodeError> {
    let file = File::open(path).unwrap();
    let mut buf = BufReader::new(file);
    GremlinValue::decode(&mut buf)
}

pub fn from_slice(slice: &mut &[u8]) -> Result<GremlinValue, DecodeError> {
    GremlinValue::decode(slice)
}

pub fn to_file(gremlin_value: GremlinValue, file: File) -> Result<(), EncodeError> {
    let mut writer = BufWriter::new(file);
    gremlin_value.encode(&mut writer)
}

pub fn encode_null_object<W: Write>(writer: &mut W) -> Result<(), EncodeError> {
    writer.write_all(&[
        CoreType::UnspecifiedNullObject.into(),
        ValueFlag::Null.into(),
    ])?;
    Ok(())
}

impl GremlinValue {
    pub fn build_fq_bytes<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        match self {
            GremlinValue::Int(val) => val.encode(writer),
            GremlinValue::Long(val) => val.encode(writer),
            GremlinValue::String(val) => val.encode(writer),
            GremlinValue::Date(val) => {
                writer.write_all(&[0x04, 0x0])?;
                val.partial_encode(writer)
            }
            GremlinValue::Timestamp(val) => {
                writer.write_all(&[0x05, 0x0])?;
                val.partial_encode(writer)
            }
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
            GremlinValue::BigDecimal(val) => val.encode(writer),
            GremlinValue::BigInteger(val) => val.encode(writer),
            GremlinValue::Byte(val) => val.encode(writer),
            GremlinValue::ByteBuffer(val) => val.encode(writer),
            GremlinValue::Short(val) => val.encode(writer),
            GremlinValue::Boolean(val) => val.encode(writer),
            GremlinValue::TextP(val) => val.encode(writer),
            GremlinValue::TraversalStrategy(val) => val.encode(writer),
            GremlinValue::BulkSet(val) => val.encode(writer),
            GremlinValue::Metrics(val) => val.encode(writer),
            GremlinValue::TraversalMetrics(val) => val.encode(writer),
            GremlinValue::Merge(val) => val.encode(writer),
            GremlinValue::UnspecifiedNullObject => encode_null_object(writer),
            #[cfg(feature = "extended")]
            GremlinValue::Char(val) => val.encode(writer),
            #[cfg(feature = "extended")]
            GremlinValue::Duration(val) => val.encode(writer),
            #[cfg(feature = "extended")]
            GremlinValue::InetAddress(val) => val.encode(writer),
            #[cfg(feature = "extended")]
            GremlinValue::Instant(val) => val.encode(writer),
            #[cfg(feature = "extended")]
            GremlinValue::LocalDate(val) => val.encode(writer),
            #[cfg(feature = "extended")]
            GremlinValue::LocalDateTime(val) => val.encode(writer),
            #[cfg(feature = "extended")]
            GremlinValue::LocalTime(val) => val.encode(writer),
            #[cfg(feature = "extended")]
            GremlinValue::MonthDay(val) => val.encode(writer),
            #[cfg(feature = "extended")]
            GremlinValue::OffsetDateTime(val) => val.encode(writer),
            #[cfg(feature = "extended")]
            GremlinValue::OffsetTime(val) => val.encode(writer),
            #[cfg(feature = "extended")]
            GremlinValue::Period(val) => val.encode(writer),
            #[cfg(feature = "extended")]
            GremlinValue::Year(val) => val.encode(writer),
            #[cfg(feature = "extended")]
            GremlinValue::YearMonth(val) => val.encode(writer),
            #[cfg(feature = "extended")]
            GremlinValue::ZonedDateTime(val) => val.encode(writer),
            #[cfg(feature = "extended")]
            GremlinValue::ZoneOffset(val) => val.encode(writer),
        }
    }
}

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
}

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

fn decode<R: Read>(reader: &mut R) -> Result<GremlinValue, DecodeError> {
    let mut buf = [255_u8; 2];
    reader.read_exact(&mut buf)?;

    let identifier = CoreType::try_from(buf[0])?;
    let value_flag = ValueFlag::try_from(buf[1])?;

    match (identifier, value_flag) {
        (_, ValueFlag::Null) => Ok(GremlinValue::UnspecifiedNullObject),
        (CoreType::Int32, _) => Ok(GremlinValue::Int(i32::partial_decode(reader)?)),
        (CoreType::Long, _) => Ok(GremlinValue::Long(i64::partial_decode(reader)?)),
        (CoreType::String, _) => Ok(GremlinValue::String(String::partial_decode(reader)?)),
        (CoreType::Date, _) => Ok(GremlinValue::Date(i64::partial_decode(reader)?)),
        (CoreType::Timestamp, _) => Ok(GremlinValue::Timestamp(i64::partial_decode(reader)?)),
        (CoreType::Class, _) => Ok(GremlinValue::Class(String::partial_decode(reader)?)),
        (CoreType::Double, _) => Ok(GremlinValue::Double(f64::partial_decode(reader)?)),
        (CoreType::Float, _) => Ok(GremlinValue::Float(f32::partial_decode(reader)?)),
        (CoreType::List, _) => Ok(GremlinValue::List(Vec::partial_decode(reader)?)),
        (CoreType::Set, _) => Ok(GremlinValue::Set(Set::partial_decode(reader)?)),
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
        (CoreType::BigDecimal, _) => Ok(GremlinValue::BigDecimal(BigDecimal::partial_decode(
            reader,
        )?)),
        (CoreType::BigInteger, _) => Ok(GremlinValue::BigInteger(BigInt::partial_decode(reader)?)),
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
        #[cfg(feature = "extended")]
        (CoreType::Duration, _) => Ok(GremlinValue::Duration(Duration::partial_decode(reader)?)),
        #[cfg(feature = "extended")]
        (CoreType::InetAddress, _) => Ok(GremlinValue::InetAddress(
            std::net::IpAddr::partial_decode(reader)?,
        )),
        #[cfg(feature = "extended")]
        (CoreType::Instant, _) => Ok(GremlinValue::Instant(Instant::partial_decode(reader)?)),
        #[cfg(feature = "extended")]
        (CoreType::LocalDate, _) => Ok(GremlinValue::LocalDate(NaiveDate::partial_decode(reader)?)),
        #[cfg(feature = "extended")]
        (CoreType::LocalDateTime, _) => Ok(GremlinValue::LocalDateTime(
            NaiveDateTime::partial_decode(reader)?,
        )),
        #[cfg(feature = "extended")]
        (CoreType::LocalTime, _) => Ok(GremlinValue::LocalTime(NaiveTime::partial_decode(reader)?)),
        #[cfg(feature = "extended")]
        (CoreType::MonthDay, _) => Ok(GremlinValue::MonthDay(MonthDay::partial_decode(reader)?)),
        #[cfg(feature = "extended")]
        (CoreType::OffsetDateTime, _) => Ok(GremlinValue::OffsetDateTime(
            DateTime::partial_decode(reader)?,
        )),
        #[cfg(feature = "extended")]
        (CoreType::OffsetTime, _) => Ok(GremlinValue::OffsetTime(OffsetTime::partial_decode(
            reader,
        )?)),
        #[cfg(feature = "extended")]
        (CoreType::Period, _) => Ok(GremlinValue::Period(Period::partial_decode(reader)?)),
        #[cfg(feature = "extended")]
        (CoreType::Year, _) => Ok(GremlinValue::Year(Year::partial_decode(reader)?)),
        #[cfg(feature = "extended")]
        (CoreType::YearMonth, _) => Ok(GremlinValue::YearMonth(YearMonth::partial_decode(reader)?)),
        #[cfg(feature = "extended")]
        (CoreType::ZonedDateTime, _) => Ok(GremlinValue::ZonedDateTime(
            ZonedDateTime::partial_decode(reader)?,
        )),
        #[cfg(feature = "extended")]
        (CoreType::ZoneOffset, _) => Ok(GremlinValue::ZoneOffset(FixedOffset::partial_decode(
            reader,
        )?)),
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
