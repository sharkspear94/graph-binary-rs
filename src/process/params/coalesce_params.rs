use crate::{process::traversal::GraphTraversal, structure::bytecode::ByteCode};

pub trait CoalesceParams {
    fn bytecode(self, name: &str, bc: &mut ByteCode);
}

impl<S, E, T> CoalesceParams for GraphTraversal<S, E, T> {
    fn bytecode(self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![self.into()])
    }
}

impl CoalesceParams for () {
    fn bytecode(self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![])
    }
}

impl<S, E, T> CoalesceParams for (GraphTraversal<S, E, T>, GraphTraversal<S, E, T>) {
    fn bytecode(self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![self.0.into(), self.1.into()])
    }
}

impl<T: Into<ByteCode>, const N: usize> CoalesceParams for [T; N] {
    fn bytecode(self, name: &str, bc: &mut ByteCode) {
        bc.add_step(
            name,
            self.into_iter().map(Into::into).map(Into::into).collect(),
        )
    }
}
