use crate::{process::bytecode_traversal::BytecodeTraversal, structure::bytecode::ByteCode};

pub trait AddElementParams {
    fn bytecode(self, step: &str, bc: &mut ByteCode);
}

impl AddElementParams for &str {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into()])
    }
}

impl AddElementParams for String {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into()])
    }
}

impl AddElementParams for BytecodeTraversal {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into()])
    }
}
