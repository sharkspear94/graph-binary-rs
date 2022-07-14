use crate::{
    conversions,
    graph_binary::{Decode, Encode, GraphBinary},
    specs::CoreType,
    struct_de_serialize,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Path {
    labels: Vec<Vec<String>>,  // List<Set<String>>
    objects: Vec<GraphBinary>, // List<T>
}

impl Encode for Path {
    fn type_code() -> u8 {
        CoreType::Path.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        writer.write_all(&[CoreType::List.into(), 0x0])?;
        let len = i32::try_from(self.labels.len())?;
        len.partial_encode(writer)?;
        for set in &self.labels {
            writer.write_all(&[CoreType::Set.into(), 0x0])?;
            set.partial_encode(writer)?;
        }
        self.objects.encode(writer)
    }
}

impl Decode for Path {
    fn expected_type_code() -> u8 {
        CoreType::Path.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        reader.read_exact(&mut [0_u8, 0])?;
        let len = i32::partial_decode(reader)? as usize;
        let mut labels = Vec::with_capacity(len);
        for _ in 0..len {
            reader.read_exact(&mut [0_u8, 0])?;
            let set = Vec::<String>::partial_decode(reader)?;
            labels.push(set);
        }
        let objects = Vec::<GraphBinary>::decode(reader)?;

        Ok(Path { labels, objects })
    }

    fn get_partial_len(bytes: &[u8]) -> Result<usize, crate::error::DecodeError> {
        let t: [u8; 4] = bytes[2..6].try_into()?;
        let vec_len = i32::from_be_bytes(t);
        let mut len = 6; //4 bytes from i32 vec_len and 2 bytes from List Typecode and value flag
        for _ in 0..vec_len {
            len += Vec::<String>::get_len(&bytes[len..])?;
        }
        len += Vec::<GraphBinary>::get_len(&bytes[len..])?;

        Ok(len)
    }
}

struct_de_serialize!((Path, PathVisitor, 64));
conversions!((Path, Path));
#[test]
fn test_encode() {
    use crate::ser::to_bytes;

    let expected = [
        0xe_u8, 0x0, 0x9, 0x0, 0x0, 0x0, 0x0, 0x3, 0xb, 0x0, 0x0, 0x0, 0x0, 0x0, 0xb, 0x0, 0x0,
        0x0, 0x0, 0x0, 0xb, 0x0, 0x0, 0x0, 0x0, 0x0, 0x9, 0x0, 0x0, 0x0, 0x0, 0x3, 0x3, 0x0, 0x0,
        0x0, 0x0, 0x5, 0x6d, 0x61, 0x72, 0x6b, 0x6f, 0x1, 0x0, 0x0, 0x0, 0x0, 0x20, 0x3, 0x0, 0x0,
        0x0, 0x0, 0x6, 0x72, 0x69, 0x70, 0x70, 0x6c, 0x65,
    ];

    let path = Path {
        labels: vec![vec![], vec![], vec![]],
        objects: vec!["marko".into(), 32_i32.into(), "ripple".into()],
    };

    let res = to_bytes(path).unwrap();

    assert_eq!(&expected, &*res)
}

#[test]
fn test_decode() {
    use crate::de::from_slice;

    let expecetd = Path {
        labels: vec![vec![], vec![], vec![]],
        objects: vec!["marko".into(), 32_i32.into(), "ripple".into()],
    };

    let buf = vec![
        0xe, 0x0, 0x9, 0x0, 0x0, 0x0, 0x0, 0x3, 0xb, 0x0, 0x0, 0x0, 0x0, 0x0, 0xb, 0x0, 0x0, 0x0,
        0x0, 0x0, 0xb, 0x0, 0x0, 0x0, 0x0, 0x0, 0x9, 0x0, 0x0, 0x0, 0x0, 0x3, 0x3, 0x0, 0x0, 0x0,
        0x0, 0x5, 0x6d, 0x61, 0x72, 0x6b, 0x6f, 0x1, 0x0, 0x0, 0x0, 0x0, 0x20, 0x3, 0x0, 0x0, 0x0,
        0x0, 0x6, 0x72, 0x69, 0x70, 0x70, 0x6c, 0x65,
    ];

    let path = from_slice(&buf).unwrap();

    assert_eq!(expecetd, path)
}

#[test]
fn test_consume_bytes() {
    let buf = vec![
        0xe, 0x0, 0x9, 0x0, 0x0, 0x0, 0x0, 0x3, 0xb, 0x0, 0x0, 0x0, 0x0, 0x0, 0xb, 0x0, 0x0, 0x0,
        0x0, 0x0, 0xb, 0x0, 0x0, 0x0, 0x0, 0x0, 0x9, 0x0, 0x0, 0x0, 0x0, 0x3, 0x3, 0x0, 0x0, 0x0,
        0x0, 0x5, 0x6d, 0x61, 0x72, 0x6b, 0x6f, 0x1, 0x0, 0x0, 0x0, 0x0, 0x20, 0x3, 0x0, 0x0, 0x0,
        0x0, 0x6, 0x72, 0x69, 0x70, 0x70, 0x6c, 0x65,
    ];

    let size = Path::get_len(&buf).unwrap();

    assert_eq!(buf.len(), size)
}
