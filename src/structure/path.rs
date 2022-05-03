use crate::{
    graph_binary::{Encode, GraphBinary},
    specs::CoreType,
};

use super::list::{List, List1};

#[derive(Debug, PartialEq)]
pub struct Path {
    labels: Vec<Vec<String>>,  // List<Set<String>>
    objects: Vec<GraphBinary>, // List<T>
}

impl Encode for Path {
    fn type_code() -> u8 {
        CoreType::Path.into()
    }

    fn gb_bytes<W: std::io::Write>(&self, writer: &mut W) -> Result<(), crate::error::EncodeError> {
        self.labels.fq_gb_bytes(writer);

        todo!() // objects
    }
}
