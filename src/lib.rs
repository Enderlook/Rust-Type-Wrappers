//! Defines simple value wrappers which implement additional traits.
//!
//! These types has a `transparent` layout.
//!
//! All types also auto-derives the traits:
//! - `Clone`.
//! - `Copy`.
//! - `Debug`, which includes the name of the wrapper type.
//! - `Display`, which simply forwards the call to its inner value, making the wrapper transparent.
//! - `Hash`.
//! - `PartialEq`.
//! - `PartialOrd`.
//! - `Eq`.
//! - `Ord`.
//!
//! Additionally, they include the functions:
//! - `Self::new(value: T) -> Self`. This function is `unsafe` in unsafe wrappers.
//! - `Self::into(self) -> T`.
//!
//! Non-unsafe wrappers also implement:
//! - `Default`.
//! - `From<T>`.
//!
//! These traits are not included in `unsafe` wrappers are the only way to construct the type is using the method `unsafe fn new(value: T)`.

mod eq_w;
mod unsafe_send;
mod unsafe_send_sync;
mod unsafe_sync;

pub use eq_w::EqW;
pub use unsafe_send::UnsafeSend;
pub use unsafe_send_sync::UnsafeSendSync;
pub use unsafe_sync::UnsafeSync;

macro_rules! impl_helpers {
    ($n:ident) => {
        impl<T> $n<T> {
            /// Wrapped value.
            #[inline(always)]
            pub fn into(self) -> T {
                self.0
            }
        }

        impl<T: std::fmt::Display> std::fmt::Display for $n<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl<T> AsMut<T> for $n<T> {
            #[inline(always)]
            fn as_mut(&mut self) -> &mut T {
                &mut self.0
            }
        }

        impl<T> AsRef<T> for $n<T> {

            #[inline(always)]
            fn as_ref(&self) -> &T {
                &self.0
            }
        }

        impl<T> std::ops::Deref for $n<T> {
            type Target = T;

            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<T> std::ops::DerefMut for $n<T> {
            #[inline(always)]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

pub(crate) use impl_helpers;