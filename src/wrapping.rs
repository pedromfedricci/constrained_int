use core::ops::{Add, AddAssign, Sub, SubAssign};

/// Provides intentionally-wrapped arithmetic on `T`.
///
/// Wrapping arithmetic can be achieved either through methods like
/// `wrapping_add`, or through the `Wrapping<T>` type, which says that
/// all standard arithmetic operations on the underlying value are
/// intended to have wrapping semantics.
///
/// # Layout
///
/// `Wrapping<T>` is guaranteed to have the same layout and ABI as `T`.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Hash)]
#[repr(transparent)]
pub struct Wrapping<T>(pub T);

// Implemets some core::fmt traits for Wrapping.
macro_rules! wrapping_fmt_impl {
    ($($Trait:ident),+ for Wrapping<T>) => {$(
        impl<T: ::core::fmt::$Trait> ::core::fmt::$Trait for Wrapping<T> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                self.0.fmt(f)
            }
        }
    )+};
}

wrapping_fmt_impl! { Debug, Display, Binary, Octal, LowerHex, UpperHex for Wrapping<T> }

// Implements core::ops Traits for a `Wrapping<Constrained>` type.
// Requires `const_trait_impl` and `const_ops` features.
macro_rules! wrapping_impl {
    ($({ $Int:ty, $Cnst:ident }),+ $(,)*) => {$(
        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const Add
            for Wrapping<$Cnst<MIN, MAX, DEF>>
        {
            type Output = Self;

            #[inline]
            fn add(self, rhs: Self) -> Self {
                Wrapping(self.0.wrapping_add(rhs.0.get()))
            }
        }
        forward_ref_binop! {
            impl<const $Int> const Add<Wrapping<$Cnst>>, add for Wrapping<$Cnst>
        }

        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const AddAssign
            for Wrapping<$Cnst<MIN, MAX, DEF>>
        {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }
        forward_ref_op_assign! {
            impl<const $Int> const AddAssign<Wrapping<$Cnst>>, add_assign for Wrapping<$Cnst>
        }

        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const AddAssign<$Int>
            for Wrapping<$Cnst<MIN, MAX, DEF>>
        {
            #[inline]
            fn add_assign(&mut self, rhs: $Int) {
                *self = Wrapping(self.0.wrapping_add(rhs));
            }
        }
        forward_ref_op_assign! {
            impl<const $Int> const AddAssign<$Int>, add_assign for Wrapping<$Cnst>
        }

        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const Sub
            for Wrapping<$Cnst<MIN, MAX, DEF>>
        {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: Self) -> Self {
                Wrapping(self.0.wrapping_sub(rhs.0.get()))
            }
        }
        forward_ref_binop! {
            impl<const $Int> const Sub<Wrapping<$Cnst>>, sub for Wrapping<$Cnst>
        }

        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const SubAssign
            for Wrapping<$Cnst<MIN, MAX, DEF>>
        {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }
        forward_ref_op_assign! {
            impl<const $Int> const SubAssign<Wrapping<$Cnst>>, sub_assign for Wrapping<$Cnst>
        }

        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const SubAssign<$Int>
            for Wrapping<$Cnst<MIN, MAX, DEF>>
        {
            #[inline]
            fn sub_assign(&mut self, rhs: $Int) {
                *self = Wrapping(self.0.wrapping_sub(rhs))
            }
        }
        forward_ref_op_assign! {
            impl<const $Int> const SubAssign<$Int>, sub_assign for Wrapping<$Cnst>
        }
    )+};
}

use crate::{
    i128::ConstrainedI128, i16::ConstrainedI16, i32::ConstrainedI32, i64::ConstrainedI64,
    i8::ConstrainedI8, isize::ConstrainedIsize, u128::ConstrainedU128, u16::ConstrainedU16,
    u32::ConstrainedU32, u64::ConstrainedU64, u8::ConstrainedU8, usize::ConstrainedUsize,
};

wrapping_impl! {
    { u8, ConstrainedU8 },
    { u16, ConstrainedU16 },
    { u32, ConstrainedU32 },
    { u64, ConstrainedU64 },
    { u128, ConstrainedU128 },
    { usize, ConstrainedUsize },

    { i8, ConstrainedI8 },
    { i16, ConstrainedI16 },
    { i32, ConstrainedI32 },
    { i64, ConstrainedI64 },
    { i128, ConstrainedI128 },
    { isize, ConstrainedIsize },
}
