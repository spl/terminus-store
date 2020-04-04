//! Conversion utilities

/// A type-safe replacement for `value as usize`, where `value: $ty` fits in `usize` without
/// modification.
///
/// We only provide implementations for numeric types that can be safely converted into `usize`. If
/// `into_usize` is used on an architecture for which the implementation is not defined, there will
/// be a compile-time error.
pub(crate) trait IntoUsize {
    fn into_usize(self) -> usize;
}

/// A type-safe replacement for `value as $ty`, where `value: usize` fits in `$ty` without
/// modification.
///
/// We only provide implementations for numeric types that can be safely converted from `usize`. If
/// `from_usize` is used on an architecture for which the implementation is not defined, there will
/// be a compile-time error.
pub(crate) trait FromUsize {
    fn from_usize(value: usize) -> Self;
}

macro_rules! impls {
    ($pointer_width:expr, [ $($source:ty),* ], [ $($target:ty),* ]) => {
        #[cfg(target_pointer_width = $pointer_width)]
        mod into_usize_impls {
            use super::*;

            $(
                impl IntoUsize for $source {
                    #[inline]
                    fn into_usize(self) -> usize {
                        self as usize
                    }
                }
            )*

            $(
                impl FromUsize for $target {
                    #[inline]
                    fn from_usize(value: usize) -> $target {
                        value as $target
                    }
                }
            )*
        }
    }
}

impls!("32", [u8, u16, u32], [u32, u64]);
impls!("64", [u8, u16, u32, u64], [u64]);
