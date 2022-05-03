use crate::{
    graph_binary::{decode, Decode, Encode, GraphBinary},
    specs,
};

#[derive(Debug, PartialEq)]
pub struct Property {
    key: String,
    value: Box<GraphBinary>,
    // parent: Option<Parent>,
}

impl Encode for Property {
    fn type_code() -> u8 {
        specs::CoreType::Property.into()
    }

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), crate::error::EncodeError> {
        self.key.gb_bytes(writer)?;

        self.value.build_fq_bytes(writer)
    }
}

impl Decode for Property {
    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let key = String::decode(reader)?;
        let value = Box::new(decode(reader)?);

        Ok(Property { key, value })
    }
}
