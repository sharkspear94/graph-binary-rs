use crate::structure::bytecode::ByteCode;

pub trait MultiStringParams {
    fn bytecode(&self, step: &str, bc: &mut ByteCode);

    fn extend_step(&self, bc: &mut ByteCode);
}

impl MultiStringParams for &str {
    fn bytecode(&self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into()])
    }

    fn extend_step(&self, bc: &mut ByteCode) {
        bc.extend_last_step([self].iter())
    }
}

impl MultiStringParams for String {
    fn bytecode(&self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![self.into()])
    }

    fn extend_step(&self, bc: &mut ByteCode) {
        bc.extend_last_step([self].iter())
    }
}

impl<const N: usize> MultiStringParams for [&str; N] {
    fn bytecode(&self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, self.iter().map(Into::into).collect())
    }

    fn extend_step(&self, bc: &mut ByteCode) {
        bc.extend_last_step(self.iter())
    }
}

impl<const N: usize> MultiStringParams for [String; N] {
    fn bytecode(&self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, self.iter().map(Into::into).collect())
    }

    fn extend_step(&self, bc: &mut ByteCode) {
        bc.extend_last_step(self.iter())
    }
}

impl MultiStringParams for Vec<&str> {
    fn bytecode(&self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, self.iter().map(Into::into).collect())
    }

    fn extend_step(&self, bc: &mut ByteCode) {
        bc.extend_last_step(self.iter())
    }
}

impl MultiStringParams for Vec<String> {
    fn bytecode(&self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, self.iter().map(Into::into).collect())
    }

    fn extend_step(&self, bc: &mut ByteCode) {
        bc.extend_last_step(self.iter())
    }
}

impl MultiStringParams for () {
    fn bytecode(&self, step: &str, bc: &mut ByteCode) {
        bc.add_step(step, vec![])
    }

    fn extend_step(&self, bc: &mut ByteCode) {
        let empty: [&str; 0] = [];
        bc.extend_last_step(empty.iter())
    }
}
