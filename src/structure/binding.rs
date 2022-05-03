use crate::graph_binary::GraphBinary;

#[derive(Debug, PartialEq)]
pub struct Binding {
    key: String,
    value: Box<GraphBinary>,
}
