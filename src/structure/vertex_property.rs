use crate::{
    graph_binary::{Decode, Encode, GraphBinary},
    specs::{self, CoreType},
};

use super::{property::Property, vertex::Vertex};

#[derive(Debug, PartialEq)]
pub struct VertexProperty {
    pub id: Box<GraphBinary>, // needs refinment
    pub label: String,
    pub value: Box<GraphBinary>,
    pub parent: Option<Vertex>,
    pub properties: Option<Vec<Property>>,
}

impl From<VertexProperty> for GraphBinary {
    fn from(v: VertexProperty) -> Self {
        GraphBinary::VertexProperty(v)
    }
}

impl Encode for VertexProperty {
    fn type_code() -> u8 {
        specs::CoreType::VertexProperty.into()
    }

    fn write_patial_bytes<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.id.write_full_qualified_bytes(writer)?;
        self.label.write_patial_bytes(writer)?;
        self.value.write_full_qualified_bytes(writer)?;
        self.parent.write_full_qualified_bytes(writer)?;
        self.properties.write_full_qualified_bytes(writer)?;
        Ok(())
    }
}

impl Decode for VertexProperty {
    fn expected_type_code() -> u8 {
        CoreType::VertexProperty.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let id = GraphBinary::fully_self_decode(reader)?;
        let label = String::partial_decode(reader)?;
        let value = GraphBinary::fully_self_decode(reader)?;
        let parent = Option::<Vertex>::fully_self_decode(reader)?;
        let properties = Option::<Vec<Property>>::fully_self_decode(reader)?;

        Ok(VertexProperty {
            id: Box::new(id),
            label,
            value: Box::new(value),
            parent,
            properties,
        })
    }

    fn partial_count_bytes(bytes: &[u8]) -> Result<usize, crate::error::DecodeError> {
        let mut len = GraphBinary::consumed_bytes(bytes)?;
        len += String::partial_count_bytes(&bytes[len..])?;
        len += GraphBinary::consumed_bytes(&bytes[len..])?;
        len += Option::<Vertex>::consumed_bytes(&bytes[len..])?;
        len += Option::<Vec<Property>>::consumed_bytes(&bytes[len..])?;

        Ok(len)
    }
}
