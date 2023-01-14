use tinkerpop_io::structure::bytecode::Bytecode;

use crate::process::traversal::GraphTraversal;

pub trait EmitParams {
    fn bytecode(self, step: &str, bc: &mut Bytecode);
}

impl EmitParams for () {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![])
    }
}

impl<E, T> EmitParams for GraphTraversal<E, T> {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.into()])
    }
}

// impl<S, E, T> EmitParams for PublicP2<> {
//     fn bytecode(self, step: &str, bc: &mut ByteCode) {
//         bc.push_new_step(step, vec![self.into()])
//     }
// }
