use std::fmt::Display;

use crate::{error::CustomError, specs::CoreType, structure::bytebuffer::ByteBuffer};

use crate::binary::{Decode, Encode};

pub trait CustomTypes {
    fn partial_encode(&self) -> (String, ByteBuffer, ByteBuffer);

    fn partial_decode(custom: Custom) -> Result<Self, CustomError>
    where
        Self: Sized;
}

pub trait CustomType {
    const NAME: &'static str;
    const TYPE_INFO: &'static [u8];

    fn partial_encode(&self) -> ByteBuffer;

    fn partial_decode(blob: ByteBuffer) -> Result<Self, CustomError>
    where
        Self: Sized;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Custom {
    name: String,
    type_info: ByteBuffer,
    blob: ByteBuffer,
}

impl Custom {
    pub fn to_type<T: CustomType>(self) -> Result<T, CustomError> {
        if T::NAME != self.name {}
        if T::TYPE_INFO != self.type_info.as_bytes() {}
        T::partial_decode(self.blob)
    }

    pub fn to_types<T: CustomTypes>(self) -> Result<T, CustomError> {
        T::partial_decode(self)
    }
}

impl Encode for Custom {
    fn type_code() -> u8 {
        CoreType::Custom.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.name.partial_encode(writer)?;
        self.type_info.partial_encode(writer)?;
        self.blob.encode(writer)
    }
}

impl Decode for Custom {
    fn expected_type_code() -> u8 {
        CoreType::Custom.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let name = String::partial_decode(reader)?;
        let type_info = ByteBuffer::partial_decode(reader)?;
        let blob = ByteBuffer::decode(reader)?;
        Ok(Custom {
            name,
            type_info,
            blob,
        })
    }
}

impl Display for Custom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: {}, type_info: {}, blob: {}",
            self.name, self.type_info, self.blob
        )
    }
}
