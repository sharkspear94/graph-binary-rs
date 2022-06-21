use crate::{process::bytecode_traversal::BytecodeTraversal, structure::bytecode::ByteCode};

pub trait CoalesceParams {
    fn bytecode(self, name: &str, bc: &mut ByteCode);
}

impl CoalesceParams for BytecodeTraversal {
    fn bytecode(self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![self.into()])
    }
}

impl CoalesceParams for () {
    fn bytecode(self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![])
    }
}

impl CoalesceParams for (BytecodeTraversal, BytecodeTraversal) {
    fn bytecode(self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![self.0.into(), self.1.into()])
    }
}

impl<const N: usize> CoalesceParams for [BytecodeTraversal; N] {
    fn bytecode(self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, self.iter().map(Into::into).collect())
    }
}
