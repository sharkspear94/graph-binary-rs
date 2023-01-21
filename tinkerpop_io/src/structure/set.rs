use crate::{
    error::DecodeError,
    macros::{TryBorrowFrom, TryMutBorrowFrom},
};
use std::fmt::Display;

use crate::GremlinValue;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Set<T>(Vec<T>);

impl<T> Set<T> {
    #[must_use]
    pub fn new(v: Vec<T>) -> Self {
        Set(v)
    }
    #[must_use]
    pub fn set(&self) -> &Vec<T> {
        &self.0
    }

    #[must_use]
    pub fn set_mut(&mut self) -> &mut Vec<T> {
        &mut self.0
    }
    #[must_use]
    pub fn inner(self) -> Vec<T> {
        self.0
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.0.iter_mut()
    }
}

impl<T> IntoIterator for Set<T> {
    type Item = T;

    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Set<T> {
    type Item = &'a T;

    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Set<T> {
    type Item = &'a mut T;

    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<T: Display> Display for Set<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for i in &self.0 {
            write!(f, "{i},")?;
        }
        write!(f, "]")
    }
}

impl<T: TryFrom<GremlinValue>> TryFrom<GremlinValue> for Vec<T> {
    type Error = DecodeError;

    fn try_from(value: GremlinValue) -> Result<Self, Self::Error> {
        match value {
            GremlinValue::List(list) => Ok(list
                .into_iter()
                .filter_map(|gb| gb.try_into().ok())
                .collect()),
            _ => Err(DecodeError::DecodeError("".to_string())),
        }
    }
}

impl TryBorrowFrom for Vec<GremlinValue> {
    fn try_borrow_from(graph_binary: &GremlinValue) -> Option<&Self> {
        match graph_binary {
            GremlinValue::List(list) => Some(list),
            _ => None,
        }
    }
}

impl TryMutBorrowFrom for Vec<GremlinValue> {
    fn try_mut_borrow_from(graph_binary: &mut GremlinValue) -> Option<&mut Self> {
        match graph_binary {
            GremlinValue::List(val) => Some(val),
            _ => None,
        }
    }
}

impl<T> From<Vec<T>> for GremlinValue
where
    T: Into<GremlinValue>,
{
    fn from(v: Vec<T>) -> Self {
        GremlinValue::List(v.into_iter().map(Into::into).collect())
    }
}
