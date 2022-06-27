use std::collections::HashMap;

use crate::{
    conversions,
    graph_binary::{Decode, Encode, GraphBinary},
    specs::CoreType,
    struct_de_serialize,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Traverser {
    pub bulk: i64,
    pub value: Box<GraphBinary>,
}

pub struct TraverserIter<'a> {
    bulk: usize,
    val: &'a GraphBinary,
}

impl Traverser {
    pub fn iter(&self) -> TraverserIter {
        TraverserIter {
            bulk: self.bulk as usize,
            val: &self.value,
        }
    }
}

impl<'a> Iterator for TraverserIter<'a> {
    type Item = &'a GraphBinary;
    fn next(&mut self) -> Option<Self::Item> {
        if self.bulk > 0 {
            self.bulk -= 1;
            Some(self.val)
        } else {
            None
        }
    }
}

pub struct IntoTraverserIter {
    bulk: usize,
    val: GraphBinary,
}

impl Iterator for IntoTraverserIter {
    type Item = GraphBinary;
    fn next(&mut self) -> Option<Self::Item> {
        if self.bulk > 0 {
            self.bulk -= 1;
            Some(self.val.clone())
        } else {
            None
        }
    }
}

impl IntoIterator for Traverser {
    type Item = GraphBinary;

    type IntoIter = IntoTraverserIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoTraverserIter {
            bulk: self.bulk as usize,
            val: *self.value,
        }
    }
}

#[test]
fn test() {
    let t = Traverser {
        bulk: 3,
        value: Box::new(1.into()),
    };
    let mut iter = t.iter();
    assert_eq!(iter.next(), Some(&1.into()));
    assert_eq!(iter.next(), Some(&1.into()));
    assert_eq!(iter.next(), Some(&1.into()));
    assert_eq!(iter.next(), None)
}

impl Encode for Traverser {
    fn type_code() -> u8 {
        CoreType::Traverser.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.bulk.partial_encode(writer)?;
        self.value.encode(writer)
    }
}

impl Decode for Traverser {
    fn expected_type_code() -> u8 {
        CoreType::Traverser.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let bulk = i64::partial_decode(reader)?;
        let value = Box::new(GraphBinary::decode(reader)?);

        Ok(Traverser { bulk, value })
    }

    fn get_partial_len(bytes: &[u8]) -> Result<usize, crate::error::DecodeError> {
        let mut len = i64::get_partial_len(bytes)?;
        len += GraphBinary::get_len(&bytes[len..])?;
        Ok(len)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TraversalStrategy {
    pub strategy_class: String,                      // class
    pub configuration: HashMap<String, GraphBinary>, // not sure if key is correct
}

impl Encode for TraversalStrategy {
    fn type_code() -> u8 {
        CoreType::TraversalStrategy.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.strategy_class.partial_encode(writer)?;
        self.configuration.partial_encode(writer)
    }
}

impl Decode for TraversalStrategy {
    fn expected_type_code() -> u8 {
        CoreType::TraversalStrategy.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let strategy_class = String::partial_decode(reader)?;
        let configuration = HashMap::<String, GraphBinary>::partial_decode(reader)?;

        Ok(TraversalStrategy {
            strategy_class,
            configuration,
        })
    }

    fn get_partial_len(bytes: &[u8]) -> Result<usize, crate::error::DecodeError> {
        let mut len = String::get_partial_len(bytes)?;
        len += HashMap::<String, GraphBinary>::get_partial_len(&bytes[len..])?;
        Ok(len)
    }
}

struct_de_serialize!(
    (Traverser, TraverserVisitor, 32),
    (TraversalStrategy, TraversalStrategyVisitor, 32)
);
conversions!(
    (Traverser, Traverser),
    (TraversalStrategy, TraversalStrategy)
);

#[test]
fn encode_traverser() {
    let expected = [
        0x21, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x3, 0x03, 0x0, 0x0, 0x0, 0x0, 0x3, b'a',
        b'b', b'c',
    ];

    let t = Traverser {
        bulk: 3,
        value: Box::new("abc".into()),
    };
    let mut writer = Vec::<u8>::new();
    t.encode(&mut writer).unwrap();
    assert_eq!(expected, &writer[..])
}

#[test]
fn decode_traverser() {
    let reader = vec![
        0x21, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x3, 0x03, 0x0, 0x0, 0x0, 0x0, 0x3, b'a',
        b'b', b'c',
    ];

    let expected = Traverser {
        bulk: 3,
        value: Box::new("abc".into()),
    };

    assert_eq!(expected, Traverser::decode(&mut &reader[..]).unwrap())
}
