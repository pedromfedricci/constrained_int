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

// Implements common traits for `Wrapping<T>` when the generic T implements them.
arithmetic_wrapper_traits_impl! { Wrapping }

wrapping_uint! {
    { u8, u8, ConstrainedU8 },
    { u16, u16, ConstrainedU16 },
    { u32, u32, ConstrainedU32 },
    { u64, u64, ConstrainedU64 },
    { u128, u128, ConstrainedU128 },
    { usize, usize, ConstrainedUsize },
}

wrapping_int! {
    { i8, i8, ConstrainedI8 },
    { i16, i16, ConstrainedI16 },
    { i32, i32, ConstrainedI32 },
    { i64, i64, ConstrainedI64 },
    { i128, i128, ConstrainedI128 },
    { isize, isize, ConstrainedIsize },
}
