use crate::graph_binary::GraphBinary;

#[macro_export]
macro_rules! struct_de_serialize {
    ($(($t:ident,$visitor:ident,$capa:literal)),*) => {
        $(
            impl<'de> serde::Deserialize<'de> for $t {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    deserializer.deserialize_any($visitor)
                }
            }

            struct $visitor;

            impl<'de> serde::de::Visitor<'de> for $visitor {
                type Value = $t;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(formatter, concat!("a struct ", stringify!($t)))
                }

                fn visit_bytes<E>(self, mut v: &[u8]) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    match $t::decode(&mut v) {
                        Ok(val) => Ok(val),
                        Err(_) => Err(E::custom(concat!(stringify!($t)," Visitor Decode Error"))),
                    }
                }
            }

            impl serde::ser::Serialize for $t {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: serde::Serializer, {
                    let mut buf: Vec<u8> = Vec::with_capacity($capa);
                    match self.encode(&mut buf) {
                        Ok(_) => serializer.serialize_bytes(&buf),
                        Err(e) => Err(serde::ser::Error::custom(format!(
                            "serilization Error of {}: reason: {}",stringify!($t),e
                        ))),
                }
                }
            }
         )*
    };
}
pub trait TryBorrowFrom {
    fn try_borrow_from(graph_binary: &GraphBinary) -> Option<&Self>;
}

pub trait TryMutBorrowFrom {
    fn try_mut_borrow_from(graph_binary: &mut GraphBinary) -> Option<&mut Self>;
}

#[macro_export]
macro_rules! conversions {
    ($(($t:ty,$variant:ident)),*) => {
        $(
        impl From<$t> for GraphBinary {
            fn from(g: $t) -> Self {
                GraphBinary::$variant(g)
            }
        }


        impl TryFrom<GraphBinary> for $t {
        type Error = crate::error::DecodeError;

        fn try_from(value: GraphBinary) -> Result<Self, Self::Error> {
            match value {
                GraphBinary::$variant(val) => Ok(val),
                _ => Err(crate::error::DecodeError::ConvertError(
                    format!("cannot convert GraphBinary to {}",stringify!($t))
                )),
            }
        }
        }

        impl crate::macros::TryBorrowFrom for $t {
        fn try_borrow_from(graph_binary: &GraphBinary) -> Option<&Self> {
            match graph_binary {
                GraphBinary::$variant(val) => Some(val),
                _ => None
            }
        }
        }

        impl crate::macros::TryMutBorrowFrom for $t {
        fn try_mut_borrow_from(graph_binary: &mut GraphBinary) -> Option<&mut Self> {
            match graph_binary {
                GraphBinary::$variant(val) => Some(val),
                _ => None
            }
        }
        }
        )*
    }
}
