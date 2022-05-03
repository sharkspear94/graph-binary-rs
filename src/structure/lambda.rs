use crate::graph_binary::GraphBinary;

#[derive(Debug, PartialEq)]
pub struct Lambda {
    language: String,
    values: Vec<GraphBinary>,
}
