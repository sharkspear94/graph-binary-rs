use crate::{process::traversal::GraphTraversal, structure::bytecode::ByteCode};

pub trait EmitParams {
    fn bytecode(self, step: &str, bc: &mut ByteCode);
}

impl EmitParams for () {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![])
    }
}

impl< E, T> EmitParams for GraphTraversal< E, T> {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into()])
    }
}

// impl<S, E, T> EmitParams for PublicP2<> {
//     fn bytecode(self, step: &str, bc: &mut ByteCode) {
//         bc.add_step(step, vec![self.into()])
//     }
// }
