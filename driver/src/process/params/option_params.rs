use std::collections::HashMap;

use gremlin_types::{
    structure::{bytecode::Bytecode, map::MapKeys},
    GremlinValue,
};

use crate::process::bytecode_traversal::BytecodeTraversal;

pub trait OptionParams {
    fn bytecode(self, step: &str, bc: &mut Bytecode);
}

impl OptionParams for BytecodeTraversal {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.into()])
    }
}

impl<Token: Into<GremlinValue>> OptionParams for (Token, BytecodeTraversal) {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.0.into(), self.1.into()])
    }
}

impl<Token: Into<GremlinValue>, K: Into<MapKeys>, V: Into<GremlinValue>> OptionParams
    for (Token, HashMap<K, V>)
{
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.0.into(), self.1.into()])
    }
}

impl<Token: Into<GremlinValue>, K: Into<MapKeys>, V: Into<GremlinValue>, const N: usize>
    OptionParams for (Token, [(K, V); N])
{
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        let map: HashMap<MapKeys, GremlinValue> = self
            .1
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        bc.push_new_step(step, vec![self.0.into(), map.into()])
    }
}
