use tinkerpop_io::{structure::bytecode::Bytecode, GremlinValue};

pub trait ObjectParam {
    fn bytecode(&self, step: &str, bc: &mut Bytecode);

    fn extend_step(&self, bc: &mut Bytecode);
}

impl<T: Into<GremlinValue> + Clone> ObjectParam for T {
    fn bytecode(&self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.into()])
    }

    fn extend_step(&self, bc: &mut Bytecode) {
        bc.add_to_last_step(self)
    }
}

// impl ObjectParam for () {
//     fn bytecode(&self, step: &str, bc: &mut Bytecode) {
//         bc.push_new_step(step, vec![])
//     }

//     fn extend_step(&self, _bc: &mut Bytecode) {}
// }

pub trait MultiObjectParam {
    fn bytecode(&self, step: &str, bc: &mut Bytecode);

    fn extend_step(&self, bc: &mut Bytecode);
}

impl<T: Into<GremlinValue> + Clone> MultiObjectParam for T {
    fn bytecode(&self, step: &str, bc: &mut Bytecode) {
        bc.push_new_step(step, vec![self.into()])
    }

    fn extend_step(&self, bc: &mut Bytecode) {
        bc.add_to_last_step(self)
    }
}

// impl MultiObjectParam for () {
//     fn bytecode(&self, step: &str, bc: &mut Bytecode) {
//         bc.push_new_step(step, vec![])
//     }

//     fn extend_step(&self, _bc: &mut Bytecode) {}
// }

// impl<T, const N: usize> MultiObjectParam for [T; N] {
//     fn bytecode(&self, step: &str, bc: &mut ByteCode) {
//         bc.push_new_step(step, self.iter().map(Into::into).collect())
//     }

//     fn extend_step(&self, bc: &mut ByteCode) {
//         bc.extend_last_step(self.iter())
//     }
// }
