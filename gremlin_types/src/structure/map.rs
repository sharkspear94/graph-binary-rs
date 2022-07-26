use uuid::Uuid;

use super::enums::{Direction, T};
use crate::{
    error::{DecodeError, EncodeError},
    specs::CoreType,
    GremlinValue,
};
use std::{
    collections::HashMap,
    fmt::Display,
    hash::{BuildHasher, Hash},
};

#[cfg(feature = "graph_binary")]
use std::io::{Read, Write};

#[cfg(feature = "graph_binary")]
use crate::graph_binary::{Decode, Encode};

#[cfg(feature = "graph_son")]
use crate::error::GraphSonError;
#[cfg(feature = "graph_son")]
use crate::graphson::{validate_type, DecodeGraphSON, EncodeGraphSON};
#[cfg(feature = "graph_son")]
use serde_json::json;

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
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let g_key = GremlinValue::decode_v3(j_val)?;
        MapKeys::try_from(g_key).map_err(|e| GraphSonError::TryFrom(e.to_string()))
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let g_key = GremlinValue::decode_v2(j_val)?;
        MapKeys::try_from(g_key).map_err(|e| GraphSonError::TryFrom(e.to_string()))
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

#[cfg(feature = "graph_binary")]
impl<K, V, S: BuildHasher> Encode for HashMap<K, V, S>
where
    K: Encode,
    V: Encode,
{
    fn type_code() -> u8 {
        CoreType::Map.into()
    }

    fn partial_encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        let len = self.len() as i32;
        len.partial_encode(writer)?;

        for (key, value) in self.iter() {
            key.encode(writer)?;
            value.encode(writer)?;
        }
        Ok(())
    }
}

