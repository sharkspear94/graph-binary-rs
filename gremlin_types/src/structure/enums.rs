use serde::Deserialize;
use serde_json::json;
use std::fmt::Display;
use std::marker::PhantomData;

use super::validate_type_entry;
use crate::{
    error::DecodeError,
    graph_binary::{decode, Decode, Encode, GremlinTypes},
    graphson::{DecodeGraphSON, EncodeGraphSON},
    specs::CoreType,
};

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

    fn to() -> Self {
        Direction::In
    }
    fn from() -> Self {
        Direction::Out
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
pub enum P {
    Eq(Box<GremlinTypes>),
    Neq(Box<GremlinTypes>),
    Lt(Box<GremlinTypes>),
    Lte(Box<GremlinTypes>),
    Gt(Box<GremlinTypes>),
    Gte(Box<GremlinTypes>),
    Inside(Box<(GremlinTypes, GremlinTypes)>),
    Outside(Box<(GremlinTypes, GremlinTypes)>),
    Between(Box<(GremlinTypes, GremlinTypes)>),
    Within(Vec<GremlinTypes>),
    Without(Vec<GremlinTypes>),
}
pub struct PublicP<T: Into<GremlinTypes> + Clone> {
    p: P,
    marker: PhantomData<T>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PublicP2<V: Into<GremlinTypes>> {
    Eq(T),
    Neq(T),
    Lt(T),
    Lte(T),
    Gt(T),
    Gte(T),
    Inside((V, V)),
    Outside((V, V)),
    Between((V, V)),
    Within(Vec<V>),
    Without(Vec<V>),
}

impl<V: Into<GremlinTypes>> PublicP2<V> {
    pub fn into_p(self) -> P {
        match self {
            PublicP2::Eq(val) => P::Eq(Box::new(val.into())),
            PublicP2::Neq(val) => P::Neq(Box::new(val.into())),
            PublicP2::Lt(val) => P::Lt(Box::new(val.into())),
            PublicP2::Lte(val) => P::Lte(Box::new(val.into())),
            PublicP2::Gt(val) => P::Gt(Box::new(val.into())),
            PublicP2::Gte(val) => P::Gte(Box::new(val.into())),
            PublicP2::Inside(val) => P::Inside(Box::new((val.0.into(), val.1.into()))),
            PublicP2::Outside(val) => P::Outside(Box::new((val.0.into(), val.1.into()))),
            PublicP2::Between(val) => P::Between(Box::new((val.0.into(), val.1.into()))),
            PublicP2::Within(val) => P::Within(val.into_iter().map(Into::into).collect()),
            PublicP2::Without(val) => P::Without(val.into_iter().map(Into::into).collect()),
        }
    }
}

impl<V: Into<GremlinTypes>> Display for PublicP2<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<T: Into<GremlinTypes> + Clone> PublicP<T> {
    pub fn eq(val: T) -> P {
        P::Eq(Box::new(val.into()))
    }
    pub fn neq(val: T) -> P {
        P::Neq(Box::new(val.into()))
    }

    pub fn lt(val: T) -> P {
        P::Lt(Box::new(val.into()))
    }

    pub fn lte(val: T) -> P {
        P::Lte(Box::new(val.into()))
    }

    pub fn gt(val: T) -> P {
        P::Gt(Box::new(val.into()))
    }

    pub fn gte(val: T) -> P {
        P::Gte(Box::new(val.into()))
    }

    pub fn between(lower: T, upper: T) -> P {
        P::Between(Box::new((lower.into(), upper.into())))
    }

    pub fn inside(lower: T, upper: T) -> P {
        P::Inside(Box::new((lower.into(), upper.into())))
    }

    pub fn outside(lower: T, upper: T) -> P {
        P::Outside(Box::new((lower.into(), upper.into())))
    }

    pub fn within(val: &[T]) -> P {
        P::Within(val.iter().map(Into::into).collect())
    }

    pub fn without(val: &[T]) -> P {
        P::Without(val.iter().map(Into::into).collect())
    }
}

impl From<P> for GremlinTypes {
    fn from(p: P) -> Self {
        GremlinTypes::P(p)
    }
}
impl P {
    fn as_str(&self) -> &str {
        match self {
            P::Eq(_) => "eq",
            P::Neq(_) => "neq",
            P::Lt(_) => "lt",
            P::Lte(_) => "lte",
            P::Gt(_) => "gt",
            P::Gte(_) => "gte",
            P::Inside(_) => "inside",
            P::Outside(_) => "outside",
            P::Between(_) => "between",
            P::Within(_) => "within",
            P::Without(_) => "without",
        }
    }

    fn single_value(&self) -> &GremlinTypes {
        match self {
            P::Eq(val) => val,
            P::Neq(val) => val,
            P::Lt(val) => val,
            P::Lte(val) => val,
            P::Gt(val) => val,
            P::Gte(val) => val,
            _ => panic!(),
        }
    }
    fn tuple_value(&self) -> &(GremlinTypes, GremlinTypes) {
        match self {
            P::Inside(val) => val,
            P::Outside(val) => val,
            P::Between(val) => val,
            _ => panic!(),
        }
    }

    fn list_value(&self) -> &Vec<GremlinTypes> {
        match self {
            P::Within(val) => val,
            P::Without(val) => val,
            _ => panic!(),
        }
    }

    pub fn eq<T: Into<GremlinTypes> + PartialEq>(val: T) -> Self {
        P::Eq(Box::new(val.into()))
    }
    pub fn neq<T: Into<GremlinTypes> + PartialEq>(val: T) -> Self {
        P::Neq(Box::new(val.into()))
    }

    pub fn lt<T: Into<GremlinTypes> + PartialOrd>(val: T) -> Self {
        P::Lt(Box::new(val.into()))
    }

    pub fn lte<T: Into<GremlinTypes> + PartialOrd>(val: T) -> Self {
        P::Lte(Box::new(val.into()))
    }

    pub fn gt<T: Into<GremlinTypes> + PartialOrd>(val: T) -> Self {
        P::Gt(Box::new(val.into()))
    }

    pub fn gte<T: Into<GremlinTypes> + PartialOrd>(val: T) -> Self {
        P::Gte(Box::new(val.into()))
    }

    pub fn between<T: Into<GremlinTypes> + PartialOrd>(lower: T, upper: T) -> Self {
        P::Between(Box::new((lower.into(), upper.into())))
    }

    pub fn inside<T: Into<GremlinTypes> + PartialOrd>(lower: T, upper: T) -> Self {
        P::Inside(Box::new((lower.into(), upper.into())))
    }

    pub fn outside<T: Into<GremlinTypes> + PartialOrd>(lower: T, upper: T) -> Self {
        P::Outside(Box::new((lower.into(), upper.into())))
    }

    pub fn within<T: Into<GremlinTypes> + Clone + PartialOrd>(val: &[T]) -> Self {
        P::Within(val.iter().map(Into::into).collect())
    }

    pub fn without<T: Into<GremlinTypes> + Clone + PartialOrd>(val: &[T]) -> Self {
        P::Without(val.iter().map(Into::into).collect())
    }

    fn write_variant<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        match self {
            P::Eq(v) => write_name_value("eq", v, writer),
            P::Neq(v) => write_name_value("neq", v, writer),
            P::Lt(v) => write_name_value("lt", v, writer),
            P::Lte(v) => write_name_value("lte", v, writer),
            P::Gt(v) => write_name_value("gt", v, writer),
            P::Gte(v) => write_name_value("gte", v, writer),
            P::Inside(v) => {
                "inside".encode(writer)?;
                2_i32.partial_encode(writer)?; // len of tuple only len = 2 will be used
                v.0.encode(writer)?;
                v.1.encode(writer)
            }
            P::Outside(v) => {
                "outside".encode(writer)?;
                2_i32.partial_encode(writer)?; // len of tuple only len = 2 will be used
                v.0.encode(writer)?;
                v.1.encode(writer)
            }
            P::Between(v) => {
                "between".encode(writer)?;
                2_i32.partial_encode(writer)?; // len of tuple only len = 2 will be used
                v.0.encode(writer)?;
                v.1.encode(writer)
            }
            P::Within(v) => {
                "within".encode(writer)?;
                v.partial_encode(writer)
            }
            P::Without(v) => {
                "without".encode(writer)?;
                v.partial_encode(writer)
            }
        }
    }
}

fn write_name_value<W: std::io::Write>(
    name: &str,
    value: &GremlinTypes,
    writer: &mut W,
) -> Result<(), crate::error::EncodeError> {
    name.partial_encode(writer)?;
    1_i32.partial_encode(writer)?; // value length
    value.encode(writer)
}

impl Encode for P {
    fn type_code() -> u8 {
        CoreType::P.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.write_variant(writer)
    }
}

impl serde::Serialize for P {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut buf = Vec::with_capacity(32);
        self.encode(&mut buf).expect("error during write of P");
        serializer.serialize_bytes(&buf[..])
    }
}

