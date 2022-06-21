use crate::{
    graph_binary::GraphBinary,
    structure::{bytecode::ByteCode, enums::Operator, lambda::Lambda},
};

pub trait SackParam {
    fn bytecode(self, name: &str, bc: &mut ByteCode);
}

impl SackParam for () {
    fn bytecode(self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![])
    }
}

impl SackParam for Operator {
    fn bytecode(self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![self.into()])
    }
}

impl SackParam for Lambda {
    fn bytecode(self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![self.into()])
    }
}

pub trait WithSackParam {
    fn bytecode(self, name: &str, bc: &mut ByteCode);
}

impl<T: Into<GraphBinary>> WithSackParam for T {
    fn bytecode(self, name: &str, bc: &mut ByteCode) {
        bc.add_source(name, vec![self.into()])
    }
}

impl<T: Into<GraphBinary>> WithSackParam for (T, Lambda) {
    fn bytecode(self, name: &str, bc: &mut ByteCode) {
        bc.add_source(name, vec![self.0.into(), self.1.into()])
    }
}

impl<T: Into<GraphBinary>> WithSackParam for (T, Operator) {
    fn bytecode(self, name: &str, bc: &mut ByteCode) {
        bc.add_source(name, vec![self.0.into(), self.1.into()])
    }
}
