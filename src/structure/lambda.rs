use crate::{
    graph_binary::{Decode, Encode, GraphBinary},
    specs::CoreType,
    struct_de_serialize,
};

#[derive(Debug, PartialEq)]
pub struct Lambda {
    language: String,
    script: String,
    arguments_length: i32,
}

impl Encode for Lambda {
    fn type_code() -> u8 {
        CoreType::Lambda.into()
    }

    fn write_patial_bytes<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.language.write_patial_bytes(writer)?;
        self.script.write_patial_bytes(writer)?;
        self.arguments_length.write_patial_bytes(writer)
    }
}

impl Decode for Lambda {
    fn expected_type_code() -> u8 {
        CoreType::Lambda.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let language = String::partial_decode(reader)?;
        let script = String::partial_decode(reader)?;
        let arguments_length = i32::partial_decode(reader)?;

        Ok(Lambda {
            language,
            script,
            arguments_length,
        })
    }

    fn partial_count_bytes(bytes: &[u8]) -> Result<usize, crate::error::DecodeError> {
        let mut len = String::partial_count_bytes(bytes)?;
        len += String::partial_count_bytes(&bytes[len..])?;
        len += i32::partial_count_bytes(&bytes[len..])?;
        Ok(len)
    }
}

struct_de_serialize!((Lambda, LambdaVisitor, 254));
