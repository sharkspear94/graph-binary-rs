use tinkerpop_io::{
    structure::{bytecode::Bytecode, enums::P},
    GremlinValue,
};

pub trait HasStringsParams {
    fn bytecode(&self, name: &str, bc: &mut Bytecode);
}

impl HasStringsParams for &str {
    fn bytecode(&self, name: &str, bc: &mut Bytecode) {
        bc.push_new_step(name, vec![self.into()])
    }
}

impl HasStringsParams for String {
    fn bytecode(&self, name: &str, bc: &mut Bytecode) {
        bc.push_new_step(name, vec![self.into()])
    }
}

impl<const N: usize> HasStringsParams for [&str; N] {
    fn bytecode(&self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, self.iter().map(Into::into).collect())
    }
}

impl<T: Into<GremlinValue> + Clone> HasStringsParams for P<T> {
    // TODO remove Clone
    fn bytecode(&self, name: &str, bc: &mut Bytecode) {
        bc.push_new_step(name, vec![self.into()])
    }
}
