use gremlin_types::structure::{bytecode::Bytecode, enums::P};

use crate::process::bytecode_traversal::BytecodeTraversal;

pub trait UntilParams {
    fn bytecode(self, step: &str, bc: &mut Bytecode);
}

impl UntilParams for P<BytecodeTraversal> {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.into()])
    }
}

impl UntilParams for BytecodeTraversal {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.into()])
    }
}
