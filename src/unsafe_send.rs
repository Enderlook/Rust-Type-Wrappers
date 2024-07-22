use crate::impl_helpers;

/// Wrapper type around a value to implement `Send`.
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[repr(transparent)]
pub struct UnsafeSend<T>(T);

impl_helpers!(UnsafeSend);

impl<T> UnsafeSend<T> {
    /// Wraps a value.
    #[inline(always)]
    pub unsafe fn new(value: T) -> UnsafeSend<T> {
        Self(value)
    }
}

unsafe impl<T> Send for UnsafeSend<T> { }