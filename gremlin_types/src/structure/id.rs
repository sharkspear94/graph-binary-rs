use std::fmt::Display;

use uuid::Uuid;

use crate::GremlinValue;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum ElementId {
    String(String),
    Int(i32),
    Long(i64),
    Uuid(Uuid),
}

impl ElementId {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            ElementId::String(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_i32(&self) -> Option<i32> {
        match self {
            ElementId::Int(val) => Some(*val),
            _ => None,
        }
    }
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            ElementId::Long(val) => Some(*val),
            _ => None,
        }
    }
    pub fn as_uuid(&self) -> Option<Uuid> {
        match self {
            ElementId::Uuid(val) => Some(*val),
            _ => None,
        }
    }
}

impl Display for ElementId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElementId::String(s) => write!(f, "{s}"),
            ElementId::Int(i) => write!(f, "{i}"),
            ElementId::Long(l) => write!(f, "{l}"),
            ElementId::Uuid(u) => write!(f, "{u}"),
        }
    }
}

impl From<String> for ElementId {
    fn from(value: String) -> Self {
        ElementId::String(value)
    }
}

impl From<i32> for ElementId {
    fn from(value: i32) -> Self {
        ElementId::Int(value)
    }
}

impl From<i64> for ElementId {
    fn from(value: i64) -> Self {
        ElementId::Long(value)
    }
}

impl From<Uuid> for ElementId {
    fn from(value: Uuid) -> Self {
        ElementId::Uuid(value)
    }
}

impl From<ElementId> for GremlinValue {
    fn from(value: ElementId) -> Self {
        match value {
            ElementId::String(s) => GremlinValue::String(s),
            ElementId::Int(i) => GremlinValue::Int(i),
            ElementId::Long(l) => GremlinValue::Long(l),
            ElementId::Uuid(u) => GremlinValue::Uuid(u),
        }
    }
}
