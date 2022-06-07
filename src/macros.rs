#[macro_export]
macro_rules! struct_de_serialize {
    ($(($t:ident,$visitor:ident,$capa:literal)),*) => {
        $(
            impl<'de> serde::Deserialize<'de> for $t {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    deserializer.deserialize_bytes($visitor)
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
                    match $t::fully_self_decode(&mut v) {
                        Ok(val) => Ok(val),
                        Err(_) => Err(E::custom(concat!(stringify!($t)," Visitor Decode Error"))),
                    }
                }
            }

            impl serde::ser::Serialize for $t {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut buf: Vec<u8> = Vec::with_capacity($capa);
        match self.write_full_qualified_bytes(&mut buf) {
            Ok(_) => serializer.serialize_bytes(&buf),
            Err(e) => Err(serde::ser::Error::custom(format!(
                "serilization Error of {}: reason: {}",stringify!($t),e
            ))),
        }
    }
            }

            impl From<$t> for GraphBinary {
                fn from(g: $t) -> Self {
                    GraphBinary::$t(g)
                }
            }
         )*
    };
}
