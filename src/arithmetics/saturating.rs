//! Container for intentionally-saturating aritmethic on `T`.

/// Provides intentionally-saturating arithmetic on `T`.
///
/// Saturating arithmetic can be achieved either through methods like
/// `saturating_add`, or through the `Saturating<T>` type, which says that
/// all standard arithmetic operations on the underlying value are
/// intended to have saturating semantics.
///
/// The underlying value can be retrieved through the `.0` index of the
/// `Saturating` tuple.
///
/// # Layout
///
/// `Saturating<T>` is guaranteed to have the same layout and ABI as `T`.
///
/// # Example
///
/// ```
/// use constrained_int::i8::ConstrainedI8;
/// use constrained_int::Saturating;
///
/// // Default set to 0.
/// type Saturated = Saturating<ConstrainedI8<-10, 12, 0>>;
///
/// let mut saturated = Saturated::default();
/// saturated += 13;
/// assert_eq!(saturated.0.get(), 12);
///
/// saturated = saturated - saturated - saturated;
/// assert_eq!(saturated.0.get(), -10);
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Hash)]
#[repr(transparent)]
pub struct Saturating<T>(pub T);

// Implements serde::{Deserialize, Serialize} for Saturating.
#[cfg(feature = "serde")]
arithmetic_wrapper_serde_impl! { Saturating }

// Implemets some core::fmt traits for Wrapping.
arithmetic_wrapper_fmt_impl! { Debug, Display, Binary, Octal, LowerHex, UpperHex for Saturating<T> }
