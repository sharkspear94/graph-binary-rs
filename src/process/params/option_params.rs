use std::collections::HashMap;

use crate::{
    graph_binary::{GraphBinary, MapKeys},
    process::bytecode_traversal::BytecodeTraversal,
    structure::bytecode::ByteCode,
};

pub trait OptionParams {
    fn bytecode(self, step: &str, bc: &mut ByteCode);
}

impl OptionParams for BytecodeTraversal {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into()])
    }
}

impl<Token: Into<GraphBinary>> OptionParams for (Token, BytecodeTraversal) {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.0.into(), self.1.into()])
    }
}

impl<Token: Into<GraphBinary>, K: Into<MapKeys>, V: Into<GraphBinary>> OptionParams
    for (Token, HashMap<K, V>)
{
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.0.into(), self.1.into()])
    }
}

impl<Token: Into<GraphBinary>, K: Into<MapKeys>, V: Into<GraphBinary>, const N: usize> OptionParams
    for (Token, [(K, V); N])
{
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        let map: HashMap<MapKeys, GraphBinary> = self
            .1
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        bc.add_step(step, vec![self.0.into(), map.into()])
    }
}
