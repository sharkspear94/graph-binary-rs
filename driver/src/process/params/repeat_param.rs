use tinkerpop_io::structure::bytecode::Bytecode;

pub trait RepeatParam {
    fn bytecode(self, step: &str, bc: &mut Bytecode);
}
