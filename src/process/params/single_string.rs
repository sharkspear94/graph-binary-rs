use crate::structure::bytecode::ByteCode;

pub trait SingleStringParam {
    fn bytecode(self, step: &str, bc: &mut ByteCode);
}

impl SingleStringParam for () {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![])
    }
}
impl SingleStringParam for &str {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into()])
    }
}
impl SingleStringParam for String {
    fn bytecode(self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into()])
    }
}
