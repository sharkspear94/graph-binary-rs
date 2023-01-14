use tinkerpop_io::structure::{bytecode::Bytecode, enums::Scope};

pub trait ScopeParams {
    fn bytecode(self, step: &str, bc: &mut Bytecode);
}

impl ScopeParams for () {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![])
    }
}

impl ScopeParams for Scope {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.into()])
    }
}
