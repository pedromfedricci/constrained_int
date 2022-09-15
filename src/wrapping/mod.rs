//! Container for intentionally-wrapped aritmethic on `T`.

// Import:
// - `wrapping_fmt_impl!`.
// - `wrapping_uint_impl_for!`.
// - `wrapping_int_impl_for!`.
#[macro_use]
mod macros;

/// Provides intentionally-wrapped arithmetic on `T`.
///
/// Wrapping arithmetic can be achieved either through methods like
/// `wrapping_add`, or through the `Wrapping<T>` type, which says that
/// all standard arithmetic operations on the underlying value are
/// intended to have wrapping semantics.
///
/// The underlying value can be retrieved through the .0 index of the
/// Wrapping tuple.
///
/// # Layout
///
/// `Wrapping<T>` is guaranteed to have the same layout and ABI as `T`.
///
/// # Example
///
/// ```
/// use constrained_int::i8::ConstrainedI8;
/// use constrained_int::wrapping::Wrapping;
///
/// // Default set to 0.
/// type Wrapped = Wrapping<ConstrainedI8<-10, 12, 0>>;
///
/// let mut wrapped = Wrapped::default();
/// wrapped += 13;
/// assert_eq!(wrapped.0.get(), -10);
///
/// wrapped = wrapped - wrapped;
/// assert_eq!(wrapped.0.get(), 0);
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Hash)]
#[repr(transparent)]
pub struct Wrapping<T>(pub T);

#[cfg(feature = "serde")]
#[doc(cfg(feature = "serde"))]
mod serde {
    use super::Wrapping;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl<T: Serialize> Serialize for Wrapping<T> {
        #[inline]
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.0.serialize(serializer)
        }
    }

    impl<'de, T: Deserialize<'de>> Deserialize<'de> for Wrapping<T> {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            Deserialize::deserialize(deserializer).map(Wrapping)
        }
    }
}

// Implemets some core::fmt traits for Wrapping.
wrapping_fmt_impl! { Debug, Display, Binary, Octal, LowerHex, UpperHex for Wrapping<T> }

// Defines Wrapping impls, tests and default doc values for unsigned integers.
// Format:
//  { uint, uint_mod, UnsType },+
wrapping_uint_impl_for! {
    { u8, u8, ConstrainedU8 },
    { u16, u16, ConstrainedU16 },
    { u32, u32, ConstrainedU32 },
    { u64, u64, ConstrainedU64 },
    { u128, u128, ConstrainedU128 },
    { usize, usize, ConstrainedUsize },
}

// Defines Wrapping impls, tests and default doc values for signed integers.
// Format:
//  { sint, sint_mod, SigType },+
wrapping_int_impl_for! {
    { i8, i8, ConstrainedI8 },
    { i16, i16, ConstrainedI16 },
    { i32, i32, ConstrainedI32 },
    { i64, i64, ConstrainedI64 },
    { i128, i128, ConstrainedI128 },
    { isize, isize, ConstrainedIsize },
}
