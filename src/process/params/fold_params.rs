use crate::{
    graph_binary::GraphBinary,
    structure::{bytecode::ByteCode, lambda::Lambda},
};

pub trait FoldParams {
    fn bytecode(self, step: &str, bc: &mut ByteCode);
}

impl FoldParams for () {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![]);
    }
}

impl<Seed: Into<GraphBinary>, L: Into<Lambda>> FoldParams for (Seed, L) {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.0.into(), self.1.into().into()]);
    }
}
