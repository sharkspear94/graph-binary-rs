use crate::{
    graph_binary::{Decode, Encode},
    specs::CoreType,
    structure::bytebuffer::ByteBuffer,
};

pub trait Custom {
    fn custom_name() -> &'static str;

    fn custom_type_info() -> ByteBuffer;

    fn partial_encode(&self) -> ByteBuffer;

    fn partial_decode(blob: ByteBuffer) -> Self;
}

impl<T: Custom> Encode for T {
    fn type_code() -> u8 {
        CoreType::Custom.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        T::custom_name().partial_encode(writer)?;
        T::custom_type_info().partial_encode(writer)?;
        self.partial_encode().partial_encode(writer)
    }
}
impl<T: Custom> Decode for T {
    fn expected_type_code() -> u8 {
        CoreType::Custom.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        if T::custom_name() != String::partial_decode(reader)? {
            // return Err(DecodeError:);
        }
        if T::custom_type_info() != ByteBuffer::partial_decode(reader)? {
            // return ;
        }
        Ok(T::partial_decode(ByteBuffer::partial_decode(reader)?))
    }
}
