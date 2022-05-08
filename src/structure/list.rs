use crate::{
    error::DecodeError,
    graph_binary::{self, Decode, Encode},
    specs::CoreType,
};
use std::{
    collections::{btree_map::IterMut, VecDeque},
    fmt::Debug,
    ops::Deref,
};

use crate::graph_binary::GraphBinary;
#[derive(Debug, PartialEq)]
pub struct List(pub Vec<GraphBinary>);

#[derive(Debug, PartialEq)]
pub struct List1<T: Encode> {
    pub list: Vec<T>,
}

impl<T: Encode> Encode for List1<T> {
    fn type_code() -> u8 {
        CoreType::List.into()
    }

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), crate::error::EncodeError> {
        let len = self.list.len() as i32;
        len.gb_bytes(writer)?;

        for item in &self.list {
            item.fq_gb_bytes(writer)?;
        }

        Ok(())
    }
}

impl<T: Encode> Encode for Vec<T> {
    fn type_code() -> u8 {
        CoreType::List.into()
    }

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), crate::error::EncodeError> {
        let len = self.len() as i32;
        len.gb_bytes(writer)?;

        for item in self {
            item.fq_gb_bytes(writer)?;
        }

        Ok(())
    }
}

impl<T: Decode> Decode for Vec<T> {


    fn expected_type_code() -> u8 {
        CoreType::List.into()
    }

    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
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
            list.push(T::decode(reader)?)
        }
        Ok(list)
    }

}

// // importent to garanty that all types are same type
// // maybe move type logic to serde traits
// impl<T> Decode for Vec<T> {
//     fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self,DecodeError>
//     where Self: std::marker::Sized {
//         let mut len_buf = [0_u8;4];
//         reader.read_exact(&mut len_buf)?;
//         let len = i32::from_be_bytes(len_buf);
//         if len.is_negative() {

//         }
//         let mut list = match len {
//             i32::MIN..=-1 => return Err(DecodeError::DecodeError("array len negativ".to_string())),
//             0 => return Ok(Vec::new()),
//             _ => Vec::with_capacity(len as usize)
//         };
//         let first_item = graph_binary::decode(reader)?;
//         let type_info = first_item.type_info();
//         list.push(first_item.);
//         println!("{first_item:?}");
//         for _ in 1..len {
//             let item = graph_binary::decode(reader)?;
//             if item.type_info() != type_info {

//             }
//             else {
//                 list.push(item)
//             }
//         };
//         Ok(list)
//     }
// }

// impl<T: Encode> Decode for List1<T> {
//     fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self,crate::error::DecodeError>
//     where Self: std::marker::Sized {
//         let mut len_buf = [0_u8;4];
//         reader.read_exact(&mut len_buf)?;
//         let len = i32::from_be_bytes(len_buf);

//         let list: Vec<GraphBinary> = Vec::with_capacity(len as usize);

//         for _ in 0..len {
//             list.push(graph_binary::decode(& mut reader)?);
//         };

//         let vec = vec![1_i32,2,3];
//         Ok(List1::new(vec))
//     }
// }

impl Decode for List {


    fn expected_type_code() -> u8 {
        CoreType::List.into()
    }

    fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let mut len_buf = [0_u8; 4];
        reader.read_exact(&mut len_buf)?;
        let len = i32::from_be_bytes(len_buf);
        if len.is_negative() {
            return Err(DecodeError::DecodeError("array len negativ".to_string()));
        }
        let mut list: Vec<GraphBinary> = Vec::with_capacity(len as usize);
        for _ in 0..len {
            list.push(graph_binary::decode(reader)?)
        }
        Ok(List(list))
    }
}

#[test]
fn list_decode_test() {
    let reader: Vec<u8> = vec![
        0x0, 0x0, 0x0, 0x04, 0x01, 0x0, 0x0, 0x0, 0x0, 0x04, 0x01, 0x0, 0x0, 0x0, 0x0, 0x04, 0x01,
        0x0, 0x0, 0x0, 0x0, 0x04, 0x01, 0x0, 0x0, 0x0, 0x0, 0x04,
    ];

    let s = List::decode(&mut &reader[..]);

    assert!(s.is_ok());
    let s = s.unwrap();
    assert_eq!(4, s.0.len());
    for gb in s.0 {
        assert_eq!(
            4,
            match gb {
                GraphBinary::Int(s) => s,
                _ => panic!(),
            }
        )
    }
}

impl<T: Encode> List1<T> {
    pub fn new(list: Vec<T>) -> List1<T> {
        List1 { list }
    }
}

impl Deref for List {
    type Target = Vec<GraphBinary>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Encode for List {
    fn type_code() -> u8 {
        CoreType::List.into()
    }

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), crate::error::EncodeError> {
        let len = self.len() as i32;
        len.gb_bytes(writer)?;

        for item in self.iter() {
            item.build_fq_bytes(writer)?;
        }

        Ok(())
    }
}

// impl FullyQualifiedBytes for List {
//     fn get_type_code(&self) -> Bytes {
//         Bytes::from_static(&[LIST_TYPE_CODE])
//     }

//     fn generate_byte_representation(&self) -> Bytes {
//         let mut ret = bytes::BytesMut::with_capacity(self.len()); // needs work initial size is not known at compile time
//         ret.put_i32(self.len() as i32);
//         self.iter()
//             .for_each(|item| ret.extend_from_slice(&item.build_fq_bytes()));
//         ret.freeze()
//     }
// }

#[test]
fn testing_list() {
    use crate::specs;

    let list = List(vec![0.into(), 1.into(), 2.into()]);

    pub const VALUE_PRESENT: u8 = 0x00;
    pub const VALUE_NULL: u8 = 0x01;

    let msg = [
        specs::CORE_TYPE_LIST,
        VALUE_PRESENT,
        0x0,
        0x0,
        0x0,
        0x3, // List len
        specs::CORE_TYPE_INT,
        VALUE_PRESENT,
        0x0,
        0x0,
        0x0,
        0x0, // List[0]
        specs::CORE_TYPE_INT,
        VALUE_PRESENT,
        0x0,
        0x0,
        0x0,
        0x1, // List[1]
        specs::CORE_TYPE_INT,
        VALUE_PRESENT,
        0x0,
        0x0,
        0x0,
        0x2, // List[2]
    ];
    let mut buf: Vec<u8> = vec![];
    list.fq_gb_bytes(&mut buf);
    assert_eq!(&msg[..], &buf);
}
