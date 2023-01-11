use std::{collections::HashMap, hash::BuildHasher};

use crate::{
    error::{DecodeError, EncodeError},
    specs::CoreType,
};

use super::{Decode, Encode};

#[cfg(feature = "graph_binary")]
impl<T: Encode> Encode for &[T] {
    fn type_code() -> u8 {
        CoreType::List.into()
    }

    fn partial_encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        let len = self.len() as i32;
        len.partial_encode(writer)?;

        for item in *self {
            item.encode(writer)?;
        }

        Ok(())
    }
}
#[cfg(feature = "graph_binary")]
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

#[cfg(feature = "graph_binary")]
impl<T: Decode> Decode for Vec<T> {
    fn expected_type_code() -> u8 {
        CoreType::List.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: std::marker::Sized,
    {
        let len = i32::partial_decode(reader)?;
        if len.is_negative() {
            return Err(DecodeError::DecodeError("vec len negativ".to_string()));
        }
        let mut list: Vec<T> = Vec::with_capacity(len as usize);
        for _ in 0..len {
            list.push(T::decode(reader)?);
        }
        Ok(list)
    }
}

#[cfg(feature = "graph_binary")]
impl<K, V, S: BuildHasher> Encode for HashMap<K, V, S>
where
    K: Encode,
    V: Encode,
{
    fn type_code() -> u8 {
        CoreType::Map.into()
    }

    fn partial_encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        let len = self.len() as i32;
        len.partial_encode(writer)?;

        for (key, value) in self.iter() {
            key.encode(writer)?;
            value.encode(writer)?;
        }
        Ok(())
    }
}

#[cfg(feature = "graph_binary")]
impl<K, V, S> Decode for HashMap<K, V, S>
where
    K: Decode + std::cmp::Eq + std::hash::Hash,
    V: Decode,
    S: BuildHasher + Default,
{
    fn expected_type_code() -> u8 {
        CoreType::Map.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let len = i32::partial_decode(reader)? as usize;
        let mut hash_map = HashMap::with_capacity_and_hasher(len, Default::default());
        for _ in 0..len {
            let key = K::decode(reader)?;
            let value = V::decode(reader)?;

            hash_map.insert(key, value); // TODO what happens if key is double present Error?
        }

        Ok(hash_map)
    }
}

#[test]
fn vec_decode_test() {
    use crate::GremlinValue;

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
                GremlinValue::Int(s) => s,
                _ => panic!(),
            }
        )
    }
}

#[test]
fn encode_hashmap() {
    use crate::GremlinValue;
    use crate::MapKeys;

    let mut map = HashMap::new();

    map.insert(MapKeys::Int(1), GremlinValue::String("test".to_owned()));

    let mut buf: Vec<u8> = vec![];
    map.encode(&mut buf).unwrap();

    let msg = [
        0x0a, 0x0, 0x0, 0x0, 0x0, 0x1, 0x01, 0x0, 0x0, 0x0, 0x0, 0x1, 0x3, 0x0, 0x0, 0x0, 0x0, 0x4,
        0x74, 0x65, 0x73, 0x74,
    ];
    assert_eq!(msg[..], buf);
}
#[test]
fn testing_nestet_map() {
    use crate::GremlinValue;
    use crate::MapKeys;

    let mut map = HashMap::new();
    let mut inner_map = HashMap::new();

    inner_map.insert(MapKeys::Int(1), GremlinValue::String("test".to_owned()));
    map.insert(MapKeys::Int(1), GremlinValue::Map(inner_map));

    let mut buf: Vec<u8> = vec![];
    map.encode(&mut buf).unwrap();

    let msg = [
        0x0a, 0x0, 0x0, 0x0, 0x0, 0x1, 0x01, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0a, 0x0, 0x0, 0x0, 0x0,
        0x1, 0x01, 0x0, 0x0, 0x0, 0x0, 0x1, 0x3, 0x0, 0x0, 0x0, 0x0, 0x4, b't', b'e', b's', b't',
    ];
    assert_eq!(msg[..], buf);
}

#[test]
fn testing_encode_hash_map() {
    let mut map = HashMap::new();

    map.insert(1, "test".to_owned());

    let mut buf: Vec<u8> = vec![];
    map.encode(&mut buf).unwrap();

    let msg = [
        0x0a, 0x0, 0x0, 0x0, 0x0, 0x1, 0x01, 0x0, 0x0, 0x0, 0x0, 0x1, 0x3, 0x0, 0x0, 0x0, 0x0, 0x4,
        0x74, 0x65, 0x73, 0x74,
    ];
    assert_eq!(msg[..], buf);
}

#[test]
fn testing_decode_hash_map() {
    let mut map = HashMap::new();

    map.insert(1, "test".to_owned());

    let msg = [
        0x0a, 0x0, 0x0, 0x0, 0x0, 0x1, 0x01, 0x0, 0x0, 0x0, 0x0, 0x1, 0x3, 0x0, 0x0, 0x0, 0x0, 0x4,
        0x74, 0x65, 0x73, 0x74,
    ];
    assert_eq!(map, HashMap::<i32, String>::decode(&mut &msg[..]).unwrap());
}
