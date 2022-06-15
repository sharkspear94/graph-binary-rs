use crate::{
    process::traversal::GraphTraversal,
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

impl<E, T> ByParams for GraphTraversal<E, T> {
    fn bytecode(self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![self.into()])
    }
}
