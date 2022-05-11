use crate::{
    graph_binary::{Encode, GraphBinary},
    specs::CoreType,
};

#[derive(Debug, PartialEq)]
pub struct Path {
    labels: Vec<Vec<String>>,  // List<Set<String>>
    objects: Vec<GraphBinary>, // List<T>
}

impl Encode for Path {
    fn type_code() -> u8 {
        CoreType::Path.into()
    }

    fn write_patial_bytes<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), crate::error::EncodeError> {
        self.labels.write_full_qualified_bytes(writer)?;

        todo!() // objects
    }
}
