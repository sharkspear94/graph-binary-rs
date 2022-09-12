use gremlin_types::structure::bytecode::Bytecode;


pub trait RepeatParam {
    fn bytecode(self, step: &str, bc: &mut Bytecode);
}

