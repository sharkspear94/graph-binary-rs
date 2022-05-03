use crate::{
    error::EncodeError,
    graph_binary::{Encode, INT32_TYPE_CODE, STRING_TYPE_CODE, VALUE_NULL, VALUE_PRESENT},
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

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        let len = self.len() as i32;
        len.gb_bytes(writer)?;

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

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        let len = self.len() as i32;
        len.gb_bytes(writer)?;

        for (key, value) in self.iter() {
            key.fq_gb_bytes(writer)?;
            value.fq_gb_bytes(writer)?;
        }
        Ok(())
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
    map.fq_gb_bytes(&mut buf);

    let msg = [
        MAP_TYPE_CODE,
        VALUE_PRESENT,
        0x0,
        0x0,
        0x0,
        0x1, // Map len
        INT32_TYPE_CODE,
        VALUE_PRESENT,
        0x0,
        0x0,
        0x0,
        0x1, // Map key
        STRING_TYPE_CODE,
        VALUE_PRESENT, //string
        0x0,
        0x0,
        0x0,
        0x4, //string len
        0x74,
        0x65,
        0x73,
        0x74, //string bytes
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
    map.fq_gb_bytes(&mut buf);

    let msg = [
        MAP_TYPE_CODE,
        VALUE_PRESENT,
        0x0,
        0x0,
        0x0,
        0x1, // Map len
        INT32_TYPE_CODE,
        VALUE_PRESENT,
        0x0,
        0x0,
        0x0,
        0x1, // Map key
        MAP_TYPE_CODE,
        VALUE_PRESENT, // Map value
        0x0,
        0x0,
        0x0,
        0x1, //inner Map len
        INT32_TYPE_CODE,
        VALUE_PRESENT,
        0x0,
        0x0,
        0x0,
        0x1, //inner Map key
        STRING_TYPE_CODE,
        VALUE_PRESENT, //string
        0x0,
        0x0,
        0x0,
        0x4, //string len
        b't',
        b'e',
        b's',
        b't', //string bytes
    ];
    assert_eq!(msg[..], buf);
}