impl Decode for P {
    fn expected_type_code() -> u8 {
        CoreType::P.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        match String::decode(reader)?.as_str() {
            "eq" => Ok({
                i32::partial_decode(reader)?;
                P::Eq(Box::new(decode(reader)?))
            }),
            "neq" => {
                i32::partial_decode(reader)?;
                Ok(P::Neq(Box::new(decode(reader)?)))
            }
            "lt" => {
                i32::partial_decode(reader)?;
                Ok(P::Lt(Box::new(decode(reader)?)))
            }
            "lte" => {
                i32::partial_decode(reader)?;
                Ok(P::Lte(Box::new(decode(reader)?)))
            }
            "gt" => {
                i32::partial_decode(reader)?;
                Ok(P::Gt(Box::new(decode(reader)?)))
            }
            "gte" => {
                i32::partial_decode(reader)?;
                Ok(P::Gte(Box::new(decode(reader)?)))
            }
            "inside" => {
                i32::partial_decode(reader)?;
                Ok(P::Inside(Box::new((decode(reader)?, decode(reader)?))))
            }
            "outside" => {
                i32::partial_decode(reader)?;
                Ok(P::Outside(Box::new((decode(reader)?, decode(reader)?))))
            }
            "between" => {
                i32::partial_decode(reader)?;
                Ok(P::Between(Box::new((decode(reader)?, decode(reader)?))))
            }
            "within" => Ok(P::Within(Vec::partial_decode(reader)?)),
            "without" => Ok(P::Without(Vec::partial_decode(reader)?)),
            v => Err(DecodeError::DecodeError(format!(
                "expected P found variant text: {}",
                v
            ))),
        }
    }

