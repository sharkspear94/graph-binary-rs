use gremlin_types::structure::bytecode::Bytecode;

use crate::process::bytecode_traversal::BytecodeTraversal;

pub trait AddElementParams {
    fn bytecode(self, step: &str, bc: &mut Bytecode);
}

impl AddElementParams for &str {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.into()])
    }
}

impl AddElementParams for String {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.into()])
    }
}

impl AddElementParams for BytecodeTraversal {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.into()])
    }
}
