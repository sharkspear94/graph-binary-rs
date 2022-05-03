use std::collections::HashMap;

use crate::graph_binary::GraphBinary;

#[derive(Debug, PartialEq)]
pub struct Metrics {
    id: String,
    name: String,
    duration: i64,
    counts: HashMap<String, i64>,
    annotation: HashMap<String, GraphBinary>,
    nested_metrics: Vec<Metrics>,
}

#[derive(Debug, PartialEq)]
pub struct TraversalMetrics {
    duration: i64,
    metrics: Vec<Metrics>,
}
