use serde_json::json;

use super::validate_type_entry;
use crate::{
    error::{DecodeError, EncodeError},
    graph_binary::{Decode, Encode, GremlinTypes, MapKeys},
    graphson::{DecodeGraphSON, EncodeGraphSON},
    specs::CoreType,
};
use std::{
    collections::HashMap,
    hash::{BuildHasher, Hash},
};
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

    fn get_partial_len(bytes: &[u8]) -> Result<usize, crate::error::DecodeError> {
        let t: [u8; 4] = bytes[0..4].try_into()?;
        let size = i32::from_be_bytes(t) as usize;
        let mut len = 4;
        for _ in 0..size {
            len += K::get_len(&bytes[len..])?;
            len += V::get_len(&bytes[len..])?;
        }
        Ok(len)
    }
}

impl<K: Into<MapKeys>, V: Into<GremlinTypes>> From<HashMap<K, V>> for GremlinTypes {
    fn from(m: HashMap<K, V>) -> Self {
        let map = m.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
        GremlinTypes::Map(map)
    }
}

impl<K, V> TryFrom<GremlinTypes> for HashMap<K, V>
where
    K: TryFrom<MapKeys, Error = DecodeError> + Eq + Hash,
    V: TryFrom<GremlinTypes, Error = DecodeError>,
{
    type Error = DecodeError;

    fn try_from(value: GremlinTypes) -> Result<Self, Self::Error> {
        match value {
            GremlinTypes::Map(map) => {
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

impl<K> TryFrom<GremlinTypes> for HashMap<K, GremlinTypes>
where
    K: TryFrom<MapKeys, Error = DecodeError> + Eq + Hash,
{
    type Error = DecodeError;

    fn try_from(value: GremlinTypes) -> Result<Self, Self::Error> {
        match value {
            GremlinTypes::Map(map) => {
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

impl<K, V> DecodeGraphSON for HashMap<K, V>
where
    K: DecodeGraphSON + ToString + std::cmp::Eq + std::hash::Hash,
    V: DecodeGraphSON,
{
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let mut map_len = 0;
        let iter = j_val
            .as_object()
            .filter(|map| validate_type_entry(*map, "g:Map"))
            .and_then(|map| map.get("@value"))
            .and_then(|val| val.as_array())
            .map(|array| {
                map_len = array.len() / 2;
                array.iter()
            })
            .ok_or_else(|| DecodeError::DecodeError("json error Map v3 in error".to_string()))?;

        let mut map = HashMap::with_capacity(map_len);
        for (k, v) in iter.clone().zip(iter.skip(1)).step_by(2) {
            let key = K::decode_v3(k)?;
            let val = V::decode_v3(v)?;
            map.insert(key, val);
        }

        Ok(map)
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, crate::error::DecodeError>
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
            .ok_or_else(|| DecodeError::DecodeError("json error Map v2 in error".to_string()))?;

        let mut map = HashMap::with_capacity(map_len);
        for (k, v) in iter {
            let key = K::decode_v2(&serde_json::Value::String(k.clone()))?;
            let val = V::decode_v2(v)?;
            map.insert(key, val);
        }

        Ok(map)
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, crate::error::DecodeError>
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
            .ok_or_else(|| DecodeError::DecodeError("json error Map v1 in error".to_string()))?;

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
    use crate::graph_binary::{GremlinTypes, MapKeys};

    let mut map = HashMap::new();

    map.insert(MapKeys::Int(1), GremlinTypes::String("test".to_owned()));

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
    use crate::graph_binary::{GremlinTypes, MapKeys};

    let mut map = HashMap::new();
    let mut inner_map = HashMap::new();

    inner_map.insert(MapKeys::Int(1), GremlinTypes::String("test".to_owned()));
    map.insert(MapKeys::Int(1), GremlinTypes::Map(inner_map));

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
    map.insert("dur".to_string(), 1.into());
    map.insert("test".to_string(), 2.into());
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
    let s: HashMap<String, GremlinTypes> = HashMap::decode_v2(&s).unwrap();
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
