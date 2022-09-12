use gremlin_types::structure::{
    bytecode::Bytecode,
    enums::{Column, Pop},
};

use crate::process::bytecode_traversal::BytecodeTraversal;

use super::multi_strings::MultiStringParams;

// pub trait SelectMapParam {
//     fn bytecode(self, step: &str, bc: &mut Bytecode);
// }

// impl SelectMapParam for &str {
//     fn bytecode(self, step: &str, bc: &mut Bytecode) {
//         bc.push_new_step(step, vec![self.into()])
//     }
// }

// impl<S, E2> SelectParam<S, E2> for Column {}

// impl<S, E2> SelectParam<S, E2> for (Pop, &str) {}

pub trait SelectParam {
    fn bytecode(self, step: &str, bc: &mut Bytecode);
}

impl SelectParam for &str {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.into()])
    }
}

impl SelectParam for Column {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.into()])
    }
}

impl SelectParam for (Pop, &str) {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.0.into(), self.1.into()])
    }
}

impl<S: MultiStringParams> SelectParam for (Pop, &str, S) {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.0.into(), self.1.into()]);
        self.2.extend_step(bc)
    }
}

impl SelectParam for BytecodeTraversal {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.into()])
    }
}

impl SelectParam for (Pop, BytecodeTraversal) {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.0.into(), self.1.into()])
    }
}
