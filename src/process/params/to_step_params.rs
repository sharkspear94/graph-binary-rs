use crate::{
    process::bytecode_traversal::BytecodeTraversal,
    structure::{bytecode::ByteCode, enums::Direction, vertex::Vertex},
};

use super::multi_strings::MultiStringParams;

pub trait ToStepParams {
    fn bytecode(self, step: &str, bc: &mut ByteCode);
}

impl<S: MultiStringParams> ToStepParams for (Direction, S) {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.0.into()]);
        self.1.extend_step(bc);
    }
}

impl ToStepParams for &str {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into()]);
    }
}
impl ToStepParams for String {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into()]);
    }
}

impl ToStepParams for BytecodeTraversal {
    fn bytecode(self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![self.into()])
    }
}

impl ToStepParams for Vertex {
    fn bytecode(self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![self.into()])
    }
}

impl ToStepParams for &Vertex {
    fn bytecode(self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![self.into()])
    }
}
