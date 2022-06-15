use crate::{
    process::traversal::GraphTraversal,
    structure::{bytecode::ByteCode, enums::PublicP2},
};

pub trait UntilParams {
    fn bytecode(self, step: &str, bc: &mut ByteCode);
}

impl<S, E, T> UntilParams for PublicP2<GraphTraversal<S, E, T>> {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into_p().into()])
    }
}

impl<S, E, T> UntilParams for GraphTraversal<S, E, T> {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into()])
    }
}
