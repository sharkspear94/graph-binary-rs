use crate::{
    process::bytecode_traversal::BytecodeTraversal,
    structure::{bytecode::ByteCode, enums::PublicP2},
};

pub trait WhereParams {
    fn bytecode(self, step: &str, bc: &mut ByteCode);
}

impl WhereParams for PublicP2<&str> {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into_p().into()])
    }
}

impl WhereParams for PublicP2<String> {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into_p().into()])
    }
}

impl WhereParams for (&str, PublicP2<&str>) {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.0.into(), self.1.into_p().into()])
    }
}

impl WhereParams for (String, PublicP2<String>) {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.0.into(), self.1.into_p().into()])
    }
}

impl WhereParams for BytecodeTraversal {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into()])
    }
}
