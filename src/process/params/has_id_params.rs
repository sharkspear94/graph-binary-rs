
use crate::{
    graph_binary::GraphBinary,
    structure::{bytecode::ByteCode, enums::PublicP2},
};
struct Assert<const COND: bool> {}
trait IsTrue {}

impl IsTrue for Assert<true> {}

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

impl<T: Into<GraphBinary>, const N: usize> HasIdParams for [T; N]
// where
//     Assert<{ N > 0 }>: IsTrue,
{
    fn bytecode(self, name: &str, bc: &mut ByteCode) {
        bc.add_step(name, self.into_iter().map(Into::into).collect())
    }
}
