use crate::structure::bytecode::ByteCode;

pub trait RepeatParam {
    fn bytecode(self, step: &str, bc: &mut ByteCode);
}

