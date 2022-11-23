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

// Implements common traits for `Saturating<T>` when the generic T implements them.
arithmetic_wrapper_traits_impl! { Saturating }

// Implements APIs for `Saturating<T>` where T is a unsigned constrained interger.
saturating_uint! {
    { u8, u8, ConstrainedU8 },
    { u16, u16, ConstrainedU16 },
    { u32, u32, ConstrainedU32 },
    { u64, u64, ConstrainedU64 },
    { u128, u128, ConstrainedU128 },
    { usize, usize, ConstrainedUsize },
}

// Implements APIs for `Saturating<T>` where T is a signed constrained interger.
saturating_int! {
    { i8, i8, ConstrainedI8 },
    { i16, i16, ConstrainedI16 },
    { i32, i32, ConstrainedI32 },
    { i64, i64, ConstrainedI64 },
    { i128, i128, ConstrainedI128 },
    { isize, isize, ConstrainedIsize },
}
