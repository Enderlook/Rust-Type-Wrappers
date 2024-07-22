use crate::impl_helpers;

/// Wrapper type around a value to provide `Eq` trait from a `PartialEq`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct EqW<T>(T);

impl_helpers!(EqW);

impl<T> EqW<T> {
    /// Wraps a value.
    #[inline(always)]
    pub fn new(value: T) -> EqW<T> {
        Self(value)
    }
}

impl<T: PartialEq> Eq for EqW<T> {}

impl<T> From<T> for EqW<T> {
    #[inline(always)]
    fn from(value: T) -> Self {
        Self(value)
    }
}