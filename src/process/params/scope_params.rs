use crate::structure::{bytecode::ByteCode, enums::Scope};

pub trait ScopeParams {
    fn bytecode(self, step: &str, bc: &mut ByteCode);
}

impl ScopeParams for () {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![])
    }
}

impl ScopeParams for Scope {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into()])
    }
}
