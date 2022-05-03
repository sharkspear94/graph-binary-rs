use std::collections::HashMap;

use crate::graph_binary::GraphBinary;

#[derive(Debug, PartialEq)]
pub struct Traverser {
    bulk: i32,
    value: Box<GraphBinary>,
}

#[derive(Debug, PartialEq)]
pub struct TraversalStrategy {
    strategy_class: String,                      // class
    configuration: HashMap<String, GraphBinary>, // not sure if key is correct
}
