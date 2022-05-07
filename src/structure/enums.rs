use serde::Serialize;

use crate::{
    error::DecodeError,
    graph_binary::{Decode, Encode, GraphBinary},
    specs::CoreType,
};

#[derive(Debug, PartialEq)]
pub enum Barrier {}

#[derive(Debug, PartialEq)]
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
            _ => Err(DecodeError::ConvertError("Cardinality")),
        }
    }
}

impl Cardinality {
    fn to_str(&self) -> &str {
        match self {
            Cardinality::List => "list",
            Cardinality::Set => "set",
            Cardinality::Single => "single",
        }
    }
}

impl Encode for Cardinality {
    fn type_code() -> u8 {
        CoreType::Cardinality.into()
    }

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), crate::error::EncodeError> {
        self.to_str().fq_gb_bytes(writer)
    }
}

impl Decode for Cardinality {
    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        Cardinality::try_from(String::decode(reader)?.as_str())
    }
}

#[derive(Debug, PartialEq)]
pub enum Column {
    Keys,
    Values,
}

impl Column {
    fn to_str(&self) -> &str {
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
            _ => Err(DecodeError::ConvertError("Column")),
        }
    }
}

impl Encode for Column {
    fn type_code() -> u8 {
        CoreType::Cloumn.into()
    }

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), crate::error::EncodeError> {
        self.to_str().fq_gb_bytes(writer)
    }
}

impl Decode for Column {
    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        Column::try_from(String::decode(reader)?.as_str())
    }
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Both,
    In,
    Out,
}

impl TryFrom<&str> for Direction {
    type Error = DecodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "both" => Ok(Direction::Both),
            "in" => Ok(Direction::In),
            "out" => Ok(Direction::Out),
            _ => Err(DecodeError::ConvertError("Direction")),
        }
    }
}

impl Direction {
    fn to_str(&self) -> &str {
        match self {
            Direction::Both => "both",
            Direction::In => "in",
            Direction::Out => "out",
        }
    }
}

impl Encode for Direction {
    fn type_code() -> u8 {
        CoreType::Direction.into()
    }

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), crate::error::EncodeError> {
        self.to_str().fq_gb_bytes(writer)
    }
}

impl Decode for Direction {
    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        Direction::try_from(String::decode(reader)?.as_str())
    }
}

#[derive(Debug, PartialEq)]
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
    fn to_str(&self) -> &str {
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
            _ => Err(DecodeError::ConvertError("Operator")),
        }
    }
}

impl Encode for Operator {
    fn type_code() -> u8 {
        CoreType::Operator.into()
    }

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), crate::error::EncodeError> {
        self.to_str().fq_gb_bytes(writer)
    }
}

impl Decode for Operator {
    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        Operator::try_from(String::decode(reader)?.as_str())
    }
}

#[derive(Debug, PartialEq)]
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
            _ => Err(DecodeError::ConvertError("Order")),
        }
    }
}

impl Order {
    fn to_str(&self) -> &str {
        match self {
            Order::Shuffle => "shuffle",
            Order::Asc => "asc",
            Order::Desc => "desc",
        }
    }
}

impl Encode for Order {
    fn type_code() -> u8 {
        CoreType::Order.into()
    }

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), crate::error::EncodeError> {
        self.to_str().fq_gb_bytes(writer)
    }
}

impl Decode for Order {
    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        Order::try_from(String::decode(reader)?.as_str())
    }
}

#[derive(Debug, PartialEq)]
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
            _ => Err(DecodeError::ConvertError("Pick")),
        }
    }
}

impl Pick {
    fn to_str(&self) -> &str {
        match self {
            Pick::Any => "any",
            Pick::None => "none",
        }
    }
}

impl Encode for Pick {
    fn type_code() -> u8 {
        CoreType::Pick.into()
    }

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), crate::error::EncodeError> {
        self.to_str().fq_gb_bytes(writer)
    }
}

impl Decode for Pick {
    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        Pick::try_from(String::decode(reader)?.as_str())
    }
}

#[derive(Debug, PartialEq)]
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
            _ => Err(DecodeError::ConvertError("Pop")),
        }
    }
}

impl Pop {
    fn to_str(&self) -> &str {
        match self {
            Pop::All => "all",
            Pop::First => "first",
            Pop::Last => "last",
            Pop::Mixed => "mixed",
        }
    }
}

impl Encode for Pop {
    fn type_code() -> u8 {
        CoreType::Pop.into()
    }

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), crate::error::EncodeError> {
        self.to_str().fq_gb_bytes(writer)
    }
}

