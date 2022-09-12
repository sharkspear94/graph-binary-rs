use gremlin_types::{
    structure::{bytecode::Bytecode, enums::Cardinality, map::MapKeys},
    GremlinValue,
};

use super::object_param::MultiObjectParam;
use std::collections::HashMap;

pub trait PropertyParam {
    fn bytecode(self, step: &str, bc: &mut Bytecode);
}

impl PropertyParam for () {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![])
    }
}

impl<K: Into<MapKeys>, V: Into<GremlinValue>> PropertyParam for HashMap<K, V> {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.into()])
    }
}

impl<K: Into<MapKeys>, V: Into<GremlinValue>> PropertyParam for (K, V) {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.0.into().into(), self.1.into()]);
    }
}

impl<K: Into<MapKeys>, V: Into<GremlinValue>> PropertyParam for (Cardinality, K, V) {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(
            step,
            vec![self.0.into(), self.1.into().into(), self.2.into()],
        );
    }
}

// impl<K: Into<MapKeys>, V: Into<GremlinValue>, O: MultiObjectParam> PropertyParam for (K, V, O) {
//     fn bytecode(self, step: &str, bc: &mut Bytecode) {
//         bc.push_new_step(step, vec![self.0.into().into(), self.1.into()]);
//         self.2.extend_step(bc)
//     }
// }

impl<K: Into<MapKeys>, V: Into<GremlinValue>, O: MultiObjectParam> PropertyParam
    for (Cardinality, K, V, O)
{
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(
            step,
            vec![self.0.into(), self.1.into().into(), self.2.into()],
        );
        self.3.extend_step(bc)
    }
}