    fn get_partial_len(bytes: &[u8]) -> Result<usize, DecodeError> {
        let mut len = String::get_len(bytes)?;
        len += Vec::<GremlinTypes>::get_partial_len(&bytes[len..])?;
        Ok(len)
    }
}

impl EncodeGraphSON for P {
    fn encode_v3(&self) -> serde_json::Value {
        match self {
            P::Eq(val) => json!({
              "@type" : "g:P",
              "@value" : {
                "predicate" : "eq",
                "value" : val.encode_v3()
              }
            }),
            P::Neq(val) => json!({
              "@type" : "g:P",
              "@value" : {
                "predicate" : "neq",
                "value" : val.encode_v3()
              }
            }),
            P::Lt(val) => json!({
              "@type" : "g:P",
              "@value" : {
                "predicate" : "lt",
                "value" : val.encode_v3()
              }
            }),
            P::Lte(val) => json!({
              "@type" : "g:P",
              "@value" : {
                "predicate" : "lte",
                "value" : val.encode_v3()
              }
            }),
            P::Gt(val) => json!({
              "@type" : "g:P",
              "@value" : {
                "predicate" : "gt",
                "value" : val.encode_v3()
              }
            }),
            P::Gte(val) => json!({
              "@type" : "g:P",
              "@value" : {
                "predicate" : "gte",
                "value" : val.encode_v3()
              }
            }),
            P::Inside(val) => json!({
              "@type" : "g:P",
              "@value" : {
                "predicate" : "inside",
                "value" : (val.0.clone(),val.1.clone()).encode_v3()
              }
            }),
            P::Outside(val) => json!({
              "@type" : "g:P",
              "@value" : {
                "predicate" : "outside",
                "value" : (val.0.clone(),val.1.clone()).encode_v3()
              }
            }),
            P::Between(val) => json!({
              "@type" : "g:P",
              "@value" : {
                "predicate" : "between",
                "value" : (val.0.clone(),val.1.clone()).encode_v3()
              }
            }),
            P::Within(val) => json!({
              "@type" : "g:P",
              "@value" : {
                "predicate" : "within",
                "value" : val.encode_v3()
              }
            }),
            P::Without(val) => json!({
              "@type" : "g:P",
              "@value" : {
                "predicate" : "inside",
                "value" : val.encode_v3()
              }
            }),
        }
    }

