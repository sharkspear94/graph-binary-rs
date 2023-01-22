use std::marker::PhantomData;

use serde_json::json;

use crate::{
    error::GraphSonError,
    structure::enums::{
        Barrier, Cardinality, Column, Direction, Merge, Operator, Order, Pick, Pop, Scope, TextP,
        P, T,
    },
    GremlinValue,
};

use super::{get_val_by_key_v2, get_val_by_key_v3, validate_type, DecodeGraphSON, EncodeGraphSON};

impl<T> EncodeGraphSON for P<T> {
    fn encode_v3(&self) -> serde_json::Value {
        match self.predicate.as_str() {
            "eq" | "neq" | "lt" | "lte" | "gt" | "gte" => json!({
                "@type" : "g:P",
                "@value" : {
                    "predicate" : self.predicate,
                    "value": self.value[0].encode_v3()
                }
            }),
            "between" | "inside" | "outside" | "within" | "without" => json!({
                "@type" : "g:P",
                "@value" :{
                    "predicate" : self.predicate,
                    "value":  self.value.encode_v3()
                }
            }),
            "and" | "or" => json!({
                "@type" : "g:P",
                "@value" : {
                    "predicate" : self.predicate,
                    "value":  self.value.iter().map(EncodeGraphSON::encode_v3).collect::<Vec<serde_json::Value>>()
                }
            }),
            //TODO replace with unreachable
            _ => panic!("predicate in P not known"),
        }
    }

