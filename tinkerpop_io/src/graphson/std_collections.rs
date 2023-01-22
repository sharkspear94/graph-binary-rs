use std::collections::HashMap;

use serde_json::json;

use crate::error::GraphSonError;

use super::{validate_type, DecodeGraphSON, EncodeGraphSON};

impl<T: EncodeGraphSON> EncodeGraphSON for Vec<T> {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
            "@type" : "g:List",
            "@value" : self.iter().map(|t| t.encode_v3()).collect::<Vec<serde_json::Value>>(),
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        json!(self
            .iter()
            .map(|t| t.encode_v2())
            .collect::<Vec<serde_json::Value>>())
    }

    fn encode_v1(&self) -> serde_json::Value {
        json!(self
            .iter()
            .map(|t| t.encode_v1())
            .collect::<Vec<serde_json::Value>>())
    }
}

impl<T: DecodeGraphSON> DecodeGraphSON for Vec<T> {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:List")?;

        value_object
            .as_array()
            .ok_or_else(|| GraphSonError::WrongJsonType("array".to_string()))?
            .iter()
            .map(|v| T::decode_v3(v))
            .collect::<Result<Vec<_>, _>>()
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        j_val
            .as_array()
            .ok_or_else(|| GraphSonError::WrongJsonType("array".to_string()))?
            .iter()
            .map(|v| T::decode_v2(v))
            .collect::<Result<Vec<_>, _>>()
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        j_val
            .as_array()
            .ok_or_else(|| GraphSonError::WrongJsonType("array".to_string()))?
            .iter()
            .map(|v| T::decode_v1(v))
            .collect::<Result<Vec<_>, _>>()
    }
}

impl<T: EncodeGraphSON> EncodeGraphSON for &[T] {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
            "@type" : "g:List",
            "@value" : self.iter().map(|t| t.encode_v3()).collect::<Vec<serde_json::Value>>(),
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        json!(self
            .iter()
            .map(|t| t.encode_v2())
            .collect::<Vec<serde_json::Value>>())
    }

    fn encode_v1(&self) -> serde_json::Value {
        json!(self
            .iter()
            .map(|t| t.encode_v1())
            .collect::<Vec<serde_json::Value>>())
    }
}

impl<T: EncodeGraphSON, const N: usize> EncodeGraphSON for [T; N] {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
            "@type" : "g:List",
            "@value" : self.iter().map(|t| t.encode_v3()).collect::<Vec<serde_json::Value>>(),
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        json!(self
            .iter()
            .map(|t| t.encode_v2())
            .collect::<Vec<serde_json::Value>>())
    }

    fn encode_v1(&self) -> serde_json::Value {
        json!(self
            .iter()
            .map(|t| t.encode_v1())
            .collect::<Vec<serde_json::Value>>())
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
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:Map")?;

        let mut map_len = 0;
        let k_v_pairs = value_object
            .as_array()
            .map(|array| {
                map_len = array.len() / 2;
                array
            })
            .ok_or_else(|| GraphSonError::WrongJsonType("array".to_string()))?;

        let mut map = HashMap::with_capacity(map_len);
        for chunk in k_v_pairs.chunks_exact(2) {
            let key = K::decode_v3(&chunk[0])?;
            let val = V::decode_v3(&chunk[1])?;
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
fn vec_decode_graphson_v3() {
    let str = r#"{
        "@type" : "g:List",
        "@value" : [ {
          "@type" : "g:Int32",
          "@value" : 1
        }, {
            "@type" : "g:Int32",
            "@value" : 2
          }]
      }"#;

    let s = serde_json::from_str(str).unwrap();
    let s: Vec<i32> = Vec::decode_v3(&s).unwrap();
    println!("{s:?}");

    assert_eq!(s, vec![1, 2]);
}

#[test]
fn vec_encode_graphson_v3() {
    let str = r#"{"@type":"g:List","@value":[{"@type":"g:Int32","@value":1},{"@type":"g:Int32","@value":2}]}"#;

    let v = vec![1, 2];
    let val = v.encode_v3();
    let val = serde_json::to_string(&val).unwrap();

    assert_eq!(str, val);
}

#[test]
fn empty_vec_decode_graphson_v3() {
    let str = r#"{
        "@type" : "g:List",
        "@value" : []
      }"#;

    let s = serde_json::from_str(str).unwrap();
    let s: Vec<i32> = Vec::decode_v3(&s).unwrap();

    assert_eq!(s, Vec::<i32>::new());
}