    fn encode_v2(&self) -> serde_json::Value {
        match self {
            P::Eq(val) => json!({
              "@type" : "g:P",
              "@value" : {
                "predicate" : "eq",
                "value" : val.encode_v2()
              }
            }),
            P::Neq(val) => json!({
              "@type" : "g:P",
              "@value" : {
                "predicate" : "neq",
                "value" : val.encode_v2()
              }
            }),
            P::Lt(val) => json!({
              "@type" : "g:P",
              "@value" : {
                "predicate" : "lt",
                "value" : val.encode_v2()
              }
            }),
            P::Lte(val) => json!({
              "@type" : "g:P",
              "@value" : {
                "predicate" : "lte",
                "value" : val.encode_v2()
              }
            }),
            P::Gt(val) => json!({
              "@type" : "g:P",
              "@value" : {
                "predicate" : "gt",
                "value" : val.encode_v2()
              }
            }),
            P::Gte(val) => json!({
              "@type" : "g:P",
              "@value" : {
                "predicate" : "gte",
                "value" : val.encode_v2()
              }
            }),
            P::Inside(val) => json!({
              "@type" : "g:P",
              "@value" : {
                "predicate" : "inside",
                "value" : (val.0.clone(),val.1.clone()).encode_v2()
              }
            }),
            P::Outside(val) => json!({
              "@type" : "g:P",
              "@value" : {
                "predicate" : "outside",
                "value" : (val.0.clone(),val.1.clone()).encode_v2()
              }
            }),
            P::Between(val) => json!({
              "@type" : "g:P",
              "@value" : {
                "predicate" : "between",
                "value" : (val.0.clone(),val.1.clone()).encode_v2()
              }
            }),
            P::Within(val) => json!({
              "@type" : "g:P",
              "@value" : {
                "predicate" : "within",
                "value" : val.encode_v2()
              }
            }),
            P::Without(val) => json!({
              "@type" : "g:P",
              "@value" : {
                "predicate" : "inside",
                "value" : val.encode_v2()
              }
            }),
        }
    }

    fn encode_v1(&self) -> serde_json::Value {
        todo!()
    }
}

impl Display for P {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            P::Eq(val) => write!(f, "eq({val})"),
            P::Neq(val) => write!(f, "neq({val})"),
            P::Lt(val) => write!(f, "lt({val})"),
            P::Lte(val) => write!(f, "lte({val})"),
            P::Gt(val) => write!(f, "gt({val})"),
            P::Gte(val) => write!(f, "gte({val})"),
            P::Inside(val) => write!(f, "inside({},{})", val.0, val.1),
            P::Outside(val) => write!(f, "outside({},{})", val.0, val.1),
            P::Between(val) => write!(f, "between({},{})", val.0, val.1),
            P::Within(val) => fmt_lists(f, "within", val),
            P::Without(val) => fmt_lists(f, "without", val),
        }
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
pub enum TextP {
    StartingWith(Vec<GremlinTypes>),
    EndingWith(Vec<GremlinTypes>),
    NotStartingWith(Vec<GremlinTypes>),
    NotEndingWith(Vec<GremlinTypes>),
    Containing(Vec<GremlinTypes>),
    NotContaining(Vec<GremlinTypes>),
    Regex(Vec<GremlinTypes>),
    NotRegex(Vec<GremlinTypes>),
}

impl From<TextP> for GremlinTypes {
    fn from(text_p: TextP) -> Self {
        GremlinTypes::TextP(text_p)
    }
}

impl TextP {
    pub fn starting_with(val: &str) -> Self {
        TextP::StartingWith(vec![val.into()])
    }
    pub fn not_starting_with(val: &str) -> Self {
        TextP::NotStartingWith(vec![val.into()])
    }
    pub fn ending_with(val: &str) -> Self {
        TextP::EndingWith(vec![val.into()])
    }
    pub fn not_ending_with(val: &str) -> Self {
        TextP::NotEndingWith(vec![val.into()])
    }
    pub fn containing(val: &str) -> Self {
        TextP::Containing(vec![val.into()])
    }
    pub fn not_containing(val: &str) -> Self {
        TextP::NotContaining(vec![val.into()])
    }
    pub fn regex(val: &str) -> Self {
        TextP::Regex(vec![val.into()])
    }
    pub fn not_regex(val: &str) -> Self {
        TextP::NotRegex(vec![val.into()])
    }
}

fn combine_text_value<W: std::io::Write>(
    name: &str,
    value: &[GremlinTypes],
    writer: &mut W,
) -> Result<(), crate::error::EncodeError> {
    name.partial_encode(writer)?;
    value.partial_encode(writer)
}

