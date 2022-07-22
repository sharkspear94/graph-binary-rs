use std::fmt::Display;
use std::marker::PhantomData;

use crate::{error::DecodeError, specs::CoreType, GremlinValue};

#[cfg(feature = "graph_binary")]
use crate::graph_binary::{Decode, Encode};

#[cfg(feature = "graph_son")]
use crate::{
    graphson::{validate_type_entry, DecodeGraphSON, EncodeGraphSON},
    val_by_key_v2, val_by_key_v3,
};
#[cfg(feature = "graph_son")]
use serde_json::json;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Barrier {
    NormSack,
}

impl TryFrom<&str> for Barrier {
    type Error = DecodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "normSack" => Ok(Barrier::NormSack),
            _ => Err(DecodeError::ConvertError("Barrier".to_string())),
        }
    }
}

impl Barrier {
    fn as_str(&self) -> &str {
        match self {
            Barrier::NormSack => "normSack",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Cardinality {
    List,
    Set,
    Single,
}

impl TryFrom<&str> for Cardinality {
    type Error = DecodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "list" => Ok(Cardinality::List),
            "set" => Ok(Cardinality::Set),
            "single" => Ok(Cardinality::Single),
            _ => Err(DecodeError::ConvertError("Cardinality".to_string())),
        }
    }
}

impl Cardinality {
    fn as_str(&self) -> &str {
        match self {
            Cardinality::List => "list",
            Cardinality::Set => "set",
            Cardinality::Single => "single",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Column {
    Keys,
    Values,
}

impl Column {
    fn as_str(&self) -> &str {
        match self {
            Column::Keys => "keys",
            Column::Values => "values",
        }
    }
}

impl TryFrom<&str> for Column {
    type Error = DecodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "keys" => Ok(Column::Keys),
            "values" => Ok(Column::Values),
            _ => Err(DecodeError::ConvertError("Column".to_string())),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    Both,
    In,
    Out,
}

impl TryFrom<&str> for Direction {
    type Error = DecodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "BOTH" => Ok(Direction::Both),
            "IN" => Ok(Direction::In),
            "OUT" => Ok(Direction::Out),
            _ => Err(DecodeError::ConvertError("Direction".to_string())),
        }
    }
}

impl Direction {
    fn as_str(&self) -> &str {
        match self {
            Direction::Both => "BOTH",
            Direction::In => "IN",
            Direction::Out => "OUT",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Operator {
    AddAll,
    And,
    Assign,
    Div,
    Max,
    Min,
    Minus,
    Mult,
    Or,
    Sum,
    SumLong,
}

impl Operator {
    fn as_str(&self) -> &str {
        match self {
            Operator::AddAll => "addAll",
            Operator::And => "and",
            Operator::Assign => "assign",
            Operator::Div => "div",
            Operator::Max => "max",
            Operator::Min => "min",
            Operator::Minus => "minus",
            Operator::Mult => "mult",
            Operator::Or => "or",
            Operator::Sum => "sum",
            Operator::SumLong => "sumLong",
        }
    }
}

impl TryFrom<&str> for Operator {
    type Error = DecodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "addAll" => Ok(Operator::AddAll),
            "and" => Ok(Operator::And),
            "assign" => Ok(Operator::Assign),
            "div" => Ok(Operator::Div),
            "max" => Ok(Operator::Max),
            "min" => Ok(Operator::Min),
            "minus" => Ok(Operator::Minus),
            "mult" => Ok(Operator::Mult),
            "or" => Ok(Operator::Or),
            "sum" => Ok(Operator::Sum),
            "sumLong" => Ok(Operator::SumLong),
            _ => Err(DecodeError::ConvertError("Operator".to_string())),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Order {
    Shuffle,
    Asc,
    Desc,
}

impl TryFrom<&str> for Order {
    type Error = DecodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "shuffle" => Ok(Order::Shuffle),
            "asc" => Ok(Order::Asc),
            "desc" => Ok(Order::Desc),
            _ => Err(DecodeError::ConvertError("Order".to_string())),
        }
    }
}

impl Order {
    fn as_str(&self) -> &str {
        match self {
            Order::Shuffle => "shuffle",
            Order::Asc => "asc",
            Order::Desc => "desc",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Pick {
    Any,
    None,
}

impl TryFrom<&str> for Pick {
    type Error = DecodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "any" => Ok(Pick::Any),
            "none" => Ok(Pick::None),
            _ => Err(DecodeError::ConvertError("Pick".to_string())),
        }
    }
}

impl Pick {
    fn as_str(&self) -> &str {
        match self {
            Pick::Any => "any",
            Pick::None => "none",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Pop {
    All,
    First,
    Last,
    Mixed,
}

impl TryFrom<&str> for Pop {
    type Error = DecodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "all" => Ok(Pop::All),
            "first" => Ok(Pop::First),
            "last" => Ok(Pop::Last),
            "mixed" => Ok(Pop::Mixed),
            _ => Err(DecodeError::ConvertError("Pop".to_string())),
        }
    }
}

impl Pop {
    fn as_str(&self) -> &str {
        match self {
            Pop::All => "all",
            Pop::First => "first",
            Pop::Last => "last",
            Pop::Mixed => "mixed",
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct P<T> {
    predicate: String,
    value: Vec<GremlinValue>,
    marker: PhantomData<T>,
}

impl<T: Into<GremlinValue>> P<T> {
    pub fn eq(a: T) -> P<T> {
        P {
            predicate: "eq".to_string(),
            value: vec![a.into()],
            marker: PhantomData,
        }
    }

    pub fn neq(a: T) -> P<T> {
        P {
            predicate: "neq".to_string(),
            value: vec![a.into()],
            marker: PhantomData,
        }
    }

    pub fn gt(a: T) -> P<T> {
        P {
            predicate: "gt".to_string(),
            value: vec![a.into()],
            marker: PhantomData,
        }
    }

    pub fn gte(a: T) -> P<T> {
        P {
            predicate: "gte".to_string(),
            value: vec![a.into()],
            marker: PhantomData,
        }
    }

    pub fn lt(a: T) -> P<T> {
        P {
            predicate: "lt".to_string(),
            value: vec![a.into()],
            marker: PhantomData,
        }
    }

    pub fn lte(a: T) -> P<T> {
        P {
            predicate: "lte".to_string(),
            value: vec![a.into()],
            marker: PhantomData,
        }
    }

    pub fn between(first: T, second: T) -> P<T> {
        P {
            predicate: "between".to_string(),
            value: vec![first.into(), second.into()],
            marker: PhantomData,
        }
    }

    pub fn inside(first: T, second: T) -> P<T> {
        P {
            predicate: "inside".to_string(),
            value: vec![first.into(), second.into()],
            marker: PhantomData,
        }
    }

    pub fn outside(first: T, second: T) -> P<T> {
        P {
            predicate: "outside".to_string(),
            value: vec![first.into(), second.into()],
            marker: PhantomData,
        }
    }

    pub fn within(values: Vec<T>) -> P<T> {
        P {
            predicate: "within".to_string(),
            value: values.into_iter().map(Into::into).collect(),
            marker: PhantomData,
        }
    }

    pub fn without(values: Vec<T>) -> P<T> {
        P {
            predicate: "without".to_string(),
            value: values.into_iter().map(Into::into).collect(),
            marker: PhantomData,
        }
    }

    pub fn and(self, f: P<T>) -> P<T> {
        P {
            predicate: "and".to_string(),
            value: vec![self.into(), f.into()],
            marker: PhantomData,
        }
    }

    pub fn or(self, f: P<T>) -> P<T> {
        P {
            predicate: "or".to_string(),
            value: vec![self.into(), f.into()],
            marker: PhantomData,
        }
    }
}

#[cfg(feature = "graph_son")]
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
                    "value":  self.value.iter().map(|t|t.encode_v3()).collect::<Vec<serde_json::Value>>()
                }
            }),
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
                    "value":  self.value.iter().map(|t|t.encode_v2()).collect::<Vec<serde_json::Value>>()
                }
            }),
            _ => panic!("predicate in P not known"),
        }
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

#[cfg(feature = "graph_son")]
impl<T> DecodeGraphSON for P<T> {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let object = j_val
            .as_object()
            .filter(|map| validate_type_entry(*map, "g:P"))
            .and_then(|map| map.get("@value"));

