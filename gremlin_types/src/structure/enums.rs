use std::fmt::Display;
use std::marker::PhantomData;

use crate::{error::DecodeError, GremlinValue};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
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
    pub(crate) const fn as_str(&self) -> &str {
        match self {
            Barrier::NormSack => "normSack",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
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
    pub(crate) const fn as_str(&self) -> &str {
        match self {
            Cardinality::List => "list",
            Cardinality::Set => "set",
            Cardinality::Single => "single",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum Column {
    Keys,
    Values,
}

impl Column {
    pub(crate) const fn as_str(&self) -> &str {
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
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
    pub(crate) const fn as_str(&self) -> &str {
        match self {
            Direction::Both => "BOTH",
            Direction::In => "IN",
            Direction::Out => "OUT",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
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
    pub(crate) const fn as_str(&self) -> &str {
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
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
    pub(crate) const fn as_str(&self) -> &str {
        match self {
            Order::Shuffle => "shuffle",
            Order::Asc => "asc",
            Order::Desc => "desc",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
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
    pub(crate) const fn as_str(&self) -> &str {
        match self {
            Pick::Any => "any",
            Pick::None => "none",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
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
    pub(crate) const fn as_str(&self) -> &str {
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
    pub(crate) predicate: String,
    pub(crate) value: Vec<GremlinValue>,
    pub(crate) marker: PhantomData<T>,
}

impl<T: Into<GremlinValue>> P<T> {
    #[must_use]
    pub fn eq(a: T) -> P<T> {
        P {
            predicate: "eq".to_string(),
            value: vec![a.into()],
            marker: PhantomData,
        }
    }

    #[must_use]
    pub fn neq(a: T) -> P<T> {
        P {
            predicate: "neq".to_string(),
            value: vec![a.into()],
            marker: PhantomData,
        }
    }

    #[must_use]
    pub fn gt(a: T) -> P<T> {
        P {
            predicate: "gt".to_string(),
            value: vec![a.into()],
            marker: PhantomData,
        }
    }

    #[must_use]
    pub fn gte(a: T) -> P<T> {
        P {
            predicate: "gte".to_string(),
            value: vec![a.into()],
            marker: PhantomData,
        }
    }

    #[must_use]
    pub fn lt(a: T) -> P<T> {
        P {
            predicate: "lt".to_string(),
            value: vec![a.into()],
            marker: PhantomData,
        }
    }

    #[must_use]
    pub fn lte(a: T) -> P<T> {
        P {
            predicate: "lte".to_string(),
            value: vec![a.into()],
            marker: PhantomData,
        }
    }

    #[must_use]
    pub fn between(first: T, second: T) -> P<T> {
        P {
            predicate: "between".to_string(),
            value: vec![first.into(), second.into()],
            marker: PhantomData,
        }
    }

    #[must_use]
    pub fn inside(first: T, second: T) -> P<T> {
        P {
            predicate: "inside".to_string(),
            value: vec![first.into(), second.into()],
            marker: PhantomData,
        }
    }

    #[must_use]
    pub fn outside(first: T, second: T) -> P<T> {
        P {
            predicate: "outside".to_string(),
            value: vec![first.into(), second.into()],
            marker: PhantomData,
        }
    }

    #[must_use]
    pub fn within(values: Vec<T>) -> P<T> {
        P {
            predicate: "within".to_string(),
            value: values.into_iter().map(Into::into).collect(),
            marker: PhantomData,
        }
    }

    #[must_use]
    pub fn without(values: Vec<T>) -> P<T> {
        P {
            predicate: "without".to_string(),
            value: values.into_iter().map(Into::into).collect(),
            marker: PhantomData,
        }
    }

    #[must_use]
    pub fn and(self, f: P<T>) -> P<T> {
        P {
            predicate: "and".to_string(),
            value: vec![self.into(), f.into()],
            marker: PhantomData,
        }
    }

    #[must_use]
    pub fn or(self, f: P<T>) -> P<T> {
        P {
            predicate: "or".to_string(),
            value: vec![self.into(), f.into()],
            marker: PhantomData,
        }
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum Scope {
    Local,
    Global,
}

impl Scope {
    pub(crate) const fn as_str(&self) -> &str {
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum T {
    Id,
    Key,
    Label,
    Value,
}

impl T {
    pub(crate) const fn as_str(&self) -> &str {
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
    pub(crate) predicate: String,
    pub(crate) value: Vec<GremlinValue>,
}

impl From<TextP> for GremlinValue {
    fn from(text_p: TextP) -> Self {
        GremlinValue::TextP(text_p)
    }
}

impl TextP {
    #[must_use]
    pub fn starting_with(val: &str) -> Self {
        TextP {
            predicate: "startingWith".to_string(),
            value: vec![val.into()],
        }
    }
    #[must_use]
    pub fn not_starting_with(val: &str) -> Self {
        TextP {
            predicate: "notStartingWith".to_string(),
            value: vec![val.into()],
        }
    }
    #[must_use]
    pub fn ending_with(val: &str) -> Self {
        TextP {
            predicate: "endingWith".to_string(),
            value: vec![val.into()],
        }
    }
    #[must_use]
    pub fn not_ending_with(val: &str) -> Self {
        TextP {
            predicate: "notEndingWith".to_string(),
            value: vec![val.into()],
        }
    }
    #[must_use]
    pub fn containing(val: &str) -> Self {
        TextP {
            predicate: "containing".to_string(),
            value: vec![val.into()],
        }
    }
    #[must_use]
    pub fn not_containing(val: &str) -> Self {
        TextP {
            predicate: "notContaining".to_string(),
            value: vec![val.into()],
        }
    }
    #[must_use]
    pub fn regex(val: &str) -> Self {
        TextP {
            predicate: "regex".to_string(),
            value: vec![val.into()],
        }
    }
    #[must_use]
    pub fn not_regex(val: &str) -> Self {
        TextP {
            predicate: "notRegex".to_string(),
            value: vec![val.into()],
        }
    }

    #[must_use]
    pub fn and(self, text_p: TextP) -> P<TextP> {
        P {
            predicate: "and".to_string(),
            value: vec![self.into(), text_p.into()],
            marker: PhantomData,
        }
    }

    #[must_use]
    pub fn or(self, text_p: TextP) -> P<TextP> {
        P {
            predicate: "or".to_string(),
            value: vec![self.into(), text_p.into()],
            marker: PhantomData,
        }
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum Merge {
    OnCreate,
    OnMatch,
}

impl Merge {
    pub(crate) const fn as_str(&self) -> &str {
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