    fn encode_v2(&self) -> serde_json::Value {
        match self.predicate.as_str() {
            "eq" | "neq" | "lt" | "lte" | "gt" | "gte" => json!({
                "@type" : "g:P",
                "@value" : {
                    "predicate" : self.predicate,
                    "value": self.value[0].encode_v2()
                }
            }),
            "between" | "inside" | "outside" | "within" | "without" => json!({
                "@type" : "g:P",
                "@value" :{
                    "predicate" : self.predicate,
                    "value":  self.value.encode_v2()
                }
            }),
            "and" | "or" => json!({
                "@type" : "g:P",
                "@value" : {
                    "predicate" : self.predicate,
                    "value":  self.value.iter().map(EncodeGraphSON::encode_v2).collect::<Vec<serde_json::Value>>()
                }
            }),
            //TODO replace with unreachable
            _ => panic!("predicate in P not known"),
        }
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

impl<T> DecodeGraphSON for P<T> {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:P")?;

        let predicate = get_val_by_key_v3::<String>(value_object, "predicate", "P")?;

        match predicate.as_ref() {
            "eq" | "neq" | "lt" | "lte" | "gt" | "gte" => {
                let value = get_val_by_key_v3(value_object, "value", "P")?;
                Ok(P {
                    predicate,
                    value: vec![value],
                    marker: PhantomData,
                })
            }
            "between" | "inside" | "outside" | "within" | "without" => {
                let value = get_val_by_key_v3(value_object, "value", "P")?;
                Ok(P {
                    predicate,
                    value,
                    marker: PhantomData,
                })
            }
            "and" | "or" => {
                let value_vec = value_object
                    .get("value")
                    .and_then(serde_json::Value::as_array)
                    .ok_or_else(|| GraphSonError::WrongJsonType("expected array".to_string()))?;
                let mut value = Vec::with_capacity(value_vec.len());
                for p in value_vec {
                    value.push(GremlinValue::decode_v3(p)?);
                }
                Ok(P {
                    predicate,
                    value,
                    marker: PhantomData,
                })
            }
            rest => Err(GraphSonError::WrongFixedValue(format!(
                "predicate not valid found: {rest}"
            ))),
        }
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:P")?;

        let predicate = get_val_by_key_v2::<String>(value_object, "predicate", "P")?;
        match predicate.as_ref() {
            "eq" | "neq" | "lt" | "lte" | "gt" | "gte" => {
                let value = get_val_by_key_v2(value_object, "value", "P")?;
                Ok(P {
                    predicate,
                    value: vec![value],
                    marker: PhantomData,
                })
            }
            "between" | "inside" | "outside" | "within" | "without" => {
                let value = get_val_by_key_v2(value_object, "value", "P")?;
                Ok(P {
                    predicate,
                    value,
                    marker: PhantomData,
                })
            }
            "and" | "or" => {
                let value_vec = value_object
                    .get("value")
                    .and_then(serde_json::Value::as_array)
                    .ok_or_else(|| GraphSonError::WrongJsonType("array".to_string()))?;
                let mut value = Vec::with_capacity(value_vec.len());
                for p in value_vec {
                    value.push(GremlinValue::decode_v2(p)?);
                }
                Ok(P {
                    predicate,
                    value,
                    marker: PhantomData,
                })
            }
            rest => Err(GraphSonError::WrongFixedValue(format!(
                "predicate not valid found: {rest}"
            ))),
        }
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

impl EncodeGraphSON for TextP {
    //FIXME need testing if values can be more than one
    fn encode_v3(&self) -> serde_json::Value {
        json!({
          "@type" : "g:TextP",
          "@value" : {
            "predicate" : self.predicate,
            "value" : self.value[0].encode_v3()
          }
        })
    }

    fn encode_v2(&self) -> serde_json::Value {
        json!({
          "@type" : "g:TextP",
          "@value" : {
            "predicate" : self.predicate,
            "value" : self.value[0].encode_v2()
          }
        })
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

impl DecodeGraphSON for TextP {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:TextP")?;

        let predicate = get_val_by_key_v3(value_object, "predicate", "TextP")?;
        let value = get_val_by_key_v3(value_object, "value", "TextP")?;
        Ok(TextP {
            predicate,
            value: vec![value],
        })
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        let value_object = validate_type(j_val, "g:TextP")?;

        let predicate = get_val_by_key_v2(value_object, "predicate", "TextP")?;
        let value = get_val_by_key_v2(value_object, "value", "TextP")?;

        Ok(TextP {
            predicate,
            value: vec![value],
        })
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, GraphSonError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

macro_rules! graph_son_impls {
    (  $($t:ident),*$(,)?) => {

        $(

            impl EncodeGraphSON for $t {
                fn encode_v3(&self) -> serde_json::Value {
                    json!({

                        "@type" : concat!("g:",stringify!($t)),
                        "@value" : self.as_str(),
                    })
                }

                fn encode_v2(&self) -> serde_json::Value {
                    self.encode_v3()
                }

                fn encode_v1(&self) -> serde_json::Value {
                    todo!()
                }
            }

            impl DecodeGraphSON for $t {
                fn decode_v3(j_val: &serde_json::Value) -> Result<Self, GraphSonError>
                where
                    Self: std::marker::Sized,
                {
                    let value_object = validate_type(j_val, concat!("g:",stringify!($t)))?;
                    let s = value_object.as_str().ok_or_else(|| GraphSonError::WrongJsonType("str".to_string()))?;
                    <$t>::try_from(s).map_err(|err| GraphSonError::TryFrom(err.to_string()))
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
        )*
    }
}

graph_son_impls!(
    Barrier,
    Cardinality,
    Column,
    Direction,
    Merge,
    Operator,
    Order,
    Pick,
    Pop,
    Scope,
    T,
);

#[test]
fn p_encode_v3() {
    let expected = r#"{"@type":"g:P","@value":{"predicate":"between","value":{"@type":"g:List","@value":[{"@type":"g:Int32","@value":1},{"@type":"g:Int32","@value":10}]}}}"#;

    let p = P::between(1, 10);

    let res = serde_json::to_string(&p.encode_v3()).unwrap();

    assert_eq!(res, expected);
}

#[test]
fn p_decode_v3() {
    let s = r#"{"@type":"g:P","@value":{"predicate":"between","value":{"@type":"g:List","@value":[{"@type":"g:Int32","@value":1},{"@type":"g:Int32","@value":10}]}}}"#;

    let expected = P::between(1, 10);

    let v = serde_json::from_str(s).unwrap();
    let res = P::decode_v3(&v).unwrap();
    assert_eq!(res, expected);
}

#[test]
fn p_and_decode_v3() {
    let s = r#"{
        "@type" : "g:P",
        "@value" : {
          "predicate" : "or",
          "value" : [ {
            "@type" : "g:P",
            "@value" : {
              "predicate" : "eq",
              "value" : {
                "@type" : "g:Int32",
                "@value" : 0
              }
            }
          }, {
            "@type" : "g:P",
            "@value" : {
              "predicate" : "gt",
              "value" : {
                "@type" : "g:Int32",
                "@value" : 10
              }
            }
          } ]
        }
      }"#;

    let expected = P::eq(0).or(P::gt(10));

    let v = serde_json::from_str(s).unwrap();
    let res = P::decode_v3(&v).unwrap();
    assert_eq!(res, expected);
}

#[test]
fn p_encode_v2() {
    let expected = r#"{"@type":"g:P","@value":{"predicate":"between","value":[{"@type":"g:Int32","@value":1},{"@type":"g:Int32","@value":10}]}}"#;

    let p = P::between(1, 10);

    let res = serde_json::to_string(&p.encode_v2()).unwrap();

    assert_eq!(res, expected);
}

#[test]
fn p_decode_v2() {
    let s = r#"{"@type":"g:P","@value":{"predicate":"between","value":[{"@type":"g:Int32","@value":1},{"@type":"g:Int32","@value":10}]}}"#;

    let expected = P::between(1, 10);

    let v = serde_json::from_str(s).unwrap();
    let res = P::decode_v2(&v).unwrap();
    assert_eq!(res, expected);
}

#[test]
fn p_and_decode_v2() {
    let s = r#"{
        "@type" : "g:P",
        "@value" : {
          "predicate" : "and",
          "value" : [ {
            "@type" : "g:P",
            "@value" : {
              "predicate" : "gt",
              "value" : {
                "@type" : "g:Int32",
                "@value" : 0
              }
            }
          }, {
            "@type" : "g:P",
            "@value" : {
              "predicate" : "lt",
              "value" : {
                "@type" : "g:Int32",
                "@value" : 10
              }
            }
          } ]
        }
      }"#;

    let expected = P::gt(0).and(P::lt(10));

    let v = serde_json::from_str(s).unwrap();
    let res = P::decode_v2(&v).unwrap();
    assert_eq!(res, expected);
}
