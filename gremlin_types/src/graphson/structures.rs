use std::collections::HashMap;
use std::str::FromStr;

use bigdecimal::BigDecimal;
use num::BigInt;

use crate::error::GraphSonError;
use crate::graphson::{validate_type, DecodeGraphSON, EncodeGraphSON};
use crate::structure::bulkset::BulkSet;

use crate::structure::bytebuffer::ByteBuffer;
use crate::structure::bytecode::{Bytecode, Source, Step};
use crate::structure::edge::Edge;
use crate::structure::graph::{Graph, GraphEdge};
use crate::structure::lambda::Lambda;
use crate::structure::map::MapKeys;
use crate::structure::metrics::{Metrics, TraversalMetrics};
use crate::structure::path::Path;
use crate::structure::property::{self, EitherParent, Property};
use crate::structure::set::Set;
use crate::structure::traverser::Traverser;
use crate::structure::vertex::Vertex;
use crate::structure::vertex_property::VertexProperty;
use crate::{Binding, GremlinValue};

use serde_json::{json, Map};

use super::{get_val_by_key_v1, get_val_by_key_v2, get_val_by_key_v3};

impl EncodeGraphSON for BigInt {
    fn encode_v3(&self) -> serde_json::Value {
        let num =
            serde_json::Value::Number(serde_json::Number::from_str(&self.to_string()).unwrap());
        json!({
          "@type" : "gx:BigInteger",
          "@value" : num
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        self.encode_v3()
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

impl DecodeGraphSON for BigInt {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "gx:BigInteger")?;
        match value_object {
            serde_json::Value::Number(val) => BigInt::from_str(&val.to_string())
                .map_err(|err| GraphSonError::Parse(format!("cannot parse BigInt: {err}"))),
            _ => Err(GraphSonError::WrongJsonType("number".to_string())),
        }
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

impl EncodeGraphSON for BigDecimal {
    fn encode_v3(&self) -> serde_json::Value {
        let num =
            serde_json::Value::Number(serde_json::Number::from_str(&self.to_string()).unwrap());
        json!({
          "@type" : "gx:BigDecimal",
          "@value" : num
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        self.encode_v3()
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

impl DecodeGraphSON for BigDecimal {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "gx:BigDecimal")?;
        match value_object {
            serde_json::Value::Number(val) => BigDecimal::from_str(&val.to_string())
                .map_err(|err| GraphSonError::Parse(format!("cannot parse BigDecimal: {err}"))),
            _ => Err(GraphSonError::WrongJsonType("number".to_string())),
        }
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

impl EncodeGraphSON for Binding {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
          "@type" : "g:Binding",
          "@value" : {
            "key" : self.key,
            "value" : self.value.encode_v3()
          }
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        json!({
          "@type" : "g:Binding",
          "@value" : {
            "key" : self.key,
            "value" : self.value.encode_v2()
          }
        })
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

impl DecodeGraphSON for Binding {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:Binding")?;

        let key = get_val_by_key_v3(value_object, "key", "Binding")?;
        let value = get_val_by_key_v3(value_object, "value", "Binding")?;

        Ok(Binding {
            key,
            value: Box::new(value),
        })
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

impl EncodeGraphSON for BulkSet {
    fn encode_v3(&self) -> serde_json::Value {
        let mut j_vec = Vec::with_capacity(self.0.len() * 2);
        for (value, bulk) in &self.0 {
            j_vec.push(value.encode_v3());
            j_vec.push(bulk.encode_v3());
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

impl DecodeGraphSON for BulkSet {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:BulkSet")?
            .as_array()
            .ok_or_else(|| GraphSonError::WrongJsonType("array".to_string()))?;

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

    fn decode_v2(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        unimplemented!("BulkSet in not supported in GraphSON V2")
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        unimplemented!("not supported in GraphSON V1")
    }
}

//TODO impl sources in encoding Bytecode
impl EncodeGraphSON for Bytecode {
    fn encode_v3(&self) -> serde_json::Value {
        let v: Vec<Vec<serde_json::Value>> = self
            .steps
            .iter()
            .map(|s| {
                let mut inner = vec![s.name.encode_v3()];
                inner.extend(s.values.iter().map(EncodeGraphSON::encode_v3));
                inner
            })
            .collect();
        json!({
          "@type" : "g:Bytecode",
          "@value" : {
            "step" : v
          }
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        self.encode_v3()
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

impl DecodeGraphSON for Bytecode {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let mut steps = Vec::<Step>::new();
        let mut sources = Vec::<Source>::new();

        let value_object = validate_type(j_val, "g:Bytecode")?;

        let steps_iter = value_object.get("step").and_then(|v| v.as_array());

        if let Some(iter) = steps_iter {
            for inner in iter.iter().flat_map(|v| v.as_array()) {
                let mut step_args = Vec::<GremlinValue>::new();
                let name = inner
                    .first()
                    .and_then(|v| String::decode_v3(v).ok())
                    .ok_or_else(|| GraphSonError::KeyNotFound("first".to_string()))?;
                for i in &inner[1..] {
                    step_args.push(GremlinValue::decode_v3(i)?);
                }
                steps.push(Step {
                    name,
                    values: step_args,
                });
            }
        };

        let source_iter = value_object.get("source").and_then(|v| v.as_array());

        if let Some(iter) = source_iter {
            for inner in iter.iter().filter_map(|v| v.as_array()) {
                let mut source_args = Vec::<GremlinValue>::new();
                let name = inner
                    .first()
                    .and_then(|v| String::decode_v3(v).ok())
                    .ok_or_else(|| GraphSonError::KeyNotFound("first".to_string()))?;
                for i in &inner[1..] {
                    source_args.push(GremlinValue::decode_v3(i)?);
                }
                sources.push(Source {
                    name,
                    values: source_args,
                });
            }
        };
        Ok(Bytecode { steps, sources })
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

impl EncodeGraphSON for Edge {
    fn encode_v3(&self) -> serde_json::Value {
        let properties_map = self.properties.as_ref().map(|vec| {
            vec.iter()
                .map(|prop| (prop.key.clone(), prop.encode_v3()))
                .collect::<Map<String, serde_json::Value>>()
        });
        // needs testing

        let mut json_value = json!({
          "@type" : "g:Edge",
          "@value" : {
            "id" : self.id.encode_v3(),
            "label" : self.label,
            "inVLabel" : self.in_v_label,
            "outVLabel" : self.out_v_label,
            "inV" : self.in_v_id.encode_v3(),
            "outV" : self.out_v_id.encode_v3(),
          }
        });
        if let Some(properties_map) = properties_map {
            json_value["@value"]
                .as_object_mut()
                .unwrap()
                .insert("properties".to_string(), json! {properties_map});
        }
        json_value
    }

    fn encode_v2(&self) -> serde_json::Value {
        let properties_map = self.properties.as_ref().map(|vec| {
            vec.iter()
                .map(|prop| (prop.key.clone(), prop.value.encode_v2()))
                .collect::<Map<String, serde_json::Value>>()
        });

        let mut json_value = json!({
          "@type" : "g:Edge",
          "@value" : {
            "id" : self.id.encode_v2(),
            "label" : self.label,
            "inVLabel" : self.in_v_label,
            "outVLabel" : self.out_v_label,
            "inV" : self.in_v_id.encode_v2(),
            "outV" : self.out_v_id.encode_v2(),
          }
        });
        if let Some(properties_map) = properties_map {
            json_value["@value"]
                .as_object_mut()
                .unwrap()
                .insert("properties".to_string(), json! {properties_map});
        }
        json_value
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

impl DecodeGraphSON for Edge {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:Edge")?;

        let id = get_val_by_key_v3(value_object, "id", "Edge")?;
        let label = get_val_by_key_v3(value_object, "label", "Edge")?;
        let in_v_id = get_val_by_key_v3(value_object, "inV", "Edge")?;
        let in_v_label = get_val_by_key_v3(value_object, "inVLabel", "Edge")?;
        let out_v_id = get_val_by_key_v3(value_object, "outV", "Edge")?;
        let out_v_label = get_val_by_key_v3(value_object, "outVLabel", "Edge")?;

        let properties = value_object
            .get("properties")
            .and_then(|map| map.as_object())
            .map(|map| {
                map.values()
                    .map(Property::decode_v3)
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;

        Ok(Edge {
            id: Box::new(id),
            label,
            in_v_id: Box::new(in_v_id),
            in_v_label,
            out_v_id: Box::new(out_v_id),
            out_v_label,
            parent: None,
            properties,
        })
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:Edge")?;

        let id = get_val_by_key_v2(value_object, "id", "Edge")?;
        let label = get_val_by_key_v2(value_object, "label", "Edge")?;
        let in_v_id = get_val_by_key_v2(value_object, "inV", "Edge")?;
        let in_v_label = get_val_by_key_v2(value_object, "inVLabel", "Edge")?;
        let out_v_id = get_val_by_key_v2(value_object, "outV", "Edge")?;
        let out_v_label = get_val_by_key_v2(value_object, "outVLabel", "Edge")?;

        let properties = value_object
            .get("properties")
            .and_then(|map| map.as_object())
            .map(|map| {
                map.iter()
                    .map(|(k, v)| {
                        GremlinValue::decode_v2(v)
                            .map(|g| Property::new(k, g, property::EitherParent::None))
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;

        Ok(Edge {
            id: Box::new(id),
            label,
            in_v_id: Box::new(in_v_id),
            in_v_label,
            out_v_id: Box::new(out_v_id),
            out_v_label,
            parent: None,
            properties,
        })
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

impl EncodeGraphSON for ByteBuffer {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
          "@type" : "gx:ByteBuffer",
          "@value" : self.0.iter().map(|byte| *byte as char).collect::<String>()
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        self.encode_v3()
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

impl DecodeGraphSON for ByteBuffer {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let inner = validate_type(j_val, "gx:ByteBuffer")?
            .as_str()
            .map(|s| s.chars().map(|c| c as u8).collect::<Vec<u8>>())
            .ok_or_else(|| GraphSonError::WrongJsonType("str".to_string()))?;
        Ok(ByteBuffer(inner))
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Self::decode_v3(j_val)
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

impl EncodeGraphSON for GraphEdge {
    fn encode_v3(&self) -> serde_json::Value {
        let properties_map = self
            .properties
            .iter()
            .map(|prop| (prop.key.clone(), prop.encode_v3()))
            .collect::<Map<String, serde_json::Value>>();

        let mut json_value = json!({
          "@type" : "g:Edge",
          "@value" : {
            "id" : self.id.encode_v3(),
            "label" : self.label.encode_v3(),
            "inVLabel" : self.in_v_label.encode_v3(),
            "outVLabel" : self.out_v_label.encode_v3(),
            "inV" : self.in_v_id.encode_v3(),
            "outV" : self.out_v_id.encode_v3(),
          }
        });
        if !properties_map.is_empty() {
            json_value["@value"]
                .as_object_mut()
                .unwrap()
                .insert("properties".to_string(), json! {properties_map});
        }
        json_value
    }

    fn encode_v2(&self) -> serde_json::Value {
        let properties_map = self
            .properties
            .iter()
            .map(|prop| (prop.key.clone(), prop.encode_v2()))
            .collect::<Map<String, serde_json::Value>>();

        let mut json_value = json!({
          "@type" : "g:Edge",
          "@value" : {
            "id" : self.id.encode_v2(),
            "label" : self.label.encode_v2(),
            "inVLabel" : self.in_v_label.encode_v2(),
            "outVLabel" : self.out_v_label.encode_v2(),
            "inV" : self.in_v_id.encode_v2(),
            "outV" : self.out_v_id.encode_v2(),
          }
        });
        if !properties_map.is_empty() {
            json_value["@value"]
                .as_object_mut()
                .unwrap()
                .insert("properties".to_string(), json! {properties_map});
        }
        json_value
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

impl DecodeGraphSON for GraphEdge {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Ok(Edge::decode_v3(j_val)?.into())
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Ok(Edge::decode_v2(j_val)?.into())
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

impl EncodeGraphSON for Graph {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
            "@type" : "tinker:graph",
            "@value" : {
                "vertices": self.vertices.iter().map(EncodeGraphSON::encode_v3).collect::<Vec<serde_json::Value>>(),
                "edges": self.edges.iter().map(EncodeGraphSON::encode_v3).collect::<Vec<serde_json::Value>>()
             }
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        json!({
            "@type" : "tinker:graph",
            "@value" : {
                "vertices": self.vertices.iter().map(EncodeGraphSON::encode_v2).collect::<Vec<serde_json::Value>>(),
                "edges": self.edges.iter().map(EncodeGraphSON::encode_v2).collect::<Vec<serde_json::Value>>()
             }
        })
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

impl DecodeGraphSON for Graph {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:TinkerGraph")?;

        let vertices = get_val_by_key_v3(value_object, "vertices", "TinkerGraph")?;
        let edges = get_val_by_key_v3(value_object, "edges", "TinkerGraph")?;

        Ok(Graph { vertices, edges })
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:TinkerGraph")?;

        let vertices = get_val_by_key_v2(value_object, "vertices", "TinkerGraph")?;
        let edges = get_val_by_key_v2(value_object, "edges", "TinkerGraph")?;

        Ok(Graph { vertices, edges })
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

impl EncodeGraphSON for Lambda {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
          "@type" : "g:Lambda",
          "@value" : {
            "script" : self.script,
            "language" : self.language,
            "arguments" : self.arguments_length // TODO test on server if io Doc is right
          }
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        self.encode_v3()
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

impl DecodeGraphSON for Lambda {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:Lambda")?;

        let script = get_val_by_key_v3(value_object, "script", "Lambda")?;
        let language = get_val_by_key_v3(value_object, "language", "Lambda")?;

        let arguments_length = value_object
            .get("arguments")
            .ok_or_else(|| GraphSonError::KeyNotFound("arguments".to_string()))?
            .as_i64()
            .ok_or_else(|| GraphSonError::WrongJsonType("i64".to_string()))
            .map(|len| len as i32)?;
        Ok(Lambda {
            language,
            script,
            arguments_length,
        })
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Lambda::decode_v3(j_val)
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

impl<T: EncodeGraphSON> EncodeGraphSON for Set<T> {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
          "@type" : "g:Set",
          "@value" : self.set().iter().map(|t| t.encode_v3()).collect::<Vec<serde_json::Value>>()
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        json!(self
            .set()
            .iter()
            .map(|t| t.encode_v2())
            .collect::<Vec<serde_json::Value>>())
    }

    fn encode_v1(&self) -> serde_json::Value {
        json!(self
            .set()
            .iter()
            .map(|t| t.encode_v1())
            .collect::<Vec<serde_json::Value>>())
    }
}

impl<T: DecodeGraphSON> DecodeGraphSON for Set<T> {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:Set")?;

        let result_vec = value_object
            .as_array()
            .ok_or_else(|| GraphSonError::WrongJsonType("array".to_string()))?
            .iter()
            .map(|v| T::decode_v3(v))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Set::new(result_vec))
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        Ok(Set::new(Vec::<T>::decode_v2(j_val)?))
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

impl EncodeGraphSON for Metrics {
    fn encode_v3(&self) -> serde_json::Value {
        let dur = self.duration as f64 / 1000. / 1000.;
        if self.nested_metrics.is_empty() {
            json!({
                "@type" : "g:Metrics",
                "@value" : {
                    "@type" : "g:Map",
                    "@value" : [
                        "dur",dur.encode_v3(),
                        "counts",self.counts.encode_v3(),
                        "name",self.name.encode_v3(),
                        "annotations", self.annotations.encode_v3(),
                        "id",self.id.encode_v3(),
                    ]
            }
            })
        } else {
            json!({
                "@type" : "g:Metrics",
                "@value" : {
                    "@type" : "g:Map",
                    "@value" : [
                        "dur",dur.encode_v3(),
                        "counts",self.counts.encode_v3(),
                        "name",self.name.encode_v3(),
                        "annotations", self.annotations.encode_v3(),
                        "id",self.id.encode_v3(),
                        "metrics", self.nested_metrics.encode_v3()
                    ]
            }
            })
        }
    }

    fn encode_v2(&self) -> serde_json::Value {
        let dur = self.duration as f64 / 1000. / 1000.;
        if self.nested_metrics.is_empty() {
            json!({
                "@type" : "g:Metrics",
                "@value" : {

                        "dur":dur.encode_v2(),
                        "counts":self.counts.encode_v2(),
                        "name":self.name.encode_v2(),
                        "annotations":self.annotations.encode_v2(),
                        "id":self.id.encode_v2(),
            }
            })
        } else {
            json!({
                "@type" : "g:Metrics",
                "@value" : {
                    "dur":dur.encode_v2(),
                    "counts":self.counts.encode_v2(),
                    "name":self.name.encode_v2(),
                    "annotations": self.annotations.encode_v2(),
                    "id":self.id.encode_v2(),
                    "metrics":self.nested_metrics.encode_v2()
            }
            })
        }
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

impl DecodeGraphSON for Metrics {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:Metrics")?;

        let metrics = HashMap::<String, GremlinValue>::decode_v3(value_object)?;

        let duration = metrics
            .get("dur")
            .and_then(|v| v.get_cloned::<f64>())
            .map(|dur| (dur * 1000. * 1000.) as i64)
            .ok_or_else(|| GraphSonError::KeyNotFound("dur".to_string()))?;
        let counts = metrics
            .get("counts")
            .and_then(|v| v.get_cloned::<HashMap<String, i64>>())
            .ok_or_else(|| GraphSonError::KeyNotFound("counts".to_string()))?;
        let name = metrics
            .get("name")
            .and_then(|v| v.get_cloned::<String>())
            .ok_or_else(|| GraphSonError::KeyNotFound("name".to_string()))?;
        let annotations = metrics
            .get("annotations")
            .and_then(|v| v.get_cloned::<HashMap<String, GremlinValue>>())
            .ok_or_else(|| GraphSonError::KeyNotFound("annotation".to_string()))?;
        let id = metrics
            .get("id")
            .and_then(|v| v.get_cloned::<String>())
            .ok_or_else(|| GraphSonError::KeyNotFound("id".to_string()))?;

        if let Some(nested_metrics) = metrics
            .get("metrics")
            .and_then(|v| v.get_cloned::<Vec<Metrics>>())
        {
            Ok(Metrics {
                id,
                name,
                duration,
                counts,
                annotations,
                nested_metrics,
            })
        } else {
            Ok(Metrics {
                id,
                name,
                duration,
                counts,
                annotations,
                nested_metrics: vec![],
            })
        }
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:Metrics")?;

        let metrics = HashMap::<String, GremlinValue>::decode_v2(value_object)?;

        let duration = metrics
            .get("dur")
            .and_then(|v| v.get_cloned::<f64>())
            .map(|dur| (dur * 1000. * 1000.) as i64)
            .ok_or_else(|| GraphSonError::KeyNotFound("dur".to_string()))?;
        let counts = metrics
            .get("counts")
            .and_then(|v| v.get_cloned::<HashMap<String, i64>>())
            .ok_or_else(|| GraphSonError::KeyNotFound("counts".to_string()))?;
        let name = metrics
            .get("name")
            .and_then(|v| v.get_cloned::<String>())
            .ok_or_else(|| GraphSonError::KeyNotFound("name".to_string()))?;
        let annotations = metrics
            .get("annotations")
            .and_then(|v| v.get_cloned::<HashMap<String, GremlinValue>>())
            .ok_or_else(|| GraphSonError::KeyNotFound("annotations".to_string()))?;
        let id = metrics
            .get("id")
            .and_then(|v| v.get_cloned::<String>())
            .ok_or_else(|| GraphSonError::KeyNotFound("id".to_string()))?;

        if let Some(nested_metrics) = metrics
            .get("metrics")
            .and_then(|v| v.get_cloned::<Vec<Metrics>>())
        {
            Ok(Metrics {
                id,
                name,
                duration,
                counts,
                annotations,
                nested_metrics,
            })
        } else {
            Ok(Metrics {
                id,
                name,
                duration,
                counts,
                annotations,
                nested_metrics: vec![],
            })
        }
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

impl EncodeGraphSON for TraversalMetrics {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
            "@type" : "g:TraversalMetrics",
            "@value" : [
                "dur", self.duration.encode_v3(),
                "metrics", self.metrics.encode_v3()
            ]
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        json!({
            "@type" : "g:TraversalMetrics",
            "@value" : [
                "dur", self.duration.encode_v2(),
                "metrics", self.metrics.encode_v2()
            ]
        })
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

impl DecodeGraphSON for TraversalMetrics {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:TraversalMetrics")?;

        let metrics = HashMap::<String, GremlinValue>::decode_v3(value_object)?;

        let duration = metrics
            .get("dur")
            .and_then(|v| v.get_cloned::<f64>())
            .map(|dur| (dur * 1000. * 1000.) as i64)
            .ok_or_else(|| GraphSonError::KeyNotFound("dur".to_string()))?;
        let metrics = metrics
            .get("metrics")
            .and_then(|v| v.get_cloned::<Vec<Metrics>>())
            .ok_or_else(|| GraphSonError::KeyNotFound("metrics".to_string()))?;
        Ok(TraversalMetrics { duration, metrics })
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:Metrics")?;

        let metrics = HashMap::<String, GremlinValue>::decode_v2(value_object)?;

        let duration = metrics
            .get("dur")
            .and_then(|v| v.get_cloned::<f64>())
            .map(|dur| (dur * 1000. * 1000.) as i64)
            .ok_or_else(|| GraphSonError::KeyNotFound("du".to_string()))?;
        let metrics = metrics
            .get("metrics")
            .and_then(|v| v.get_cloned::<Vec<Metrics>>())
            .ok_or_else(|| GraphSonError::KeyNotFound("metrics".to_string()))?;
        Ok(TraversalMetrics { duration, metrics })
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

impl EncodeGraphSON for Path {
    fn encode_v3(&self) -> serde_json::Value {
        json!(
            {
                "@type" : "g:Path",
                "@value" : {
                  "labels" : self.labels.encode_v3(),
                  "objects" : self.objects.encode_v3()
                }
            }
        )
    }
    fn encode_v2(&self) -> serde_json::Value {
        json!(
            {
                "@type" : "g:Path",
                "@value" : {
                  "labels" : self.labels.encode_v2(),
                  "objects" : self.objects.encode_v2()
                }
            }
        )
    }

    fn encode_v1(&self) -> serde_json::Value {
        json!({
            "labels": self.labels.encode_v1(),
            "objects" : self.objects.encode_v1(),
        })
    }
}

impl DecodeGraphSON for Path {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:Path")?;

        let labels = get_val_by_key_v3(value_object, "labels", "Path")?;
        let objects = get_val_by_key_v3(value_object, "objects", "Path")?;

        Ok(Path { labels, objects })
    }
    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:Path")?;

        let labels = get_val_by_key_v2(value_object, "labels", "Path")?;
        let objects = get_val_by_key_v2(value_object, "objects", "Path")?;

        Ok(Path { labels, objects })
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

impl EncodeGraphSON for Vertex {
    fn encode_v3(&self) -> serde_json::Value {
        if let Some(properties) = &self.properties {
            let mut map = HashMap::<String, Vec<serde_json::Value>>::new();
            for property in properties {
                if map.contains_key(&property.label) {
                    let v = map.get_mut(&property.label).unwrap();
                    v.push(property.encode_v3());
                } else {
                    map.insert(property.label.clone(), vec![property.encode_v3()]);
                }
            }
            json!({
                  "@type" : "g:Vertex",
                  "@value" : {
                    "id" : self.id.encode_v3(),
                    "label" : self.label,
                    "properties" : map
            }})
        } else {
            json!({
                  "@type" : "g:Vertex",
                  "@value" : {
                    "id" : self.id.encode_v3(),
                    "label" : self.label,
            }})
        }
    }

    fn encode_v2(&self) -> serde_json::Value {
        if let Some(properties) = &self.properties {
            let mut map = HashMap::<String, Vec<serde_json::Value>>::new();
            for property in properties {
                if map.contains_key(&property.label) {
                    let v = map.get_mut(&property.label).unwrap();
                    v.push(property.encode_v2());
                } else {
                    map.insert(property.label.clone(), vec![property.encode_v2()]);
                }
            }
            json!({
                  "@type" : "g:Vertex",
                  "@value" : {
                    "id" : self.id.encode_v2(),
                    "label" : self.label,
                    "properties" : map
            }})
        } else {
            json!({
                  "@type" : "g:Vertex",
                  "@value" : {
                    "id" : self.id.encode_v2(),
                    "label" : self.label,
            }})
        }
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

impl DecodeGraphSON for Vertex {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:Vertex")?;

        let id = get_val_by_key_v3(value_object, "id", "Vertex")?;
        let label = get_val_by_key_v3(value_object, "label", "Vertex")?;

        let properties = value_object
            .get("properties")
            .and_then(|obj| obj.as_object())
            .map(|map| {
                map.values()
                    .flat_map(|val| val.as_array())
                    .flatten()
                    .map(DecodeGraphSON::decode_v3)
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;
        Ok(Vertex {
            id: Box::new(id),
            label,
            properties,
        })
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:Vertex")?;

        let id = get_val_by_key_v2(value_object, "id", "Vertex")?;
        let label = get_val_by_key_v2(value_object, "label", "Vertex")?;

        let properties = value_object
            .get("properties")
            .and_then(|obj| obj.as_object())
            .map(|map| {
                map.values()
                    .flat_map(|val| val.as_array())
                    .flatten()
                    .map(DecodeGraphSON::decode_v2)
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;

        Ok(Vertex {
            id: Box::new(id),
            label,
            properties,
        })
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

impl EncodeGraphSON for VertexProperty {
    fn encode_v3(&self) -> serde_json::Value {
        let mut jval_map = Map::new();
        if let Some(props) = &self.properties {
            for property in props {
                jval_map.insert(property.key.clone(), property.value.encode_v3());
            }
            json!(
                {
                    "@type" : "g:VertexProperty",
                    "@value" : {
                      "id" : self.id.encode_v3(),
                      "value" : self.value.encode_v3(),
                      "label" : self.label,
                      "properties" : jval_map
                    }
                }
            )
        } else {
            json!(
                {
                    "@type" : "g:VertexProperty",
                    "@value" : {
                      "id" : self.id.encode_v3(),
                      "value" : self.value.encode_v3(),
                      "label" : self.label
                    }
                }
            )
        }
    }

    fn encode_v2(&self) -> serde_json::Value {
        let mut jval_map = Map::new();
        if let Some(props) = &self.properties {
            for property in props {
                jval_map.insert(property.key.clone(), property.value.encode_v2());
            }
            json!(
                {
                    "@type" : "g:VertexProperty",
                    "@value" : {
                      "id" : self.id.encode_v2(),
                      "value" : self.value.encode_v2(),
                      "label" : self.label,
                      "vertex" : self.parent.as_ref().map(|v| v.id.encode_v2()),
                      "properties" : jval_map
                    }
                }
            )
        } else {
            json!(
                {
                    "@type" : "g:VertexProperty",
                    "@value" : {
                      "id" : self.id.encode_v2(),
                      "value" : self.value.encode_v2(),
                      "vertex" : self.parent.as_ref().map(|v| v.id.encode_v2()),
                      "label" : self.label
                    }
                }
            )
        }
    }

    fn encode_v1(&self) -> serde_json::Value {
        json!({
          "id" : self.id.encode_v1(),
          "value" : self.value.encode_v1(),
          "label" : self.label
        })
    }
}

impl DecodeGraphSON for VertexProperty {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:VertexProperty")?;

        let id = get_val_by_key_v3(value_object, "id", "VertexProperty")?;
        let label = get_val_by_key_v3(value_object, "label", "VertexProperty")?;
        let value = get_val_by_key_v3(value_object, "value", "VertexProperty")?;

        let properties = value_object
            .get("properties")
            .and_then(|prop_obj| prop_obj.as_object())
            .map(|map| {
                map.iter()
                    .map(|(k, v)| {
                        GremlinValue::decode_v3(v).map(|g| Property::new(k, g, EitherParent::None))
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;

        Ok(VertexProperty {
            id: Box::new(id),
            label,
            value: Box::new(value),
            parent: None,
            properties,
        })
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:VertexProperty")?;

        let id = get_val_by_key_v2(value_object, "id", "VertexProperty")?;
        let label = get_val_by_key_v2(value_object, "label", "VertexProperty")?;
        let value = get_val_by_key_v2(value_object, "value", "VertexProperty")?;
        let vertex_id = get_val_by_key_v2(value_object, "vertex", "VertexProperty")?;

        let properties = value_object
            .get("properties")
            .and_then(|prop_obj| prop_obj.as_object())
            .map(|map| {
                map.iter()
                    .map(|(k, v)| {
                        GremlinValue::decode_v2(v).map(|g| Property::new(k, g, EitherParent::None))
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;

        Ok(VertexProperty {
            id: Box::new(id),
            label,
            value: Box::new(value),
            parent: Some(Vertex {
                id: Box::new(vertex_id),
                label: String::default(),
                properties: None,
            }),
            properties,
        })
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

impl EncodeGraphSON for Property {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
          "@type" : "g:Property",
          "@value" : {
            "key" : self.key,
            "value" : self.value.encode_v3()
          }
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        json!({
          "@type" : "g:Property",
          "@value" : {
            "key" : self.key,
            "value" : self.value.encode_v2(),
            "element" : self.parent.encode_v2()
          }
        })
    }

    fn encode_v1(&self) -> serde_json::Value {
        json!({
          "key" : self.key,
          "value" : self.value.encode_v1()
        })
    }
}

impl DecodeGraphSON for Property {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:Property")?;

        let key = get_val_by_key_v3(value_object, "key", "Property")?;

        let value = get_val_by_key_v3(value_object, "value", "Property")?;

        Ok(Property {
            key,
            value: Box::new(value),
            parent: EitherParent::None,
        })
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:Property")?;

        let key = get_val_by_key_v2(value_object, "key", "Property")?;

        let value = get_val_by_key_v2(value_object, "value", "Property")?;

        let parent = get_val_by_key_v2(value_object, "element", "Property")?;

        Ok(Property {
            key,
            value: Box::new(value),
            parent,
        })
    }

    fn decode_v1(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let key = get_val_by_key_v1(j_val, "key", "Property")?;

        let value = get_val_by_key_v1(j_val, "value", "Property")?;

        Ok(Property {
            key,
            value: Box::new(value),
            parent: EitherParent::None,
        })
    }
}

impl EncodeGraphSON for EitherParent {
    fn encode_v3(&self) -> serde_json::Value {
        unimplemented!()
    }

    fn encode_v2(&self) -> serde_json::Value {
        match self {
            EitherParent::Edge(e) => json!({
              "@type" : "g:Edge",
              "@value" : {
                "id" : e.id.encode_v2(),
                "label" : e.label.encode_v2(),
                "outV" : e.out_v_id.encode_v2(),
                "inV" : e.in_v_id.encode_v2()
              }
            }),
            EitherParent::VertexProperty(v) => v.encode_v2(),
            EitherParent::None => json!(null),
        }
    }

    fn encode_v1(&self) -> serde_json::Value {
        unimplemented!()
    }
}

impl DecodeGraphSON for EitherParent {
    fn decode_v3(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        unimplemented!()
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        if let Ok(value_object) = validate_type(j_val, "g:Edge") {
            let id = get_val_by_key_v2(value_object, "id", "EitherParent")?;
            let label = get_val_by_key_v2(value_object, "label", "EitherParent")?;
            let out_v_id = get_val_by_key_v2(value_object, "outV", "EitherParent")?;
            let in_v_id = get_val_by_key_v2(value_object, "inV", "EitherParent")?;

            Ok(EitherParent::Edge(Edge {
                id: Box::new(id),
                label,
                in_v_id: Box::new(in_v_id),
                in_v_label: Default::default(),
                out_v_id: Box::new(out_v_id),
                out_v_label: Default::default(),
                parent: None,
                properties: None,
            }))
        } else if let Ok(value_object) = validate_type(j_val, "g:VertexProperty") {
            // Not sure what VertexProptery looks like
            let id = get_val_by_key_v2(value_object, "id", "EitherParent")?;
            let label = get_val_by_key_v2(value_object, "label", "EitherParent")?;
            // let out_v_id = get_val_by_key_v2(value_object, "outV", "EitherParent")?;
            // let in_v_id = get_val_by_key_v2(value_object, "inV", "EitherParent")?;
            Ok(EitherParent::VertexProperty(VertexProperty {
                id: Box::new(id),
                label,
                value: Box::new(GremlinValue::UnspecifiedNullObject),
                parent: None,
                properties: None,
            }))
        } else {
            Err(GraphSonError::KeyNotFound(
                "Edge or VertexProperty".to_string(),
            ))
        }
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        unimplemented!()
    }
}

impl EncodeGraphSON for Traverser {
    fn encode_v3(&self) -> serde_json::Value {
        json!({
        "@type" : "g:Traverser",
        "@value" : {
          "bulk" : self.bulk.encode_v3(),
          "value": self.value.encode_v3()
        }})
    }

    fn encode_v2(&self) -> serde_json::Value {
        json!({
        "@type" : "g:Traverser",
        "@value" : {
          "bulk" : self.bulk.encode_v2(),
          "value": self.value.encode_v2()
        }})
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

impl DecodeGraphSON for Traverser {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:Traverser")?;

        let bulk = get_val_by_key_v3(value_object, "bulk", "Traverser")?;
        let value = get_val_by_key_v3(value_object, "value", "Traverser")?;

        Ok(Traverser {
            bulk,
            value: Box::new(value),
        })
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:Traverser")?;

        let bulk = get_val_by_key_v2(value_object, "bulk", "Traverser")?;
        let value = get_val_by_key_v2(value_object, "value", "Traverser")?;

        Ok(Traverser {
            bulk,
            value: Box::new(value),
        })
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

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

#[test]
fn big_int_encode_v3() {
    let s = BigInt::from_str("123456789987654321123456789987654321").unwrap();
    let expected = r#"{"@type":"gx:BigInteger","@value":123456789987654321123456789987654321}"#;
    let res = serde_json::to_string(&s.encode_v3()).unwrap();
    assert_eq!(res, expected)
}

#[test]
fn big_int_decode_v3() {
    let expected = BigInt::from_str("123456789987654321123456789987654321").unwrap();
    let s = r#"{"@type":"gx:BigInteger","@value":123456789987654321123456789987654321}"#;
    let val = serde_json::from_str(s).unwrap();
    let res = BigInt::decode_v3(&val).unwrap();
    assert_eq!(res, expected)
}

#[test]
fn big_dec_encode_v3() {
    let s = BigDecimal::from_str("123456789987654321123456789987654321").unwrap();
    let expected = r#"{"@type":"gx:BigDecimal","@value":123456789987654321123456789987654321}"#;
    let res = serde_json::to_string(&s.encode_v3()).unwrap();
    assert_eq!(res, expected)
}

#[test]
fn big_dec_scale_encode_v3() {
    let s = BigDecimal::from_str("123456789987654321.123456789987654321").unwrap();
    let expected = r#"{"@type":"gx:BigDecimal","@value":123456789987654321.123456789987654321}"#;
    let res = serde_json::to_string(&s.encode_v3()).unwrap();
    assert_eq!(res, expected)
}

#[test]
fn big_dec_decode_v3() {
    let expected = BigDecimal::from_str("123456789987654321123456789987654321").unwrap();
    let s = r#"{"@type":"gx:BigDecimal","@value":123456789987654321123456789987654321}"#;
    let val = serde_json::from_str(s).unwrap();
    let res = BigDecimal::decode_v3(&val).unwrap();
    assert_eq!(res, expected)
}

#[test]
fn big_dec_scale_decode_v3() {
    let expected = BigDecimal::from_str("123456789987654321.123456789987654321").unwrap();
    let s = r#"{"@type":"gx:BigDecimal","@value":123456789987654321.123456789987654321}"#;
    let val = serde_json::from_str(s).unwrap();
    let res = BigDecimal::decode_v3(&val).unwrap();
    assert_eq!(res, expected)
}

#[test]
fn bulkset_encode_v3() {
    let expected = r#"{"@type":"g:BulkSet","@value":["marko",{"@type":"g:Int64","@value":1},"josh",{"@type":"g:Int64","@value":2}]}"#;

    let bulk_set = BulkSet(vec![("marko".into(), 1), ("josh".into(), 2)]).encode_v3();

    let res = serde_json::to_string(&bulk_set).unwrap();

    assert_eq!(res, expected)
}

#[test]
fn bulkset_decode_v3() {
    let s = r#"{"@type":"g:BulkSet","@value":["marko",{"@type":"g:Int64","@value":1},"josh",{"@type":"g:Int64","@value":2}]}"#;

    let expected = BulkSet(vec![("marko".into(), 1), ("josh".into(), 2)]);

    let v = serde_json::from_str(s).unwrap();
    let res = BulkSet::decode_v3(&v).unwrap();
    assert_eq!(res, expected)
}

#[test]
fn bytecode_encode_v3() {
    let byte_buffer = ByteBuffer(vec![b'a', b'b', b'c', b'd', 255, 128, 129, 130]);

    let v = byte_buffer.encode_v3();
    assert_eq!(
        v.to_string(),
        "{\"@type\":\"gx:ByteBuffer\",\"@value\":\"abcdÿ\u{80}\u{81}\u{82}\"}"
    )
}

#[test]
fn bytbuffer_decode_v3() {
    let jstr = "{\"@type\":\"gx:ByteBuffer\",\"@value\":\"abcdÿ\u{80}\u{81}\u{82}\"}";
    let expected = ByteBuffer(vec![b'a', b'b', b'c', b'd', 255, 128, 129, 130]);

    let v: serde_json::Value = serde_json::from_str(jstr).unwrap();
    let res = ByteBuffer::decode_v3(&v).unwrap();
    assert_eq!(res, expected)
}

#[test]
fn bytecode_decode_v3() {
    let string = r#"{
        "@type" : "g:Bytecode",
        "@value" : {
          "step" : [ [ "V" ], [ "hasLabel", "person" ], [ "out" ], [ "in" ], [ "tree" ] ],
          "source": [["inject",{"@type" : "g:Int32","@value" : 29}]]
        }
      }"#;

    let mut expected = Bytecode::default();
    expected.push_new_step("V", vec![]);
    expected.push_new_step("hasLabel", vec!["person".into()]);
    expected.push_new_step("out", vec![]);
    expected.push_new_step("in", vec![]);
    expected.push_new_step("tree", vec![]);
    expected.push_new_source("inject", vec![29.into()]);

    let j_val = serde_json::from_str(string).unwrap();
    let bc = Bytecode::decode_v3(&j_val).unwrap();
    assert_eq!(bc, expected)
}

#[test]
fn bytecode_fail_decode_v3() {
    let string = r#"{
        "@type" : "g:Bytecode",
        "@value" : {
          "step" : [ [], [ "hasLabel", "person" ], [], [ "in" ], [ "tree" ] ]
        }
      }"#;

    let j_val = serde_json::from_str(string).unwrap();
    let bc = Bytecode::decode_v3(&j_val);
    assert!(bc.is_err())
}

#[test]
fn bytecode_fail2_decode_v3() {
    let string = r#"{
        "@type" : "g:Bytecode",
        "@value" : {
          "step" : [ [true], [ "hasLabel", "person" ], [ "in" ], [ "tree" ] ]
        }
      }"#;

    let j_val = serde_json::from_str(string).unwrap();
    let bc = Bytecode::decode_v3(&j_val);
    assert!(bc.is_err())
}

#[test]
fn bytecode_decode_int_parameter_v3() {
    let string = r#"{
        "@type" : "g:Bytecode",
        "@value" : {
          "step" : [ [ "V" ], [ "has", "person","age",{"@type" : "g:Int32","@value" : 29} ], [ "out" ], [ "in" ], [ "tree" ] ]
        }
      }"#;

    let mut expected = Bytecode::default();
    expected.push_new_step("V", vec![]);
    expected.push_new_step("has", vec!["person".into(), "age".into(), 29.into()]);
    expected.push_new_step("out", vec![]);
    expected.push_new_step("in", vec![]);
    expected.push_new_step("tree", vec![]);

    let j_val = serde_json::from_str(string).unwrap();
    let bc = Bytecode::decode_v3(&j_val).unwrap();
    assert_eq!(bc, expected)
}

#[test]
fn edge_encode_v3() {
    let e = Edge {
        id: Box::new(13.into()),
        label: "develops".to_string(),
        in_v_id: Box::new(10.into()),
        in_v_label: "software".to_string(),
        out_v_id: Box::new(1.into()),
        out_v_label: "person".to_string(),
        parent: None,
        properties: Some(vec![Property {
            key: "since".to_string(),
            value: Box::new(2009.into()),
            parent: property::EitherParent::None,
        }]),
    };

    let s = serde_json::to_string(&e.encode_v3()).unwrap();

    let expected = r#"{"@type":"g:Edge","@value":{"id":{"@type":"g:Int32","@value":13},"inV":{"@type":"g:Int32","@value":10},"inVLabel":"software","label":"develops","outV":{"@type":"g:Int32","@value":1},"outVLabel":"person","properties":{"since":{"@type":"g:Property","@value":{"key":"since","value":{"@type":"g:Int32","@value":2009}}}}}}"#;
    assert_eq!(s, expected)
}

#[test]
fn edge_encode_v3_without_props() {
    let e = Edge {
        id: Box::new(13.into()),
        label: "develops".to_string(),
        in_v_id: Box::new(10.into()),
        in_v_label: "software".to_string(),
        out_v_id: Box::new(1.into()),
        out_v_label: "person".to_string(),
        parent: None,
        properties: None,
    };

    let s = serde_json::to_string(&e.encode_v3()).unwrap();

    let expected = r#"{"@type":"g:Edge","@value":{"id":{"@type":"g:Int32","@value":13},"inV":{"@type":"g:Int32","@value":10},"inVLabel":"software","label":"develops","outV":{"@type":"g:Int32","@value":1},"outVLabel":"person"}}"#;
    assert_eq!(s, expected)
}

#[test]
fn edge_decode_v3() {
    let expected = Edge {
        id: Box::new(13.into()),
        label: "develops".to_string(),
        in_v_id: Box::new(10.into()),
        in_v_label: "software".to_string(),
        out_v_id: Box::new(1.into()),
        out_v_label: "person".to_string(),
        parent: None,
        properties: Some(vec![Property {
            key: "since".to_string(),
            value: Box::new(2009.into()),
            parent: property::EitherParent::None,
        }]),
    };

    let input = r#"{
        "@type" : "g:Edge",
        "@value" : {
          "id" : {
            "@type" : "g:Int32",
            "@value" : 13
          },
          "label" : "develops",
          "inVLabel" : "software",
          "outVLabel" : "person",
          "inV" : {
            "@type" : "g:Int32",
            "@value" : 10
          },
          "outV" : {
            "@type" : "g:Int32",
            "@value" : 1
          },
          "properties" : {
            "since" : {
              "@type" : "g:Property",
              "@value" : {
                "key" : "since",
                "value" : {
                  "@type" : "g:Int32",
                  "@value" : 2009
                }
              }
            }
          }
        }
      }"#;

    let v = serde_json::from_str(input).unwrap();
    let e = Edge::decode_v3(&v).unwrap();
    assert_eq!(e, expected)
}

#[test]
fn edge_decode_v3_without_props() {
    let expected = Edge {
        id: Box::new(13.into()),
        label: "develops".to_string(),
        in_v_id: Box::new(10.into()),
        in_v_label: "software".to_string(),
        out_v_id: Box::new(1.into()),
        out_v_label: "person".to_string(),
        parent: None,
        properties: None,
    };

    let input = r#"{
        "@type" : "g:Edge",
        "@value" : {
          "id" : {
            "@type" : "g:Int32",
            "@value" : 13
          },
          "label" : "develops",
          "inVLabel" : "software",
          "outVLabel" : "person",
          "inV" : {
            "@type" : "g:Int32",
            "@value" : 10
          },
          "outV" : {
            "@type" : "g:Int32",
            "@value" : 1
          }
        }
      }"#;

    let v = serde_json::from_str(input).unwrap();
    let e = Edge::decode_v3(&v).unwrap();
    assert_eq!(e, expected)
}

#[test]
fn edge_encode_v2() {
    let e = Edge {
        id: Box::new(13.into()),
        label: "develops".to_string(),
        in_v_id: Box::new(10.into()),
        in_v_label: "software".to_string(),
        out_v_id: Box::new(1.into()),
        out_v_label: "person".to_string(),
        parent: None,
        properties: Some(vec![Property {
            key: "since".to_string(),
            value: Box::new(2009.into()),
            parent: property::EitherParent::None,
        }]),
    };

    let s = serde_json::to_string(&e.encode_v2()).unwrap();

    let expected = r#"{"@type":"g:Edge","@value":{"id":{"@type":"g:Int32","@value":13},"inV":{"@type":"g:Int32","@value":10},"inVLabel":"software","label":"develops","outV":{"@type":"g:Int32","@value":1},"outVLabel":"person","properties":{"since":{"@type":"g:Int32","@value":2009}}}}"#;
    assert_eq!(s, expected)
}

#[test]
fn edge_encode_v2_without_props() {
    let e = Edge {
        id: Box::new(13.into()),
        label: "develops".to_string(),
        in_v_id: Box::new(10.into()),
        in_v_label: "software".to_string(),
        out_v_id: Box::new(1.into()),
        out_v_label: "person".to_string(),
        parent: None,
        properties: None,
    };

    let s = serde_json::to_string(&e.encode_v2()).unwrap();

    let expected = r#"{"@type":"g:Edge","@value":{"id":{"@type":"g:Int32","@value":13},"inV":{"@type":"g:Int32","@value":10},"inVLabel":"software","label":"develops","outV":{"@type":"g:Int32","@value":1},"outVLabel":"person"}}"#;
    assert_eq!(s, expected)
}

#[test]
fn edge_decode_v2() {
    let expected = Edge {
        id: Box::new(13.into()),
        label: "develops".to_string(),
        in_v_id: Box::new(10.into()),
        in_v_label: "software".to_string(),
        out_v_id: Box::new(1.into()),
        out_v_label: "person".to_string(),
        parent: None,
        properties: Some(vec![Property {
            key: "since".to_string(),
            value: Box::new(2009.into()),
            parent: property::EitherParent::None,
        }]),
    };

    let input = r#"{
        "@type" : "g:Edge",
        "@value" : {
          "id" : {
            "@type" : "g:Int32",
            "@value" : 13
          },
          "label" : "develops",
          "inVLabel" : "software",
          "outVLabel" : "person",
          "inV" : {
            "@type" : "g:Int32",
            "@value" : 10
          },
          "outV" : {
            "@type" : "g:Int32",
            "@value" : 1
          },
          "properties" : {
            "since" : {
              "@type" : "g:Int32",
              "@value" : 2009
            }
          }
        }
      }"#;

    let v = serde_json::from_str(input).unwrap();
    let e = Edge::decode_v2(&v).unwrap();
    assert_eq!(e, expected)
}

#[test]
fn edge_decode_v2_without_props() {
    let expected = Edge {
        id: Box::new(13.into()),
        label: "develops".to_string(),
        in_v_id: Box::new(10.into()),
        in_v_label: "software".to_string(),
        out_v_id: Box::new(1.into()),
        out_v_label: "person".to_string(),
        parent: None,
        properties: None,
    };

    let input = r#"{
        "@type" : "g:Edge",
        "@value" : {
          "id" : {
            "@type" : "g:Int32",
            "@value" : 13
          },
          "label" : "develops",
          "inVLabel" : "software",
          "outVLabel" : "person",
          "inV" : {
            "@type" : "g:Int32",
            "@value" : 10
          },
          "outV" : {
            "@type" : "g:Int32",
            "@value" : 1
          }
        }
      }"#;

    let v = serde_json::from_str(input).unwrap();
    let e = Edge::decode_v2(&v).unwrap();
    assert_eq!(e, expected)
}

#[test]
fn lambda_encode_v3() {
    let l = Lambda {
        language: "gremlin-groovy".to_string(),
        script: "{ it.get() }".to_string(),
        arguments_length: 1,
    };

    let s = l.encode_v3();
    let res = serde_json::to_string(&s).unwrap();
    let v: serde_json::Value = serde_json::from_str(&res).unwrap();

    assert_eq!(s, v);
}

#[test]
fn lambda_decode_v3() {
    let expected = Lambda {
        language: "gremlin-groovy".to_string(),
        script: "{ it.get() }".to_string(),
        arguments_length: 1,
    };
    let s = r#"{
        "@type" : "g:Lambda",
        "@value" : {
          "script" : "{ it.get() }",
          "language" : "gremlin-groovy",
          "arguments" : 1
        }
      }"#;
    let v: serde_json::Value = serde_json::from_str(s).unwrap();
    let res = Lambda::decode_v3(&v).unwrap();

    assert_eq!(res, expected);
}

#[test]
fn metrics_encode_v3() {
    let metric = Metrics {
        id: "7.0.0()".to_string(),
        name: "TinkerGraphStep(vertex,[~label.eq(person)])".to_string(),
        duration: 100000000,
        counts: HashMap::from([
            // ("traverserCount".to_string(), 4),
            ("elementCount".to_string(), 4),
        ]),
        annotations: HashMap::from([("percentDur".to_string(), 25.0f64.into())]),
        nested_metrics: vec![Metrics {
            id: "3.0.0()".to_string(),
            name: "VertexStep(OUT,vertex)".to_string(),
            duration: 100000000,
            counts: HashMap::from([
                // ("traverserCount".to_string(), 7),
                ("elementCount".to_string(), 7),
            ]),
            annotations: HashMap::from([("percentDur".to_string(), 25f64.into())]),
            nested_metrics: vec![],
        }],
    };

    let s = metric.encode_v3();

    let expected_str = r#"{"@type":"g:Metrics","@value":{"@type":"g:Map","@value":["dur",{"@type":"g:Double","@value":100.0},"counts",{"@type":"g:Map","@value":["elementCount",{"@type":"g:Int64","@value":4}]},"name","TinkerGraphStep(vertex,[~label.eq(person)])","annotations",{"@type":"g:Map","@value":["percentDur",{"@type":"g:Double","@value":25.0}]},"id","7.0.0()","metrics",{"@type":"g:List","@value":[{"@type":"g:Metrics","@value":{"@type":"g:Map","@value":["dur",{"@type":"g:Double","@value":100.0},"counts",{"@type":"g:Map","@value":["elementCount",{"@type":"g:Int64","@value":7}]},"name","VertexStep(OUT,vertex)","annotations",{"@type":"g:Map","@value":["percentDur",{"@type":"g:Double","@value":25.0}]},"id","3.0.0()"]}}]}]}}"#;
    let expected_jval: serde_json::Value = serde_json::from_str(expected_str).unwrap();
    assert_eq!(s, expected_jval)
}

#[test]
fn metrics_decode_v3() {
    let expected = Metrics {
        id: "7.0.0()".to_string(),
        name: "TinkerGraphStep(vertex,[~label.eq(person)])".to_string(),
        duration: 100000000,
        counts: HashMap::from([
            ("traverserCount".to_string(), 4),
            ("elementCount".to_string(), 4),
        ]),
        annotations: HashMap::from([("percentDur".to_string(), 25.0f64.into())]),
        nested_metrics: vec![Metrics {
            id: "3.0.0()".to_string(),
            name: "VertexStep(OUT,vertex)".to_string(),
            duration: 100000000,
            counts: HashMap::from([
                ("traverserCount".to_string(), 7),
                ("elementCount".to_string(), 7),
            ]),
            annotations: HashMap::from([("percentDur".to_string(), 25f64.into())]),
            nested_metrics: vec![],
        }],
    };

    let str = r#"{"@type":"g:Metrics","@value":{"@type":"g:Map","@value":["dur",{"@type":"g:Double","@value":100.0},"counts",{"@type":"g:Map","@value":["traverserCount",{"@type":"g:Int64","@value":4},"elementCount",{"@type":"g:Int64","@value":4}]},"name","TinkerGraphStep(vertex,[~label.eq(person)])","annotations",{"@type":"g:Map","@value":["percentDur",{"@type":"g:Double","@value":25.0}]},"id","7.0.0()","metrics",{"@type":"g:List","@value":[{"@type":"g:Metrics","@value":{"@type":"g:Map","@value":["dur",{"@type":"g:Double","@value":100.0},"counts",{"@type":"g:Map","@value":["traverserCount",{"@type":"g:Int64","@value":7},"elementCount",{"@type":"g:Int64","@value":7}]},"name","VertexStep(OUT,vertex)","annotations",{"@type":"g:Map","@value":["percentDur",{"@type":"g:Double","@value":25.0}]},"id","3.0.0()"]}}]}]}}"#;
    let jval: serde_json::Value = serde_json::from_str(str).unwrap();
    let metrics_res = Metrics::decode_v3(&jval).unwrap();
    assert_eq!(metrics_res, expected)
}

#[test]
fn metrics_encode_v2() {
    let metric = Metrics {
        id: "7.0.0()".to_string(),
        name: "TinkerGraphStep(vertex,[~label.eq(person)])".to_string(),
        duration: 100000000,
        counts: HashMap::from([
            // ("traverserCount".to_string(), 4),
            ("elementCount".to_string(), 4),
        ]),
        annotations: HashMap::from([("percentDur".to_string(), 25.0f64.into())]),
        nested_metrics: vec![Metrics {
            id: "3.0.0()".to_string(),
            name: "VertexStep(OUT,vertex)".to_string(),
            duration: 100000000,
            counts: HashMap::from([
                // ("traverserCount".to_string(), 7),
                ("elementCount".to_string(), 7),
            ]),
            annotations: HashMap::from([("percentDur".to_string(), 25f64.into())]),
            nested_metrics: vec![],
        }],
    };

    let s = metric.encode_v2();

    let expected_str = r#"{"@type":"g:Metrics","@value":{"dur":{"@type":"g:Double","@value":100.0},"counts":{"elementCount":{"@type":"g:Int64","@value":4}},"name":"TinkerGraphStep(vertex,[~label.eq(person)])","annotations":{"percentDur":{"@type":"g:Double","@value":25.0}},"id":"7.0.0()","metrics":[{"@type":"g:Metrics","@value":{"dur":{"@type":"g:Double","@value":100.0},"counts":{"elementCount":{"@type":"g:Int64","@value":7}},"name":"VertexStep(OUT,vertex)","annotations":{"percentDur":{"@type":"g:Double","@value":25.0}},"id":"3.0.0()"}}]}}"#;
    let expected_jval: serde_json::Value = serde_json::from_str(expected_str).unwrap();
    assert_eq!(s, expected_jval)
}

#[test]
fn metrics_decode_v2() {
    let expected = Metrics {
        id: "7.0.0()".to_string(),
        name: "TinkerGraphStep(vertex,[~label.eq(person)])".to_string(),
        duration: 100000000,
        counts: HashMap::from([
            ("traverserCount".to_string(), 4),
            ("elementCount".to_string(), 4),
        ]),
        annotations: HashMap::from([("percentDur".to_string(), 25.0f64.into())]),
        nested_metrics: vec![Metrics {
            id: "3.0.0()".to_string(),
            name: "VertexStep(OUT,vertex)".to_string(),
            duration: 100000000,
            counts: HashMap::from([
                ("traverserCount".to_string(), 7),
                ("elementCount".to_string(), 7),
            ]),
            annotations: HashMap::from([("percentDur".to_string(), 25f64.into())]),
            nested_metrics: vec![],
        }],
    };

    let str = r#"{"@type":"g:Metrics","@value":{"dur":{"@type":"g:Double","@value":100.0},"counts":{"traverserCount":{"@type":"g:Int64","@value":4},"elementCount":{"@type":"g:Int64","@value":4}},"name":"TinkerGraphStep(vertex,[~label.eq(person)])","annotations":{"percentDur":{"@type":"g:Double","@value":25.0}},"id":"7.0.0()","metrics":[{"@type":"g:Metrics","@value":{"dur":{"@type":"g:Double","@value":100.0},"counts":{"traverserCount":{"@type":"g:Int64","@value":7},"elementCount":{"@type":"g:Int64","@value":7}},"name":"VertexStep(OUT,vertex)","annotations":{"percentDur":{"@type":"g:Double","@value":25.0}},"id":"3.0.0()"}}]}}"#;
    let jval: serde_json::Value = serde_json::from_str(str).unwrap();
    let metrics_res = Metrics::decode_v2(&jval).unwrap();
    assert_eq!(metrics_res, expected)
}

#[test]
fn metrics_encode_v3_traversal() {
    let metric = Metrics {
        id: "7.0.0()".to_string(),
        name: "TinkerGraphStep(vertex,[~label.eq(person)])".to_string(),
        duration: 100000000,
        counts: HashMap::from([
            // ("traverserCount".to_string(), 4),
            ("elementCount".to_string(), 4),
        ]),
        annotations: HashMap::from([("percentDur".to_string(), 25.0f64.into())]),
        nested_metrics: vec![Metrics {
            id: "3.0.0()".to_string(),
            name: "VertexStep(OUT,vertex)".to_string(),
            duration: 100000000,
            counts: HashMap::from([
                // ("traverserCount".to_string(), 7),
                ("elementCount".to_string(), 7),
            ]),
            annotations: HashMap::from([("percentDur".to_string(), 25f64.into())]),
            nested_metrics: vec![],
        }],
    };

    let s = metric.encode_v3();

    let expected_str = r#"{"@type":"g:Metrics","@value":{"@type":"g:Map","@value":["dur",{"@type":"g:Double","@value":100.0},"counts",{"@type":"g:Map","@value":["elementCount",{"@type":"g:Int64","@value":4}]},"name","TinkerGraphStep(vertex,[~label.eq(person)])","annotations",{"@type":"g:Map","@value":["percentDur",{"@type":"g:Double","@value":25.0}]},"id","7.0.0()","metrics",{"@type":"g:List","@value":[{"@type":"g:Metrics","@value":{"@type":"g:Map","@value":["dur",{"@type":"g:Double","@value":100.0},"counts",{"@type":"g:Map","@value":["elementCount",{"@type":"g:Int64","@value":7}]},"name","VertexStep(OUT,vertex)","annotations",{"@type":"g:Map","@value":["percentDur",{"@type":"g:Double","@value":25.0}]},"id","3.0.0()"]}}]}]}}"#;
    let expected_jval: serde_json::Value = serde_json::from_str(expected_str).unwrap();
    assert_eq!(s, expected_jval)
}

#[test]
fn metrics_decode_v3_traversal() {
    let expected = Metrics {
        id: "7.0.0()".to_string(),
        name: "TinkerGraphStep(vertex,[~label.eq(person)])".to_string(),
        duration: 100000000,
        counts: HashMap::from([
            ("traverserCount".to_string(), 4),
            ("elementCount".to_string(), 4),
        ]),
        annotations: HashMap::from([("percentDur".to_string(), 25.0f64.into())]),
        nested_metrics: vec![Metrics {
            id: "3.0.0()".to_string(),
            name: "VertexStep(OUT,vertex)".to_string(),
            duration: 100000000,
            counts: HashMap::from([
                ("traverserCount".to_string(), 7),
                ("elementCount".to_string(), 7),
            ]),
            annotations: HashMap::from([("percentDur".to_string(), 25f64.into())]),
            nested_metrics: vec![],
        }],
    };

    let str = r#"{"@type":"g:Metrics","@value":{"@type":"g:Map","@value":["dur",{"@type":"g:Double","@value":100.0},"counts",{"@type":"g:Map","@value":["traverserCount",{"@type":"g:Int64","@value":4},"elementCount",{"@type":"g:Int64","@value":4}]},"name","TinkerGraphStep(vertex,[~label.eq(person)])","annotations",{"@type":"g:Map","@value":["percentDur",{"@type":"g:Double","@value":25.0}]},"id","7.0.0()","metrics",{"@type":"g:List","@value":[{"@type":"g:Metrics","@value":{"@type":"g:Map","@value":["dur",{"@type":"g:Double","@value":100.0},"counts",{"@type":"g:Map","@value":["traverserCount",{"@type":"g:Int64","@value":7},"elementCount",{"@type":"g:Int64","@value":7}]},"name","VertexStep(OUT,vertex)","annotations",{"@type":"g:Map","@value":["percentDur",{"@type":"g:Double","@value":25.0}]},"id","3.0.0()"]}}]}]}}"#;
    let jval: serde_json::Value = serde_json::from_str(str).unwrap();
    let metrics_res = Metrics::decode_v3(&jval).unwrap();
    assert_eq!(metrics_res, expected)
}

#[test]
fn metrics_encode_v2_traversal() {
    let metric = Metrics {
        id: "7.0.0()".to_string(),
        name: "TinkerGraphStep(vertex,[~label.eq(person)])".to_string(),
        duration: 100000000,
        counts: HashMap::from([
            // ("traverserCount".to_string(), 4),
            ("elementCount".to_string(), 4),
        ]),
        annotations: HashMap::from([("percentDur".to_string(), 25.0f64.into())]),
        nested_metrics: vec![Metrics {
            id: "3.0.0()".to_string(),
            name: "VertexStep(OUT,vertex)".to_string(),
            duration: 100000000,
            counts: HashMap::from([
                // ("traverserCount".to_string(), 7),
                ("elementCount".to_string(), 7),
            ]),
            annotations: HashMap::from([("percentDur".to_string(), 25f64.into())]),
            nested_metrics: vec![],
        }],
    };

    let s = metric.encode_v2();

    let expected_str = r#"{"@type":"g:Metrics","@value":{"dur":{"@type":"g:Double","@value":100.0},"counts":{"elementCount":{"@type":"g:Int64","@value":4}},"name":"TinkerGraphStep(vertex,[~label.eq(person)])","annotations":{"percentDur":{"@type":"g:Double","@value":25.0}},"id":"7.0.0()","metrics":[{"@type":"g:Metrics","@value":{"dur":{"@type":"g:Double","@value":100.0},"counts":{"elementCount":{"@type":"g:Int64","@value":7}},"name":"VertexStep(OUT,vertex)","annotations":{"percentDur":{"@type":"g:Double","@value":25.0}},"id":"3.0.0()"}}]}}"#;
    let expected_jval: serde_json::Value = serde_json::from_str(expected_str).unwrap();
    assert_eq!(s, expected_jval)
}

#[test]
fn metrics_decode_v2_traversal() {
    let expected = Metrics {
        id: "7.0.0()".to_string(),
        name: "TinkerGraphStep(vertex,[~label.eq(person)])".to_string(),
        duration: 100000000,
        counts: HashMap::from([
            ("traverserCount".to_string(), 4),
            ("elementCount".to_string(), 4),
        ]),
        annotations: HashMap::from([("percentDur".to_string(), 25.0f64.into())]),
        nested_metrics: vec![Metrics {
            id: "3.0.0()".to_string(),
            name: "VertexStep(OUT,vertex)".to_string(),
            duration: 100000000,
            counts: HashMap::from([
                ("traverserCount".to_string(), 7),
                ("elementCount".to_string(), 7),
            ]),
            annotations: HashMap::from([("percentDur".to_string(), 25f64.into())]),
            nested_metrics: vec![],
        }],
    };

    let str = r#"{"@type":"g:Metrics","@value":{"dur":{"@type":"g:Double","@value":100.0},"counts":{"traverserCount":{"@type":"g:Int64","@value":4},"elementCount":{"@type":"g:Int64","@value":4}},"name":"TinkerGraphStep(vertex,[~label.eq(person)])","annotations":{"percentDur":{"@type":"g:Double","@value":25.0}},"id":"7.0.0()","metrics":[{"@type":"g:Metrics","@value":{"dur":{"@type":"g:Double","@value":100.0},"counts":{"traverserCount":{"@type":"g:Int64","@value":7},"elementCount":{"@type":"g:Int64","@value":7}},"name":"VertexStep(OUT,vertex)","annotations":{"percentDur":{"@type":"g:Double","@value":25.0}},"id":"3.0.0()"}}]}}"#;
    let jval: serde_json::Value = serde_json::from_str(str).unwrap();
    let metrics_res = Metrics::decode_v2(&jval).unwrap();
    assert_eq!(metrics_res, expected)
}

#[test]
fn path_encode_v3() {
    let p = Path {
        labels: vec![Set::new(vec![]), Set::new(vec![]), Set::new(vec![])],
        objects: vec![
            Vertex::new(1, "person", None).into(),
            Vertex::new(10, "sofware", None).into(),
            Vertex::new(11, "software", None).into(),
        ],
    };

    let s = serde_json::to_string(&p.encode_v3()).unwrap();
    let expected = r#"{"@type":"g:Path","@value":{"labels":{"@type":"g:List","@value":[{"@type":"g:Set","@value":[]},{"@type":"g:Set","@value":[]},{"@type":"g:Set","@value":[]}]},"objects":{"@type":"g:List","@value":[{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":1},"label":"person"}},{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":10},"label":"sofware"}},{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":11},"label":"software"}}]}}}"#;
    assert_eq!(s, expected)
}

#[test]
fn path_decode_v3() {
    let s = r#"{"@type":"g:Path","@value":{"labels":{"@type":"g:List","@value":[{"@type":"g:Set","@value":[]},{"@type":"g:Set","@value":[]},{"@type":"g:Set","@value":[]}]},"objects":{"@type":"g:List","@value":[{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":1},"label":"person"}},{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":10},"label":"sofware"}},{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":11},"label":"software"}}]}}}"#;
    let expected = Path {
        labels: vec![Set::new(vec![]), Set::new(vec![]), Set::new(vec![])],
        objects: vec![
            Vertex::new(1, "person", None).into(),
            Vertex::new(10, "sofware", None).into(),
            Vertex::new(11, "software", None).into(),
        ],
    };

    let jval = serde_json::from_str(s).unwrap();
    let path = Path::decode_v3(&jval).unwrap();
    assert_eq!(path, expected)
}

#[test]
fn path_encode_v2() {
    let p = Path {
        labels: vec![Set::new(vec![]), Set::new(vec![]), Set::new(vec![])],
        objects: vec![
            Vertex::new(1, "person", None).into(),
            Vertex::new(
                10,
                "software",
                Some(vec![VertexProperty::new(
                    4i64,
                    "name",
                    "gremlin",
                    Some(Vertex::new(10, "software", None)),
                    None,
                )]),
            )
            .into(),
            Vertex::new(
                11,
                "software",
                Some(vec![VertexProperty::new(
                    5i64,
                    "name",
                    "tinkergraph",
                    Some(Vertex::new(11, "software", None)),
                    None,
                )]),
            )
            .into(),
        ],
    };

    let s = serde_json::to_string(&p.encode_v2()).unwrap();
    let expected = r#"{"@type":"g:Path","@value":{"labels":[[],[],[]],"objects":[{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":1},"label":"person"}},{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":10},"label":"software","properties":{"name":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":4},"value":"gremlin","vertex":{"@type":"g:Int32","@value":10},"label":"name"}}]}}},{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":11},"label":"software","properties":{"name":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":5},"value":"tinkergraph","vertex":{"@type":"g:Int32","@value":11},"label":"name"}}]}}}]}}"#;
    let value: serde_json::Value = serde_json::from_str(expected).unwrap();
    let own_value: serde_json::Value = serde_json::from_str(&s).unwrap();
    assert_eq!(own_value, value);
}

#[test]
fn path_decode_v2() {
    let s = r#"{"@type":"g:Path","@value":{"labels":[[],[],[]],"objects":[{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":1},"label":"person"}},{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":10},"label":"software","properties":{"name":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":4},"value":"gremlin","vertex":{"@type":"g:Int32","@value":10},"label":"name"}}]}}},{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":11},"label":"software","properties":{"name":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":5},"value":"tinkergraph","vertex":{"@type":"g:Int32","@value":11},"label":"name"}}]}}}]}}"#;
    let expected = Path {
        labels: vec![Set::new(vec![]), Set::new(vec![]), Set::new(vec![])],
        objects: vec![
            Vertex::new(1, "person", None).into(),
            Vertex::new(
                10,
                "software",
                Some(vec![VertexProperty::new(
                    4i64,
                    "name",
                    "gremlin",
                    Some(Vertex::new(10, "", None)),
                    None,
                )]),
            )
            .into(),
            Vertex::new(
                11,
                "software",
                Some(vec![VertexProperty::new(
                    5i64,
                    "name",
                    "tinkergraph",
                    Some(Vertex::new(11, "", None)),
                    None,
                )]),
            )
            .into(),
        ],
    };

    let jval = serde_json::from_str(s).unwrap();
    let path = Path::decode_v2(&jval).unwrap();
    assert_eq!(path, expected)
}

#[test]
fn vertex_encode_v3() {
    let v = Vertex {
        id: Box::new(1_i32.into()),
        label: String::from("person"),
        properties: Some(vec![
            VertexProperty {
                id: Box::new(0i64.into()),
                label: "name".into(),
                value: Box::new("marko".into()),
                parent: None,
                properties: None,
            },
            VertexProperty {
                id: Box::new(8i64.into()),
                label: "location".into(),
                value: Box::new("brussels".into()),
                parent: None,
                properties: Some(vec![
                    Property {
                        key: "startTime".into(),
                        value: Box::new(2004.into()),
                        parent: EitherParent::None,
                    },
                    Property {
                        key: "endTime".into(),
                        value: Box::new(2005.into()),
                        parent: EitherParent::None,
                    },
                ]),
            },
            VertexProperty {
                id: Box::new(6i64.into()),
                label: "location".into(),
                value: Box::new("san diego".into()),
                parent: None,
                properties: Some(vec![
                    Property {
                        key: "startTime".into(),
                        value: Box::new(1997.into()),
                        parent: EitherParent::None,
                    },
                    Property {
                        key: "endTime".into(),
                        value: Box::new(2001.into()),
                        parent: EitherParent::None,
                    },
                ]),
            },
        ]),
    };
    let v = v.encode_v3();
    println!("{}", serde_json::to_string_pretty(&v).unwrap());
}

#[test]
fn vertex_decode_v3() {
    let str = r#"{
        "@type" : "g:Vertex",
        "@value" : {
          "id" : {
            "@type" : "g:Int32",
            "@value" : 1
          },
          "label" : "person",
          "properties" : {
            "name" : [ {
              "@type" : "g:VertexProperty",
              "@value" : {
                "id" : {
                  "@type" : "g:Int64",
                  "@value" : 0
                },
                "value" : "marko",
                "label" : "name"
              }
            } ],
            "location" : [ {
              "@type" : "g:VertexProperty",
              "@value" : {
                "id" : {
                  "@type" : "g:Int64",
                  "@value" : 6
                },
                "value" : "san diego",
                "label" : "location",
                "properties" : {
                  "startTime" : {
                    "@type" : "g:Int32",
                    "@value" : 1997
                  },
                  "endTime" : {
                    "@type" : "g:Int32",
                    "@value" : 2001
                  }
                }
              }
            }, {
              "@type" : "g:VertexProperty",
              "@value" : {
                "id" : {
                  "@type" : "g:Int64",
                  "@value" : 7
                },
                "value" : "santa cruz",
                "label" : "location",
                "properties" : {
                  "startTime" : {
                    "@type" : "g:Int32",
                    "@value" : 2001
                  },
                  "endTime" : {
                    "@type" : "g:Int32",
                    "@value" : 2004
                  }
                }
              }
            }, {
              "@type" : "g:VertexProperty",
              "@value" : {
                "id" : {
                  "@type" : "g:Int64",
                  "@value" : 8
                },
                "value" : "brussels",
                "label" : "location",
                "properties" : {
                  "startTime" : {
                    "@type" : "g:Int32",
                    "@value" : 2004
                  },
                  "endTime" : {
                    "@type" : "g:Int32",
                    "@value" : 2005
                  }
                }
              }
            }, {
              "@type" : "g:VertexProperty",
              "@value" : {
                "id" : {
                  "@type" : "g:Int64",
                  "@value" : 9
                },
                "value" : "santa fe",
                "label" : "location",
                "properties" : {
                  "startTime" : {
                    "@type" : "g:Int32",
                    "@value" : 2005
                  }
                }
              }
            } ]
          }
        }
      }"#;

    let expected = Vertex {
        id: Box::new(1_i32.into()),
        label: String::from("person"),
        properties: Some(vec![
            VertexProperty::new(0i64, "name", "marko", None, None),
            VertexProperty::new(
                6i64,
                "location",
                "san diego",
                None,
                Some(vec![
                    Property::new("startTime", 1997, EitherParent::None),
                    Property::new("endTime", 2001, EitherParent::None),
                ]),
            ),
            VertexProperty::new(
                7i64,
                "location",
                "santa cruz",
                None,
                Some(vec![
                    Property::new("startTime", 2001, EitherParent::None),
                    Property::new("endTime", 2004, EitherParent::None),
                ]),
            ),
            VertexProperty::new(
                8i64,
                "location",
                "brussels",
                None,
                Some(vec![
                    Property::new("startTime", 2004, EitherParent::None),
                    Property::new("endTime", 2005, EitherParent::None),
                ]),
            ),
            VertexProperty::new(
                9i64,
                "location",
                "santa fe",
                None,
                Some(vec![Property::new("startTime", 2005, EitherParent::None)]),
            ),
        ]),
    };

    let value = serde_json::from_str(str).unwrap();
    let mut v = Vertex::decode_v3(&value).unwrap();
    v.properties.as_mut().into_iter().for_each(|p| {
        for i in p {
            if i.properties.is_some() {
                i.properties
                    .as_mut()
                    .unwrap()
                    .sort_by(|p1, p2| p1.key.cmp(&p2.key).reverse())
            }
        }
    });

    v.properties.as_mut().unwrap().sort_by(|p1, p2| {
        p1.id
            .get_ref::<i64>()
            .unwrap()
            .cmp(p2.id.get_ref::<i64>().unwrap())
    });
    assert_eq!(v, expected)
}

#[test]
fn vertex_decode_v3_without_props() {
    let str = r#"{
        "@type" : "g:Vertex",
        "@value" : {
          "id" : {
            "@type" : "g:Int32",
            "@value" : 1
          },
          "label" : "person"
        }
      }"#;

    let expected = Vertex {
        id: Box::new(1_i32.into()),
        label: String::from("person"),
        properties: None,
    };

    let value = serde_json::from_str(str).unwrap();
    let v = Vertex::decode_v3(&value).unwrap();
    assert_eq!(v, expected)
}
#[test]
fn vertex_encode_v2() {
    let v = Vertex {
        id: Box::new(1_i32.into()),
        label: String::from("person"),
        properties: Some(vec![
            VertexProperty {
                id: Box::new(0i64.into()),
                label: "name".into(),
                value: Box::new("marko".into()),
                parent: None,
                properties: None,
            },
            VertexProperty {
                id: Box::new(8i64.into()),
                label: "location".into(),
                value: Box::new("brussels".into()),
                parent: None,
                properties: Some(vec![
                    Property {
                        key: "startTime".into(),
                        value: Box::new(2004.into()),
                        parent: EitherParent::None,
                    },
                    Property {
                        key: "endTime".into(),
                        value: Box::new(2005.into()),
                        parent: EitherParent::None,
                    },
                ]),
            },
            VertexProperty {
                id: Box::new(6i64.into()),
                label: "location".into(),
                value: Box::new("san diego".into()),
                parent: None,
                properties: Some(vec![
                    Property {
                        key: "startTime".into(),
                        value: Box::new(1997.into()),
                        parent: EitherParent::None,
                    },
                    Property {
                        key: "endTime".into(),
                        value: Box::new(2001.into()),
                        parent: EitherParent::None,
                    },
                ]),
            },
        ]),
    };
    let v = v.encode_v2();
    println!("{}", serde_json::to_string_pretty(&v).unwrap());
}

#[test]
fn vertex_decode_v2() {
    let str = r#"{
        "@type" : "g:Vertex",
        "@value" : {
          "id" : {
            "@type" : "g:Int32",
            "@value" : 1
          },
          "label" : "person",
          "properties" : {
            "name" : [ {
              "@type" : "g:VertexProperty",
              "@value" : {
                "id" : {
                  "@type" : "g:Int64",
                  "@value" : 0
                },
                "value" : "marko",
                "vertex" : {
                  "@type" : "g:Int32",
                  "@value" : 1
                },
                "label" : "name"
              }
            } ],
            "location" : [ {
              "@type" : "g:VertexProperty",
              "@value" : {
                "id" : {
                  "@type" : "g:Int64",
                  "@value" : 6
                },
                "value" : "san diego",
                "vertex" : {
                  "@type" : "g:Int32",
                  "@value" : 1
                },
                "label" : "location",
                "properties" : {
                  "startTime" : {
                    "@type" : "g:Int32",
                    "@value" : 1997
                  },
                  "endTime" : {
                    "@type" : "g:Int32",
                    "@value" : 2001
                  }
                }
              }
            }, {
              "@type" : "g:VertexProperty",
              "@value" : {
                "id" : {
                  "@type" : "g:Int64",
                  "@value" : 7
                },
                "value" : "santa cruz",
                "vertex" : {
                  "@type" : "g:Int32",
                  "@value" : 1
                },
                "label" : "location",
                "properties" : {
                  "startTime" : {
                    "@type" : "g:Int32",
                    "@value" : 2001
                  },
                  "endTime" : {
                    "@type" : "g:Int32",
                    "@value" : 2004
                  }
                }
              }
            }, {
              "@type" : "g:VertexProperty",
              "@value" : {
                "id" : {
                  "@type" : "g:Int64",
                  "@value" : 8
                },
                "value" : "brussels",
                "vertex" : {
                  "@type" : "g:Int32",
                  "@value" : 1
                },
                "label" : "location",
                "properties" : {
                  "startTime" : {
                    "@type" : "g:Int32",
                    "@value" : 2004
                  },
                  "endTime" : {
                    "@type" : "g:Int32",
                    "@value" : 2005
                  }
                }
              }
            }, {
              "@type" : "g:VertexProperty",
              "@value" : {
                "id" : {
                  "@type" : "g:Int64",
                  "@value" : 9
                },
                "value" : "santa fe",
                "vertex" : {
                  "@type" : "g:Int32",
                  "@value" : 1
                },
                "label" : "location",
                "properties" : {
                  "startTime" : {
                    "@type" : "g:Int32",
                    "@value" : 2005
                  }
                }
              }
            } ]
          }
        }
      }"#;

    let expected = Vertex {
        id: Box::new(1_i32.into()),
        label: String::from("person"),
        properties: Some(vec![
            VertexProperty::new(0i64, "name", "marko", Some(Vertex::new(1, "", None)), None),
            VertexProperty::new(
                6i64,
                "location",
                "san diego",
                Some(Vertex::new(1, "", None)),
                Some(vec![
                    Property::new("startTime", 1997, EitherParent::None),
                    Property::new("endTime", 2001, EitherParent::None),
                ]),
            ),
            VertexProperty::new(
                7i64,
                "location",
                "santa cruz",
                Some(Vertex::new(1, "", None)),
                Some(vec![
                    Property::new("startTime", 2001, EitherParent::None),
                    Property::new("endTime", 2004, EitherParent::None),
                ]),
            ),
            VertexProperty::new(
                8i64,
                "location",
                "brussels",
                Some(Vertex::new(1, "", None)),
                Some(vec![
                    Property::new("startTime", 2004, EitherParent::None),
                    Property::new("endTime", 2005, EitherParent::None),
                ]),
            ),
            VertexProperty::new(
                9i64,
                "location",
                "santa fe",
                Some(Vertex::new(1, "", None)),
                Some(vec![Property::new("startTime", 2005, EitherParent::None)]),
            ),
        ]),
    };

    let value = serde_json::from_str(str).unwrap();
    let mut v = Vertex::decode_v2(&value).unwrap();
    v.properties.as_mut().into_iter().for_each(|p| {
        for i in p {
            if i.properties.is_some() {
                i.properties
                    .as_mut()
                    .unwrap()
                    .sort_by(|p1, p2| p1.key.cmp(&p2.key).reverse())
            }
        }
    });

    v.properties.as_mut().unwrap().sort_by(|p1, p2| {
        p1.id
            .get_ref::<i64>()
            .unwrap()
            .cmp(p2.id.get_ref::<i64>().unwrap())
    });
    assert_eq!(v, expected)
}

#[test]
fn vertex_decode_v2_without_props() {
    let str = r#"{
        "@type" : "g:Vertex",
        "@value" : {
          "id" : {
            "@type" : "g:Int32",
            "@value" : 1
          },
          "label" : "person"
        }
      }"#;

    let expected = Vertex {
        id: Box::new(1_i32.into()),
        label: String::from("person"),
        properties: None,
    };

    let value = serde_json::from_str(str).unwrap();
    let v = Vertex::decode_v2(&value).unwrap();
    assert_eq!(v, expected)
}

#[test]
fn property_decode_v3() {
    let s = r#"{
        "@type" : "g:Property",
        "@value" : {
          "key" : "since",
          "value" : {
            "@type" : "g:Int32",
            "@value" : 2009
          }
        }
      }"#;

    let val = serde_json::from_str(s).unwrap();
    let p = Property::decode_v3(&val).unwrap();
    let expected = Property {
        key: "since".to_string(),
        value: Box::new(2009.into()),
        parent: EitherParent::None,
    };
    assert_eq!(p, expected)
}

#[test]
fn property_encode_v3() {
    let property = Property {
        key: "since".to_string(),
        value: Box::new(2009.into()),
        parent: EitherParent::None,
    };

    let res_str = serde_json::to_string(&property.encode_v3()).unwrap();
    let expeced = r#"{"@type":"g:Property","@value":{"key":"since","value":{"@type":"g:Int32","@value":2009}}}"#;
    assert_eq!(res_str, expeced)
}

#[test]
fn property_decode_v2() {
    let s = r#"{
        "@type" : "g:Property",
        "@value" : {
          "key" : "since",
          "value" : {
            "@type" : "g:Int32",
            "@value" : 2009
          },
          "element" : {
            "@type" : "g:Edge",
            "@value" : {
              "id" : {
                "@type" : "g:Int32",
                "@value" : 13
              },
              "label" : "develops",
              "outV" : {
                "@type" : "g:Int32",
                "@value" : 1
              },
              "inV" : {
                "@type" : "g:Int32",
                "@value" : 10
              }
            }
          }
        }
      }"#;

    let val = serde_json::from_str(s).unwrap();
    let p = Property::decode_v2(&val).unwrap();
    let expected = Property {
        key: "since".to_string(),
        value: Box::new(2009.into()),
        parent: EitherParent::Edge(Edge {
            id: Box::new(13.into()),
            label: "develops".to_string(),
            in_v_id: Box::new(10.into()),
            in_v_label: "".to_string(),
            out_v_id: Box::new(1.into()),
            out_v_label: Default::default(),
            parent: None,
            properties: None,
        }),
    };
    assert_eq!(p, expected);
}

#[test]
fn property_encode_v2() {
    let property = Property {
        key: "since".to_string(),
        value: Box::new(2009.into()),
        parent: EitherParent::Edge(Edge {
            id: Box::new(13.into()),
            label: "develops".to_string(),
            in_v_id: Box::new(10.into()),
            in_v_label: "".to_string(),
            out_v_id: Box::new(1.into()),
            out_v_label: Default::default(),
            parent: None,
            properties: None,
        }),
    };
    let res_str = serde_json::to_string(&property.encode_v2()).unwrap();
    let expected = r#"{"@type":"g:Property","@value":{"element":{"@type":"g:Edge","@value":{"id":{"@type":"g:Int32","@value":13},"inV":{"@type":"g:Int32","@value":10},"label":"develops","outV":{"@type":"g:Int32","@value":1}}},"key":"since","value":{"@type":"g:Int32","@value":2009}}}"#;
    assert_eq!(res_str, expected);
}

#[test]
fn traverser_encode_v3() {
    let expected = r#"{"@type":"g:Traverser","@value":{"bulk":{"@type":"g:Int64","@value":1},"value":{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":1},"label":"person","properties":{"name":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":0},"value":"marko","label":"name"}}],"location":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":6},"value":"san diego","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":1997},"endTime":{"@type":"g:Int32","@value":2001}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":7},"value":"santa cruz","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2001},"endTime":{"@type":"g:Int32","@value":2004}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":8},"value":"brussels","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2004},"endTime":{"@type":"g:Int32","@value":2005}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":9},"value":"santa fe","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2005}}}}]}}}}}"#;
    let t = Traverser {
        bulk: 1,
        value: Box::new(
            Vertex::new(
                1,
                "person",
                Some(vec![
                    VertexProperty::new(0i64, "name", "marko", None, None),
                    VertexProperty::new(
                        6i64,
                        "location",
                        "san diego",
                        None,
                        Some(vec![
                            Property::new("startTime", 1997, EitherParent::None),
                            Property::new("endTime", 2001, EitherParent::None),
                        ]),
                    ),
                    VertexProperty::new(
                        7i64,
                        "location",
                        "santa cruz",
                        None,
                        Some(vec![
                            Property::new("startTime", 2001, EitherParent::None),
                            Property::new("endTime", 2004, EitherParent::None),
                        ]),
                    ),
                    VertexProperty::new(
                        8i64,
                        "location",
                        "brussels",
                        None,
                        Some(vec![
                            Property::new("startTime", 2004, EitherParent::None),
                            Property::new("endTime", 2005, EitherParent::None),
                        ]),
                    ),
                    VertexProperty::new(
                        9i64,
                        "location",
                        "santa fe",
                        None,
                        Some(vec![Property::new("startTime", 2005, EitherParent::None)]),
                    ),
                ]),
            )
            .into(),
        ),
    };
    let s = t.encode_v3();
    let res: serde_json::Value = serde_json::from_str(expected).unwrap();
    assert_eq!(s, res)
}

#[test]
fn traverser_decode_v3() {
    let s = r#"{"@type":"g:Traverser","@value":{"bulk":{"@type":"g:Int64","@value":1},"value":{"@type":"g:Vertex","@value":{"id":{"@type":"g:Int32","@value":1},"label":"person","properties":{"name":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":0},"value":"marko","label":"name"}}],"location":[{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":6},"value":"san diego","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":1997},"endTime":{"@type":"g:Int32","@value":2001}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":7},"value":"santa cruz","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2001},"endTime":{"@type":"g:Int32","@value":2004}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":8},"value":"brussels","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2004},"endTime":{"@type":"g:Int32","@value":2005}}}},{"@type":"g:VertexProperty","@value":{"id":{"@type":"g:Int64","@value":9},"value":"santa fe","label":"location","properties":{"startTime":{"@type":"g:Int32","@value":2005}}}}]}}}}}"#;
    let expected = Traverser {
        bulk: 1,
        value: Box::new(
            Vertex::new(
                1,
                "person",
                Some(vec![
                    VertexProperty::new(0i64, "name", "marko", None, None),
                    VertexProperty::new(
                        6i64,
                        "location",
                        "san diego",
                        None,
                        Some(vec![
                            Property::new("startTime", 1997, EitherParent::None),
                            Property::new("endTime", 2001, EitherParent::None),
                        ]),
                    ),
                    VertexProperty::new(
                        7i64,
                        "location",
                        "santa cruz",
                        None,
                        Some(vec![
                            Property::new("startTime", 2001, EitherParent::None),
                            Property::new("endTime", 2004, EitherParent::None),
                        ]),
                    ),
                    VertexProperty::new(
                        8i64,
                        "location",
                        "brussels",
                        None,
                        Some(vec![
                            Property::new("startTime", 2004, EitherParent::None),
                            Property::new("endTime", 2005, EitherParent::None),
                        ]),
                    ),
                    VertexProperty::new(
                        9i64,
                        "location",
                        "santa fe",
                        None,
                        Some(vec![Property::new("startTime", 2005, EitherParent::None)]),
                    ),
                ]),
            )
            .into(),
        ),
    };

    let res: serde_json::Value = serde_json::from_str(s).unwrap();
    let mut res = Traverser::decode_v3(&res).unwrap();

    res.value
        .get_ref_mut::<Vertex>()
        .unwrap()
        .properties
        .as_mut()
        .into_iter()
        .for_each(|p| {
            for i in p {
                if i.properties.is_some() {
                    i.properties
                        .as_mut()
                        .unwrap()
                        .sort_by(|p1, p2| p1.key.cmp(&p2.key).reverse())
                }
            }
        });
    res.value
        .get_ref_mut::<Vertex>()
        .unwrap()
        .properties
        .as_mut()
        .unwrap()
        .sort_by(|p1, p2| {
            p1.id
                .get_ref::<i64>()
                .unwrap()
                .cmp(p2.id.get_ref::<i64>().unwrap())
        });
    assert_eq!(res, expected)
}

#[test]
fn traverser_decode_v2() {
    let s = r#"{"@type":"g:Traverser","@value":{"bulk":{"@type":"g:Int64","@value":1},"value":{
        "@type" : "g:Vertex",
        "@value" : {
          "id" : {
            "@type" : "g:Int32",
            "@value" : 1
          },
          "label" : "person",
          "properties" : {
            "name" : [ {
              "@type" : "g:VertexProperty",
              "@value" : {
                "id" : {
                  "@type" : "g:Int64",
                  "@value" : 0
                },
                "value" : "marko",
                "vertex" : {
                  "@type" : "g:Int32",
                  "@value" : 1
                },
                "label" : "name"
              }
            } ],
            "location" : [ {
              "@type" : "g:VertexProperty",
              "@value" : {
                "id" : {
                  "@type" : "g:Int64",
                  "@value" : 6
                },
                "value" : "san diego",
                "vertex" : {
                  "@type" : "g:Int32",
                  "@value" : 1
                },
                "label" : "location",
                "properties" : {
                  "startTime" : {
                    "@type" : "g:Int32",
                    "@value" : 1997
                  },
                  "endTime" : {
                    "@type" : "g:Int32",
                    "@value" : 2001
                  }
                }
              }
            }, {
              "@type" : "g:VertexProperty",
              "@value" : {
                "id" : {
                  "@type" : "g:Int64",
                  "@value" : 7
                },
                "value" : "santa cruz",
                "vertex" : {
                  "@type" : "g:Int32",
                  "@value" : 1
                },
                "label" : "location",
                "properties" : {
                  "startTime" : {
                    "@type" : "g:Int32",
                    "@value" : 2001
                  },
                  "endTime" : {
                    "@type" : "g:Int32",
                    "@value" : 2004
                  }
                }
              }
            }, {
              "@type" : "g:VertexProperty",
              "@value" : {
                "id" : {
                  "@type" : "g:Int64",
                  "@value" : 8
                },
                "value" : "brussels",
                "vertex" : {
                  "@type" : "g:Int32",
                  "@value" : 1
                },
                "label" : "location",
                "properties" : {
                  "startTime" : {
                    "@type" : "g:Int32",
                    "@value" : 2004
                  },
                  "endTime" : {
                    "@type" : "g:Int32",
                    "@value" : 2005
                  }
                }
              }
            }, {
              "@type" : "g:VertexProperty",
              "@value" : {
                "id" : {
                  "@type" : "g:Int64",
                  "@value" : 9
                },
                "value" : "santa fe",
                "vertex" : {
                  "@type" : "g:Int32",
                  "@value" : 1
                },
                "label" : "location",
                "properties" : {
                  "startTime" : {
                    "@type" : "g:Int32",
                    "@value" : 2005
                  }
                }
              }
            } ]
          }
        }
      }}}"#;
    let expected = Traverser {
        bulk: 1,
        value: Box::new(
            Vertex {
                id: Box::new(1_i32.into()),
                label: String::from("person"),
                properties: Some(vec![
                    VertexProperty::new(
                        0i64,
                        "name",
                        "marko",
                        Some(Vertex::new(1, "", None)),
                        None,
                    ),
                    VertexProperty::new(
                        6i64,
                        "location",
                        "san diego",
                        Some(Vertex::new(1, "", None)),
                        Some(vec![
                            Property::new("startTime", 1997, EitherParent::None),
                            Property::new("endTime", 2001, EitherParent::None),
                        ]),
                    ),
                    VertexProperty::new(
                        7i64,
                        "location",
                        "santa cruz",
                        Some(Vertex::new(1, "", None)),
                        Some(vec![
                            Property::new("startTime", 2001, EitherParent::None),
                            Property::new("endTime", 2004, EitherParent::None),
                        ]),
                    ),
                    VertexProperty::new(
                        8i64,
                        "location",
                        "brussels",
                        Some(Vertex::new(1, "", None)),
                        Some(vec![
                            Property::new("startTime", 2004, EitherParent::None),
                            Property::new("endTime", 2005, EitherParent::None),
                        ]),
                    ),
                    VertexProperty::new(
                        9i64,
                        "location",
                        "santa fe",
                        Some(Vertex::new(1, "", None)),
                        Some(vec![Property::new("startTime", 2005, EitherParent::None)]),
                    ),
                ]),
            }
            .into(),
        ),
    };

    let res: serde_json::Value = serde_json::from_str(s).unwrap();
    let mut res = Traverser::decode_v2(&res).unwrap();

    res.value
        .get_ref_mut::<Vertex>()
        .unwrap()
        .properties
        .as_mut()
        .into_iter()
        .for_each(|p| {
            for i in p {
                if i.properties.is_some() {
                    i.properties
                        .as_mut()
                        .unwrap()
                        .sort_by(|p1, p2| p1.key.cmp(&p2.key).reverse())
                }
            }
        });
    res.value
        .get_ref_mut::<Vertex>()
        .unwrap()
        .properties
        .as_mut()
        .unwrap()
        .sort_by(|p1, p2| {
            p1.id
                .get_ref::<i64>()
                .unwrap()
                .cmp(p2.id.get_ref::<i64>().unwrap())
        });
    assert_eq!(res, expected)
}
