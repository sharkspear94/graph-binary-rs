use crate::{process::traversal::GraphTraversal, structure::bytecode::ByteCode};

pub trait FilterParam {
    fn bytecode(self, step: &str, bc: &mut ByteCode);
}

impl<S, E, T> FilterParam for GraphTraversal<S, E, T> {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into()])
    }
}
