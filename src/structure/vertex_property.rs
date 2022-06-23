use crate::{
    graph_binary::{Decode, Encode, GraphBinary},
    specs::{self, CoreType},
    struct_de_serialize,
};

use super::{property::Property, vertex::Vertex};

#[derive(Debug, PartialEq, Clone)]
pub struct VertexProperty {
    pub id: Box<GraphBinary>, // needs refinment
    pub label: String,
    pub value: Box<GraphBinary>,
    pub parent: Option<Vertex>,
    pub properties: Option<Vec<Property>>,
}

impl Encode for VertexProperty {
    fn type_code() -> u8 {
        specs::CoreType::VertexProperty.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.id.encode(writer)?;
        self.label.partial_encode(writer)?;
        self.value.encode(writer)?;
        self.parent.encode(writer)?;
        self.properties.encode(writer)?;
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
        let id = GraphBinary::decode(reader)?;
        let label = String::partial_decode(reader)?;
        let value = GraphBinary::decode(reader)?;
        let parent = Option::<Vertex>::decode(reader)?;
        let properties = Option::<Vec<Property>>::decode(reader)?;

        Ok(VertexProperty {
            id: Box::new(id),
            label,
            value: Box::new(value),
            parent,
            properties,
        })
    }

    fn get_partial_len(bytes: &[u8]) -> Result<usize, crate::error::DecodeError> {
        let mut len = GraphBinary::get_len(bytes)?;
        len += String::get_partial_len(&bytes[len..])?;
        len += GraphBinary::get_len(&bytes[len..])?;
        len += Option::<Vertex>::get_len(&bytes[len..])?;
        len += Option::<Vec<Property>>::get_len(&bytes[len..])?;

        Ok(len)
    }
}

struct_de_serialize!((VertexProperty, VertexVertexProperty, 32));
