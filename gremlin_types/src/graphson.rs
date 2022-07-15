use std::{
    collections::HashMap,
    io::{Read, Write},
};

use serde_json::Map;
use uuid::Uuid;

use crate::{
    error::{DecodeError, EncodeError},
    graph_binary::{GremlinTypes, MapKeys},
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

impl EncodeGraphSON for GremlinTypes {
    fn encode_v3(&self) -> serde_json::Value {
        match self {
            GremlinTypes::Int(val) => val.encode_v3(),
            GremlinTypes::Long(val) => val.encode_v3(),
            GremlinTypes::String(val) => val.encode_v3(),
            GremlinTypes::Class(val) => val.encode_v3(),
            GremlinTypes::Double(val) => val.encode_v3(),
            GremlinTypes::Float(val) => val.encode_v3(),
            GremlinTypes::List(val) => val.encode_v3(),
            GremlinTypes::Set(val) => val.encode_v3(), // FIXME
            GremlinTypes::Map(val) => val.encode_v3(),
            GremlinTypes::Uuid(val) => val.encode_v3(),
            // GremlinTypes::Edge(val) => val.encode_v3(),
            // GremlinTypes::Path(val) => val.encode_v3(),
            // GremlinTypes::Property(val) => val.encode_v3(),
            // GremlinTypes::Graph(val) => val.encode_v3(),
            GremlinTypes::Vertex(val) => val.encode_v3(),
            // GremlinTypes::VertexProperty(val) => val.encode_v3(),
            // GremlinTypes::Barrier(val) => val.encode_v3(),
            // GremlinTypes::Binding(val) => val.encode_v3(),
            // GremlinTypes::Bytecode(val) => val.encode_v3(),
            // GremlinTypes::Cardinality(val) => val.encode_v3(),
            // GremlinTypes::Column(val) => val.encode_v3(),
            // GremlinTypes::Direction(val) => val.encode_v3(),
            // GremlinTypes::Operator(val) => val.encode_v3(),
            // GremlinTypes::Order(val) => val.encode_v3(),
            // GremlinTypes::Pick(val) => val.encode_v3(),
            // GremlinTypes::Pop(val) => val.encode_v3(),
            // GremlinTypes::Lambda(val) => val.encode_v3(),
            // GremlinTypes::P(val) => val.encode_v3(),
            // GremlinTypes::Scope(val) => val.encode_v3(),
            // GremlinTypes::T(val) => val.encode_v3(),
            // GremlinTypes::Traverser(val) => val.encode_v3(),
            // GremlinTypes::Byte(val) => val.encode_v3(),
            // GremlinTypes::ByteBuffer(val) => val.encode_v3(),
            // GremlinTypes::Short(val) => val.encode_v3(),
            // GremlinTypes::Boolean(val) => val.encode_v3(),
            // GremlinTypes::TextP(val) => val.encode_v3(),
            // GremlinTypes::TraversalStrategy(val) => val.encode_v3(),
            // GremlinTypes::BulkSet(val) => val.encode_v3(),
            // GremlinTypes::Tree(val) => val.encode_v3(),
            // GremlinTypes::Metrics(val) => val.encode_v3(),
            // GremlinTypes::TraversalMetrics(val) => val.encode_v3(),
            // GremlinTypes::Merge(val) => val.encode_v3(),
            GremlinTypes::UnspecifiedNullObject => serde_json::Value::Null,
            GremlinTypes::Char(val) => val.encode_v3(),
            _ => unimplemented!(),
        }
    }

    fn encode_v2(&self) -> serde_json::Value {
        match self {
            GremlinTypes::Int(val) => val.encode_v2(),
            GremlinTypes::Long(val) => val.encode_v2(),
            GremlinTypes::String(val) => val.encode_v2(),
            GremlinTypes::Class(val) => val.encode_v2(),
            GremlinTypes::Double(val) => val.encode_v2(),
            GremlinTypes::Float(val) => val.encode_v2(),
            GremlinTypes::List(val) => val.encode_v2(),
            GremlinTypes::Set(val) => val.encode_v2(), // FIXME
            GremlinTypes::Map(val) => val.encode_v2(),
            GremlinTypes::Uuid(val) => val.encode_v2(),
            GremlinTypes::Edge(val) => val.encode_v2(),
            GremlinTypes::Path(val) => val.encode_v2(),
            GremlinTypes::Property(val) => val.encode_v2(),
            // GremlinTypes::Graph(val) => val.encode_v2(),
            GremlinTypes::Vertex(val) => val.encode_v2(),
            GremlinTypes::VertexProperty(val) => val.encode_v2(),
            GremlinTypes::Barrier(val) => val.encode_v2(),
            GremlinTypes::Binding(val) => val.encode_v2(),
            GremlinTypes::Bytecode(val) => val.encode_v2(),
            GremlinTypes::Cardinality(val) => val.encode_v2(),
            GremlinTypes::Column(val) => val.encode_v2(),
            GremlinTypes::Direction(val) => val.encode_v2(),
            GremlinTypes::Operator(val) => val.encode_v2(),
            GremlinTypes::Order(val) => val.encode_v2(),
            GremlinTypes::Pick(val) => val.encode_v2(),
            GremlinTypes::Pop(val) => val.encode_v2(),
            // GremlinTypes::Lambda(val) => val.encode_v2(),
            GremlinTypes::P(val) => val.encode_v2(),
            GremlinTypes::Scope(val) => val.encode_v2(),
            GremlinTypes::T(val) => val.encode_v2(),
            // GremlinTypes::Traverser(val) => val.encode_v2(),
            GremlinTypes::Byte(val) => val.encode_v2(),
            GremlinTypes::ByteBuffer(val) => val.encode_v2(),
            GremlinTypes::Short(val) => val.encode_v2(),
            GremlinTypes::Boolean(val) => val.encode_v2(),
            // GremlinTypes::TextP(val) => val.encode_v2(),
            // GremlinTypes::TraversalStrategy(val) => val.encode_v2(),
            // GremlinTypes::BulkSet(val) => val.encode_v2(),
            // GremlinTypes::Tree(val) => val.encode_v2(),
            // GremlinTypes::Metrics(val) => val.encode_v2(),
            // GremlinTypes::TraversalMetrics(val) => val.encode_v2(),
            GremlinTypes::Merge(val) => val.encode_v2(),
            GremlinTypes::UnspecifiedNullObject => serde_json::Value::Null,
            GremlinTypes::Char(val) => val.encode_v2(),
            _ => unimplemented!(),
        }
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

impl DecodeGraphSON for GremlinTypes {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        match j_val {
            serde_json::Value::Null => Ok(GremlinTypes::UnspecifiedNullObject),
            serde_json::Value::Bool(b) => Ok(GremlinTypes::Boolean(*b)),
            serde_json::Value::String(s) => Ok(GremlinTypes::String(s.clone())),
            serde_json::Value::Object(o) => {
                match o
                    .get("@type")
                    .and_then(|s| s.as_str())
                    .ok_or_else(|| DecodeError::DecodeError("".to_string()))?
                {
                    "g:Int32" => Ok(GremlinTypes::Int(i32::decode_v3(j_val)?)),
                    "g:Int64" => Ok(GremlinTypes::Long(i64::decode_v3(j_val)?)),
                    "g:Class" => Ok(GremlinTypes::Class(
                        o.get("@value")
                            .and_then(|c| c.as_str())
                            .ok_or_else(|| {
                                crate::error::DecodeError::DecodeError(
                                    "json error Class v3 in error".to_string(),
                                )
                            })
                            .map(|class| class.to_string())?,
                    )),
                    "g:Double" => Ok(GremlinTypes::Double(f64::decode_v3(j_val)?)),
                    "g:Float" => Ok(GremlinTypes::Float(f32::decode_v3(j_val)?)),
                    "g:List" => Ok(GremlinTypes::List(Vec::<GremlinTypes>::decode_v3(j_val)?)),
                    "g:Map" => Ok(GremlinTypes::Map(
                        HashMap::<MapKeys, GremlinTypes>::decode_v3(j_val)?,
                    )),
                    "g:UUID" => Ok(GremlinTypes::Uuid(Uuid::decode_v3(j_val)?)),
                    "g:Edge" => Ok(GremlinTypes::Edge(Edge::decode_v3(j_val)?)),
                    "g:Path" => Ok(GremlinTypes::Path(Path::decode_v3(j_val)?)),
                    "g:Property" => Ok(GremlinTypes::Property(Property::decode_v3(j_val)?)),
                    "g:tinker:graph" => Ok(GremlinTypes::Graph(Graph::decode_v3(j_val)?)),
                    "g:Vertex" => Ok(GremlinTypes::Vertex(Vertex::decode_v3(j_val)?)),
                    "g:VertexProperty" => Ok(GremlinTypes::VertexProperty(
                        VertexProperty::decode_v3(j_val)?,
                    )),
                    "g:Barrier" => Ok(GremlinTypes::Barrier(Barrier::decode_v3(j_val)?)),
                    "g:Binding" => Ok(GremlinTypes::Binding(Binding::decode_v3(j_val)?)),
                    "g:Bytecode" => Ok(GremlinTypes::Bytecode(Bytecode::decode_v3(j_val)?)),
                    "g:Cardinality" => {
                        Ok(GremlinTypes::Cardinality(Cardinality::decode_v3(j_val)?))
                    }
                    "g:Column" => Ok(GremlinTypes::Column(Column::decode_v3(j_val)?)),
                    "g:Direction" => Ok(GremlinTypes::Direction(Direction::decode_v3(j_val)?)),
                    "g:Lambda" => Ok(GremlinTypes::Lambda(Lambda::decode_v3(j_val)?)),
                    "g:Merge" => Ok(GremlinTypes::Merge(Merge::decode_v3(j_val)?)),
                    "g:Metrics" => Ok(GremlinTypes::Metrics(Metrics::decode_v3(j_val)?)),
                    "g:Operator" => Ok(GremlinTypes::Operator(Operator::decode_v3(j_val)?)),
                    "g:Order" => Ok(GremlinTypes::Order(Order::decode_v3(j_val)?)),
                    // "g:P" => Ok(GremlinTypes::P(P::decode_v3(j_val)?)),
                    "g:Pick" => Ok(GremlinTypes::Pick(Pick::decode_v3(j_val)?)),
                    "g:Pop" => Ok(GremlinTypes::Pop(Pop::decode_v3(j_val)?)),
                    "g:Scope" => Ok(GremlinTypes::Scope(Scope::decode_v3(j_val)?)),
                    "g:T" => Ok(GremlinTypes::T(T::decode_v3(j_val)?)),
                    // "g:TextP" => Ok(GremlinTypes::TextP(TextP::decode_v3(j_val)?)),
                    "g:TraversalMetrics" => Ok(GremlinTypes::TraversalMetrics(
                        TraversalMetrics::decode_v3(j_val)?,
                    )),
                    "g:Traverser" => Ok(GremlinTypes::Traverser(Traverser::decode_v3(j_val)?)),
                    // "gx:BigDecimal" => Ok(GremlinTypes::BigDecimal(BigDecimal::decode_v3(j_val)?)),
                    // "gx:BigInteger" => Ok(GremlinTypes::BigInteger(BigInteger::decode_v3(j_val)?)),
                    "gx:Byte" => Ok(GremlinTypes::Byte(u8::decode_v3(j_val)?)),
                    // "gx:ByteBuffer" => Ok(GremlinTypes::ByteBuffer(ByteBuffer::decode_v3(j_val)?)),
                    "gx:Char" => Ok(GremlinTypes::Char(char::decode_v3(j_val)?)),
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
                    "gx:Int16" => Ok(GremlinTypes::Short(i16::decode_v3(j_val)?)),
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
            serde_json::Value::Null => Ok(GremlinTypes::UnspecifiedNullObject),
            serde_json::Value::Bool(b) => Ok(GremlinTypes::Boolean(*b)),
            serde_json::Value::String(s) => Ok(GremlinTypes::String(s.clone())),
            serde_json::Value::Array(arr) => {
                let mut vec = Vec::with_capacity(arr.len());
                for item in arr {
                    vec.push(GremlinTypes::decode_v2(item)?);
                }
                Ok(GremlinTypes::List(vec))
            }
            serde_json::Value::Object(o) => {
                if let Some(type_identifier) = o.get("@type").and_then(|s| s.as_str()) {
                    match type_identifier {
                        "g:Int32" => Ok(GremlinTypes::Int(i32::decode_v2(j_val)?)),
                        "g:Int64" => Ok(GremlinTypes::Long(i64::decode_v2(j_val)?)),
                        "g:Class" => Ok(GremlinTypes::Class(
                            o.get("@value")
                                .and_then(|c| c.as_str())
                                .ok_or_else(|| {
                                    crate::error::DecodeError::DecodeError(
                                        "json error Class v3 in error".to_string(),
                                    )
                                })
                                .map(|class| class.to_string())?,
                        )),
                        "g:Double" => Ok(GremlinTypes::Double(f64::decode_v2(j_val)?)),
                        "g:Float" => Ok(GremlinTypes::Float(f32::decode_v2(j_val)?)),
                        "g:List" => Ok(GremlinTypes::List(Vec::<GremlinTypes>::decode_v2(j_val)?)),
                        "g:Map" => Ok(GremlinTypes::Map(
                            HashMap::<MapKeys, GremlinTypes>::decode_v2(j_val)?,
                        )),
                        "g:UUID" => Ok(GremlinTypes::Uuid(Uuid::decode_v2(j_val)?)),
                        "g:Edge" => Ok(GremlinTypes::Edge(Edge::decode_v2(j_val)?)),
                        "g:Path" => Ok(GremlinTypes::Path(Path::decode_v2(j_val)?)),
                        "g:Property" => Ok(GremlinTypes::Property(Property::decode_v2(j_val)?)),
                        "g:tinker:graph" => Ok(GremlinTypes::Graph(Graph::decode_v2(j_val)?)),
                        "g:Vertex" => Ok(GremlinTypes::Vertex(Vertex::decode_v2(j_val)?)),
                        "g:VertexProperty" => Ok(GremlinTypes::VertexProperty(
                            VertexProperty::decode_v2(j_val)?,
                        )),
                        "g:Barrier" => Ok(GremlinTypes::Barrier(Barrier::decode_v2(j_val)?)),
                        "g:Binding" => Ok(GremlinTypes::Binding(Binding::decode_v2(j_val)?)),
                        "g:Bytecode" => Ok(GremlinTypes::Bytecode(Bytecode::decode_v2(j_val)?)),
                        "g:Cardinality" => {
                            Ok(GremlinTypes::Cardinality(Cardinality::decode_v2(j_val)?))
                        }
                        "g:Column" => Ok(GremlinTypes::Column(Column::decode_v2(j_val)?)),
                        "g:Direction" => Ok(GremlinTypes::Direction(Direction::decode_v2(j_val)?)),
                        "g:Lambda" => Ok(GremlinTypes::Lambda(Lambda::decode_v2(j_val)?)),
                        "g:Merge" => Ok(GremlinTypes::Merge(Merge::decode_v2(j_val)?)),
                        "g:Metrics" => Ok(GremlinTypes::Metrics(Metrics::decode_v2(j_val)?)),
                        "g:Operator" => Ok(GremlinTypes::Operator(Operator::decode_v2(j_val)?)),
                        "g:Order" => Ok(GremlinTypes::Order(Order::decode_v2(j_val)?)),
                        // "g:P" => Ok(GremlinTypes::P(P::decode_v2(j_val)?)),
                        "g:Pick" => Ok(GremlinTypes::Pick(Pick::decode_v2(j_val)?)),
                        "g:Pop" => Ok(GremlinTypes::Pop(Pop::decode_v2(j_val)?)),
                        "g:Scope" => Ok(GremlinTypes::Scope(Scope::decode_v2(j_val)?)),
                        "g:T" => Ok(GremlinTypes::T(T::decode_v2(j_val)?)),
                        // "g:TextP" => Ok(GremlinTypes::TextP(TextP::decode_v2(j_val)?)),
                        "g:TraversalMetrics" => Ok(GremlinTypes::TraversalMetrics(
                            TraversalMetrics::decode_v2(j_val)?,
                        )),
                        "g:Traverser" => Ok(GremlinTypes::Traverser(Traverser::decode_v2(j_val)?)),
                        // "gx:BigDecimal" => Ok(GremlinTypes::BigDecimal(BigDecimal::decode_v2(j_val)?)),
                        // "gx:BigInteger" => Ok(GremlinTypes::BigInteger(BigInteger::decode_v2(j_val)?)),
                        "gx:Byte" => Ok(GremlinTypes::Byte(u8::decode_v2(j_val)?)),
                        // "gx:ByteBuffer" => Ok(GremlinTypes::ByteBuffer(ByteBuffer::decode_v2(j_val)?)),
                        "gx:Char" => Ok(GremlinTypes::Char(char::decode_v2(j_val)?)),
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
                        "gx:Int16" => Ok(GremlinTypes::Short(i16::decode_v2(j_val)?)),
                        // "gx:Year" => Ok(GremlinTypes::Year(Year::decode_v2(j_val)?)),
                        // "gx:YearMonth" => Ok(GremlinTypes::YearMonth(YearMonth::decode_v2(j_val)?)),
                        // "gx:ZonedDateTime" => Ok(GremlinTypes::ZonedDateTime(ZonedDateTime::decode_v2(j_val)?)),
                        // "gx:ZoneOffset" => Ok(GremlinTypes::ZoneOffset(ZoneOffset::decode_v2(j_val)?)),
                        _ => todo!(),
                    }
                } else {
                    Ok(GremlinTypes::Map(HashMap::decode_v2(j_val)?))
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
