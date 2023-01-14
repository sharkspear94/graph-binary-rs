use tinkerpop_io::{
    structure::{bytecode::Bytecode, lambda::Lambda},
    GremlinValue,
};

pub trait FoldParams {
    fn bytecode(self, step: &str, bc: &mut Bytecode);
}

impl FoldParams for () {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![]);
    }
}

impl<Seed: Into<GremlinValue>, L: Into<Lambda>> FoldParams for (Seed, L) {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.0.into(), self.1.into().into()]);
    }
}
