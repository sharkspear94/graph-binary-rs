use std::{collections::HashMap, fmt::Display};

use crate::{conversion, GremlinValue};

#[derive(Debug, PartialEq, Clone)]
pub struct Traverser {
    pub bulk: i64,
    pub value: Box<GremlinValue>,
}

impl Traverser {
    #[must_use]
    pub fn new(bulk: i64, value: GremlinValue) -> Traverser {
        Traverser {
            bulk,
            value: Box::new(value),
        }
    }
    #[must_use]
    pub fn bulk(&self) -> &i64 {
        &self.bulk
    }

    #[must_use]
    pub fn value(&self) -> &GremlinValue {
        &self.value
    }
}

impl Display for Traverser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "bulk:{},{}", self.bulk, self.value)
    }
}

pub struct TraverserIter<'a> {
    bulk: usize,
    val: &'a GremlinValue,
}

impl Traverser {
    #[must_use]
    pub fn iter(&self) -> TraverserIter {
        TraverserIter {
            bulk: self.bulk as usize,
            val: &self.value,
        }
    }
}

impl<'a> Iterator for TraverserIter<'a> {
    type Item = &'a GremlinValue;
    fn next(&mut self) -> Option<Self::Item> {
        if self.bulk > 0 {
            self.bulk -= 1;
            Some(self.val)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.bulk, Some(self.bulk))
    }
}

pub struct IntoTraverserIter {
    bulk: usize,
    val: GremlinValue,
}

impl Iterator for IntoTraverserIter {
    type Item = GremlinValue;
    fn next(&mut self) -> Option<Self::Item> {
        if self.bulk > 0 {
            self.bulk -= 1;
            Some(self.val.clone())
        } else {
            None
        }
    }
}

impl IntoIterator for Traverser {
    type Item = GremlinValue;

    type IntoIter = IntoTraverserIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoTraverserIter {
            bulk: self.bulk as usize,
            val: *self.value,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TraversalStrategy {
    pub strategy_class: String,                       // class
    pub configuration: HashMap<String, GremlinValue>, // not sure if key is correct
}

impl Display for TraversalStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "class:{},config:[", self.strategy_class)?;
        for (key, val) in &self.configuration {
            write!(f, "({key}:{val}),")?;
        }
        write!(f, "]")
    }
}

conversion!(Traverser, Traverser);
conversion!(TraversalStrategy, TraversalStrategy);

#[test]
fn test_iter() {
    let t = Traverser {
        bulk: 3,
        value: Box::new(1.into()),
    };
    let mut iter = t.iter();
    assert_eq!(iter.next(), Some(&1.into()));
    assert_eq!(iter.next(), Some(&1.into()));
    assert_eq!(iter.next(), Some(&1.into()));
    assert_eq!(iter.next(), None)
}
#[test]
fn iter() {
    let t = Traverser {
        bulk: 3,
        value: Box::new(1.into()),
    };
    let mut iter = t.iter();
    assert_eq!(iter.next(), Some(&1.into()));
    assert_eq!(iter.next(), Some(&1.into()));
    assert_eq!(iter.next(), Some(&1.into()));
    assert_eq!(iter.next(), None)
}
