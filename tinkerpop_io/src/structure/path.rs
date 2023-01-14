use std::fmt::Display;

use crate::conversion;
use crate::GremlinValue;

use super::set::Set;

#[derive(Debug, PartialEq, Clone)]
pub struct Path {
    pub(crate) labels: Vec<Set<String>>,   // List<Set<String>>
    pub(crate) objects: Vec<GremlinValue>, // List<T>
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (labels, object) in self.labels.iter().zip(&self.objects) {
            write!(f, "labels:[")?;
            if !labels.set().is_empty() {
                for label in &labels.set()[..labels.set().len() - 1] {
                    write!(f, "{label},")?;
                }
                write!(f, "{}", labels.set().last().unwrap())?;
            }
            writeln!(f, "],object[{object}]")?;
        }
        Ok(())
    }
}

conversion!(Path, Path);
