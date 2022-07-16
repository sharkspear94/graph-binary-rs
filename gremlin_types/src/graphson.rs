use std::{
    collections::HashMap,
    io::{Read, Write}, time::{SystemTime, UNIX_EPOCH, Duration},
};

use std::time::Instant;

use serde_json::Map;
use uuid::Uuid;

use crate::{
    error::{DecodeError, EncodeError},
    graph_binary::{GremlinValue, MapKeys},
    structure::{
        bulkset::BulkSet,
        bytecode::Bytecode,
        edge::Edge,
        enums::{
            Barrier, Cardinality, Column, Direction, Merge, Operator, Order, Pick, Pop, Scope,
            TextP, P, T,
        },
        graph::Graph,
        lambda::Lambda,
        metrics::{Metrics, TraversalMetrics},
        path::Path,
        property::Property,
        traverser::Traverser,
        vertex::Vertex,
        vertex_property::VertexProperty,
    },
    Binding,
};

pub trait EncodeGraphSON {
    fn encode_v3(&self) -> serde_json::Value;

    fn encode_v2(&self) -> serde_json::Value;

    fn encode_v1(&self) -> serde_json::Value;
}

pub trait DecodeGraphSON {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized;

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized;

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized;
}

impl EncodeGraphSON for GremlinValue {
    fn encode_v3(&self) -> serde_json::Value {
        match self {
            GremlinValue::Int(val) => val.encode_v3(),
            GremlinValue::Long(val) => val.encode_v3(),
            GremlinValue::String(val) => val.encode_v3(),
            GremlinValue::Class(val) => val.encode_v3(),
            GremlinValue::Double(val) => val.encode_v3(),
            GremlinValue::Float(val) => val.encode_v3(),
            GremlinValue::List(val) => val.encode_v3(),
            GremlinValue::Set(val) => val.encode_v3(), // FIXME
            GremlinValue::Map(val) => val.encode_v3(),
            GremlinValue::Uuid(val) => val.encode_v3(),
            GremlinValue::Edge(val) => val.encode_v3(),
            GremlinValue::Path(val) => val.encode_v3(),
            GremlinValue::Property(val) => val.encode_v3(),
            GremlinValue::Graph(val) => val.encode_v3(),
            GremlinValue::Vertex(val) => val.encode_v3(),
            GremlinValue::VertexProperty(val) => val.encode_v3(),
            GremlinValue::Barrier(val) => val.encode_v3(),
            GremlinValue::Binding(val) => val.encode_v3(),
            GremlinValue::Bytecode(val) => val.encode_v3(),
            GremlinValue::Cardinality(val) => val.encode_v3(),
            GremlinValue::Column(val) => val.encode_v3(),
            GremlinValue::Direction(val) => val.encode_v3(),
            GremlinValue::Operator(val) => val.encode_v3(),
            GremlinValue::Order(val) => val.encode_v3(),
            GremlinValue::Pick(val) => val.encode_v3(),
            GremlinValue::Pop(val) => val.encode_v3(),
            GremlinValue::Lambda(val) => val.encode_v3(),
            GremlinValue::P(val) => val.encode_v3(),
            GremlinValue::Scope(val) => val.encode_v3(),
            GremlinValue::T(val) => val.encode_v3(),
            GremlinValue::Traverser(val) => val.encode_v3(),
            GremlinValue::Byte(val) => val.encode_v3(),
            GremlinValue::ByteBuffer(val) => val.encode_v3(),
            GremlinValue::Short(val) => val.encode_v3(),
            GremlinValue::Boolean(val) => val.encode_v3(),
            GremlinValue::TextP(val) => val.encode_v3(),
            // GremlinTypes::TraversalStrategy(val) => val.encode_v3(),
            // GremlinTypes::BulkSet(val) => val.encode_v3(),
            // GremlinTypes::Tree(val) => val.encode_v3(),
            GremlinValue::Metrics(val) => val.encode_v3(),
            GremlinValue::TraversalMetrics(val) => val.encode_v3(),
            GremlinValue::Merge(val) => val.encode_v3(),
            GremlinValue::UnspecifiedNullObject => serde_json::Value::Null,
            #[cfg(feature = "extended")]
            GremlinValue::Char(val) => val.encode_v3(),
            _ => unimplemented!(),
        }
    }

