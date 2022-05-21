use serde::Deserialize;

use crate::{
    error::DecodeError,
    graph_binary::{decode, Decode, Encode, GraphBinary},
    specs::CoreType,
};

#[derive(Debug, PartialEq)]
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
    fn to_str(&self) -> &str {
        match self {
            Barrier::NormSack => "normSack",
        }
    }
}

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
            _ => Err(DecodeError::ConvertError("Cardinality".to_string())),
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

// impl Encode for Cardinality {
//     fn type_code() -> u8 {
//         CoreType::Cardinality.into()
//     }

//     fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), crate::error::EncodeError> {
//         self.to_str().fq_gb_bytes(writer)
//     }
// }

// impl Decode for Cardinality {
//     fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
//     where
//         Self: std::marker::Sized,
//     {
//         Cardinality::try_from(String::decode(reader)?.as_str())
//     }
// }

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
            _ => Err(DecodeError::ConvertError("Column".to_string())),
        }
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
            _ => Err(DecodeError::ConvertError("Direction".to_string())),
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
            _ => Err(DecodeError::ConvertError("Operator".to_string())),
        }
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
            _ => Err(DecodeError::ConvertError("Order".to_string())),
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
            _ => Err(DecodeError::ConvertError("Pick".to_string())),
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
            _ => Err(DecodeError::ConvertError("Pop".to_string())),
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
    Within(Vec<GraphBinary>),
    Without(Vec<GraphBinary>),
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
            P::Inside(v1, v2) => {
                "inside".write_full_qualified_bytes(writer)?;
                2_i32.write_patial_bytes(writer)?; // len of tuple only len = 2 will be used
                v1.build_fq_bytes(writer)?;
                v2.build_fq_bytes(writer)
            }
            P::Outside(v1, v2) => {
                "outside".write_full_qualified_bytes(writer)?;
                2_i32.write_patial_bytes(writer)?; // len of tuple only len = 2 will be used
                v1.build_fq_bytes(writer)?;
                v2.build_fq_bytes(writer)
            }
            P::Between(v1, v2) => {
                "between".write_full_qualified_bytes(writer)?;
                2_i32.write_patial_bytes(writer)?; // len of tuple only len = 2 will be used
                v1.build_fq_bytes(writer)?;
                v2.build_fq_bytes(writer)
            }
            P::Within(v) => {
                "within".write_full_qualified_bytes(writer)?;
                v.write_patial_bytes(writer)
            }
            P::Without(v) => {
                "without".write_full_qualified_bytes(writer)?;
                v.write_patial_bytes(writer)
            }
        }
    }
}

fn write_name_value<W: std::io::Write>(
    name: &str,
    value: &GraphBinary,
    writer: &mut W,
) -> Result<(), crate::error::EncodeError> {
    name.write_full_qualified_bytes(writer)?;
    value.build_fq_bytes(writer)
}

impl Encode for P {
    fn type_code() -> u8 {
        CoreType::P.into()
    }

    fn write_patial_bytes<W: std::io::Write>(
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
        let mut buf = Vec::new(); // TODO capacity??
        self.write_full_qualified_bytes(&mut buf)
            .expect("error during write of P");
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
        match String::fully_self_decode(reader)?.as_str() {
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
                Ok(P::Inside(
                    Box::new(decode(reader)?),
                    Box::new(decode(reader)?),
                ))
            }
            "outside" => {
                i32::partial_decode(reader)?;
                Ok(P::Outside(
                    Box::new(decode(reader)?),
                    Box::new(decode(reader)?),
                ))
            }
            "between" => {
                i32::partial_decode(reader)?;
                Ok(P::Between(
                    Box::new(decode(reader)?),
                    Box::new(decode(reader)?),
                ))
            }
            "within" => Ok(P::Within(Vec::partial_decode(reader)?)),
            "without" => Ok(P::Without(Vec::partial_decode(reader)?)),
            v => Err(DecodeError::DecodeError(format!(
                "expected P found variant text: {}",
                v
            ))),
        }
    }

    fn partial_count_bytes(bytes: &[u8]) -> Result<usize, DecodeError> {
        let mut len = String::consumed_bytes(bytes)?;
        len += Vec::<GraphBinary>::partial_count_bytes(&bytes[len..])?;
        Ok(len)
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
            _ => Err(DecodeError::ConvertError("Scope".to_string())),
        }
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
            _ => Err(DecodeError::ConvertError("T".to_string())),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TextP {
    // TODO not sure if graphbinray or String
    StartingWith(Vec<GraphBinary>),
    EndingWith(Vec<GraphBinary>),
    Containing(Vec<GraphBinary>),
    NotStartingWith(Vec<GraphBinary>),
    NotEndingWith(Vec<GraphBinary>),
    NotContaining(Vec<GraphBinary>),
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

fn combine_text_value<W: std::io::Write>(
    name: &str,
    value: &[GraphBinary],
    writer: &mut W,
) -> Result<(), crate::error::EncodeError> {
    name.write_full_qualified_bytes(writer)?;
    value.write_patial_bytes(writer)
}

impl Encode for TextP {
    fn type_code() -> u8 {
        CoreType::TextP.into()
    }

    fn write_patial_bytes<W: std::io::Write>(
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
        }
    }
}

