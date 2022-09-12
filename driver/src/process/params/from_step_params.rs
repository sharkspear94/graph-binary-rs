use gremlin_types::structure::{bytecode::Bytecode, vertex::Vertex};

use crate::process::bytecode_traversal::BytecodeTraversal;

pub trait FromStepParams {
    fn bytecode(self, name: &str, bc: &mut Bytecode);
}

impl FromStepParams for Vertex {
    fn bytecode(self, name: &str, bc: &mut Bytecode) {
        bc.push_new_step(name, vec![self.into()])
    }
}

impl FromStepParams for &Vertex {
    fn bytecode(self, name: &str, bc: &mut Bytecode) {
        bc.push_new_step(name, vec![self.into()])
    }
}

impl FromStepParams for BytecodeTraversal {
    fn bytecode(self, name: &str, bc: &mut Bytecode) {
        bc.push_new_step(name, vec![self.into()])
    }
}

impl FromStepParams for &str {
    fn bytecode(self, name: &str, bc: &mut Bytecode) {
        bc.push_new_step(name, vec![self.into()])
    }
}

impl FromStepParams for String {
    fn bytecode(self, name: &str, bc: &mut Bytecode) {
        bc.push_new_step(name, vec![self.into()])
    }
}
