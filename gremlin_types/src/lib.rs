// #![feature(generic_const_exprs)]

use core::slice;
use std::{collections::BinaryHeap, mem::size_of};
#[macro_use]

mod error;
pub mod graph_binary;
mod macros;
mod specs;
mod structure;

#[cfg(any(feature = "graph_son_v3", feature = "graph_son_v2"))]
pub mod graphson;

#[cfg(feature = "serde")]
pub mod de;
#[cfg(feature = "serde")]
pub mod ser;

use structure::map::MapKeys;
pub use structure::Binding;

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

impl GremlinValue {
    /// Returns an Option of an owned value if the Type was the GremlinValue variant.
    /// Returns None if GremlinValue enum holds another Type
    ///
    /// ```
    /// # use gremlin_types::GremlinValue;
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
    /// # use gremlin_types::GremlinValue;
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
    /// # use gremlin_types::GremlinValue;
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
    /// # use gremlin_types::GremlinValue;
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
