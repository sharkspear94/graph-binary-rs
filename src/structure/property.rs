use crate::{
    graph_binary::{Decode, Encode, GraphBinary},
    specs::{self, CoreType},
};

#[derive(Debug, PartialEq)]
pub struct Property {
    pub key: String,
    pub value: Box<GraphBinary>,
    pub parent: Box<GraphBinary>,
}

impl Encode for Property {
    fn type_code() -> u8 {
        specs::CoreType::Property.into()
    }

    fn write_patial_bytes<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.key.write_patial_bytes(writer)?;
        self.value.write_full_qualified_bytes(writer)?;
        self.parent.write_full_qualified_bytes(writer)
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
        let value = Box::new(GraphBinary::fully_self_decode(reader)?);
        let parent = Box::new(GraphBinary::fully_self_decode(reader)?);

        Ok(Property { key, value, parent })
    }

    fn partial_count_bytes(bytes: &[u8]) -> Result<usize, crate::error::DecodeError> {
        let mut len = String::partial_count_bytes(bytes)?;
        len += GraphBinary::consumed_bytes(&bytes[len..])?;
        len += GraphBinary::consumed_bytes(&bytes[len..])?;
        Ok(len)
    }
}