impl Decode for Pop {
    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        Pop::try_from(String::decode(reader)?.as_str())
    }
}

#[derive(Debug, PartialEq)]
pub enum P {
    Eq(Box<GraphBinary>),
    Neq(Box<GraphBinary>),
    Lt(Box<GraphBinary>),
    Lte(Box<GraphBinary>),
    Gt(Box<GraphBinary>),
    Gte(Box<GraphBinary>),
    Inside(Box<GraphBinary>, Box<GraphBinary>),
    Outside(Box<GraphBinary>, Box<GraphBinary>),
    Between(Box<GraphBinary>, Box<GraphBinary>),
    Within(Box<GraphBinary>),
    Without(Box<GraphBinary>),
}

impl P {
    fn to_str(&self) -> &str {
        match self {
            P::Eq(_) => "eq",
            P::Neq(_) => "neq",
            P::Lt(_) => "lt",
            P::Lte(_) => "lte",
            P::Gt(_) => "gt",
            P::Gte(_) => "gte",
            P::Inside(_, _) => "inside",
            P::Outside(_, _) => "outside",
            P::Between(_, _) => "between",
            P::Within(_) => "within",
            P::Without(_) => "without",
        }
    }
}

impl Encode for P {
    fn type_code() -> u8 {
        CoreType::P.into()
    }

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), crate::error::EncodeError> {
        self.to_str().fq_gb_bytes(writer);
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub enum Scope {
    Local,
    Global,
}

impl Scope {
    fn to_str(&self) -> &str {
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
            _ => Err(DecodeError::ConvertError("Scope")),
        }
    }
}

impl Encode for Scope {
    fn type_code() -> u8 {
        CoreType::Scope.into()
    }

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), crate::error::EncodeError> {
        self.to_str().fq_gb_bytes(writer)
    }
}

impl Decode for Scope {
    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        Scope::try_from(String::decode(reader)?.as_str())
    }
}

#[derive(Debug, PartialEq)]
pub enum T {
    Id,
    Key,
    Lable,
    Value,
}

impl T {
    fn to_str(&self) -> &str {
        match self {
            T::Id => "id",
            T::Key => "key",
            T::Lable => "label",
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
            "label" => Ok(T::Lable),
            "value" => Ok(T::Value),
            _ => Err(DecodeError::ConvertError("T")),
        }
    }
}

impl Encode for T {
    fn type_code() -> u8 {
        CoreType::T.into()
    }

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), crate::error::EncodeError> {
        self.to_str().fq_gb_bytes(writer)
    }
}

impl Decode for T {
    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        T::try_from(String::decode(reader)?.as_str())
    }
}

impl Serialize for T {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut buf = Vec::with_capacity(8);
        self.fq_gb_bytes(&mut buf);
        serializer.serialize_bytes(&buf[..])
    }
}

#[derive(Debug, PartialEq)]
pub enum TextP {
    // TODO not sure if graphbinray or String
    StartingWith(Box<GraphBinary>),
    EndingWith(Box<GraphBinary>),
    Containing(Box<GraphBinary>),
    NotStartingWith(Box<GraphBinary>),
    NotEndingWith(Box<GraphBinary>),
    NotContaining(Box<GraphBinary>),
}

impl TextP {
    fn to_str(&self) -> &str {
        match self {
            TextP::StartingWith(_) => "startingWith",
            TextP::EndingWith(_) => "endingWith",
            TextP::Containing(_) => "containing",
            TextP::NotStartingWith(_) => "notStartingWith",
            TextP::NotEndingWith(_) => "notEndingWith",
            TextP::NotContaining(_) => "notContaining",
        }
    }
}

impl Encode for TextP {
    fn type_code() -> u8 {
        CoreType::TextP.into()
    }

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), crate::error::EncodeError> {
        self.to_str().fq_gb_bytes(writer)
    }
}

#[derive(Debug, PartialEq)]
pub enum Merge {
    OnCreate,
    OnMatch,
}

impl Merge {
    fn to_str(&self) -> &str {
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
            _ => Err(DecodeError::ConvertError("Merge")),
        }
    }
}

impl Encode for Merge {
    fn type_code() -> u8 {
        CoreType::Merge.into()
    }

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), crate::error::EncodeError> {
        self.to_str().fq_gb_bytes(writer)
    }
}

impl Decode for Merge {
    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        Merge::try_from(String::decode(reader)?.as_str())
    }
}