        let predicate = val_by_key_v3!(object, "predicate", String, "P")?;
        match predicate.as_ref() {
            "eq" | "neq" | "lt" | "lte" | "gt" | "gte" => {
                let value = val_by_key_v3!(object, "value", GremlinValue, "P")?;
                Ok(P {
                    predicate,
                    value: vec![value],
                    marker: PhantomData,
                })
            }
            "between" | "inside" | "outside" | "within" | "without" => {
                let value = val_by_key_v3!(object, "value", Vec<GremlinValue>, "P")?;
                Ok(P {
                    predicate,
                    value,
                    marker: PhantomData,
                })
            }
            "and" | "or" => {
                let value_vec = object
                    .and_then(|a| a.get("value"))
                    .and_then(|a| a.as_array())
                    .ok_or_else(|| {
                        DecodeError::DecodeError(
                            "and,or predicate in P does not as array as value".to_string(),
                        )
                    })?;
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
            error => Err(DecodeError::DecodeError(format!(
                "predicate :{error} in P is not valid"
            ))),
        }
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let object = j_val
            .as_object()
            .filter(|map| validate_type_entry(*map, "g:P"))
            .and_then(|map| map.get("@value"));

        let predicate = val_by_key_v2!(object, "predicate", String, "P")?;
        match predicate.as_ref() {
            "eq" | "neq" | "lt" | "lte" | "gt" | "gte" => {
                let value = val_by_key_v2!(object, "value", GremlinValue, "P")?;
                Ok(P {
                    predicate,
                    value: vec![value],
                    marker: PhantomData,
                })
            }
            "between" | "inside" | "outside" | "within" | "without" => {
                let value = val_by_key_v2!(object, "value", Vec<GremlinValue>, "P")?;
                Ok(P {
                    predicate,
                    value,
                    marker: PhantomData,
                })
            }
            "and" | "or" => {
                let value_vec = object
                    .and_then(|a| a.get("value"))
                    .and_then(|a| a.as_array())
                    .ok_or_else(|| {
                        DecodeError::DecodeError(
                            "and,or predicate in P does not as array as value".to_string(),
                        )
                    })?;
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
            error => Err(DecodeError::DecodeError(format!(
                "predicate :{error} in P is not valid"
            ))),
        }
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

impl<T: Into<GremlinValue>> From<P<T>> for GremlinValue {
    fn from(p: P<T>) -> Self {
        GremlinValue::P(P {
            predicate: p.predicate,
            value: p.value,
            marker: PhantomData,
        })
    }
}

#[cfg(feature = "graph_binary")]
impl<T> Encode for P<T> {
    fn type_code() -> u8 {
        CoreType::P.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.predicate.encode(writer)?;
        self.value.partial_encode(writer)
    }
}

#[cfg(feature = "graph_binary")]
impl<T> Decode for P<T> {
    fn expected_type_code() -> u8 {
        CoreType::P.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let predicate = String::decode(reader)?;
        let value = Vec::<GremlinValue>::partial_decode(reader)?;

        Ok(P {
            predicate,
            value,
            marker: PhantomData,
        })
    }
}

impl Display for P<GremlinValue> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "p:{}", self.predicate)?;
        write!(f, "value:")?;
        for i in &self.value {
            write!(f, "{i},")?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Scope {
    Local,
    Global,
}

impl Scope {
    fn as_str(&self) -> &str {
        match self {
            Scope::Local => "local",
            Scope::Global => "global",
        }
    }
}

impl TryFrom<&str> for Scope {
    type Error = DecodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "local" => Ok(Scope::Local),
            "global" => Ok(Scope::Global),
            _ => Err(DecodeError::ConvertError("Scope".to_string())),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum T {
    Id,
    Key,
    Label,
    Value,
}

impl T {
    fn as_str(&self) -> &str {
        match self {
            T::Id => "id",
            T::Key => "key",
            T::Label => "label",
            T::Value => "value",
        }
    }
}

impl TryFrom<&str> for T {
    type Error = DecodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "id" => Ok(T::Id),
            "key" => Ok(T::Key),
            "label" => Ok(T::Label),
            "value" => Ok(T::Value),
            _ => Err(DecodeError::ConvertError("T".to_string())),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TextP {
    predicate: String,
    value: Vec<GremlinValue>,
}

impl From<TextP> for GremlinValue {
    fn from(text_p: TextP) -> Self {
        GremlinValue::TextP(text_p)
    }
}

impl TextP {
    pub fn starting_with(val: &str) -> Self {
        TextP {
            predicate: "startingWith".to_string(),
            value: vec![val.into()],
        }
    }
    pub fn not_starting_with(val: &str) -> Self {
        TextP {
            predicate: "notStartingWith".to_string(),
            value: vec![val.into()],
        }
    }
    pub fn ending_with(val: &str) -> Self {
        TextP {
            predicate: "endingWith".to_string(),
            value: vec![val.into()],
        }
    }
    pub fn not_ending_with(val: &str) -> Self {
        TextP {
            predicate: "notEndingWith".to_string(),
            value: vec![val.into()],
        }
    }
    pub fn containing(val: &str) -> Self {
        TextP {
            predicate: "containing".to_string(),
            value: vec![val.into()],
        }
    }
    pub fn not_containing(val: &str) -> Self {
        TextP {
            predicate: "notContaining".to_string(),
            value: vec![val.into()],
        }
    }
    pub fn regex(val: &str) -> Self {
        TextP {
            predicate: "regex".to_string(),
            value: vec![val.into()],
        }
    }
    pub fn not_regex(val: &str) -> Self {
        TextP {
            predicate: "notRegex".to_string(),
            value: vec![val.into()],
        }
    }

    pub fn and(self, text_p: TextP) -> P<TextP> {
        P {
            predicate: "and".to_string(),
            value: vec![self.into(), text_p.into()],
            marker: PhantomData,
        }
    }

    pub fn or(self, text_p: TextP) -> P<TextP> {
        P {
            predicate: "or".to_string(),
            value: vec![self.into(), text_p.into()],
            marker: PhantomData,
        }
    }
}

#[cfg(feature = "graph_binary")]
impl Encode for TextP {
    fn type_code() -> u8 {
        CoreType::TextP.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.predicate.encode(writer)?;
        self.value.partial_encode(writer)
    }
}

#[cfg(feature = "graph_binary")]
impl Decode for TextP {
    fn expected_type_code() -> u8 {
        CoreType::TextP.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let predicate = String::decode(reader)?;
        let value = Vec::<GremlinValue>::partial_decode(reader)?;

        Ok(TextP { predicate, value })
    }
}

impl Display for TextP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "predicate {}", self.predicate)?;
        write! {f,"value:"}?;
        for i in &self.value {
            write!(f, "{},", i)?;
        }
        Ok(())
    }
}

#[cfg(feature = "graph_son")]
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

#[cfg(feature = "graph_son")]
impl DecodeGraphSON for TextP {
    fn decode_v3(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let object = j_val
            .as_object()
            .filter(|map| validate_type_entry(*map, "g:TextP"))
            .and_then(|m| m.get("@value"))
            .and_then(|m| m.as_object());

        let predicate = val_by_key_v3!(object, "predicate", String, "TextP")?;
        let value = val_by_key_v3!(object, "value", GremlinValue, "TextP")?;
        Ok(TextP {
            predicate,
            value: vec![value],
        })
    }

    fn decode_v2(j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let object = j_val
            .as_object()
            .filter(|map| validate_type_entry(*map, "g:TextP"))
            .and_then(|m| m.get("@value"))
            .and_then(|m| m.as_object());

        let predicate = val_by_key_v2!(object, "predicate", String, "TextP")?;
        let value = val_by_key_v2!(object, "value", GremlinValue, "TextP")?;
        Ok(TextP {
            predicate,
            value: vec![value],
        })
    }

    fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Merge {
    OnCreate,
    OnMatch,
}

impl Merge {
    fn as_str(&self) -> &str {
        match self {
            Merge::OnCreate => "onCreate",
            Merge::OnMatch => "onMatch",
        }
    }
}

impl TryFrom<&str> for Merge {
    type Error = DecodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "onCreate" => Ok(Merge::OnCreate),
            "onMatch" => Ok(Merge::OnMatch),
            _ => Err(DecodeError::ConvertError("Merge".to_string())),
        }
    }
}

#[cfg(feature = "graph_binary")]
#[macro_export]
macro_rules! graph_binary_impls {
    (  $($t:ident),*$(,)? ) => {

        $(
        impl Encode for $t {
            fn type_code() -> u8 {
                CoreType::$t.into()
            }

            fn partial_encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), $crate::error::EncodeError> {
                self.as_str().encode(writer)
            }
        }

        impl Decode for $t {

            fn expected_type_code() -> u8 {
                CoreType::$t.into()
            }

            fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, $crate::error::DecodeError>
            where
                Self: std::marker::Sized,
            {
                $t::try_from(String::decode(reader)?.as_str())
            }
        }
    )*
    };
}

#[macro_export]
macro_rules! enum_conversion {
    (  $($t:ident),*$(,)? ) => {

        $(
            impl From<$t> for GremlinValue {
                fn from(val: $t) -> Self {
                    GremlinValue::$t(val)
                }
            }

            impl Display for $t {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f,"{}",self.as_str())
                }
            }

            impl TryFrom<GremlinValue> for $t {
                type Error = $crate::error::DecodeError;

                fn try_from(value: GremlinValue) -> Result<Self, Self::Error> {
                    match value {
                        GremlinValue::$t(val) => Ok(val),
                        _ => Err($crate::error::DecodeError::ConvertError(
                            format!("cannot convert GremlinValue to {}",stringify!($t))
                        )),
                    }
                }
            }

            impl $crate::macros::TryBorrowFrom for $t {
                fn try_borrow_from(graph_binary: &GremlinValue) -> Option<&Self> {
                    match graph_binary {
                        GremlinValue::$t(val) => Some(val),
                        _ => None
                    }
                }
            }

            impl $crate::macros::TryMutBorrowFrom for $t {
                fn try_mut_borrow_from(graph_binary: &mut GremlinValue) -> Option<&mut Self> {
                    match graph_binary {
                        GremlinValue::$t(val) => Some(val),
                        _ => None
                    }
                }
            }
        )*
    }
}

