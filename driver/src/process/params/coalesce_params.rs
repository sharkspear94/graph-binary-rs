use gremlin_types::structure::bytecode::Bytecode;

use crate::process::bytecode_traversal::BytecodeTraversal;

pub trait CoalesceParams {
    fn bytecode(self, name: &str, bc: &mut Bytecode);
}

impl CoalesceParams for BytecodeTraversal {
    fn bytecode(self, name: &str, bc: &mut Bytecode) {
        bc.push_new_step(name, vec![self.into()])
    }
}

impl CoalesceParams for () {
    fn bytecode(self, name: &str, bc: &mut Bytecode) {
        bc.push_new_step(name, vec![])
    }
}

impl CoalesceParams for (BytecodeTraversal, BytecodeTraversal) {
    fn bytecode(self, name: &str, bc: &mut Bytecode) {
        bc.push_new_step(name, vec![self.0.into(), self.1.into()])
    }
}

impl<const N: usize> CoalesceParams for [BytecodeTraversal; N] {
    fn bytecode(self, name: &str, bc: &mut Bytecode) {
        bc.push_new_step(name, self.iter().map(Into::into).collect())
    }
}
