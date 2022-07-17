use crate::GremlinValue;

pub trait TryBorrowFrom {
    fn try_borrow_from(graph_binary: &GremlinValue) -> Option<&Self>;
}

pub trait TryMutBorrowFrom {
    fn try_mut_borrow_from(graph_binary: &mut GremlinValue) -> Option<&mut Self>;
}

#[macro_export]
macro_rules! conversion {
    ($t:ty,$variant:ident) => {
        impl From<$t> for $crate::GremlinValue {
            fn from(g: $t) -> Self {
                $crate::GremlinValue::$variant(g)
            }
        }

        impl TryFrom<$crate::GremlinValue> for $t {
            type Error = $crate::error::DecodeError;

            fn try_from(value: $crate::GremlinValue) -> Result<Self, Self::Error> {
                match value {
                    $crate::GremlinValue::$variant(val) => Ok(val),
                    _ => Err($crate::error::DecodeError::ConvertError(format!(
                        "cannot convert Value to {}",
                        stringify!($t)
                    ))),
                }
            }
        }

        impl $crate::macros::TryBorrowFrom for $t {
            fn try_borrow_from(graph_binary: &$crate::GremlinValue) -> Option<&Self> {
                match graph_binary {
                    $crate::GremlinValue::$variant(val) => Some(val),
                    _ => None,
                }
            }
        }

        impl $crate::macros::TryMutBorrowFrom for $t {
            fn try_mut_borrow_from(graph_binary: &mut $crate::GremlinValue) -> Option<&mut Self> {
                match graph_binary {
                    $crate::GremlinValue::$variant(val) => Some(val),
                    _ => None,
                }
            }
        }
    };
}
