use std::collections::HashMap;

use crate::{
    graph_binary::{GraphBinary, MapKeys},
    process::bytecode_traversal::BytecodeTraversal,
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

impl MergeParams for BytecodeTraversal {
    fn bytecode(&self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![self.into()]);
    }
}

impl<K: Into<MapKeys> + Clone, V: Into<GraphBinary> + Clone> MergeParams for HashMap<K, V> {
    fn bytecode(&self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![self.into()])
    }
}
