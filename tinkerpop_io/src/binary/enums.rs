use std::marker::PhantomData;

use crate::{
    error::DecodeError,
    specs::CoreType,
    structure::enums::{
        Barrier, Cardinality, Column, Direction, Merge, Operator, Order, Pick, Pop, Scope, TextP,
        P, T,
    },
    GremlinValue,
};

use super::{Decode, Encode};

impl<T> Encode for P<T> {
    fn type_code() -> u8 {
        CoreType::P.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.predicate.partial_encode(writer)?;
        self.value.partial_encode(writer)
    }
}

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

impl Encode for TextP {
    fn type_code() -> u8 {
        CoreType::TextP.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.predicate.partial_encode(writer)?;
        self.value.partial_encode(writer)
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
        let predicate = String::decode(reader)?;
        let value = Vec::<GremlinValue>::partial_decode(reader)?;

        Ok(TextP { predicate, value })
    }
}

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

#[test]
fn t_decode() {
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
        0x0, 0x0, 0x0, 0x07, b'b', b'e', b't', b'w', b'e', b'e', b'n', 0x0, 0x0, 0x0, 0x02, 0x1,
        0x0, 0x0, 0x0, 0x0, 0x01, 0x01, 0x0, 0x0, 0x0, 0x0, 0x0a,
    ];

    let p = P::between(1, 10);
    let mut w = vec![];
    p.partial_encode(&mut w).unwrap();

    assert_eq!(w, expected);
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
