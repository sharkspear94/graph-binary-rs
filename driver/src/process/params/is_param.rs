use gremlin_types::{
    structure::{bytecode::Bytecode, enums::P},
    GremlinValue,
};

pub trait IsParam<E> {
    fn bytecode(&self, name: &str, bc: &mut Bytecode);
}

impl<E: Into<GremlinValue> + Clone> IsParam<E> for E {
    fn bytecode(&self, name: &str, bc: &mut Bytecode) {
        bc.push_new_step(name, vec![self.into()])
    }
}

impl<E: Into<GremlinValue> + Clone> IsParam<E> for P<E> {
    fn bytecode(&self, name: &str, bc: &mut Bytecode) {
        bc.push_new_step(name, vec![self.clone().into()])
    }
}
