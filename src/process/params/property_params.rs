use std::collections::HashMap;

use crate::{
    graph_binary::{GraphBinary, MapKeys},
    structure::{bytecode::ByteCode, enums::Cardinality},
};

use super::object_param::MultiObjectParam;

pub trait PropertyParam {
    fn bytecode(self, step: &str, bc: &mut ByteCode);
}

impl PropertyParam for () {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![])
    }
}

impl<K: Into<MapKeys>, V: Into<GraphBinary>> PropertyParam for HashMap<K, V> {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into()])
    }
}

impl<K: Into<MapKeys>, V: Into<GraphBinary>> PropertyParam for (K, V) {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.0.into().into(), self.1.into()]);
    }
}

impl<K: Into<MapKeys>, V: Into<GraphBinary>> PropertyParam for (Cardinality, K, V) {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(
            step,
            vec![self.0.into(), self.1.into().into(), self.2.into()],
        );
    }
}

impl<K: Into<MapKeys>, V: Into<GraphBinary>, O: MultiObjectParam> PropertyParam for (K, V, O) {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.0.into().into(), self.1.into()]);
        self.2.extend_step(bc)
    }
}

impl<K: Into<MapKeys>, V: Into<GraphBinary>, O: MultiObjectParam> PropertyParam
    for (Cardinality, K, V, O)
{
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(
            step,
            vec![self.0.into(), self.1.into().into(), self.2.into()],
        );
        self.3.extend_step(bc)
    }
}