impl Encode for TextP {
    fn type_code() -> u8 {
        CoreType::TextP.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        match self {
            TextP::StartingWith(text) => combine_text_value("startingWith", text, writer),
            TextP::EndingWith(text) => combine_text_value("endingWith", text, writer),
            TextP::Containing(text) => combine_text_value("containing", text, writer),
            TextP::NotStartingWith(text) => combine_text_value("notStartingWith", text, writer),
            TextP::NotEndingWith(text) => combine_text_value("notEndingWith", text, writer),
            TextP::NotContaining(text) => combine_text_value("notContaining", text, writer),
            TextP::Regex(text) => combine_text_value("regex", text, writer),
            TextP::NotRegex(text) => combine_text_value("notRegex", text, writer),
        }
    }
}

impl serde::Serialize for TextP {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut buf = Vec::with_capacity(32);
        self.encode(&mut buf).expect("error during write of TextP");
        serializer.serialize_bytes(&buf[..])
    }
}

impl Decode for TextP {
    fn expected_type_code() -> u8 {
        CoreType::TextP.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        match String::decode(reader)?.as_str() {
            "startingWith" => Ok(TextP::StartingWith(Vec::partial_decode(reader)?)),
            "endingWith" => Ok(TextP::EndingWith(Vec::partial_decode(reader)?)),
            "containing" => Ok(TextP::Containing(Vec::partial_decode(reader)?)),
            "notStartingWith" => Ok(TextP::NotStartingWith(Vec::partial_decode(reader)?)),
            "notEndingWith" => Ok(TextP::NotEndingWith(Vec::partial_decode(reader)?)),
            "notContaining" => Ok(TextP::NotContaining(Vec::partial_decode(reader)?)),
            "regex" => Ok(TextP::Regex(Vec::partial_decode(reader)?)),
            "notRegex" => Ok(TextP::NotRegex(Vec::partial_decode(reader)?)),
            v => Err(DecodeError::DecodeError(format!(
                "expected TextP found variant text: {}",
                v
            ))),
        }
    }
    fn get_partial_len(bytes: &[u8]) -> Result<usize, DecodeError> {
        let mut len = String::get_len(bytes)?;
        len += Vec::<GremlinTypes>::get_partial_len(&bytes[len..])?;
        Ok(len)
    }
}

fn fmt_lists(
    f: &mut std::fmt::Formatter<'_>,
    name: &str,
    list: &[GremlinTypes],
) -> std::fmt::Result {
    write!(f, "{name}([")?;
    if !list.is_empty() {
        for element in &list[..list.len() - 1] {
            write!(f, "{element},")?;
        }
        write!(f, "{}", list.last().unwrap())?;
    }
    write!(f, "])")
}

impl Display for TextP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextP::StartingWith(val) => fmt_lists(f, "startingWith", val),
            TextP::EndingWith(val) => fmt_lists(f, "endingWith", val),
            TextP::NotStartingWith(val) => fmt_lists(f, "notStartingWith", val),
            TextP::NotEndingWith(val) => fmt_lists(f, "notEndingWith", val),
            TextP::Containing(val) => fmt_lists(f, "containing", val),
            TextP::NotContaining(val) => fmt_lists(f, "notContaining", val),
            TextP::Regex(val) => fmt_lists(f, "regex", val),
            TextP::NotRegex(val) => fmt_lists(f, "notRegex", val),
        }
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

#[macro_export]
macro_rules! enum_deserialize {
    ($(($t:ident,$visitor:ident)),*) => {
        $(
            impl<'de> Deserialize<'de> for $t {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    deserializer.deserialize_bytes($visitor)
                }
            }

            struct $visitor;

            impl<'de> serde::de::Visitor<'de> for $visitor {
                type Value = $t;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(formatter, concat!("a enum ", stringify!($t)))
                }

                fn visit_bytes<E>(self, mut v: &[u8]) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    match $t::decode(&mut v) {
                        Ok(val) => Ok(val),
                        Err(_) => Err(E::custom(concat!(stringify!($t)," Visitor Decode Error"))),
                    }
                }
            }
         )*
    };
}

