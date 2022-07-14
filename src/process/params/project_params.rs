use crate::structure::bytecode::ByteCode;

pub trait ProjectParams {
    fn bytecode(&self, step: &str, bc: &mut ByteCode);

    fn extend_step(&self, bc: &mut ByteCode);
}

impl ProjectParams for &str {
    fn bytecode(&self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into()])
    }

    fn extend_step(&self, bc: &mut ByteCode) {
        bc.extend_last_step([self].iter())
    }
}

impl ProjectParams for String {
    fn bytecode(&self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into()])
    }

    fn extend_step(&self, bc: &mut ByteCode) {
        bc.extend_last_step([self].iter())
    }
}

impl<const N: usize> ProjectParams for [&str; N] {
    fn bytecode(&self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, self.iter().map(Into::into).collect())
    }

    fn extend_step(&self, bc: &mut ByteCode) {
        bc.extend_last_step(self.iter())
    }
}

impl<const N: usize> ProjectParams for [String; N] {
    fn bytecode(&self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, self.iter().map(Into::into).collect())
    }

    fn extend_step(&self, bc: &mut ByteCode) {
        bc.extend_last_step(self.iter())
    }
}

impl ProjectParams for Vec<&str> {
    fn bytecode(&self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, self.iter().map(Into::into).collect())
    }

    fn extend_step(&self, bc: &mut ByteCode) {
        bc.extend_last_step(self.iter())
    }
}

impl ProjectParams for Vec<String> {
    fn bytecode(&self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, self.iter().map(Into::into).collect())
    }

    fn extend_step(&self, bc: &mut ByteCode) {
        bc.extend_last_step(self.iter())
    }
}