#[cfg(feature = "graph_son")]
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
                fn decode_v3(j_val: &serde_json::Value) -> Result<Self, DecodeError>
                where
                    Self: std::marker::Sized,
                {
                    j_val
                        .as_object()
                        .filter(|map| validate_type_entry(*map, concat!("g:",stringify!($t))))
                        .and_then(|map| map.get("@value"))
                        .and_then(|value| value.as_str())
                        .map(<$t>::try_from)
                        .unwrap_or_else(||
                            Err(DecodeError::DecodeError(
                                concat!("g:",stringify!($t)).to_string()
                            )))
                }

                fn decode_v2(j_val: &serde_json::Value) -> Result<Self, DecodeError>
                where
                    Self: std::marker::Sized,
                {
                    Self::decode_v3(j_val)
                }

                fn decode_v1(_j_val: &serde_json::Value) -> Result<Self, DecodeError>
                where
                    Self: std::marker::Sized,
                {
                    todo!()
                }
            }
        )*
    }
}

#[cfg(feature = "graph_binary")]
graph_binary_impls!(
    Barrier,
    Cardinality,
    Column,
    Direction,
    Operator,
    Order,
    Pick,
    Pop,
    Scope,
    T,
    Merge
);

enum_conversion!(
    Barrier,
    Cardinality,
    Column,
    Direction,
    Operator,
    Order,
    Pick,
    Pop,
    Scope,
    T,
    Merge
);

