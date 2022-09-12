use gremlin_types::{
    structure::{bytecode::Bytecode, enums::Operator, lambda::Lambda},
    GremlinValue,
};

pub trait SackParam {
    fn bytecode(self, name: &str, bc: &mut Bytecode);
}

impl SackParam for () {
    fn bytecode(self, name: &str, bc: &mut Bytecode) {
        bc.push_new_step(name, vec![])
    }
}

impl SackParam for Operator {
    fn bytecode(self, name: &str, bc: &mut Bytecode) {
        bc.push_new_step(name, vec![self.into()])
    }
}

impl SackParam for Lambda {
    fn bytecode(self, name: &str, bc: &mut Bytecode) {
        bc.push_new_step(name, vec![self.into()])
    }
}

pub trait WithSackParam {
    fn bytecode(self, name: &str, bc: &mut Bytecode);
}

impl<T: Into<GremlinValue>> WithSackParam for T {
    fn bytecode(self, name: &str, bc: &mut Bytecode) {
        bc.push_new_source(name, vec![self.into()])
    }
}

// impl<T: Into<GremlinValue>> WithSackParam for (T, Lambda) {
//     fn bytecode(self, name: &str, bc: &mut Bytecode) {
//         bc.add_source(name, vec![self.0.into(), self.1.into()])
//     }
// }

// impl<T: Into<GremlinValue>> WithSackParam for (T, Operator) {
//     fn bytecode(self, name: &str, bc: &mut Bytecode) {
//         bc.add_source(name, vec![self.0.into(), self.1.into()])
//     }
// }