#[cfg(feature = "graph_binary")]
impl<K, V, S> Decode for HashMap<K, V, S>
where
    K: Decode + std::cmp::Eq + std::hash::Hash,
    V: Decode,
    S: BuildHasher + Default,
{
    fn expected_type_code() -> u8 {
        CoreType::Map.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let len = i32::partial_decode(reader)? as usize;
        let mut hash_map = HashMap::with_capacity_and_hasher(len, Default::default());
        for _ in 0..len {
            let key = K::decode(reader)?;
            let value = V::decode(reader)?;

            hash_map.insert(key, value); // TODO what happens if key is double present Error?
        }

        Ok(hash_map)
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

#[cfg(feature = "graph_son")]
impl<K, V> EncodeGraphSON for HashMap<K, V>
where
    K: EncodeGraphSON + ToString + std::cmp::Eq + std::hash::Hash,
    V: EncodeGraphSON,
{
    fn encode_v3(&self) -> serde_json::Value {
        let mut vec = Vec::with_capacity(self.len() * 2);
        for (k, v) in self {
            vec.push(k.encode_v3());
            vec.push(v.encode_v3());
        }
        json!({
            "@type" : "g:Map",
            "@value" : vec
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        let mut map = HashMap::with_capacity(self.len());
        for (k, v) in self {
            map.insert(k.to_string(), v.encode_v2());
        }
        json!(map)
    }

    fn encode_v1(&self) -> serde_json::Value {
        let mut map = HashMap::with_capacity(self.len());
        for (k, v) in self {
            map.insert(k.to_string(), v.encode_v1());
        }
        json!(map)
    }
}

#[cfg(feature = "graph_son")]
impl<K, V> DecodeGraphSON for HashMap<K, V>
where
    K: DecodeGraphSON + ToString + std::cmp::Eq + std::hash::Hash,
    V: DecodeGraphSON,
{
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:Map")?;

        let mut map_len = 0;
        let iter = value_object
            .as_array()
            .map(|array| {
                map_len = array.len() / 2;
                array.iter()
            })
            .ok_or_else(|| GraphSonError::WrongJsonType("array".to_string()))?;

        let mut map = HashMap::with_capacity(map_len);
        for (k, v) in iter.clone().zip(iter.skip(1)).step_by(2) {
            let key = K::decode_v3(k)?;
            let val = V::decode_v3(v)?;
            map.insert(key, val);
        }

        Ok(map)
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let mut map_len = 0;
        let iter = j_val
            .as_object()
            .map(|map| {
                map_len = map.len();
                map.iter()
            })
            .ok_or_else(|| GraphSonError::WrongJsonType("object".to_string()))?;

        let mut map = HashMap::with_capacity(map_len);
        for (k, v) in iter {
            let key = K::decode_v2(&serde_json::Value::String(k.clone()))?;
            let val = V::decode_v2(v)?;
            map.insert(key, val);
        }

        Ok(map)
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let mut map_len = 0;
        let iter = j_val
            .as_object()
            .map(|map| {
                map_len = map.len();
                map.iter()
            })
            .ok_or_else(|| GraphSonError::WrongJsonType("object".to_string()))?;

        let mut map = HashMap::with_capacity(map_len);
        for (k, v) in iter {
            let key = K::decode_v1(&serde_json::Value::String(k.clone()))?;
            let val = V::decode_v1(v)?;
            map.insert(key, val);
        }

        Ok(map)
    }
}

#[test]
fn testing_map() {
    let mut map = HashMap::new();

    map.insert(MapKeys::Int(1), GremlinValue::String("test".to_owned()));

    let mut buf: Vec<u8> = vec![];
    map.encode(&mut buf).unwrap();

    let msg = [
        0x0a, 0x0, 0x0, 0x0, 0x0, 0x1, 0x01, 0x0, 0x0, 0x0, 0x0, 0x1, 0x3, 0x0, 0x0, 0x0, 0x0, 0x4,
        0x74, 0x65, 0x73, 0x74,
    ];
    assert_eq!(msg[..], buf);
}
#[test]
fn testing_nestet_map() {
    let mut map = HashMap::new();
    let mut inner_map = HashMap::new();

    inner_map.insert(MapKeys::Int(1), GremlinValue::String("test".to_owned()));
    map.insert(MapKeys::Int(1), GremlinValue::Map(inner_map));

    let mut buf: Vec<u8> = vec![];
    map.encode(&mut buf).unwrap();

    let msg = [
        0x0a, 0x0, 0x0, 0x0, 0x0, 0x1, 0x01, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0a, 0x0, 0x0, 0x0, 0x0,
        0x1, 0x01, 0x0, 0x0, 0x0, 0x0, 0x1, 0x3, 0x0, 0x0, 0x0, 0x0, 0x4, b't', b'e', b's', b't',
    ];
    assert_eq!(msg[..], buf);
}

#[test]
fn testing_encode_hash_map() {
    let mut map = HashMap::new();

    map.insert(1, "test".to_owned());

    let mut buf: Vec<u8> = vec![];
    map.encode(&mut buf).unwrap();

    let msg = [
        0x0a, 0x0, 0x0, 0x0, 0x0, 0x1, 0x01, 0x0, 0x0, 0x0, 0x0, 0x1, 0x3, 0x0, 0x0, 0x0, 0x0, 0x4,
        0x74, 0x65, 0x73, 0x74,
    ];
    assert_eq!(msg[..], buf);
}

#[test]
fn testing_decode_hash_map() {
    let mut map = HashMap::new();

    map.insert(1, "test".to_owned());

    let msg = [
        0x0a, 0x0, 0x0, 0x0, 0x0, 0x1, 0x01, 0x0, 0x0, 0x0, 0x0, 0x1, 0x3, 0x0, 0x0, 0x0, 0x0, 0x4,
        0x74, 0x65, 0x73, 0x74,
    ];
    assert_eq!(map, HashMap::<i32, String>::decode(&mut &msg[..]).unwrap());
}

#[test]
fn map_decode_graphson_v3() {
    let str = r#"{"@type":"g:Map","@value":["dur",{"@type":"g:Double","@value":1.0},"less",{
            "@type" : "g:Double",
            "@value" : 0.0
          } ]
        }
        "#;

    let s = serde_json::from_str(str).unwrap();
    let s: HashMap<String, f64> = HashMap::decode_v3(&s).unwrap();
    let mut map = HashMap::new();
    map.insert("dur".to_string(), 1.0);
    map.insert("less".to_string(), 0.0);
    assert_eq!(s, map);
}

#[test]
fn map_encode_graphson_v3() {
    let str = r#"{"@type":"g:Map","@value":["dur",{"@type":"g:Double","@value":1.0}]}"#;

    let mut map = HashMap::new();
    map.insert("dur", 1.0);
    let val = map.encode_v3();
    let val = serde_json::to_string(&val).unwrap();

    assert_eq!(str, val);
}

#[test]
fn empty_map_decode_graphson_v3() {
    let str = r#"{
        "@type" : "g:Map",
        "@value" : []
      }"#;

    let s = serde_json::from_str(str).unwrap();
    let s: HashMap<String, i32> = HashMap::decode_v3(&s).unwrap();

    assert_eq!(s, HashMap::new());
}

