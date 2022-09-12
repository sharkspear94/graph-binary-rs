use gremlin_types::structure::bytecode::Bytecode;

use crate::process::traversal::GraphTraversal;

pub trait FilterParam {
    fn bytecode(self, step: &str, bc: &mut Bytecode);
}

impl<E, T> FilterParam for GraphTraversal<E, T> {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.into()])
    }
}
