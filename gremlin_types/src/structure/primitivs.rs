use crate::macros::{TryBorrowFrom, TryMutBorrowFrom};

use crate::{conversion, GremlinValue};
use uuid::Uuid;

impl TryBorrowFrom for str {
    fn try_borrow_from(graph_binary: &GremlinValue) -> Option<&Self> {
        match graph_binary {
            GremlinValue::String(s) => Some(s),
            _ => None,
        }
    }
}

impl TryMutBorrowFrom for str {
    fn try_mut_borrow_from(graph_binary: &mut GremlinValue) -> Option<&mut Self> {
        match graph_binary {
            GremlinValue::String(s) => Some(s),
            _ => None,
        }
    }
}

impl From<&str> for GremlinValue {
    fn from(s: &str) -> Self {
        GremlinValue::String(s.to_owned())
    }
}

conversion!(String, String);
conversion!(u8, Byte);
conversion!(i16, Short);
conversion!(i32, Int);
conversion!(i64, Long);
conversion!(f32, Float);
conversion!(f64, Double);
conversion!(bool, Boolean);
conversion!(Uuid, Uuid);
