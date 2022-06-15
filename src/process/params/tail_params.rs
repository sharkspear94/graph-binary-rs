use crate::structure::{bytecode::ByteCode, enums::Scope};

pub trait TailParams {
    fn bytecode(self, step: &str, bc: &mut ByteCode);
}

impl TailParams for () {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![])
    }
}

impl TailParams for i64 {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into()])
    }
}

impl TailParams for Scope {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into()])
    }
}

impl TailParams for (Scope, i64) {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.0.into(), self.1.into()])
    }
}
