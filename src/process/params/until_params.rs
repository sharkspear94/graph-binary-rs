use crate::{
    process::traversal::GraphTraversal,
    structure::{bytecode::ByteCode, enums::PublicP2},
};

pub trait UntilParams {
    fn bytecode(self, step: &str, bc: &mut ByteCode);
}

impl< E, T> UntilParams for PublicP2<GraphTraversal< E, T>> {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into_p().into()])
    }
}

impl< E, T> UntilParams for GraphTraversal< E, T> {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into()])
    }
}