#[cfg(feature = "graph_son")]
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
fn t_decode_test() {
    let reader = vec![0x03, 0x0, 0x0, 0x0, 0x0, 0x02, b'i', b'd'];

    let p = T::partial_decode(&mut &reader[..]);

    assert_eq!(T::Id, p.unwrap());
}

#[test]
fn p_decode() {
    let reader = vec![
        0x03, 0x0, 0x0, 0x0, 0x0, 0x07, b'w', b'i', b't', b'h', b'o', b'u', b't', 0x0, 0x0, 0x0,
        0x03, 0x1, 0x0, 0x0, 0x0, 0x0, 0x01, 0x01, 0x0, 0x0, 0x0, 0x0, 0x2, 0x01, 0x00, 0x0, 0x0,
        0x0, 0x3,
    ];

    let p = P::<GremlinValue>::partial_decode(&mut &reader[..]);

    assert_eq!(P::without(vec![1.into(), 2.into(), 3.into()]), p.unwrap());
}

#[test]
fn p_decode_inside() {
    let reader = vec![
        0x03, 0x0, 0x0, 0x0, 0x0, 0x06, b'i', b'n', b's', b'i', b'd', b'e', 0x0, 0x0, 0x0, 0x02,
        0x1, 0x0, 0x0, 0x0, 0x0, 0x01, 0x01, 0x0, 0x0, 0x0, 0x0, 0xff,
    ];

    let p = P::<i32>::partial_decode(&mut &reader[..]);

    assert_eq!(P::inside(1, 255), p.unwrap());
}

#[test]
fn p_encode() {
    let expected = [
        0x03, 0x0, 0x0, 0x0, 0x0, 0x07, b'b', b'e', b't', b'w', b'e', b'e', b'n', 0x0, 0x0, 0x0,
        0x02, 0x1, 0x0, 0x0, 0x0, 0x0, 0x01, 0x01, 0x0, 0x0, 0x0, 0x0, 0x0a,
    ];

    let p = P::between(1, 10);
    let mut w = vec![];
    p.partial_encode(&mut w).unwrap();

    assert_eq!(w, expected);
}

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

#[test]
fn text_p_decode() {
    let reader = vec![
        0x28, 0x00, 0x03, 0x0, 0x0, 0x0, 0x0, 0x0c, b's', b't', b'a', b'r', b't', b'i', b'n', b'g',
        b'W', b'i', b't', b'h', 0x0, 0x0, 0x0, 0x01, 0x3, 0x0, 0x0, 0x0, 0x0, 0x04, b't', b'e',
        b's', b't',
    ];

    let p = TextP::decode(&mut &reader[..]).unwrap();
    assert_eq!(p, TextP::starting_with("test"));
}
