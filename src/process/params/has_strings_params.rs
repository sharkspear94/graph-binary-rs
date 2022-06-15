use crate::structure::{bytecode::ByteCode, enums::P};

pub trait HasStringsParams {
    fn bytecode(&self, name: &str, bc: &mut ByteCode);
}

impl HasStringsParams for &str {
    fn bytecode(&self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![self.into()])
    }
}

impl HasStringsParams for String {
    fn bytecode(&self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![self.into()])
    }
}

impl<const N: usize> HasStringsParams for [&str; N] {
    fn bytecode(&self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, self.iter().map(Into::into).collect())
    }
}

impl HasStringsParams for P {
    fn bytecode(&self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![self.into()])
    }
}
