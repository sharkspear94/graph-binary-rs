use crate::{
    error::DecodeError,
    graph_binary::{Decode, Encode},
    specs::CoreType,
};

use crate::graph_binary::GraphBinary;

impl<T: Encode> Encode for Vec<T> {
    fn type_code() -> u8 {
        CoreType::List.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        let len = self.len() as i32;
        len.partial_encode(writer)?;

        for item in self {
            item.encode(writer)?;
        }

        Ok(())
    }
}

impl<T> From<Vec<T>> for GraphBinary
where
    T: Into<GraphBinary>,
{
    fn from(v: Vec<T>) -> Self {
        GraphBinary::List(v.into_iter().map(Into::into).collect())
    }
}

impl<T: Decode> Decode for Vec<T> {
    fn expected_type_code() -> u8 {
        CoreType::List.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let mut len_buf = [0_u8; 4];
        reader.read_exact(&mut len_buf)?;
        let len = i32::from_be_bytes(len_buf);
        if len.is_negative() {
            return Err(DecodeError::DecodeError("vec len negativ".to_string()));
        }
        let mut list: Vec<T> = Vec::with_capacity(len as usize);
        for _ in 0..len {
            list.push(T::decode(reader)?);
        }
        Ok(list)
    }

    fn get_partial_len(bytes: &[u8]) -> Result<usize, DecodeError> {
        let t: [u8; 4] = bytes[0..4].try_into()?;
        let vec_len = i32::from_be_bytes(t);
        let mut len = 4;
        for _ in 0..vec_len {
            len += T::get_len(&bytes[len..])?;
        }
        Ok(len)
    }
}

#[test]
fn vec_decode_test() {
    let reader: Vec<u8> = vec![
        0x0, 0x0, 0x0, 0x04, 0x01, 0x0, 0x0, 0x0, 0x0, 0x04, 0x01, 0x0, 0x0, 0x0, 0x0, 0x04, 0x01,
        0x0, 0x0, 0x0, 0x0, 0x04, 0x01, 0x0, 0x0, 0x0, 0x0, 0x04,
    ];

    let s = Vec::partial_decode(&mut &reader[..]);

    assert!(s.is_ok());
    let s = s.unwrap();
    assert_eq!(4, s.len());
    for gb in s {
        assert_eq!(
            4,
            match gb {
                GraphBinary::Int(s) => s,
                _ => panic!(),
            }
        )
    }
}

#[test]
fn vec_consume_bytes() {
    let reader: Vec<u8> = vec![
        0x0, 0x0, 0x0, 0x04, 0x01, 0x0, 0x0, 0x0, 0x0, 0x04, 0x01, 0x0, 0x0, 0x0, 0x0, 0x04, 0x01,
        0x0, 0x0, 0x0, 0x0, 0x04, 0x01, 0x0, 0x0, 0x0, 0x0, 0x04,
    ];

    let s = Vec::<GraphBinary>::get_partial_len(&reader);

    assert!(s.is_ok());
    let s = s.unwrap();
    assert_eq!(reader.len(), s);
}