impl serde::Serialize for TextP {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut buf = Vec::new(); // TODO capacity??
        self.write_full_qualified_bytes(&mut buf)
            .expect("error during write of TextP");
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
        match String::fully_self_decode(reader)?.as_str() {
            "startingWith" => Ok(TextP::StartingWith(Vec::partial_decode(reader)?)),
            "endingWith" => Ok(TextP::EndingWith(Vec::partial_decode(reader)?)),
            "containing" => Ok(TextP::Containing(Vec::partial_decode(reader)?)),
            "notStartingWith" => Ok(TextP::NotStartingWith(Vec::partial_decode(reader)?)),
            "notEndingWith" => Ok(TextP::NotEndingWith(Vec::partial_decode(reader)?)),
            "notContaining" => Ok(TextP::NotContaining(Vec::partial_decode(reader)?)),
            v => Err(DecodeError::DecodeError(format!(
                "expected TextP found variant text: {}",
                v
            ))),
        }
    }
    fn partial_count_bytes(bytes: &[u8]) -> Result<usize, DecodeError> {
        let mut len = String::consumed_bytes(bytes)?;
        len += Vec::<GraphBinary>::partial_count_bytes(&bytes[len..])?;
        Ok(len)
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
                    match $t::fully_self_decode(&mut v) {
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

            fn write_patial_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), crate::error::EncodeError> {
                self.to_str().write_full_qualified_bytes(writer)
            }
        }

        impl Decode for $t {

            fn expected_type_code() -> u8 {
                CoreType::$t.into()
            }

            fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
            where
                Self: std::marker::Sized,
            {
                $t::try_from(String::fully_self_decode(reader)?.as_str())
            }

            fn partial_count_bytes(bytes: &[u8]) -> Result<usize, DecodeError> {
                String::consumed_bytes(bytes)
            }
        }

        impl serde::Serialize for $t {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut buf = Vec::with_capacity(16);
                self.write_full_qualified_bytes(&mut buf)
                    .expect(concat!("error during write of ", stringify!($t)));
                serializer.serialize_bytes(&buf[..])
            }
        }

        enum_deserialize!(($t,$v));
    )*
    };
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

enum_deserialize!((TextP, TextPVisitor), (P, PVisitor));

#[test]
fn t_decode_test() {
    let reader = vec![0x03, 0x0, 0x0, 0x0, 0x0, 0x02, b'i', b'd'];

    let p = T::partial_decode(&mut &reader[..]);

    // assert!(p.is_ok());

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

    // assert!(p.is_ok());

    assert_eq!(P::Without(vec![1.into(), 2.into(), 3.into()]), p.unwrap());
}

#[test]
fn text_p_fq_decode_test() {
    let reader = vec![
        0x28, 0x00, 0x03, 0x0, 0x0, 0x0, 0x0, 0x0c, b's', b't', b'a', b'r', b't', b'i', b'n', b'g',
        b'W', b'i', b't', b'h', 0x0, 0x0, 0x0, 0x01, 0x3, 0x0, 0x0, 0x0, 0x0, 0x04, b't', b'e',
        b's', b't',
    ];

    let p = TextP::fully_self_decode(&mut &reader[..]);

    // assert!(p.is_ok());

    assert_eq!(TextP::StartingWith(vec!["test".into()]), p.unwrap());
}

#[test]
fn text_p_consumed_bytes() {
    let reader = vec![
        0x28, 0x00, 0x03, 0x0, 0x0, 0x0, 0x0, 0x0c, b's', b't', b'a', b'r', b't', b'i', b'n', b'g',
        b'W', b'i', b't', b'h', 0x0, 0x0, 0x0, 0x01, 0x3, 0x0, 0x0, 0x0, 0x0, 0x04, b't', b'e',
        b's', b't',
    ];

    let p = TextP::consumed_bytes(&reader);

    // assert!(p.is_ok());

    assert_eq!(reader.len(), p.unwrap());
}
