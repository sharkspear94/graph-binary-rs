use crate::{
    process::traversal::GraphTraversal,
    structure::{bytecode::ByteCode, enums::Direction, vertex::Vertex},
};

use super::multi_strings::MultiStringParams;

pub trait ToStepParams {
    fn bytecode(self, step: &str, bc: &mut ByteCode);
}

impl<T: MultiStringParams> ToStepParams for (Direction, T) {
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

impl ToStepParams for GraphTraversal<Vertex, Vertex> {
    fn bytecode(self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![self.bytecode.into()])
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
