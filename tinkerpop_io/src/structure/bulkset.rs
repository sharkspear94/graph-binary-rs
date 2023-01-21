use std::{
    fmt::Display,
    slice::{Iter, IterMut},
};

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
    pub fn iter(&self) -> BulkSetIter {
        BulkSetIter {
            iter: self.0.iter(),
        }
    }
    pub fn iter_mut(&mut self) -> BulkSetIterMut {
        BulkSetIterMut {
            iter: self.0.iter_mut(),
        }
    }
}

pub struct BulkSetIter<'a> {
    iter: Iter<'a, (GremlinValue, i64)>,
}
pub struct BulkSetIterMut<'a> {
    iter: IterMut<'a, (GremlinValue, i64)>,
}

impl<'a> Iterator for BulkSetIter<'a> {
    type Item = &'a (GremlinValue, i64);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a> Iterator for BulkSetIterMut<'a> {
    type Item = &'a mut (GremlinValue, i64);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl IntoIterator for BulkSet {
    type Item = (GremlinValue, i64);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a BulkSet {
    type Item = &'a (GremlinValue, i64);

    type IntoIter = BulkSetIter<'a>;

    fn into_iter(self) -> BulkSetIter<'a> {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut BulkSet {
    type Item = &'a mut (GremlinValue, i64);

    type IntoIter = BulkSetIterMut<'a>;

    fn into_iter(self) -> BulkSetIterMut<'a> {
        self.iter_mut()
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
