use tinkerpop_io::structure::bytecode::Bytecode;

pub trait SingleStringParam {
    fn bytecode(self, step: &str, bc: &mut Bytecode);
}

impl SingleStringParam for () {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![])
    }
}
impl SingleStringParam for &str {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.into()])
    }
}
impl SingleStringParam for String {
    fn bytecode(self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.into()])
    }
}
