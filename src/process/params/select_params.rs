use crate::{
    process::traversal::GraphTraversal,
    structure::{
        bytecode::ByteCode,
        enums::{Column, Pop},
    },
};

use super::multi_strings::MultiStringParams;

// pub trait SelectMapParam {
//     fn bytecode(self, step: &str, bc: &mut ByteCode);
// }

// impl SelectMapParam for &str {
//     fn bytecode(self, step: &str, bc: &mut ByteCode) {
//         bc.add_step(step, vec![self.into()])
//     }
// }

// impl<S, E2> SelectParam<S, E2> for Column {}

// impl<S, E2> SelectParam<S, E2> for (Pop, &str) {}

pub trait SelectParam {
    fn bytecode(self, step: &str, bc: &mut ByteCode);
}

impl SelectParam for &str {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into()])
    }
}

impl SelectParam for Column {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into()])
    }
}

impl SelectParam for (Pop, &str) {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.0.into(), self.1.into()])
    }
}

impl<S: MultiStringParams> SelectParam for (Pop, &str, S) {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.0.into(), self.1.into()]);
        self.2.extend_step(bc)
    }
}

impl< E, T> SelectParam for GraphTraversal< E, T> {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into()])
    }
}

impl< E, T> SelectParam for (Pop, GraphTraversal< E, T>) {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.0.into(), self.1.into()])
    }
}
