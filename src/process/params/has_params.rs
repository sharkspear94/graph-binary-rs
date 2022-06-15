use crate::{
    graph_binary::GraphBinary,
    process::traversal::GraphTraversal,
    structure::{bytecode::ByteCode, enums::T},
};

pub trait HasStepParams {
    fn bytecode(self, step: &str, bc: &mut ByteCode);
}

impl HasStepParams for &str {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into()]);
    }
}

impl HasStepParams for String {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into()]);
    }
}

impl<V: Into<GraphBinary>> HasStepParams for (String, V) {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.0.into(), self.1.into()]);
    }
}

// impl<M> HasStepParams for (String, GraphTraversal<M, M, M>) {
//     fn bytecode(self, step: &str, bc: &mut ByteCode) {
//         bc.add_step(step, vec![self.0.into(), self.1.bytecode.into()]);
//     }
// }

impl<M> HasStepParams for (&str, GraphTraversal< M, M>) {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.0.into(), self.1.bytecode.into()]);
    }
}

impl<M> HasStepParams for (T, GraphTraversal< M, M>) {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.0.into(), self.1.bytecode.into()]);
    }
}

// impl<V: Into<GraphBinary>> HasStepParams for (&str, V) {
//     fn bytecode(self, step: &str, bc: &mut ByteCode) {
//         bc.add_step(step, vec![self.0.into(), self.1.into()]);
//     }
// }

// impl<V: Into<GraphBinary>> HasStepParams for (T, V) {
//     fn bytecode(self, step: &str, bc: &mut ByteCode) {
//         bc.add_step(step, vec![self.0.into(), self.1.into()]);
//     }
// }

impl<V> HasStepParams for (&str, &str, V)
where
    V: Into<GraphBinary>,
{
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.0.into(), self.1.into(), self.2.into()]);
    }
}

impl<V> HasStepParams for (String, String, V)
where
    V: Into<GraphBinary>,
{
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.0.into(), self.1.into(), self.2.into()]);
    }
}

impl<V> HasStepParams for (&str, T, V)
where
    V: Into<GraphBinary>,
{
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.0.into(), self.1.into(), self.2.into()]);
    }
}

impl<V> HasStepParams for (String, T, V)
where
    V: Into<GraphBinary>,
{
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.0.into(), self.1.into(), self.2.into()]);
    }
}
