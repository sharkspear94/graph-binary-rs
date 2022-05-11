use crate::{
    graph_binary::{decode, Decode, Encode, GraphBinary},
    specs::{self, CoreType},
};

#[derive(Debug, PartialEq)]
pub struct Property {
    pub key: String,
    pub value: Box<GraphBinary>,
    // parent: Option<Parent>,
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

        self.value.build_fq_bytes(writer)
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
        let value = Box::new(decode(reader)?);

        Ok(Property { key, value })
    }
}
