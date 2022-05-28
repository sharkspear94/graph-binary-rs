use std::collections::HashMap;

use crate::{
    graph_binary::{Decode, Encode, GraphBinary},
    specs::CoreType,
};

#[derive(Debug, PartialEq)]
pub struct Traverser {
    bulk: i64,
    value: Box<GraphBinary>,
}

impl Encode for Traverser {
    fn type_code() -> u8 {
        CoreType::Traverser.into()
    }

    fn write_patial_bytes<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.bulk.write_patial_bytes(writer)?;
        self.value.write_full_qualified_bytes(writer)
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
        let value = Box::new(GraphBinary::fully_self_decode(reader)?);

        Ok(Traverser { bulk, value })
    }

    fn partial_count_bytes(bytes: &[u8]) -> Result<usize, crate::error::DecodeError> {
        let mut len = i64::partial_count_bytes(bytes)?;
        len += GraphBinary::consumed_bytes(&bytes[len..])?;
        Ok(len)
    }
}

#[derive(Debug, PartialEq)]
pub struct TraversalStrategy {
    strategy_class: String,                      // class
    configuration: HashMap<String, GraphBinary>, // not sure if key is correct
}

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
    t.write_full_qualified_bytes(&mut writer).unwrap();
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

    assert_eq!(
        expected,
        Traverser::fully_self_decode(&mut &reader[..]).unwrap()
    )
}
