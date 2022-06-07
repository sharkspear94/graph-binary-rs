use crate::{
    graph_binary::{Decode, Encode, GraphBinary},
    specs::CoreType,
    struct_de_serialize,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Binding {
    key: String,
    value: Box<GraphBinary>,
}

impl Encode for Binding {
    fn type_code() -> u8 {
        CoreType::Binding.into()
    }

    fn write_patial_bytes<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.key.write_patial_bytes(writer)?;
        self.value.write_full_qualified_bytes(writer)
    }
}

impl Decode for Binding {
    fn expected_type_code() -> u8 {
        CoreType::Binding.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let key = String::partial_decode(reader)?;
        let value = Box::new(GraphBinary::fully_self_decode(reader)?);

        Ok(Binding { key, value })
    }

    fn partial_count_bytes(bytes: &[u8]) -> Result<usize, crate::error::DecodeError> {
        let mut len = String::partial_count_bytes(bytes)?;
        len += GraphBinary::consumed_bytes(&bytes[len..])?;
        Ok(len)
    }
}
struct_de_serialize!((Binding, BindingVisitor, 16));

#[test]
fn test_binding_encode() {
    let expected = [
        0x14_u8, 0x0, 0x0, 0x00, 0x00, 0x04, 0x74, 0x65, 0x73, 0x74, 0x01, 0x00, 0x00, 0x0, 0x0,
        0x01,
    ];
    let mut buf: Vec<u8> = vec![];
    let b = Binding {
        key: "test".to_string(),
        value: Box::new(1_i32.into()),
    };
    b.write_full_qualified_bytes(&mut buf).unwrap();
    assert_eq!(expected, &*buf)
}

#[test]
fn test_binding_decode() {
    let buf = vec![
        0x14_u8, 0x0, 0x0, 0x00, 0x00, 0x04, 0x74, 0x65, 0x73, 0x74, 0x01, 0x00, 0x00, 0x0, 0x0,
        0x01,
    ];
    let expected = Binding {
        key: "test".to_string(),
        value: Box::new(1_i32.into()),
    };
    let b = Binding::fully_self_decode(&mut &buf[..]).unwrap();
    assert_eq!(expected, b)
}

#[test]
fn test_binding_count_bytes() {
    let expected = vec![
        0x14_u8, 0x0, 0x0, 0x00, 0x00, 0x04, 0x74, 0x65, 0x73, 0x74, 0x01, 0x00, 0x00, 0x0, 0x0,
        0x01,
    ];
    let count = Binding::consumed_bytes(&expected).unwrap();
    assert_eq!(count, expected.len())
}
