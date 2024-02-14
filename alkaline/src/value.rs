use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

#[cfg(feature = "serde")]
pub mod ser;

/// Early limited dynamic value type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Value {
    Number(Number),
}
// TODO: macro these Froms.
impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Self::Number(Number::Integer(v))
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Number {
    Integer(i64),
    Float(Float),
}
/// An f64 wrapper which is `Hash`, `Ord` and `Eq`. Based on Ron/SerdeJson's `Float(f64)` wrappers.
///
/// Note that the inner value _can_ be NaN/Inf (though Ron's says it cannot be), but NaN/Inf are
/// considered equal and less than numbers/etc, to allow ord to work.
#[derive(Debug, Clone, Copy)]
pub struct Float(f64);
impl From<f64> for Float {
    fn from(value: f64) -> Self {
        Self(value)
    }
}
impl From<Float> for f64 {
    fn from(Float(f): Float) -> Self {
        f
    }
}
impl Hash for Float {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.0.to_bits());
    }
}
impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        self.0.is_nan() && other.0.is_nan() || self.0 == other.0
    }
}
impl Eq for Float {}
#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for Float {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.0.is_nan(), other.0.is_nan()) {
            (true, true) => Some(Ordering::Equal),
            (true, false) => Some(Ordering::Less),
            (false, true) => Some(Ordering::Greater),
            (false, false) => self.0.partial_cmp(&other.0),
        }
    }
}
impl Ord for Float {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other)
            .expect("during Float::eq, f64 returned None on partial_cmp")
    }
}

pub struct Map;
