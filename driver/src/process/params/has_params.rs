use tinkerpop_io::{
    structure::{bytecode::Bytecode, enums::T},
    GremlinValue,
};

use crate::process::bytecode_traversal::BytecodeTraversal;

pub trait HasStepParams {
    fn bytecode(self, step: &str, bc: &mut Bytecode);
}

impl HasStepParams for &str {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.into()]);
    }
}

impl HasStepParams for String {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.into()]);
    }
}

impl<V: Into<GremlinValue>> HasStepParams for (String, V) {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.0.into(), self.1.into()]);
    }
}

// impl<M> HasStepParams for (String, GraphTraversal<M, M, M>) {
//     fn bytecode(self, step: &str, bc: &mut Bytecode) {
//         bc.push_new_step(step, vec![self.0.into(), self.1.bytecode.into()]);
//     }
// }

impl<V: Into<GremlinValue>> HasStepParams for (&str, V) {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.0.into(), self.1.into()]);
    }
}

impl HasStepParams for (T, BytecodeTraversal) {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.0.into(), self.1.into()]);
    }
}

// impl<V: Into<GremlinValue>> HasStepParams for (T, V) {
//     fn bytecode(self, step: &str, bc: &mut Bytecode) {
//         bc.push_new_step(step, vec![self.0.into(), self.1.into()]);
//     }
// }

impl<V> HasStepParams for (&str, &str, V)
where
    V: Into<GremlinValue>,
{
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.0.into(), self.1.into(), self.2.into()]);
    }
}

impl<V> HasStepParams for (String, String, V)
where
    V: Into<GremlinValue>,
{
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.0.into(), self.1.into(), self.2.into()]);
    }
}

impl<V> HasStepParams for (&str, T, V)
where
    V: Into<GremlinValue>,
{
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.0.into(), self.1.into(), self.2.into()]);
    }
}

impl<V> HasStepParams for (String, T, V)
where
    V: Into<GremlinValue>,
{
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.0.into(), self.1.into(), self.2.into()]);
    }
}
