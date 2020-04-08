//! Conversion utilities

// Static assertion: We expect the system architecture bus width to be >= 32 bits. If it is not,
// the following line will cause a compiler error. (Ignore the unrelated error message itself.)
const _: usize = 0 - !(std::mem::size_of::<usize>() >= 32 >> 3) as usize;

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
        mod impls {
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

/// Replacement for using `as` to clearly indicate truncation.
pub(crate) trait Truncate<T> {
    fn truncate(value: T) -> Self;
}

impl Truncate<u64> for u8 {
    #[inline]
    fn truncate(value: u64) -> u8 {
        value as u8
    }
}

/// Type-safe replacement for a bitmask that can reduce the size of the lefthand side.
///
/// By requiring that `mask` is `Mask`, we get an error if a value larger than `Mask::max_value()`
/// is used.
pub(crate) trait BitMask<Mask> {
    fn bitmask(self, mask: Mask) -> Mask;
}

impl BitMask<u8> for usize {
    #[inline]
    fn bitmask(self, mask: u8) -> u8 {
        (self & mask as usize) as u8
    }
}

impl BitMask<u8> for u32 {
    #[inline]
    fn bitmask(self, mask: u8) -> u8 {
        (self & mask as u32) as u8
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn into_usize_common() {
        assert_eq!(1u8.into_usize(), 1usize);
        assert_eq!(1u16.into_usize(), 1usize);
        assert_eq!(1u32.into_usize(), 1usize);
    }

    #[test]
    #[cfg(target_pointer_width = "32")]
    fn from_usize_only_32() {
        assert_eq!(u32::from_usize(1), 1);
        assert_eq!(u64::from_usize(1), 1);
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn from_usize_only_64() {
        // Should be compile-time error:
        //assert_eq!(u32::from_usize(1), 1);
        assert_eq!(u64::from_usize(1), 1);
    }

    #[test]
    fn truncate_pass() {
        assert_eq!(u8::truncate(0xaa_aau64), 0xaa);
    }

    #[test]
    fn bitmask_pass() {
        assert_eq!(usize::max_value().bitmask(0x0f), 0x0f);
    }
}
