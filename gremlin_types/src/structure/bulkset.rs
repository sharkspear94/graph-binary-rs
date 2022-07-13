use crate::{
    graph_binary::{Decode, Encode, GremlinTypes},
    specs::CoreType,
};

#[derive(Debug, PartialEq, Clone)]
pub struct BulkSet(Vec<(GremlinTypes, i64)>);

impl Encode for BulkSet {
    fn type_code() -> u8 {
        CoreType::BulkSet.into()
    }

    fn partial_encode<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        let vec_len = self.0.len() as i32;
        vec_len.partial_encode(writer)?;
        for (gb, bulk) in &self.0 {
            gb.encode(writer)?;
            bulk.partial_encode(writer)?;
        }
        Ok(())
    }
}

impl Decode for BulkSet {
    fn expected_type_code() -> u8 {
        CoreType::BulkSet.into()
    }

    fn partial_decode<R: std::io::Read>(reader: &mut R) -> Result<Self, crate::error::DecodeError>
    where
        Self: std::marker::Sized,
    {
        let len = i32::partial_decode(reader)?;
        let len = usize::try_from(len)?;
        let mut items = Vec::with_capacity(len);
        for _ in 0..len {
            let gb = GremlinTypes::decode(reader)?;
            let bulk = i64::partial_decode(reader)?;
            items.push((gb, bulk));
        }
        Ok(BulkSet(items))
    }

    fn get_partial_len(bytes: &[u8]) -> Result<usize, crate::error::DecodeError> {
        let t: [u8; 4] = bytes[0..4].try_into()?;
        let bulkset_len = i32::from_be_bytes(t);
        let mut len = 4;
        for _ in 0..bulkset_len {
            len += GremlinTypes::get_len(&bytes[len..])?;
            len += i64::get_partial_len(&bytes[len..])?;
        }
        Ok(len)
    }
}
