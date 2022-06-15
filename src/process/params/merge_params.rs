use std::collections::HashMap;

use crate::{
    graph_binary::{GraphBinary, MapKeys},
    process::traversal::GraphTraversal,
    structure::bytecode::ByteCode,
};

pub trait MergeParams {
    fn bytecode(&self, name: &str, bc: &mut ByteCode);
}

impl MergeParams for () {
    fn bytecode(&self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![]);
    }
}

impl<S: Clone> MergeParams
    for GraphTraversal<S, HashMap<MapKeys, GraphBinary>, HashMap<MapKeys, GraphBinary>>
{
    fn bytecode(&self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![self.into()]);
    }
}

impl MergeParams for HashMap<MapKeys, GraphBinary> {
    fn bytecode(&self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![self.into()]);
    }
}
