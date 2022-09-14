// Implements core::ops Traits for a `Wrapping<`Constrained`>` type.
// Requires `const_trait_impl` and `const_mut_refs` features.
macro_rules! wrapping_ops_impl {
    ($Int:ty, $Cnst:ident) => {
        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const
            Add for Wrapping<$Cnst<MIN, MAX, DEF>>
        {
            type Output = Self;

            #[inline]
            fn add(self, rhs: Self) -> Self {
                Wrapping(self.0.wrapping_add(rhs.0.get()))
            }
        }

        forward_ref_binop! {
            impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const
                Add<Wrapping<$Cnst<MIN, MAX, DEF>>>, add for Wrapping<$Cnst<MIN, MAX, DEF>>
        }

        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const
            AddAssign for Wrapping<$Cnst<MIN, MAX, DEF>>
        {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        forward_ref_op_assign! {
            impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const
                AddAssign<Wrapping<$Cnst<MIN, MAX, DEF>>>, add_assign for Wrapping<$Cnst<MIN, MAX, DEF>>
        }

        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const
            AddAssign<$Int> for Wrapping<$Cnst<MIN, MAX, DEF>>
        {
            #[inline]
            fn add_assign(&mut self, rhs: $Int) {
                *self = Wrapping(self.0.wrapping_add(rhs));
            }
        }

        forward_ref_op_assign! {
            impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const
                AddAssign<$Int>, add_assign for Wrapping<$Cnst<MIN, MAX, DEF>>
        }

        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const
            Sub for Wrapping<$Cnst<MIN, MAX, DEF>>
        {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: Self) -> Self {
                Wrapping(self.0.wrapping_sub(rhs.0.get()))
            }
        }

        forward_ref_binop! {
            impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const
                Sub<Wrapping<$Cnst<MIN, MAX, DEF>>>, sub for Wrapping<$Cnst<MIN, MAX, DEF>>
        }

        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const
            SubAssign for Wrapping<$Cnst<MIN, MAX, DEF>>
        {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        forward_ref_op_assign! {
            impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const
                SubAssign<Wrapping<$Cnst<MIN, MAX, DEF>>>, sub_assign for Wrapping<$Cnst<MIN, MAX, DEF>>
        }

        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const
            SubAssign<$Int> for Wrapping<$Cnst<MIN, MAX, DEF>>
        {
            #[inline]
            fn sub_assign(&mut self, rhs: $Int) {
                *self = Wrapping(self.0.wrapping_sub(rhs))
            }
        }

        forward_ref_op_assign! {
            impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const
                SubAssign<$Int>, sub_assign for Wrapping<$Cnst<MIN, MAX, DEF>>
        }
    };
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

// Implements tests that are common to both signed and unsigned.
#[cfg(test)]
macro_rules! wrapping_tests_common {
    ($Int:ty, $Cnst:ident) => {
        type CnstTest = $Cnst<{ <$Int>::MIN + 1 }, { <$Int>::MAX - 1 }>;

        #[cfg(feature = "std")]
        #[test]
        fn fmt_impl() {
            let wrapping = Wrapping(CnstTest::default());

            // Debug
            assert_eq!(format!("{:?}", wrapping), format!("{:?}", wrapping.0));
            // Display
            assert_eq!(format!("{}", wrapping), format!("{}", wrapping.0));
            // Binary
            assert_eq!(format!("{:b}", wrapping), format!("{:b}", wrapping.0));
            // Octal
            assert_eq!(format!("{:o}", wrapping), format!("{:o}", wrapping.0));
            // LowerHex
            assert_eq!(format!("{:x}", wrapping), format!("{:x}", wrapping.0));
            // UpperHex
            assert_eq!(format!("{:X}", wrapping), format!("{:X}", wrapping.0));
        }

        wrapping_binop_tests_for! {
            { add, +, wrapping_add },
            { sub, -, wrapping_sub },
        }

        wrapping_op_assign_tests_for! {
            { add_assign, +=, wrapping_add },
            { sub_assign, -=, wrapping_sub },
        }
    };
}

#[cfg(test)]
macro_rules! wrapping_binop_tests_for {
    ($({ $md:ident, $op:tt, $func:ident }),+ $(,)?) => {$(
        #[cfg(test)]
        mod $md {
            use super::*;

            #[test]
            fn self_rhs_to_self() {
                let mut cnst = CnstTest::default();
                let mut wrapping = Wrapping(cnst);

                cnst = cnst.$func(cnst.get());
                wrapping = wrapping $op wrapping;

                assert_eq!(cnst.get(), wrapping.0.get());
            }

            #[test]
            fn self_rhs_to_ref_self() {
                let mut cnst = CnstTest::default();
                let mut wrapping = Wrapping(cnst);

                cnst = cnst.$func(cnst.get());
                wrapping = &wrapping $op wrapping;

                assert_eq!(cnst.get(), wrapping.0.get());
            }

            #[test]
            fn ref_self_rhs_to_self() {
                let mut cnst = CnstTest::default();
                let mut wrapping = Wrapping(cnst);

                cnst = cnst.$func(cnst.get());
                wrapping = wrapping $op &wrapping;

                assert_eq!(cnst.get(), wrapping.0.get());
            }

            #[test]
            fn ref_self_rhs_to_ref_self() {
                let mut cnst = CnstTest::default();
                let mut wrapping = Wrapping(cnst);

                cnst = cnst.$func(cnst.get());
                wrapping = &wrapping $op &wrapping;

                assert_eq!(cnst.get(), wrapping.0.get());
            }
        }
    )+};
}

#[cfg(test)]
macro_rules! wrapping_op_assign_tests_for {
    ($({ $md:ident, $op:tt, $func:ident }),+ $(,)?) => {$(
        #[cfg(test)]
        mod $md {
            use super::*;

            #[test]
            fn ref_self_to_self() {
                let mut cnst = CnstTest::default();
                let mut wrapping = Wrapping(cnst);
                let rhs = wrapping;

                cnst = cnst.$func(cnst.get());
                wrapping $op &rhs;

                assert_eq!(cnst.get(), wrapping.0.get());
            }

            #[test]
            fn ref_prim_to_self() {
                let mut cnst = CnstTest::default();
                let mut wrapping = Wrapping(cnst);
                let rhs = wrapping;

                cnst = cnst.$func(cnst.get());
                wrapping $op &rhs.0.get();

                assert_eq!(cnst.get(), wrapping.0.get());
            }
        }
    )+};
}
