use crate::{
    process::bytecode_traversal::BytecodeTraversal,
    structure::{bytecode::ByteCode, enums::PublicP2},
};

pub trait UntilParams {
    fn bytecode(self, step: &str, bc: &mut ByteCode);
}

impl UntilParams for PublicP2<BytecodeTraversal> {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into_p().into()])
    }
}

impl UntilParams for BytecodeTraversal {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into()])
    }
}
