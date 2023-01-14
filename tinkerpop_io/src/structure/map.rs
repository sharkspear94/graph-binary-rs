use uuid::Uuid;

use super::enums::{Direction, T};
use crate::{error::DecodeError, GremlinValue};
use std::{collections::HashMap, fmt::Display, hash::Hash};

#[derive(Debug, Hash, PartialEq, Eq, Clone, PartialOrd, Ord)]
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

impl<K: Into<MapKeys>, V: Into<GremlinValue>> From<HashMap<K, V>> for GremlinValue {
    fn from(m: HashMap<K, V>) -> Self {
        let map = m.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
        GremlinValue::Map(map)
    }
}

impl<K, V> TryFrom<GremlinValue> for HashMap<K, V>
where
    K: TryFrom<MapKeys, Error = DecodeError> + Eq + Hash,
    V: TryFrom<GremlinValue, Error = DecodeError>,
{
    type Error = DecodeError;

    fn try_from(value: GremlinValue) -> Result<Self, Self::Error> {
        match value {
            GremlinValue::Map(map) => {
                let mut ret_map = HashMap::with_capacity(map.len());
                for (k, v) in map {
                    ret_map.insert(K::try_from(k)?, V::try_from(v)?);
                }
                Ok(ret_map)
            }
            _ => Err(DecodeError::ConvertError(String::new())),
        }
    }
}

impl<K> TryFrom<GremlinValue> for HashMap<K, GremlinValue>
where
    K: TryFrom<MapKeys, Error = DecodeError> + Eq + Hash,
{
    type Error = DecodeError;

    fn try_from(value: GremlinValue) -> Result<Self, Self::Error> {
        match value {
            GremlinValue::Map(map) => {
                let mut ret_map = HashMap::with_capacity(map.len());
                for (k, v) in map {
                    ret_map.insert(K::try_from(k)?, v);
                }
                Ok(ret_map)
            }
            _ => Err(DecodeError::ConvertError(String::new())),
        }
    }
}
