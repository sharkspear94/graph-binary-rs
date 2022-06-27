use crate::{
    conversions,
    graph_binary::{Decode, Encode, GraphBinary},
    specs::{self, CoreType},
    struct_de_serialize,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Property {
    pub key: String,
    pub value: Box<GraphBinary>,
    pub parent: Box<GraphBinary>,
}

impl Encode for Property {
    fn type_code() -> u8 {
        specs::CoreType::Property.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.key.partial_encode(writer)?;
        self.value.encode(writer)?;
        self.parent.encode(writer)
    }
}

impl Decode for Property {
    fn expected_type_code() -> u8 {
        CoreType::Property.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let key = String::partial_decode(reader)?;
        let value = Box::new(GraphBinary::decode(reader)?);
        let parent = Box::new(GraphBinary::decode(reader)?);

        Ok(Property { key, value, parent })
    }

    fn get_partial_len(bytes: &[u8]) -> Result<usize, crate::error::DecodeError> {
        let mut len = String::get_partial_len(bytes)?;
        len += GraphBinary::get_len(&bytes[len..])?;
        len += GraphBinary::get_len(&bytes[len..])?;
        Ok(len)
    }
}

struct_de_serialize!((Property, PropertyVisitor, 32));
conversions!((Property, Property));
