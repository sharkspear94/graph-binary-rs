use tinkerpop_io::structure::{bytecode::Bytecode, enums::Scope};

pub trait DedupStepParams {
    fn bytecode(&self, step: &str, bc: &mut Bytecode);
}

impl<const N: usize> DedupStepParams for [&str; N] {
    fn bytecode(&self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, self.iter().map(Into::into).collect())
    }
}
impl<const N: usize> DedupStepParams for (Scope, [&str; N]) {
    fn bytecode(&self, step: &str, bc: &mut Bytecode) {
        let mut values = Vec::with_capacity(N + 1);
        values.push(self.0.into());
        values.extend(self.1.map(Into::into));
        bc.push_new_step(step, values)
    }
}

impl DedupStepParams for &str {
    fn bytecode(&self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.into()])
    }
}

impl DedupStepParams for String {
    fn bytecode(&self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.into()])
    }
}

impl DedupStepParams for Vec<&str> {
    fn bytecode(&self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, self.iter().map(Into::into).collect())
    }
}

impl DedupStepParams for Vec<String> {
    fn bytecode(&self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, self.iter().map(Into::into).collect())
    }
}

impl DedupStepParams for Scope {
    fn bytecode(&self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.into()])
    }
}
