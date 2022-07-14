use crate::{
    process::bytecode_traversal::BytecodeTraversal,
    structure::{bytecode::ByteCode, vertex::Vertex},
};

pub trait FromStepParams {
    fn bytecode(self, name: &str, bc: &mut ByteCode);
}

impl FromStepParams for Vertex {
    fn bytecode(self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![self.into()])
    }
}

impl FromStepParams for &Vertex {
    fn bytecode(self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![self.into()])
    }
}

impl FromStepParams for BytecodeTraversal {
    fn bytecode(self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![self.into()])
    }
}

impl FromStepParams for &str {
    fn bytecode(self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![self.into()])
    }
}

impl FromStepParams for String {
    fn bytecode(self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![self.into()])
    }
}
