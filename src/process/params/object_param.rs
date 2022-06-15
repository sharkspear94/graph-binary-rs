use crate::{graph_binary::GraphBinary, structure::bytecode::ByteCode};

pub trait ObjectParam {
    fn bytecode(&self, step: &str, bc: &mut ByteCode);

    fn extend_step(&self, bc: &mut ByteCode);
}

impl<T: Into<GraphBinary> + Clone> ObjectParam for T {
    fn bytecode(&self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into()])
    }

    fn extend_step(&self, bc: &mut ByteCode) {
        bc.add_to_last_step(self)
    }
}

impl ObjectParam for () {
    fn bytecode(&self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![])
    }

    fn extend_step(&self, _bc: &mut ByteCode) {}
}

pub trait MultiObjectParam {
    fn bytecode(&self, step: &str, bc: &mut ByteCode);

    fn extend_step(&self, bc: &mut ByteCode);
}

impl<T: Into<GraphBinary> + Clone> MultiObjectParam for T {
    fn bytecode(&self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into()])
    }

    fn extend_step(&self, bc: &mut ByteCode) {
        bc.add_to_last_step(self)
    }
}

impl MultiObjectParam for () {
    fn bytecode(&self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![])
    }

    fn extend_step(&self, _bc: &mut ByteCode) {}
}

impl<T: Into<GraphBinary> + Clone, const N: usize> MultiObjectParam for [T; N] {
    fn bytecode(&self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, self.iter().map(Into::into).collect())
    }

    fn extend_step(&self, bc: &mut ByteCode) {
        bc.extend_last_step(self.iter())
    }
}
