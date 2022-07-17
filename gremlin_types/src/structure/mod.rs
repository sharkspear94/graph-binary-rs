pub mod edge;
pub mod list;
pub mod map;
pub mod primitivs;

pub mod vertex;

pub mod property;

mod binding;

pub use self::binding::Binding;
pub mod bulkset;
pub mod bytebuffer;
pub mod bytecode;
pub mod enums;
pub mod graph;
pub mod lambda;
pub mod metrics;
pub mod path;
pub mod traverser;
pub mod tree;
pub mod vertex_property;

#[macro_export]
macro_rules! val_by_key_v1 {
    ($obj:expr,$key:literal,$expected:ty,$context:literal) => {
        $obj.and_then(|m| m.get($key))
            .and_then(|j_val| <$expected>::decode_v1(j_val).ok())
            .ok_or_else(|| {
                DecodeError::DecodeError(format!(
                    "Error extracting a {} from key: {}, during {} v1 decoding",
                    stringify!($expected),
                    $key,
                    $context
                ))
            })
    };
}

#[macro_export]
macro_rules! val_by_key_v2 {
    ($obj:expr,$key:literal,$expected:ty,$context:literal) => {
        $obj.and_then(|m| m.get($key))
            .and_then(|j_val| <$expected>::decode_v2(j_val).ok())
            .ok_or_else(|| {
                DecodeError::DecodeError(format!(
                    "Error extracting a {} from key: {}, during {} v2 decoding",
                    stringify!($expected),
                    $key,
                    $context
                ))
            })
    };
}

#[macro_export]
macro_rules! val_by_key_v3 {
    ($obj:expr,$key:literal,$expected:ty,$context:literal) => {
        $obj.and_then(|m| m.get($key))
            .and_then(|j_val| <$expected>::decode_v3(j_val).ok())
            .ok_or_else(|| {
                DecodeError::DecodeError(format!(
                    "Error extracting a {} from key: {}, during {} v3 decoding",
                    stringify!($expected),
                    $key,
                    $context
                ))
            })
    };
}

#[cfg(feature = "graph_son")]
pub fn validate_type_entry(
    map: &serde_json::Map<String, serde_json::Value>,
    type_value: &str,
) -> bool {
    map.get("@type")
        .and_then(|val| val.as_str())
        .filter(|s| s.eq(&type_value))
        .is_some()
}
