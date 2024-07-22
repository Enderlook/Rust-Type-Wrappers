use crate::impl_helpers;

/// Wrapper type around a value to implement `Send`.
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[repr(transparent)]
pub struct UnsafeSendSync<T>(T);

impl_helpers!(UnsafeSendSync);

impl<T> UnsafeSendSync<T> {
    /// Wraps a value.
    #[inline(always)]
    pub unsafe fn new(value: T) -> UnsafeSendSync<T> {
        Self(value)
    }
}

unsafe impl<T> Send for UnsafeSendSync<T> { }

unsafe impl<T> Sync for UnsafeSendSync<T> { }