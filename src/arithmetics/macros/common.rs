#[cfg(feature = "serde")]
macro_rules! arithmetic_wrapper_serde_impl {
    ($Wrapper:ident) => {
        #[cfg(feature = "serde")]
        #[doc(cfg(feature = "serde"))]
        impl<T: serde::Serialize> serde::Serialize for $Wrapper<T> {
            #[inline]
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                self.0.serialize(serializer)
            }
        }

        #[cfg(feature = "serde")]
        #[doc(cfg(feature = "serde"))]
        impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for $Wrapper<T> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                serde::Deserialize::deserialize(deserializer).map($Wrapper)
            }
        }
    };
}

// Implemets some core::fmt traits for a arithmetic wrapper around Constrained types.
macro_rules! arithmetic_wrapper_fmt_impl {
    ($($Trait:ident),+ for $Wrapper:ident<T>) => {$(
        impl<T: ::core::fmt::$Trait> ::core::fmt::$Trait for $Wrapper<T> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                self.0.fmt(f)
            }
        }
    )+};
}

// Implements core::ops Traits for a arithmetic wrapper around Constrained types.
// Requires `const_trait_impl` and `const_mut_refs` features.
macro_rules! arithmetic_wrapper_ops_impl {
    ($Int:ty, $Cnst:ident, $Wrapper:ident, $add:ident, $sub:ident) => {
        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const
            Add for $Wrapper<$Cnst<MIN, MAX, DEF>>
        {
            type Output = Self;

            #[inline]
            fn add(self, rhs: Self) -> Self {
                $Wrapper(self.0.$add(rhs.0.get()))
            }
        }

        forward_ref_binop! {
            impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const
                Add<$Wrapper<$Cnst<MIN, MAX, DEF>>>, add for $Wrapper<$Cnst<MIN, MAX, DEF>>
        }

        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const
            AddAssign for $Wrapper<$Cnst<MIN, MAX, DEF>>
        {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        forward_ref_op_assign! {
            impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const
                AddAssign<$Wrapper<$Cnst<MIN, MAX, DEF>>>, add_assign for $Wrapper<$Cnst<MIN, MAX, DEF>>
        }

        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const
            AddAssign<$Int> for $Wrapper<$Cnst<MIN, MAX, DEF>>
        {
            #[inline]
            fn add_assign(&mut self, rhs: $Int) {
                *self = $Wrapper(self.0.$add(rhs));
            }
        }

        forward_ref_op_assign! {
            impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const
                AddAssign<$Int>, add_assign for $Wrapper<$Cnst<MIN, MAX, DEF>>
        }

        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const
            Sub for $Wrapper<$Cnst<MIN, MAX, DEF>>
        {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: Self) -> Self {
                $Wrapper(self.0.$sub(rhs.0.get()))
            }
        }

        forward_ref_binop! {
            impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const
                Sub<$Wrapper<$Cnst<MIN, MAX, DEF>>>, sub for $Wrapper<$Cnst<MIN, MAX, DEF>>
        }

        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const
            SubAssign for $Wrapper<$Cnst<MIN, MAX, DEF>>
        {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        forward_ref_op_assign! {
            impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const
                SubAssign<$Wrapper<$Cnst<MIN, MAX, DEF>>>, sub_assign for $Wrapper<$Cnst<MIN, MAX, DEF>>
        }

        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const
            SubAssign<$Int> for $Wrapper<$Cnst<MIN, MAX, DEF>>
        {
            #[inline]
            fn sub_assign(&mut self, rhs: $Int) {
                *self = $Wrapper(self.0.$sub(rhs))
            }
        }

        forward_ref_op_assign! {
            impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const
                SubAssign<$Int>, sub_assign for $Wrapper<$Cnst<MIN, MAX, DEF>>
        }
    };
}

// Implements tests that are common to both signed and unsigned.
#[cfg(test)]
macro_rules! arithmetic_wrapper_tests_common {
    ($Int:ty, $Cnst:ident, $Wrapper:ident, $add:ident, $sub:ident) => {
        type CnstTest = $Cnst<{ <$Int>::MIN + 1 }, { <$Int>::MAX - 1 }>;

        #[cfg(feature = "std")]
        #[test]
        fn fmt_impl() {
            let wrapper = $Wrapper(CnstTest::default());

            // Debug
            assert_eq!(format!("{:?}", wrapper), format!("{:?}", wrapper.0));
            // Display
            assert_eq!(format!("{}", wrapper), format!("{}", wrapper.0));
            // Binary
            assert_eq!(format!("{:b}", wrapper), format!("{:b}", wrapper.0));
            // Octal
            assert_eq!(format!("{:o}", wrapper), format!("{:o}", wrapper.0));
            // LowerHex
            assert_eq!(format!("{:x}", wrapper), format!("{:x}", wrapper.0));
            // UpperHex
            assert_eq!(format!("{:X}", wrapper), format!("{:X}", wrapper.0));
        }

        arithmetic_wrapper_binop_tests_for! {
            // ( wrapper_type )
            ( $Wrapper ),
            // { mod_name, operator, func_name },
            { add, +, $add },
            { sub, -, $sub },
        }

        aritmetic_wrapper_op_assign_tests_for! {
            // ( wrapper_type )
            ( $Wrapper ),
            // { mod_name, assign_operator, assign_func_name }
            { add_assign, +=, $add },
            { sub_assign, -=, $sub },
        }
    };
}

#[cfg(test)]
macro_rules! arithmetic_wrapper_binop_tests_for {
    (( $Wrapper:ident ), $({ $md:ident, $op:tt, $func:ident }),+ $(,)?) => {$(
        #[cfg(test)]
        mod $md {
            use super::*;

            #[test]
            fn self_rhs_to_self() {
                let mut cnst = CnstTest::default();
                let mut wrapper = $Wrapper(cnst);

                cnst = cnst.$func(cnst.get());
                wrapper = wrapper $op wrapper;

                assert_eq!(cnst.get(), wrapper.0.get());
            }

            #[test]
            fn self_rhs_to_ref_self() {
                let mut cnst = CnstTest::default();
                let mut wrapper = $Wrapper(cnst);

                cnst = cnst.$func(cnst.get());
                wrapper = &wrapper $op wrapper;

                assert_eq!(cnst.get(), wrapper.0.get());
            }

            #[test]
            fn ref_self_rhs_to_self() {
                let mut cnst = CnstTest::default();
                let mut wrapper = $Wrapper(cnst);

                cnst = cnst.$func(cnst.get());
                wrapper = wrapper $op &wrapper;

                assert_eq!(cnst.get(), wrapper.0.get());
            }

            #[test]
            fn ref_self_rhs_to_ref_self() {
                let mut cnst = CnstTest::default();
                let mut wrapper = $Wrapper(cnst);

                cnst = cnst.$func(cnst.get());
                wrapper = &wrapper $op &wrapper;

                assert_eq!(cnst.get(), wrapper.0.get());
            }
        }
    )+};
}

#[cfg(test)]
macro_rules! aritmetic_wrapper_op_assign_tests_for {
    (( $Wrapper:ident ), $({ $md:ident, op:tt, $func:ident }),+ $(,)?) => {$(
        #[cfg(test)]
        mod $md {
            use super::*;

            #[test]
            fn ref_self_to_self() {
                let mut cnst = CnstTest::default();
                let mut wrapper = $Wrapper(cnst);
                let rhs = wrapper;

                cnst = cnst.$func(cnst.get());
                wrapper $op &rhs;

                assert_eq!(cnst.get(), wrapper.0.get());
            }

            #[test]
            fn ref_prim_to_self() {
                let mut cnst = CnstTest::default();
                let mut wrapper = $Wrapper(cnst);
                let rhs = wrapper;

                cnst = cnst.$func(cnst.get());
                wrapper $op &rhs.0.get();

                assert_eq!(cnst.get(), wrapper.0.get());
            }
        }
    )+};
}
