use crate::{
    graph_binary::{Decode, Encode, GraphBinary},
    specs::CoreType,
};

#[derive(Debug, PartialEq, Clone)]
pub struct BulkSet(Vec<(GraphBinary, i64)>);

impl Encode for BulkSet {
    fn type_code() -> u8 {
        CoreType::BulkSet.into()
    }

    fn write_patial_bytes<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        let vec_len = self.0.len() as i32;
        vec_len.write_patial_bytes(writer)?;
        for (gb, bulk) in &self.0 {
            gb.write_full_qualified_bytes(writer)?;
            bulk.write_patial_bytes(writer)?;
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
            let gb = GraphBinary::fully_self_decode(reader)?;
            let bulk = i64::partial_decode(reader)?;
            items.push((gb, bulk));
        }
        Ok(BulkSet(items))
    }

    fn partial_count_bytes(bytes: &[u8]) -> Result<usize, crate::error::DecodeError> {
        let t: [u8; 4] = bytes[0..4].try_into()?;
        let bulkset_len = i32::from_be_bytes(t);
        let mut len = 4;
        for _ in 0..bulkset_len {
            len += GraphBinary::consumed_bytes(&bytes[len..])?;
            len += i64::partial_count_bytes(&bytes[len..])?;
        }
        Ok(len)
    }
}
