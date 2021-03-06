use crate::{
    conversions,
    graph_binary::{Decode, Encode, GraphBinary},
    specs::CoreType,
    struct_de_serialize,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Lambda {
    pub language: String,
    pub script: String,
    pub arguments_length: i32,
}

impl Lambda {
    pub fn new(script: &str) -> Self {
        Lambda {
            language: "gremlin-groovy".to_string(),
            script: script.to_string(),
            arguments_length: 1,
        }
    }
}

impl Encode for Lambda {
    fn type_code() -> u8 {
        CoreType::Lambda.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.language.partial_encode(writer)?;
        self.script.partial_encode(writer)?;
        self.arguments_length.partial_encode(writer)
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

    fn get_partial_len(bytes: &[u8]) -> Result<usize, crate::error::DecodeError> {
        let mut len = String::get_partial_len(bytes)?;
        len += String::get_partial_len(&bytes[len..])?;
        len += i32::get_partial_len(&bytes[len..])?;
        Ok(len)
    }
}

struct_de_serialize!((Lambda, LambdaVisitor, 254));
conversions!((Lambda, Lambda));

#[test]
fn test() {
    use crate::ser::to_bytes;

    let l = Lambda {
        language: "java".to_string(),
        script: "asd".to_string(),
        arguments_length: 5,
    };

    let json = serde_json::to_string_pretty(&l).unwrap();
    let b = to_bytes(&l).unwrap();

    println!("{json}");
    println!("{:?}", b);
}
