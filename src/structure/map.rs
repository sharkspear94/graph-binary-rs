use crate::{
    error::EncodeError,
    graph_binary::{Decode, Encode, INT32_TYPE_CODE, VALUE_NULL, VALUE_PRESENT},
    specs::CoreType,
};
use std::{collections::HashMap, ops::Deref};

use crate::graph_binary::{GraphBinary, MapKeys, MAP_TYPE_CODE};

#[derive(Debug, PartialEq)]
pub struct Map {
    pub(crate) map: HashMap<MapKeys, GraphBinary>,
}

impl Map {
    fn new(map: HashMap<MapKeys, GraphBinary>) -> Map {
        Map { map }
    }
}

impl Deref for Map {
    type Target = HashMap<MapKeys, GraphBinary>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}
impl Encode for Map {
    fn type_code() -> u8 {
        MAP_TYPE_CODE
    }

    fn write_patial_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        let len = self.len() as i32;
        len.write_patial_bytes(writer)?;

        for (k, v) in self.iter() {
            GraphBinary::from(k).build_fq_bytes(writer)?;
            v.build_fq_bytes(writer)?;
        }

        Ok(())
    }
}

impl<K, V> Encode for HashMap<K, V>
where
    K: Encode,
    V: Encode,
{
    fn type_code() -> u8 {
        CoreType::Map.into()
    }

    fn write_patial_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        let len = self.len() as i32;
        len.write_patial_bytes(writer)?;

        for (key, value) in self.iter() {
            key.write_full_qualified_bytes(writer)?;
            value.write_full_qualified_bytes(writer)?;
        }
        Ok(())
    }
}

impl<K, V> Decode for HashMap<K, V>
where
    K: Decode + std::cmp::Eq + std::hash::Hash,
    V: Decode,
{
    fn expected_type_code() -> u8 {
        CoreType::Map.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let len = i32::partial_decode(reader)? as usize;
        let mut hash_map = HashMap::with_capacity(len);
        for _ in 0..len {
            let key = K::fully_self_decode(reader)?;
            let value = V::fully_self_decode(reader)?;

            hash_map.insert(key, value); // TODO what happens if key is double present Error?
        }

        Ok(hash_map)
    }

    fn partial_count_bytes(bytes: &[u8]) -> Result<usize, crate::error::DecodeError> {
        let t: [u8; 4] = bytes[0..4].try_into()?;
        let size = i32::from_be_bytes(t) as usize;
        let mut len = 4;
        for _ in 0..size {
            len += K::consumed_bytes(&bytes[len..])?;
            len += V::consumed_bytes(&bytes[len..])?;
        }
        Ok(len)
    }
}
// impl FullyQualifiedBytes for Map {
//     fn get_type_code(&self) -> Bytes {
//         Bytes::from_static(&[MAP_TYPE_CODE])
//     }

//     fn generate_byte_representation(&self) -> Bytes {
//         let mut ret = bytes::BytesMut::with_capacity(/*self.len() + INT32_LEN*/ 64); // needs work initial size is not known at compile time
//         ret.put_i32(self.len() as i32);
//         self.iter().for_each(|(key, val)| {
//             ret.extend_from_slice(&GraphBinary::from(key).build_fq_bytes());
//             ret.extend_from_slice(&val.build_fq_bytes());
//         });
//         ret.freeze()
//     }
// }

#[test]
fn testing_map() {
    let mut map = HashMap::new();

    map.insert(MapKeys::Int(1), GraphBinary::String("test".to_owned()));
    let map = Map::new(map);

    let mut buf: Vec<u8> = vec![];
    map.write_full_qualified_bytes(&mut buf).unwrap();

    let msg = [
        0x0a, 0x0, 0x0, 0x0, 0x0, 0x1, 0x01, 0x0, 0x0, 0x0, 0x0, 0x1, 0x3, 0x0, 0x0, 0x0, 0x0, 0x4,
        0x74, 0x65, 0x73, 0x74,
    ];
    assert_eq!(msg[..], buf);
}
#[test]
fn testing_nestet_map() {
    let mut map = HashMap::new();
    let mut inner_map = HashMap::new();

    inner_map.insert(MapKeys::Int(1), GraphBinary::String("test".to_owned()));
    map.insert(MapKeys::Int(1), GraphBinary::Map(Map::new(inner_map)));
    let map = Map::new(map);

    let mut buf: Vec<u8> = vec![];
    map.write_full_qualified_bytes(&mut buf).unwrap();

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
    map.write_full_qualified_bytes(&mut buf).unwrap();

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
    assert_eq!(
        map,
        HashMap::<i32, String>::fully_self_decode(&mut &msg[..]).unwrap()
    );
}
