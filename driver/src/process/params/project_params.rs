use gremlin_types::structure::bytecode::Bytecode;

pub trait ProjectParams {
    fn bytecode(&self, step: &str, bc: &mut Bytecode);

    fn extend_step(&self, bc: &mut Bytecode);
}

impl ProjectParams for &str {
    fn bytecode(&self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.into()])
    }

    fn extend_step(&self, bc: &mut Bytecode) {
        bc.extend_last_step([self].iter())
    }
}

impl ProjectParams for String {
    fn bytecode(&self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.into()])
    }

    fn extend_step(&self, bc: &mut Bytecode) {
        bc.extend_last_step([self].iter())
    }
}

impl<const N: usize> ProjectParams for [&str; N] {
    fn bytecode(&self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, self.iter().map(Into::into).collect())
    }

    fn extend_step(&self, bc: &mut Bytecode) {
        bc.extend_last_step(self.iter())
    }
}

impl<const N: usize> ProjectParams for [String; N] {
    fn bytecode(&self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, self.iter().map(Into::into).collect())
    }

    fn extend_step(&self, bc: &mut Bytecode) {
        bc.extend_last_step(self.iter())
    }
}

impl ProjectParams for Vec<&str> {
    fn bytecode(&self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, self.iter().map(Into::into).collect())
    }

    fn extend_step(&self, bc: &mut Bytecode) {
        bc.extend_last_step(self.iter())
    }
}

impl ProjectParams for Vec<String> {
    fn bytecode(&self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, self.iter().map(Into::into).collect())
    }

    fn extend_step(&self, bc: &mut Bytecode) {
        bc.extend_last_step(self.iter())
    }
}
