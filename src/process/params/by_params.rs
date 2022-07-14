use crate::{
    process::{bytecode_traversal::BytecodeTraversal},
    structure::{
        bytecode::ByteCode,
        enums::{Order, T},
    },
};

pub trait ByParams {
    fn bytecode(self, name: &str, bc: &mut ByteCode);
}

impl ByParams for () {
    fn bytecode(self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![])
    }
}

impl ByParams for Order {
    fn bytecode(self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![self.into()])
    }
}

impl ByParams for T {
    fn bytecode(self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![self.into()])
    }
}

impl ByParams for &str {
    fn bytecode(self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![self.into()])
    }
}

impl ByParams for (&str, Order) {
    fn bytecode(self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![self.0.into(), self.1.into()])
    }
}

impl ByParams for BytecodeTraversal {
    fn bytecode(self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![self.into()])
    }
}
