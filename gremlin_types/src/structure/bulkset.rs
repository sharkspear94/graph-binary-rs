use std::fmt::Display;

use crate::{error::DecodeError, specs::CoreType, GremlinValue};

#[cfg(feature = "graph_binary")]
use crate::graph_binary::{Decode, Encode};

#[cfg(feature = "graph_son")]
use crate::graphson::{validate_type_entry, DecodeGraphSON, EncodeGraphSON};
#[cfg(feature = "graph_son")]
use serde_json::json;

#[derive(Debug, PartialEq, Clone)]
pub struct BulkSet(Vec<(GremlinValue, i64)>);

#[cfg(feature = "graph_binary")]
impl Encode for BulkSet {
    fn type_code() -> u8 {
        CoreType::BulkSet.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        let vec_len = self.0.len() as i32;
        vec_len.partial_encode(writer)?;
        for (gb, bulk) in &self.0 {
            gb.encode(writer)?;
            bulk.partial_encode(writer)?;
        }
        Ok(())
    }
}

#[cfg(feature = "graph_binary")]
impl Decode for BulkSet {
    fn expected_type_code() -> u8 {
        CoreType::BulkSet.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let len = i32::partial_decode(reader)?;
        let len = usize::try_from(len)?;
        let mut items = Vec::with_capacity(len);
        for _ in 0..len {
            let gb = GremlinValue::decode(reader)?;
            let bulk = i64::partial_decode(reader)?;
            items.push((gb, bulk));
        }
        Ok(BulkSet(items))
    }
}

#[cfg(feature = "graph_son")]
impl EncodeGraphSON for BulkSet {
    fn encode_v3(&self) -> serde_json::Value {
        let mut j_vec = Vec::with_capacity(self.0.len() * 2);
        for (value, bulk) in &self.0 {
            j_vec.push(value.encode_v3());
            j_vec.push(bulk.encode_v3())
        }

        json!(
            {
                "@type" : "g:BulkSet",
                "@value" : j_vec
            }
        )
    }

    fn encode_v2(&self) -> serde_json::Value {
        unimplemented!("not supported in GraphSON V2")
    }

    fn encode_v1(&self) -> serde_json::Value {
        unimplemented!("not supported in GraphSON V1")
    }
}

#[cfg(feature = "graph_son")]
impl DecodeGraphSON for BulkSet {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let value_object = j_val
            .as_object()
            .filter(|map| validate_type_entry(*map, "g:BulkSet"))
            .and_then(|m| m.get("@value"))
            .and_then(|m| m.as_array())
            .ok_or_else(|| DecodeError::DecodeError("".to_string()))?;

        let mut bulk_set = Vec::with_capacity(value_object.len() / 2);
        for (value, bulk) in value_object
            .iter()
            .zip(value_object.iter().skip(1))
            .step_by(2)
        {
            let value = GremlinValue::decode_v3(value)?;
            let bulk = i64::decode_v3(bulk)?;
            bulk_set.push((value, bulk));
        }
        Ok(BulkSet(bulk_set))
    }

    fn decode_v2(_j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        unimplemented!("BulkSet in not supported in GraphSON V2")
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        unimplemented!("not supported in GraphSON V1")
    }
}

impl Display for BulkSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (val, bulk) in &self.0 {
            write!(f, "bulk: {bulk},value: {val}",)?;
        }
        write!(f, "]")
    }
}

#[test]
fn encode_v3() {
    let expected = r#"{"@type":"g:BulkSet","@value":["marko",{"@type":"g:Int64","@value":1},"josh",{"@type":"g:Int64","@value":2}]}"#;

    let bulk_set = BulkSet(vec![("marko".into(), 1), ("josh".into(), 2)]).encode_v3();

    let res = serde_json::to_string(&bulk_set).unwrap();

    assert_eq!(res, expected)
}

#[test]
fn decode_v3() {
    let s = r#"{"@type":"g:BulkSet","@value":["marko",{"@type":"g:Int64","@value":1},"josh",{"@type":"g:Int64","@value":2}]}"#;

    let expected = BulkSet(vec![("marko".into(), 1), ("josh".into(), 2)]);

    let v = serde_json::from_str(s).unwrap();
    let res = BulkSet::decode_v3(&v).unwrap();
    assert_eq!(res, expected)
}