#[test]
fn vec_empty_encode_graphson_v3() {
    let str = r#"{"@type":"g:List","@value":[]}"#;

    let v: Vec<i32> = vec![];
    let val = v.encode_v3();
    let val = serde_json::to_string(&val).unwrap();

    assert_eq!(str, val);
}

#[test]
fn vec_decode_graphson_v3_error() {
    let str = r#"{
        "@type" : "g:List",
        "@value" : [ {
          "@type" : "g:Int32",
          "@value" : 1
        }, {
            "@type" : "g:Error",
            "@value" : 2
          }]
      }"#;

    let s = serde_json::from_str(str).unwrap();
    let s = Vec::<i32>::decode_v3(&s);
    assert!(s.is_err())
}

#[test]
fn vec_decode_graphson_v2() {
    let str = r#"[ {
          "@type" : "g:Int32",
          "@value" : 1
        }, {
            "@type" : "g:Int32",
            "@value" : 2
          }]"#;

    let s = serde_json::from_str(str).unwrap();
    let s: Vec<i32> = Vec::decode_v2(&s).unwrap();
    println!("{s:?}");

    assert_eq!(s, vec![1, 2]);
}

#[test]
fn vec_encode_graphson_v2() {
    let str = r#"[{"@type":"g:Int32","@value":1},{"@type":"g:Int32","@value":2}]"#;

    let v = vec![1, 2];
    let val = v.encode_v2();
    let val = serde_json::to_string(&val).unwrap();

    assert_eq!(str, val);
}

#[test]
fn empty_vec_decode_graphson_v2() {
    let str = r#"[]"#;

    let s = serde_json::from_str(str).unwrap();
    let s: Vec<i32> = Vec::decode_v2(&s).unwrap();

    assert_eq!(s, Vec::<i32>::new());
}

#[test]
fn empty_vec_encode_graphson_v2() {
    let str = r#"[]"#;

    let v: Vec<i32> = vec![];
    let val = v.encode_v2();
    let val = serde_json::to_string(&val).unwrap();

    assert_eq!(str, val);
}

#[test]
fn vec_decode_graphson_v1() {
    let str = r#"[1,2]"#;

    let s = serde_json::from_str(str).unwrap();
    let s: Vec<i32> = Vec::decode_v1(&s).unwrap();
    println!("{:?}", json!('a'));

    assert_eq!(s, vec![1, 2]);
}

#[test]
fn vec_encode_graphson_v1() {
    let str = r#"[1,2]"#;

    let v = vec![1, 2];
    let val = v.encode_v1();
    let val = serde_json::to_string(&val).unwrap();

    assert_eq!(str, val);
}

#[test]
fn empty_vec_decode_graphson_v1() {
    let str = r#"[]"#;

    let s = serde_json::from_str(str).unwrap();
    let s: Vec<i32> = Vec::decode_v2(&s).unwrap();

    assert_eq!(s, Vec::<i32>::new());
}

#[test]
fn empty_vec_encode_graphson_v1() {
    let str = r#"[]"#;

    let v: Vec<i32> = vec![];
    let val = v.encode_v2();
    let val = serde_json::to_string(&val).unwrap();

    assert_eq!(str, val);
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
fn map_decode_v2_tinkterpop_io_rs() {
    use crate::GremlinValue;
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
fn empty_map_encode_graphson_v1() {
    let str = r#"{}"#;

    let v: HashMap<String, i32> = HashMap::new();
    let val = v.encode_v1();
    let val = serde_json::to_string(&val).unwrap();

    assert_eq!(str, val);
}
