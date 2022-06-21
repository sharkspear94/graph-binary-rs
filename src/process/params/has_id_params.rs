use crate::{
    graph_binary::GraphBinary,
    structure::{bytecode::ByteCode, enums::PublicP2},
};

pub trait HasIdParams {
    fn bytecode(self, name: &str, bc: &mut ByteCode);
}

// TODO maybe Into<mapkeys>
impl<T: Into<GraphBinary> + Clone> HasIdParams for T {
    fn bytecode(self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![self.into()])
    }
}

impl<T> HasIdParams for PublicP2<T>
where
    T: Into<GraphBinary> + Clone,
{
    fn bytecode(self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, vec![self.into_p().into()])
    }
}