#[macro_export]
macro_rules! de_serialize_impls {
    (  $(($t:ident,$v:ident)),* ) => {

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

            fn get_partial_len(bytes: &[u8]) -> Result<usize, DecodeError> {
                String::get_len(bytes)
            }
        }

        impl serde::Serialize for $t {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut buf = Vec::with_capacity(16);
                self.encode(&mut buf)
                    .expect(concat!("error during write of ", stringify!($t)));
                serializer.serialize_bytes(&buf[..])
            }
        }

        impl From<$t> for GremlinTypes {
            fn from(val: $t) -> Self {
                GremlinTypes::$t(val)
            }
        }

        impl Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f,"{}",self.as_str())
            }
        }

        impl TryFrom<GremlinTypes> for $t {
            type Error = crate::error::DecodeError;

            fn try_from(value: GremlinTypes) -> Result<Self, Self::Error> {
                match value {
                    GremlinTypes::$t(val) => Ok(val),
                    _ => Err(crate::error::DecodeError::ConvertError(
                        format!("cannot convert GraphBinary to {}",stringify!($t))
                    )),
                }
            }
        }

        impl crate::macros::TryBorrowFrom for $t {
            fn try_borrow_from(graph_binary: &GremlinTypes) -> Option<&Self> {
                match graph_binary {
                    GremlinTypes::$t(val) => Some(val),
                    _ => None
                }
            }
        }

        impl crate::macros::TryMutBorrowFrom for $t {
            fn try_mut_borrow_from(graph_binary: &mut GremlinTypes) -> Option<&mut Self> {
                match graph_binary {
                    GremlinTypes::$t(val) => Some(val),
                    _ => None
                }
            }
        }

        enum_deserialize!(($t,$v));
    )*
    };
}

macro_rules! graph_son_impls {
    (  $($t:ty),+$(,)?) => {

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

de_serialize_impls!(
    (Barrier, BarrierVisitor),
    (Cardinality, CardinalityVisitor),
    (Column, ColumnVisitor),
    (Direction, DirectionVisitor),
    (Operator, OperatorVisitor),
    (Order, OrderVisitor),
    (Pick, PickVisitor),
    (Pop, PopVisitor),
    (Scope, ScopeVisitor),
    (T, TVisitor),
    (Merge, MergeVisitor)
);

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

enum_deserialize!((TextP, TextPVisitor), (P, PVisitor));

#[test]
fn t_decode_test() {
    let reader = vec![0x03, 0x0, 0x0, 0x0, 0x0, 0x02, b'i', b'd'];

    let p = T::partial_decode(&mut &reader[..]);

    assert_eq!(T::Id, p.unwrap());
}

#[test]
fn p_decode_test() {
    let reader = vec![
        0x03, 0x0, 0x0, 0x0, 0x0, 0x07, b'w', b'i', b't', b'h', b'o', b'u', b't', 0x0, 0x0, 0x0,
        0x03, 0x1, 0x0, 0x0, 0x0, 0x0, 0x01, 0x01, 0x0, 0x0, 0x0, 0x0, 0x2, 0x01, 0x00, 0x0, 0x0,
        0x0, 0x3,
    ];

    let p = P::partial_decode(&mut &reader[..]);

    assert_eq!(P::Without(vec![1.into(), 2.into(), 3.into()]), p.unwrap());
}

#[test]
fn p_decode_inside_test() {
    let reader = vec![
        0x03, 0x0, 0x0, 0x0, 0x0, 0x06, b'i', b'n', b's', b'i', b'd', b'e', 0x0, 0x0, 0x0, 0x02,
        0x1, 0x0, 0x0, 0x0, 0x0, 0x01, 0x01, 0x0, 0x0, 0x0, 0x0, 0xff,
    ];

    let p = P::partial_decode(&mut &reader[..]);

    assert_eq!(P::inside(1, 255), p.unwrap());
}

#[test]
fn text_p_fq_decode_test() {
    let reader = vec![
        0x28, 0x00, 0x03, 0x0, 0x0, 0x0, 0x0, 0x0c, b's', b't', b'a', b'r', b't', b'i', b'n', b'g',
        b'W', b'i', b't', b'h', 0x0, 0x0, 0x0, 0x01, 0x3, 0x0, 0x0, 0x0, 0x0, 0x04, b't', b'e',
        b's', b't',
    ];

    let p = TextP::decode(&mut &reader[..]);
    let t = GremlinTypes::TextP(TextP::StartingWith(vec!["test".into()]));
    println!("{t}");
    assert_eq!(TextP::StartingWith(vec!["test".into()]), p.unwrap());
}

#[test]
fn text_p_consumed_bytes() {
    let reader = vec![
        0x28, 0x00, 0x03, 0x0, 0x0, 0x0, 0x0, 0x0c, b's', b't', b'a', b'r', b't', b'i', b'n', b'g',
        b'W', b'i', b't', b'h', 0x0, 0x0, 0x0, 0x01, 0x3, 0x0, 0x0, 0x0, 0x0, 0x04, b't', b'e',
        b's', b't',
    ];

    let p = TextP::get_len(&reader);

    assert_eq!(reader.len(), p.unwrap());
}
