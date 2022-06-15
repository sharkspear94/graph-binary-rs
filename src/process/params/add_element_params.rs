use crate::{process::traversal::GraphTraversal, structure::bytecode::ByteCode};

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

impl AddElementParams for GraphTraversal<String, String> {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.bytecode.into()])
    }
}
