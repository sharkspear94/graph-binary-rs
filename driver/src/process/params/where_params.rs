use gremlin_types::structure::{bytecode::Bytecode, enums::P};

use crate::process::bytecode_traversal::BytecodeTraversal;

pub trait WhereParams {
    fn bytecode(self, step: &str, bc: &mut Bytecode);
}

impl WhereParams for P<&str> {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.into()])
    }
}

impl WhereParams for P<String> {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.into()])
    }
}

impl WhereParams for (&str, P<&str>) {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.0.into(), self.1.into()])
    }
}

impl WhereParams for (String, P<String>) {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.0.into(), self.1.into()])
    }
}

impl WhereParams for BytecodeTraversal {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.into()])
    }
}
