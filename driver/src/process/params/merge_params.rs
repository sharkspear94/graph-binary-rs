use std::collections::HashMap;

use gremlin_types::{
    structure::{bytecode::Bytecode, map::MapKeys},
    GremlinValue,
};

use crate::process::bytecode_traversal::BytecodeTraversal;
pub trait MergeParams {
    fn bytecode(&self, name: &str, bc: &mut Bytecode);
}

impl MergeParams for () {
    fn bytecode(&self, name: &str, bc: &mut Bytecode) {
        bc.push_new_step(name, vec![]);
    }
}

impl MergeParams for BytecodeTraversal {
    fn bytecode(&self, name: &str, bc: &mut Bytecode) {
        bc.push_new_step(name, vec![self.into()]);
    }
}

impl<K: Into<MapKeys> + Clone, V: Into<GremlinValue> + Clone> MergeParams for HashMap<K, V> {
    fn bytecode(&self, name: &str, bc: &mut Bytecode) {
        bc.push_new_step(name, vec![self.into()])
    }
}
