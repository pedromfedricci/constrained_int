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

//
macro_rules! wrapping_impl {
    ($({ $Int:ty, $md: ident, $Cnst:ident }),+ $(,)*) => {$(
        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> ::core::ops::Add
            for Wrapping<$crate::$md::$Cnst<MIN, MAX, DEF>>
        {
            type Output = Self;

            #[inline]
            fn add(self, rhs: Self) -> Self {
                Wrapping(self.0.wrapping_add(rhs.0.get()))
            }
        }

        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> ::core::ops::AddAssign
            for Wrapping<$crate::$md::$Cnst<MIN, MAX, DEF>>
        {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> ::core::ops::AddAssign<$Int>
            for Wrapping<$crate::$md::$Cnst<MIN, MAX, DEF>>
        {
            #[inline]
            fn add_assign(&mut self, rhs: $Int) {
                *self = Wrapping(self.0.wrapping_add(rhs));
            }
        }

        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> ::core::ops::Sub
            for Wrapping<$crate::$md::$Cnst<MIN, MAX, DEF>>
        {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: Self) -> Self {
                Wrapping(self.0.wrapping_sub(rhs.0.get()))
            }
        }

        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> ::core::ops::SubAssign
            for Wrapping<$crate::$md::$Cnst<MIN, MAX, DEF>>
        {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> ::core::ops::SubAssign<$Int>
            for Wrapping<$crate::$md::$Cnst<MIN, MAX, DEF>>
        {
            #[inline]
            fn sub_assign(&mut self, rhs: $Int) {
                *self = Wrapping(self.0.wrapping_sub(rhs))
            }
        }
    )+};
}

wrapping_impl! {
    { u8, u8, ConstrainedU8 }, { u16, u16, ConstrainedU16 }, { u32, u32, ConstrainedU32 },
    { u64, u64, ConstrainedU64 }, { u128, u128, ConstrainedU128 }, { usize, usize, ConstrainedUsize },

    { i8, i8, ConstrainedI8 }, { i16, i16, ConstrainedI16 }, { i32, i32, ConstrainedI32},
    { i64, i64, ConstrainedI64 }, { i128, i128, ConstrainedI128 }, { isize, isize, ConstrainedIsize },
}

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
