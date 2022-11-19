//! Container for intentionally-wrapped aritmethic on `T`.

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
/// use constrained_int::Wrapping;
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

// Implements serde::{Deserialize, Serialize} for Wrapping.
#[cfg(feature = "serde")]
arithmetic_wrapper_serde_impl! { Wrapping }

// Implemets some core::fmt traits for Wrapping.
arithmetic_wrapper_fmt_impl! { Debug, Display, Binary, Octal, LowerHex, UpperHex for Wrapping<T> }
