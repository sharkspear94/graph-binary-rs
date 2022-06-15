use crate::{
    graph_binary::GraphBinary,
    structure::{bytecode::ByteCode, enums::PublicP2},
};

pub trait IsParam<E> {
    fn bytecode(&self, name: &str, bc: &mut ByteCode);
}

impl<E: Into<GraphBinary> + Clone> IsParam<E> for E {
    fn bytecode(&self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![self.into()])
    }
}

impl<E: Into<GraphBinary> + Clone> IsParam<E> for PublicP2<E> {
    fn bytecode(&self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![self.clone().into_p().into()])
    }
}
