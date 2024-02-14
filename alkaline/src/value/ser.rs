use self::serializer::Serializer;
use super::Value;
use serde::Serialize;

pub mod serializer;

/// Construct a [`Value`] from a serde serializable value.
pub fn to_value<T: Serialize>(t: T) -> Result<Value, serializer::Error> {
    t.serialize(Serializer)
}