    fn encode_v2(&self) -> serde_json::Value {
        match self {
            GremlinValue::Int(val) => val.encode_v2(),
            GremlinValue::Long(val) => val.encode_v2(),
            GremlinValue::String(val) => val.encode_v2(),
            GremlinValue::Class(val) => val.encode_v2(),
            GremlinValue::Double(val) => val.encode_v2(),
            GremlinValue::Float(val) => val.encode_v2(),
            GremlinValue::List(val) => val.encode_v2(),
            GremlinValue::Set(val) => val.encode_v2(), // FIXME
            GremlinValue::Map(val) => val.encode_v2(),
            GremlinValue::Uuid(val) => val.encode_v2(),
            GremlinValue::Edge(val) => val.encode_v2(),
            GremlinValue::Path(val) => val.encode_v2(),
            GremlinValue::Property(val) => val.encode_v2(),
            GremlinValue::Graph(val) => val.encode_v2(),
            GremlinValue::Vertex(val) => val.encode_v2(),
            GremlinValue::VertexProperty(val) => val.encode_v2(),
            GremlinValue::Barrier(val) => val.encode_v2(),
            GremlinValue::Binding(val) => val.encode_v2(),
            GremlinValue::Bytecode(val) => val.encode_v2(),
            GremlinValue::Cardinality(val) => val.encode_v2(),
            GremlinValue::Column(val) => val.encode_v2(),
            GremlinValue::Direction(val) => val.encode_v2(),
            GremlinValue::Operator(val) => val.encode_v2(),
            GremlinValue::Order(val) => val.encode_v2(),
            GremlinValue::Pick(val) => val.encode_v2(),
            GremlinValue::Pop(val) => val.encode_v2(),
            GremlinValue::Lambda(val) => val.encode_v2(),
            GremlinValue::P(val) => val.encode_v2(),
            GremlinValue::Scope(val) => val.encode_v2(),
            GremlinValue::T(val) => val.encode_v2(),
            GremlinValue::Traverser(val) => val.encode_v2(),
            GremlinValue::Byte(val) => val.encode_v2(),
            GremlinValue::ByteBuffer(val) => val.encode_v2(),
            GremlinValue::Short(val) => val.encode_v2(),
            GremlinValue::Boolean(val) => val.encode_v2(),
            GremlinValue::TextP(val) => val.encode_v2(),
            // GremlinTypes::TraversalStrategy(val) => val.encode_v2(),
            // GremlinTypes::BulkSet(val) => val.encode_v2(),
            // GremlinTypes::Tree(val) => val.encode_v2(),
            GremlinValue::Metrics(val) => val.encode_v2(),
            GremlinValue::TraversalMetrics(val) => val.encode_v2(),
            GremlinValue::Merge(val) => val.encode_v2(),
            GremlinValue::UnspecifiedNullObject => serde_json::Value::Null,
            #[cfg(feature = "extended")]
            GremlinValue::Char(val) => val.encode_v2(),
            _ => unimplemented!(),
        }
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

impl DecodeGraphSON for GremlinValue {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        match j_val {
            serde_json::Value::Null => Ok(GremlinValue::UnspecifiedNullObject),
            serde_json::Value::Bool(b) => Ok(GremlinValue::Boolean(*b)),
            serde_json::Value::String(s) => Ok(GremlinValue::String(s.clone())),
            serde_json::Value::Object(o) => {
                match o
                    .get("@type")
                    .and_then(|s| s.as_str())
                    .ok_or_else(|| DecodeError::DecodeError("".to_string()))?
                {
                    "g:Int32" => Ok(GremlinValue::Int(i32::decode_v3(j_val)?)),
                    "g:Int64" => Ok(GremlinValue::Long(i64::decode_v3(j_val)?)),
                    "g:Class" => Ok(GremlinValue::Class(
                        o.get("@value")
                            .and_then(|c| c.as_str())
                            .ok_or_else(|| {
                                crate::error::DecodeError::DecodeError(
                                    "json error Class v3 in error".to_string(),
                                )
                            })
                            .map(|class| class.to_string())?,
                    )),
                    "g:Double" => Ok(GremlinValue::Double(f64::decode_v3(j_val)?)),
                    "g:Float" => Ok(GremlinValue::Float(f32::decode_v3(j_val)?)),
                    "g:List" => Ok(GremlinValue::List(Vec::<GremlinValue>::decode_v3(j_val)?)),
                    "g:Map" => Ok(GremlinValue::Map(
                        HashMap::<MapKeys, GremlinValue>::decode_v3(j_val)?,
                    )),
                    "g:UUID" => Ok(GremlinValue::Uuid(Uuid::decode_v3(j_val)?)),
                    "g:Edge" => Ok(GremlinValue::Edge(Edge::decode_v3(j_val)?)),
                    "g:Path" => Ok(GremlinValue::Path(Path::decode_v3(j_val)?)),
                    "g:Property" => Ok(GremlinValue::Property(Property::decode_v3(j_val)?)),
                    "g:tinker:graph" => Ok(GremlinValue::Graph(Graph::decode_v3(j_val)?)),
                    "g:Vertex" => Ok(GremlinValue::Vertex(Vertex::decode_v3(j_val)?)),
                    "g:VertexProperty" => Ok(GremlinValue::VertexProperty(
                        VertexProperty::decode_v3(j_val)?,
                    )),
                    "g:Barrier" => Ok(GremlinValue::Barrier(Barrier::decode_v3(j_val)?)),
                    "g:Binding" => Ok(GremlinValue::Binding(Binding::decode_v3(j_val)?)),
                    "g:Bytecode" => Ok(GremlinValue::Bytecode(Bytecode::decode_v3(j_val)?)),
                    "g:Cardinality" => {
                        Ok(GremlinValue::Cardinality(Cardinality::decode_v3(j_val)?))
                    }
                    "g:Column" => Ok(GremlinValue::Column(Column::decode_v3(j_val)?)),
                    "g:Direction" => Ok(GremlinValue::Direction(Direction::decode_v3(j_val)?)),
                    "g:Lambda" => Ok(GremlinValue::Lambda(Lambda::decode_v3(j_val)?)),
                    "g:Merge" => Ok(GremlinValue::Merge(Merge::decode_v3(j_val)?)),
                    "g:Metrics" => Ok(GremlinValue::Metrics(Metrics::decode_v3(j_val)?)),
                    "g:Operator" => Ok(GremlinValue::Operator(Operator::decode_v3(j_val)?)),
                    "g:Order" => Ok(GremlinValue::Order(Order::decode_v3(j_val)?)),
                    "g:P" => Ok(GremlinValue::P(P::decode_v3(j_val)?)),
                    "g:Pick" => Ok(GremlinValue::Pick(Pick::decode_v3(j_val)?)),
                    "g:Pop" => Ok(GremlinValue::Pop(Pop::decode_v3(j_val)?)),
                    "g:Scope" => Ok(GremlinValue::Scope(Scope::decode_v3(j_val)?)),
                    "g:T" => Ok(GremlinValue::T(T::decode_v3(j_val)?)),
                    "g:TextP" => Ok(GremlinValue::TextP(TextP::decode_v3(j_val)?)),
                    "g:TraversalMetrics" => Ok(GremlinValue::TraversalMetrics(
                        TraversalMetrics::decode_v3(j_val)?,
                    )),
                    "g:Traverser" => Ok(GremlinValue::Traverser(Traverser::decode_v3(j_val)?)),
                    // "gx:BigDecimal" => Ok(GremlinTypes::BigDecimal(BigDecimal::decode_v3(j_val)?)),
                    // "gx:BigInteger" => Ok(GremlinTypes::BigInteger(BigInteger::decode_v3(j_val)?)),
                    "gx:Byte" => Ok(GremlinValue::Byte(u8::decode_v3(j_val)?)),
                    // "gx:ByteBuffer" => Ok(GremlinTypes::ByteBuffer(ByteBuffer::decode_v3(j_val)?)),
                    #[cfg(feature = "extended")]
                    "gx:Char" => Ok(GremlinValue::Char(char::decode_v3(j_val)?)),
                    // "gx:Duration" => Ok(GremlinTypes::Duration(Duration::decode_v3(j_val)?)),
                    // "gx:InetAddress" => Ok(GremlinTypes::InetAddress(InetAddress::decode_v3(j_val)?)),
                    // "gx:Instant" => Ok(GremlinTypes::Instant(Instant::decode_v3(j_val)?)),
                    // "gx:LocalDate" => Ok(GremlinTypes::LocalDate(LocalDate::decode_v3(j_val)?)),
                    // "gx:LocalDateTime" => Ok(GremlinTypes::LocalDateTime(LocalDateTime::decode_v3(j_val)?)),
                    // "gx:LocalTime" => Ok(GremlinTypes::LocalTime(LocalTime::decode_v3(j_val)?)),
                    // "gx:MonthDay" => Ok(GremlinTypes::MonthDay(MonthDay::decode_v3(j_val)?)),
                    // "gx:OffsetDateTime" => Ok(GremlinTypes::OffsetDateTime(OffsetDateTime::decode_v3(j_val)?)),
                    // "gx:OffsetTime" => Ok(GremlinTypes::OffsetTime(OffsetTime::decode_v3(j_val)?)),
                    // "gx:Period" => Ok(GremlinTypes::Period(Period::decode_v3(j_val)?)),
                    "gx:Int16" => Ok(GremlinValue::Short(i16::decode_v3(j_val)?)),
                    // "gx:Year" => Ok(GremlinTypes::Year(Year::decode_v3(j_val)?)),
                    // "gx:YearMonth" => Ok(GremlinTypes::YearMonth(YearMonth::decode_v3(j_val)?)),
                    // "gx:ZonedDateTime" => Ok(GremlinTypes::ZonedDateTime(ZonedDateTime::decode_v3(j_val)?)),
                    // "gx:ZoneOffset" => Ok(GremlinTypes::ZoneOffset(ZoneOffset::decode_v3(j_val)?)),
                    _ => todo!(),
                }
            }
            _ => todo!(),
        }
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        match j_val {
            serde_json::Value::Null => Ok(GremlinValue::UnspecifiedNullObject),
            serde_json::Value::Bool(b) => Ok(GremlinValue::Boolean(*b)),
            serde_json::Value::String(s) => Ok(GremlinValue::String(s.clone())),
            serde_json::Value::Array(arr) => {
                let mut vec = Vec::with_capacity(arr.len());
                for item in arr {
                    vec.push(GremlinValue::decode_v2(item)?);
                }
                Ok(GremlinValue::List(vec))
            }
            serde_json::Value::Object(o) => {
                if let Some(type_identifier) = o.get("@type").and_then(|s| s.as_str()) {
                    match type_identifier {
                        "g:Int32" => Ok(GremlinValue::Int(i32::decode_v2(j_val)?)),
                        "g:Int64" => Ok(GremlinValue::Long(i64::decode_v2(j_val)?)),
                        "g:Class" => Ok(GremlinValue::Class(
                            o.get("@value")
                                .and_then(|c| c.as_str())
                                .ok_or_else(|| {
                                    crate::error::DecodeError::DecodeError(
                                        "json error Class v3 in error".to_string(),
                                    )
                                })
                                .map(|class| class.to_string())?,
                        )),
                        "g:Double" => Ok(GremlinValue::Double(f64::decode_v2(j_val)?)),
                        "g:Float" => Ok(GremlinValue::Float(f32::decode_v2(j_val)?)),
                        "g:List" => Ok(GremlinValue::List(Vec::<GremlinValue>::decode_v2(j_val)?)),
                        "g:Map" => Ok(GremlinValue::Map(
                            HashMap::<MapKeys, GremlinValue>::decode_v2(j_val)?,
                        )),
                        "g:UUID" => Ok(GremlinValue::Uuid(Uuid::decode_v2(j_val)?)),
                        "g:Edge" => Ok(GremlinValue::Edge(Edge::decode_v2(j_val)?)),
                        "g:Path" => Ok(GremlinValue::Path(Path::decode_v2(j_val)?)),
                        "g:Property" => Ok(GremlinValue::Property(Property::decode_v2(j_val)?)),
                        "g:tinker:graph" => Ok(GremlinValue::Graph(Graph::decode_v2(j_val)?)),
                        "g:Vertex" => Ok(GremlinValue::Vertex(Vertex::decode_v2(j_val)?)),
                        "g:VertexProperty" => Ok(GremlinValue::VertexProperty(
                            VertexProperty::decode_v2(j_val)?,
                        )),
                        "g:Barrier" => Ok(GremlinValue::Barrier(Barrier::decode_v2(j_val)?)),
                        "g:Binding" => Ok(GremlinValue::Binding(Binding::decode_v2(j_val)?)),
                        "g:Bytecode" => Ok(GremlinValue::Bytecode(Bytecode::decode_v2(j_val)?)),
                        "g:Cardinality" => {
                            Ok(GremlinValue::Cardinality(Cardinality::decode_v2(j_val)?))
                        }
                        "g:Column" => Ok(GremlinValue::Column(Column::decode_v2(j_val)?)),
                        "g:Direction" => Ok(GremlinValue::Direction(Direction::decode_v2(j_val)?)),
                        "g:Lambda" => Ok(GremlinValue::Lambda(Lambda::decode_v2(j_val)?)),
                        "g:Merge" => Ok(GremlinValue::Merge(Merge::decode_v2(j_val)?)),
                        "g:Metrics" => Ok(GremlinValue::Metrics(Metrics::decode_v2(j_val)?)),
                        "g:Operator" => Ok(GremlinValue::Operator(Operator::decode_v2(j_val)?)),
                        "g:Order" => Ok(GremlinValue::Order(Order::decode_v2(j_val)?)),
                        "g:P" => Ok(GremlinValue::P(P::decode_v2(j_val)?)),
                        "g:Pick" => Ok(GremlinValue::Pick(Pick::decode_v2(j_val)?)),
                        "g:Pop" => Ok(GremlinValue::Pop(Pop::decode_v2(j_val)?)),
                        "g:Scope" => Ok(GremlinValue::Scope(Scope::decode_v2(j_val)?)),
                        "g:T" => Ok(GremlinValue::T(T::decode_v2(j_val)?)),
                        "g:TextP" => Ok(GremlinValue::TextP(TextP::decode_v2(j_val)?)),
                        "g:TraversalMetrics" => Ok(GremlinValue::TraversalMetrics(
                            TraversalMetrics::decode_v2(j_val)?,
                        )),
                        "g:Traverser" => Ok(GremlinValue::Traverser(Traverser::decode_v2(j_val)?)),
                        // "gx:BigDecimal" => Ok(GremlinTypes::BigDecimal(BigDecimal::decode_v2(j_val)?)),
                        // "gx:BigInteger" => Ok(GremlinTypes::BigInteger(BigInteger::decode_v2(j_val)?)),
                        "gx:Byte" => Ok(GremlinValue::Byte(u8::decode_v2(j_val)?)),
                        // "gx:ByteBuffer" => Ok(GremlinTypes::ByteBuffer(ByteBuffer::decode_v2(j_val)?)),
                        #[cfg(feature = "extended")]
                        "gx:Char" => Ok(GremlinValue::Char(char::decode_v2(j_val)?)),
                        // "gx:Duration" => Ok(GremlinTypes::Duration(Duration::decode_v2(j_val)?)),
                        // "gx:InetAddress" => Ok(GremlinTypes::InetAddress(InetAddress::decode_v2(j_val)?)),
                        // "gx:Instant" => Ok(GremlinTypes::Instant(Instant::decode_v2(j_val)?)),
                        // "gx:LocalDate" => Ok(GremlinTypes::LocalDate(LocalDate::decode_v2(j_val)?)),
                        // "gx:LocalDateTime" => Ok(GremlinTypes::LocalDateTime(LocalDateTime::decode_v2(j_val)?)),
                        // "gx:LocalTime" => Ok(GremlinTypes::LocalTime(LocalTime::decode_v2(j_val)?)),
                        // "gx:MonthDay" => Ok(GremlinTypes::MonthDay(MonthDay::decode_v2(j_val)?)),
                        // "gx:OffsetDateTime" => Ok(GremlinTypes::OffsetDateTime(OffsetDateTime::decode_v2(j_val)?)),
                        // "gx:OffsetTime" => Ok(GremlinTypes::OffsetTime(OffsetTime::decode_v2(j_val)?)),
                        // "gx:Period" => Ok(GremlinTypes::Period(Period::decode_v2(j_val)?)),
                        "gx:Int16" => Ok(GremlinValue::Short(i16::decode_v2(j_val)?)),
                        // "gx:Year" => Ok(GremlinTypes::Year(Year::decode_v2(j_val)?)),
                        // "gx:YearMonth" => Ok(GremlinTypes::YearMonth(YearMonth::decode_v2(j_val)?)),
                        // "gx:ZonedDateTime" => Ok(GremlinTypes::ZonedDateTime(ZonedDateTime::decode_v2(j_val)?)),
                        // "gx:ZoneOffset" => Ok(GremlinTypes::ZoneOffset(ZoneOffset::decode_v2(j_val)?)),
                        _ => todo!(),
                    }
                } else {
                    Ok(GremlinValue::Map(HashMap::decode_v2(j_val)?))
                }
            }
            _ => todo!(),
        }
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

#[test]
fn test() {
    let i = Duration::from_millis(1481750076295);
    // let x: i64 = SystemTime::from(i);
    println!("{:?}",i)
}
