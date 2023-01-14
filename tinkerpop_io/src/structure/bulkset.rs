use std::fmt::Display;

use crate::GremlinValue;

#[derive(Debug, PartialEq, Clone)]
pub struct BulkSet(pub(crate) Vec<(GremlinValue, i64)>);

impl BulkSet {
    #[must_use]
    pub fn new(vec: Vec<(GremlinValue, i64)>) -> BulkSet {
        BulkSet(vec)
    }
    #[must_use]
    pub fn bulk_set(&self) -> &Vec<(GremlinValue, i64)> {
        &self.0
    }
    #[must_use]
    pub fn bulk_set_mut(&mut self) -> &mut Vec<(GremlinValue, i64)> {
        &mut self.0
    }
}

impl Display for BulkSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (val, bulk) in &self.0 {
            write!(f, "bulk: {bulk},value: {val}",)?;
        }
        write!(f, "]")
    }
}


