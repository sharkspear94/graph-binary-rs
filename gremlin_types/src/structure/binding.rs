use std::fmt::Display;

use crate::{conversion, GremlinValue};

#[derive(Debug, PartialEq, Clone)]
pub struct Binding {
    pub(crate) key: String,
    pub(crate) value: Box<GremlinValue>,
}

impl Binding {
    #[must_use]
    pub fn new(key: &str, value: impl Into<GremlinValue>) -> Self {
        Binding {
            key: key.to_owned(),
            value: Box::new(value.into()),
        }
    }
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }
    #[must_use]
    pub fn value(&self) -> &GremlinValue {
        &self.value
    }
    #[must_use]
    pub fn value_mut(&mut self) -> &mut GremlinValue {
        &mut self.value
    }
}

impl<S: ToString, I: Into<GremlinValue>> From<(S, I)> for Binding {
    fn from(pair: (S, I)) -> Self {
        Binding {
            key: pair.0.to_string(),
            value: Box::new(pair.1.into()),
        }
    }
}

impl Display for Binding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.key, self.value)
    }
}

conversion!(Binding, Binding);
