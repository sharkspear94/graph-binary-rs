use std::{collections::HashMap, net::IpAddr};

use bigdecimal::BigDecimal;
#[cfg(feature = "extended")]
use chrono::{DateTime, Duration, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime};
use num::BigInt;
use serde_json::json;
use uuid::Uuid;

#[cfg(feature = "extended")]
use crate::extended::chrono::{
    Instant, MonthDay, OffsetTime, Period, Year, YearMonth, ZonedDateTime,
};
use crate::{
    error::GraphSonError,
    structure::{
        bytebuffer::ByteBuffer,
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
    Binding, GremlinValue,
};

mod enums;
#[cfg(feature = "extended")]
mod extended;
mod primitivs;
mod std_collections;
mod structures;

pub trait EncodeGraphSON {
    fn encode_v3(&self) -> serde_json::Value;

    fn encode_v2(&self) -> serde_json::Value;

    fn encode_v1(&self) -> serde_json::Value;
}

pub trait DecodeGraphSON {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized;

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized;

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized;
}

impl EncodeGraphSON for GremlinValue {
    fn encode_v3(&self) -> serde_json::Value {
        match self {
            GremlinValue::Int(val) => val.encode_v3(),
            GremlinValue::Long(val) => val.encode_v3(),
            GremlinValue::String(val) => val.encode_v3(),
            GremlinValue::Date(val) => json!({
              "@type" : "g:Date",
              "@value" : val
            }),
            GremlinValue::Timestamp(val) => json!({
              "@type" : "g:Timestamp",
              "@value" : val
            }),
            GremlinValue::Class(val) => val.encode_v3(),
            GremlinValue::Double(val) => val.encode_v3(),
            GremlinValue::Float(val) => val.encode_v3(),
            GremlinValue::List(val) => val.encode_v3(),
            GremlinValue::Set(val) => val.encode_v3(),
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
            GremlinValue::BigInteger(val) => val.encode_v3(),
            GremlinValue::BigDecimal(val) => val.encode_v3(),
            GremlinValue::Traverser(val) => val.encode_v3(),
            GremlinValue::Byte(val) => val.encode_v3(),
            GremlinValue::ByteBuffer(val) => val.encode_v3(),
            GremlinValue::Short(val) => val.encode_v3(),
            GremlinValue::Boolean(val) => val.encode_v3(),
            GremlinValue::TextP(val) => val.encode_v3(),
            GremlinValue::BulkSet(val) => val.encode_v3(),
            // GremlinTypes::Tree(val) => val.encode_v3(),
            GremlinValue::Metrics(val) => val.encode_v3(),
            GremlinValue::TraversalMetrics(val) => val.encode_v3(),
            GremlinValue::Merge(val) => val.encode_v3(),
            GremlinValue::UnspecifiedNullObject => serde_json::Value::Null,
            #[cfg(feature = "extended")]
            GremlinValue::Char(val) => val.encode_v3(),
            #[cfg(feature = "extended")]
            GremlinValue::Duration(val) => val.encode_v3(),
            #[cfg(feature = "extended")]
            GremlinValue::InetAddress(val) => val.encode_v3(),
            #[cfg(feature = "extended")]
            GremlinValue::Instant(val) => val.encode_v3(),
            #[cfg(feature = "extended")]
            GremlinValue::LocalDate(val) => val.encode_v3(),
            #[cfg(feature = "extended")]
            GremlinValue::LocalDateTime(val) => val.encode_v3(),
            #[cfg(feature = "extended")]
            GremlinValue::LocalTime(val) => val.encode_v3(),
            #[cfg(feature = "extended")]
            GremlinValue::MonthDay(val) => val.encode_v3(),
            #[cfg(feature = "extended")]
            GremlinValue::OffsetDateTime(val) => val.encode_v3(),
            #[cfg(feature = "extended")]
            GremlinValue::OffsetTime(val) => val.encode_v3(),
            #[cfg(feature = "extended")]
            GremlinValue::Period(val) => val.encode_v3(),
            #[cfg(feature = "extended")]
            GremlinValue::Year(val) => val.encode_v3(),
            #[cfg(feature = "extended")]
            GremlinValue::YearMonth(val) => val.encode_v3(),
            #[cfg(feature = "extended")]
            GremlinValue::ZonedDateTime(val) => val.encode_v3(),
            #[cfg(feature = "extended")]
            GremlinValue::ZoneOffset(val) => val.encode_v3(),
            _ => unimplemented!("not supported with GraphSON V3 encode"),
        }
    }
    fn encode_v2(&self) -> serde_json::Value {
        match self {
            GremlinValue::Int(val) => val.encode_v2(),
            GremlinValue::Long(val) => val.encode_v2(),
            GremlinValue::String(val) => val.encode_v2(),
            GremlinValue::Date(val) => json!({
              "@type" : "g:Date",
              "@value" : val
            }),
            GremlinValue::Timestamp(val) => json!({
              "@type" : "g:Timestamp",
              "@value" : val
            }),
            GremlinValue::Class(val) => val.encode_v2(),
            GremlinValue::Double(val) => val.encode_v2(),
            GremlinValue::Float(val) => val.encode_v2(),
            GremlinValue::List(val) => val.encode_v2(),
            GremlinValue::Set(val) => val.encode_v2(),
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
            GremlinValue::BigInteger(val) => val.encode_v2(),
            GremlinValue::BigDecimal(val) => val.encode_v2(),
            GremlinValue::Traverser(val) => val.encode_v2(),
            GremlinValue::Byte(val) => val.encode_v2(),
            GremlinValue::ByteBuffer(val) => val.encode_v2(),
            GremlinValue::Short(val) => val.encode_v2(),
            GremlinValue::Boolean(val) => val.encode_v2(),
            GremlinValue::TextP(val) => val.encode_v2(),
            GremlinValue::BulkSet(val) => val.encode_v2(),
            // GremlinTypes::Tree(val) => val.encode_v2(),
            GremlinValue::Metrics(val) => val.encode_v2(),
            GremlinValue::TraversalMetrics(val) => val.encode_v2(),
            GremlinValue::Merge(val) => val.encode_v2(),
            GremlinValue::UnspecifiedNullObject => serde_json::Value::Null,
            #[cfg(feature = "extended")]
            GremlinValue::Char(val) => val.encode_v2(),
            #[cfg(feature = "extended")]
            GremlinValue::Duration(val) => val.encode_v2(),
            #[cfg(feature = "extended")]
            GremlinValue::InetAddress(val) => val.encode_v2(),
            #[cfg(feature = "extended")]
            GremlinValue::Instant(val) => val.encode_v2(),
            #[cfg(feature = "extended")]
            GremlinValue::LocalDate(val) => val.encode_v2(),
            #[cfg(feature = "extended")]
            GremlinValue::LocalDateTime(val) => val.encode_v2(),
            #[cfg(feature = "extended")]
            GremlinValue::LocalTime(val) => val.encode_v2(),
            #[cfg(feature = "extended")]
            GremlinValue::MonthDay(val) => val.encode_v2(),
            #[cfg(feature = "extended")]
            GremlinValue::OffsetDateTime(val) => val.encode_v2(),
            #[cfg(feature = "extended")]
            GremlinValue::OffsetTime(val) => val.encode_v2(),
            #[cfg(feature = "extended")]
            GremlinValue::Period(val) => val.encode_v2(),
            #[cfg(feature = "extended")]
            GremlinValue::Year(val) => val.encode_v2(),
            #[cfg(feature = "extended")]
            GremlinValue::YearMonth(val) => val.encode_v2(),
            #[cfg(feature = "extended")]
            GremlinValue::ZonedDateTime(val) => val.encode_v2(),
            #[cfg(feature = "extended")]
            GremlinValue::ZoneOffset(val) => val.encode_v2(),
            _ => unimplemented!("not supported with GraphSON V2 encode"),
        }
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

impl DecodeGraphSON for GremlinValue {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        use crate::structure::map::MapKeys;

        match j_val {
            serde_json::Value::Null => Ok(GremlinValue::UnspecifiedNullObject),
            serde_json::Value::Bool(b) => Ok(GremlinValue::Boolean(*b)),
            serde_json::Value::String(s) => Ok(GremlinValue::String(s.clone())),
            serde_json::Value::Object(o) => {
                match o
                    .get("@type")
                    .and_then(|s| s.as_str())
                    .ok_or_else(|| GraphSonError::KeyNotFound("@type".to_string()))?
                {
                    "g:Int32" => Ok(GremlinValue::Int(i32::decode_v3(j_val)?)),
                    "g:Int64" => Ok(GremlinValue::Long(i64::decode_v3(j_val)?)),
                    "g:Class" => Ok(GremlinValue::Class(
                        o.get("@value")
                            .and_then(|c| c.as_str())
                            .ok_or_else(|| GraphSonError::WrongJsonType("str".to_string()))
                            .map(|class| class.to_string())?,
                    )),
                    "g:Date" => Ok(GremlinValue::Date(
                        o.get("@value")
                            .and_then(|c| c.as_i64())
                            .ok_or_else(|| GraphSonError::WrongJsonType("i64".to_string()))?,
                    )),
                    "g:Timestamp" => Ok(GremlinValue::Timestamp(
                        o.get("@value")
                            .and_then(|c| c.as_i64())
                            .ok_or_else(|| GraphSonError::WrongJsonType("i64".to_string()))?,
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
                    "gx:BigDecimal" => Ok(GremlinValue::BigDecimal(BigDecimal::decode_v3(j_val)?)),
                    "gx:BigInteger" => Ok(GremlinValue::BigInteger(BigInt::decode_v3(j_val)?)),
                    "gx:Byte" => Ok(GremlinValue::Byte(u8::decode_v3(j_val)?)),
                    "gx:ByteBuffer" => Ok(GremlinValue::ByteBuffer(ByteBuffer::decode_v3(j_val)?)),
                    "gx:Int16" => Ok(GremlinValue::Short(i16::decode_v3(j_val)?)),
                    #[cfg(feature = "extended")]
                    "gx:Char" => Ok(GremlinValue::Char(char::decode_v3(j_val)?)),
                    #[cfg(feature = "extended")]
                    "gx:Duration" => Ok(GremlinValue::Duration(Duration::decode_v3(j_val)?)),
                    #[cfg(feature = "extended")]
                    "gx:InetAddress" => Ok(GremlinValue::InetAddress(IpAddr::decode_v3(j_val)?)),
                    #[cfg(feature = "extended")]
                    "gx:Instant" => Ok(GremlinValue::Instant(Instant::decode_v3(j_val)?)),
                    #[cfg(feature = "extended")]
                    "gx:LocalDate" => Ok(GremlinValue::LocalDate(NaiveDate::decode_v3(j_val)?)),
                    #[cfg(feature = "extended")]
                    "gx:LocalDateTime" => Ok(GremlinValue::LocalDateTime(
                        NaiveDateTime::decode_v3(j_val)?,
                    )),
                    #[cfg(feature = "extended")]
                    "gx:LocalTime" => Ok(GremlinValue::LocalTime(NaiveTime::decode_v3(j_val)?)),
                    #[cfg(feature = "extended")]
                    "gx:MonthDay" => Ok(GremlinValue::MonthDay(MonthDay::decode_v3(j_val)?)),
                    #[cfg(feature = "extended")]
                    "gx:OffsetDateTime" => {
                        Ok(GremlinValue::OffsetDateTime(DateTime::decode_v3(j_val)?))
                    }
                    #[cfg(feature = "extended")]
                    "gx:OffsetTime" => Ok(GremlinValue::OffsetTime(OffsetTime::decode_v3(j_val)?)),
                    #[cfg(feature = "extended")]
                    "gx:Period" => Ok(GremlinValue::Period(Period::decode_v3(j_val)?)),
                    #[cfg(feature = "extended")]
                    "gx:Year" => Ok(GremlinValue::Year(Year::decode_v3(j_val)?)),
                    #[cfg(feature = "extended")]
                    "gx:YearMonth" => Ok(GremlinValue::YearMonth(YearMonth::decode_v3(j_val)?)),
                    #[cfg(feature = "extended")]
                    "gx:ZonedDateTime" => Ok(GremlinValue::ZonedDateTime(
                        ZonedDateTime::decode_v3(j_val)?,
                    )),
                    #[cfg(feature = "extended")]
                    "gx:ZoneOffset" => Ok(GremlinValue::ZoneOffset(FixedOffset::decode_v3(j_val)?)),
                    rest => Err(GraphSonError::WrongTypeIdentifier {
                        expected: "a GremlinValue identifier".to_string(),
                        found: rest.to_string(),
                    }),
                }
            }
            _ => Err(GraphSonError::WrongJsonType("arr/num".to_string())),
        }
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        use crate::structure::map::MapKeys;

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
                                .ok_or_else(|| GraphSonError::WrongJsonType("str".to_string()))
                                .map(ToString::to_string)?,
                        )),
                        "g:Date" => Ok(GremlinValue::Date(
                            o.get("@value")
                                .and_then(|c| c.as_i64())
                                .ok_or_else(|| GraphSonError::WrongJsonType("i64".to_string()))?,
                        )),
                        "g:Timestamp" => Ok(GremlinValue::Timestamp(
                            o.get("@value")
                                .and_then(|c| c.as_i64())
                                .ok_or_else(|| GraphSonError::WrongJsonType("i64".to_string()))?,
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
                        "gx:BigDecimal" => {
                            Ok(GremlinValue::BigDecimal(BigDecimal::decode_v2(j_val)?))
                        }
                        "gx:BigInteger" => Ok(GremlinValue::BigInteger(BigInt::decode_v2(j_val)?)),
                        "gx:Byte" => Ok(GremlinValue::Byte(u8::decode_v2(j_val)?)),
                        "gx:ByteBuffer" => {
                            Ok(GremlinValue::ByteBuffer(ByteBuffer::decode_v2(j_val)?))
                        }
                        "gx:Int16" => Ok(GremlinValue::Short(i16::decode_v2(j_val)?)),
                        #[cfg(feature = "extended")]
                        "gx:Char" => Ok(GremlinValue::Char(char::decode_v2(j_val)?)),
                        #[cfg(feature = "extended")]
                        "gx:Duration" => Ok(GremlinValue::Duration(Duration::decode_v2(j_val)?)),
                        #[cfg(feature = "extended")]
                        "gx:InetAddress" => {
                            Ok(GremlinValue::InetAddress(IpAddr::decode_v2(j_val)?))
                        }
                        #[cfg(feature = "extended")]
                        "gx:Instant" => Ok(GremlinValue::Instant(Instant::decode_v2(j_val)?)),
                        #[cfg(feature = "extended")]
                        "gx:LocalDate" => Ok(GremlinValue::LocalDate(NaiveDate::decode_v2(j_val)?)),
                        #[cfg(feature = "extended")]
                        "gx:LocalDateTime" => Ok(GremlinValue::LocalDateTime(
                            NaiveDateTime::decode_v2(j_val)?,
                        )),
                        #[cfg(feature = "extended")]
                        "gx:LocalTime" => Ok(GremlinValue::LocalTime(NaiveTime::decode_v2(j_val)?)),
                        #[cfg(feature = "extended")]
                        "gx:MonthDay" => Ok(GremlinValue::MonthDay(MonthDay::decode_v2(j_val)?)),
                        #[cfg(feature = "extended")]
                        "gx:OffsetDateTime" => {
                            Ok(GremlinValue::OffsetDateTime(DateTime::decode_v2(j_val)?))
                        }
                        #[cfg(feature = "extended")]
                        "gx:OffsetTime" => {
                            Ok(GremlinValue::OffsetTime(OffsetTime::decode_v2(j_val)?))
                        }
                        #[cfg(feature = "extended")]
                        "gx:Period" => Ok(GremlinValue::Period(Period::decode_v2(j_val)?)),
                        #[cfg(feature = "extended")]
                        "gx:Year" => Ok(GremlinValue::Year(Year::decode_v2(j_val)?)),
                        #[cfg(feature = "extended")]
                        "gx:YearMonth" => Ok(GremlinValue::YearMonth(YearMonth::decode_v2(j_val)?)),
                        #[cfg(feature = "extended")]
                        "gx:ZonedDateTime" => Ok(GremlinValue::ZonedDateTime(
                            ZonedDateTime::decode_v2(j_val)?,
                        )),
                        #[cfg(feature = "extended")]
                        "gx:ZoneOffset" => {
                            Ok(GremlinValue::ZoneOffset(FixedOffset::decode_v2(j_val)?))
                        }
                        rest => Err(GraphSonError::WrongTypeIdentifier {
                            expected: "a GremlinValue identifier".to_string(),
                            found: rest.to_string(),
                        }),
                    }
                } else {
                    Ok(GremlinValue::Map(HashMap::decode_v2(j_val)?))
                }
            }
            serde_json::Value::Number(_) => Err(GraphSonError::WrongJsonType("number".to_string())),
        }
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

#[macro_export]
macro_rules! val_by_key_v1 {
    ($obj:expr,$key:literal,$expected:ty,$context:literal) => {
        $obj.and_then(|m| m.get($key))
            .and_then(|j_val| <$expected>::decode_v1(j_val).ok())
            .ok_or_else(|| {
                DecodeError::DecodeError(format!(
                    "Error extracting a {} from key: {}, during {} v1 decoding",
                    stringify!($expected),
                    $key,
                    $context
                ))
            })
    };
}

#[macro_export]
macro_rules! val_by_key_v2 {
    ($obj:expr,$key:literal,$expected:ty,$context:literal) => {
        $obj.and_then(|m| m.get($key))
            .and_then(|j_val| <$expected>::decode_v2(j_val).ok())
            .ok_or_else(|| {
                DecodeError::DecodeError(format!(
                    "Error extracting a {} from key: {}, during {} v2 decoding",
                    stringify!($expected),
                    $key,
                    $context
                ))
            })
    };
}

#[macro_export]
macro_rules! val_by_key_v2_new {
    ($obj:expr,$key:literal,$expected:ty,$context:literal) => {
        $obj.get($key)
            .map(|j_val| <$expected>::decode_v2(j_val))
            .transpose()?
            .ok_or_else(|| {
                DecodeError::DecodeError(format!(
                    "Error extracting a {} from key: {}, during {} v2 decoding",
                    stringify!($expected),
                    $key,
                    $context
                ))
            })
    };
}

#[macro_export]
macro_rules! val_by_key_v3 {
    ($obj:expr,$key:literal,$expected:ty,$context:literal) => {
        $obj.and_then(|m| m.get($key))
            .and_then(|j_val| <$expected>::decode_v3(j_val).ok())
            .ok_or_else(|| {
                DecodeError::DecodeError(format!(
                    "Error extracting a {} from key: {}, during {} v3 decoding",
                    stringify!($expected),
                    $key,
                    $context
                ))
            })
    };
}

#[macro_export]
macro_rules! val_by_key_v3_new {
    ($obj:expr,$key:literal,$expected:ty,$context:literal) => {
        $obj.get($key)
            .ok_or_else(|| GraphSonError::KeyNotFound(stringify!($key)))?

        <$expected>::decode_v3(j_val).map_err(|e| GraphSonError::FieldError(context: $context.to_string(),source: e))?
    };
}

pub(crate) fn get_val_by_key_v3<T: DecodeGraphSON>(
    jval: &serde_json::Value,
    key: &str,
    context: &str,
) -> Result<T, GraphSonError> {
    let val = jval.get(key).ok_or_else(|| {
        GraphSonError::KeyNotFound(format!("{key} not found during graphson decode {context}"))
    })?;
    T::decode_v3(val)
}

pub(crate) fn get_val_by_key_v2<T: DecodeGraphSON>(
    jval: &serde_json::Value,
    key: &str,
    context: &str,
) -> Result<T, GraphSonError> {
    let val = jval.get(key).ok_or_else(|| {
        GraphSonError::KeyNotFound(format!("{key} not found during graphson decode {context}"))
    })?;
    T::decode_v2(val)
}

pub(crate) fn get_val_by_key_v1<T: DecodeGraphSON>(
    jval: &serde_json::Value,
    key: &str,
    _context: &str,
) -> Result<T, GraphSonError> {
    let val = jval
        .get(key)
        .ok_or_else(|| GraphSonError::KeyNotFound(key.to_string()))?;
    T::decode_v1(val)
}

pub(crate) fn validate_type_entry(
    map: &serde_json::Map<String, serde_json::Value>,
    type_value: &str,
) -> bool {
    map.get("@type")
        .and_then(|val| val.as_str())
        .filter(|s| s.eq(&type_value))
        .is_some()
}

pub(crate) fn validate_type<'a>(
    jval: &'a serde_json::Value,
    identifier: &str,
) -> Result<&'a serde_json::Value, GraphSonError> {
    let a = jval
        .get("@type")
        .ok_or_else(|| GraphSonError::KeyNotFound("@type".to_string()))?
        .as_str()
        .ok_or_else(|| GraphSonError::WrongJsonType("str".to_string()))?;
    if a.ne(identifier) {
        return Err(GraphSonError::WrongTypeIdentifier {
            expected: identifier.to_string(),
            found: a.to_string(),
        });
    }

    jval.get("@value")
        .ok_or_else(|| GraphSonError::KeyNotFound("@value".to_string()))
}