#[test]
fn map_empty_encode_graphson_v3() {
    let str = r#"{"@type":"g:Map","@value":[]}"#;

    let v: HashMap<&str, i32> = HashMap::new();
    let val = v.encode_v3();
    let val = serde_json::to_string(&val).unwrap();

    assert_eq!(str, val);
}

#[test]
fn map_decode_graphson_v3_error() {
    let str = r#"{
        "@type" : "g:Map",
        "@value" : [ {
          "@type" : "g:Int32",
          "@value" : 1
        }, {
            "@type" : "g:Error",
            "@value" : 2
          }]
      }"#;

    let s = serde_json::from_str(str).unwrap();
    let s = HashMap::<i32, i32>::decode_v3(&s);
    assert!(s.is_err())
}

#[test]
fn map_decode_graphson_v2() {
    let str = r#"{ 
        "dur" :{
          "@type" : "g:Int32",
          "@value" : 1
        }, 
        "test": {
            "@type" : "g:Int32",
            "@value" : 2
        }
    }"#;

    let s = serde_json::from_str(str).unwrap();
    let s: HashMap<String, i32> = HashMap::decode_v2(&s).unwrap();
    let mut map = HashMap::new();
    map.insert("dur".to_string(), 1);
    map.insert("test".to_string(), 2);
    println!("{s:?}");

    assert_eq!(s, map);
}

#[test]
fn map_decode_v2_gremlin_types() {
    let str = r#"{
        "dur" : {
          "@type" : "g:Double",
          "@value" : 100.0
        },
        "counts" : {
          "traverserCount" : {
            "@type" : "g:Int64",
            "@value" : 4
          },
          "elementCount" : {
            "@type" : "g:Int64",
            "@value" : 4
          }
        }}"#;

    let s = serde_json::from_str(str).unwrap();
    let s: HashMap<String, GremlinValue> = HashMap::decode_v2(&s).unwrap();
    let mut map = HashMap::new();
    map.insert("dur".to_string(), 100f64.into());
    map.insert(
        "counts".to_string(),
        HashMap::from([("elementCount", 4i64), ("traverserCount", 4i64)]).into(),
    );
    println!("{s:?}");

    assert_eq!(s, map);
}

#[test]
fn map_encode_graphson_v2() {
    let str = r#"{"dur":{"@type":"g:Int32","@value":1},"test":{"@type":"g:Int32","@value":2}}"#;

    let mut map = HashMap::new();
    map.insert("dur".to_string(), 1);
    map.insert("test".to_string(), 2);
    let val = map.encode_v2();
    let val = serde_json::to_string(&val).unwrap();

    assert_eq!(str, val);
}

#[test]
fn empty_map_decode_graphson_v2() {
    let str = r#"{}"#;

    let s = serde_json::from_str(str).unwrap();
    let s: HashMap<i32, i32> = HashMap::decode_v2(&s).unwrap();

    assert_eq!(s, HashMap::new());
}

#[test]
fn empty_map_encode_graphson_v2() {
    let str = r#"{}"#;

    let v: HashMap<i32, i32> = HashMap::new();
    let val = v.encode_v2();
    let val = serde_json::to_string(&val).unwrap();

    assert_eq!(str, val);
}

#[test]
fn map_decode_graphson_v1() {
    let str = r#"{"dur":1,"test":2}"#;

    let s = serde_json::from_str(str).unwrap();
    let s: HashMap<_, _> = HashMap::decode_v1(&s).unwrap();
    let mut map = HashMap::new();
    map.insert("dur".to_string(), 1);
    map.insert("test".to_string(), 2);
    assert_eq!(s, map);
}

#[test]
fn map_encode_graphson_v1() {
    let str = r#"{"dur":1,"test":2}"#;

    let mut map = HashMap::new();
    map.insert("dur".to_string(), 1);
    map.insert("test".to_string(), 2);
    let val = map.encode_v1();
    let val = serde_json::to_string(&val).unwrap();

    assert_eq!(str, val);
}

#[test]
fn empty_map_decode_graphson_v1() {
    let str = r#"{}"#;

    let s = serde_json::from_str(str).unwrap();
    let s: HashMap<i32, i32> = HashMap::decode_v1(&s).unwrap();

    assert_eq!(s, HashMap::new());
}

#[test]
fn empty_vec_encode_graphson_v1() {
    let str = r#"{}"#;

    let v: HashMap<String, i32> = HashMap::new();
    let val = v.encode_v1();
    let val = serde_json::to_string(&val).unwrap();

    assert_eq!(str, val);
}
