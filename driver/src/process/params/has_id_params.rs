use gremlin_types::{structure::bytecode::Bytecode, GremlinValue};

pub trait HasIdParams {
    fn bytecode(self, name: &str, bc: &mut Bytecode);
}

// TODO maybe Into<mapkeys>
impl<T: Into<GremlinValue> + Clone> HasIdParams for T {
    fn bytecode(self, name: &str, bc: &mut Bytecode) {
        bc.push_new_step(name, vec![self.into()])
    }
}
