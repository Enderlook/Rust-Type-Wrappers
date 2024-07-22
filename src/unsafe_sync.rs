use crate::impl_helpers;

/// Wrapper type around a value to implement `Send`.
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[repr(transparent)]
pub struct UnsafeSync<T>(T);

impl_helpers!(UnsafeSync);

impl<T> UnsafeSync<T> {
    /// Wraps a value.
    #[inline(always)]
    pub unsafe fn new(value: T) -> UnsafeSync<T> {
        Self(value)
    }
}

unsafe impl<T> Sync for UnsafeSync<T> { }