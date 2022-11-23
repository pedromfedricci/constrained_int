// Implements serde::{Deserialize, Serialize} for $Wrapper.
#[cfg(feature = "serde")]
macro_rules! arithmetic_wrapper_serde_impl {
    ($Wrapper:ident) => {
        #[cfg(feature = "serde")]
        #[doc(cfg(feature = "serde"))]
        impl<T: ::serde::Serialize> ::serde::Serialize for $Wrapper<T> {
            #[inline]
            fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                self.0.serialize(serializer)
            }
        }

        #[cfg(feature = "serde")]
        #[doc(cfg(feature = "serde"))]
        impl<'de, T: ::serde::Deserialize<'de>> ::serde::Deserialize<'de> for $Wrapper<T> {
            fn deserialize<D: ::serde::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Self, D::Error> {
                ::serde::Deserialize::deserialize(deserializer).map($Wrapper)
            }
        }
    };
}

// Implemets some core::fmt traits for $Wrapper.
macro_rules! arithmetic_wrapper_fmt_impl {
    ($($Trait:ident),+ for $Wrapper:ident) => {$(
        impl<T: ::core::fmt::$Trait> ::core::fmt::$Trait for $Wrapper<T> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                self.0.fmt(f)
            }
        }
    )+};

    ($Wrapper:ident) => {
        arithmetic_wrapper_fmt_impl! {
            Debug, Display, Binary, Octal, LowerHex, UpperHex for $Wrapper
        }
    };
}

// Implements common traits for $Wrapper when the generic T implements them.
macro_rules! arithmetic_wrapper_traits_impl {
    ($Wrapper:ident) => {
        #[cfg(feature = "serde")]
        arithmetic_wrapper_serde_impl! { $Wrapper }

        arithmetic_wrapper_fmt_impl! { $Wrapper }
    };
}

// Implements core::ops traits for $Wrapper.
macro_rules! arithmetic_wrapper_ops_impl {
    (  { $Int:ty, $Cnst:ident, $Wrapper:ident },
     $({ $Bop:ident($bop_f:ident), $Aop:ident($aop_f:ident) => $inner_f:ident }),+ $(,)?
    ) => {$(
        use ::core::ops::{$Bop, $Aop};

        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const
            $Bop for $Wrapper<$Cnst<MIN, MAX, DEF>>
        {
            type Output = Self;

            #[inline]
            fn $bop_f(self, rhs: Self) -> Self {
                $Wrapper(self.0.$inner_f(rhs.0.get()))
            }
        }

        forward_ref_binop! {
            impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const
                $Bop<$Wrapper<$Cnst<MIN, MAX, DEF>>>, $bop_f for $Wrapper<$Cnst<MIN, MAX, DEF>>
        }

        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const
            $Aop for $Wrapper<$Cnst<MIN, MAX, DEF>>
        {
            #[inline]
            fn $aop_f(&mut self, rhs: Self) {
                *self = <Self as $Bop>::$bop_f(*self, rhs);
            }
        }

        forward_ref_op_assign! {
            impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const
                $Aop<$Wrapper<$Cnst<MIN, MAX, DEF>>>, $aop_f for $Wrapper<$Cnst<MIN, MAX, DEF>>
        }

        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const
            $Aop<$Int> for $Wrapper<$Cnst<MIN, MAX, DEF>>
        {
            #[inline]
            fn $aop_f(&mut self, rhs: $Int) {
                *self = $Wrapper(self.0.$inner_f(rhs));
            }
        }

        forward_ref_op_assign! {
            impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> const
                $Aop<$Int>, $aop_f for $Wrapper<$Cnst<MIN, MAX, DEF>>
        }
    )+};
}

// Implements common APIs and tests for $Wrapper.
macro_rules! arithmetic_wrapper_common {
    (  { $Int:ty, $Cnst:ident, $Wrapper:tt, $test_mod:ident },
     $({ $Bop:ident($bop_f:ident), $Aop:ident($aop_f:ident) => $inner_f:ident }),+ $(,)?
    ) => {
        arithmetic_wrapper_ops_impl! {
            { $Int, $Cnst, $Wrapper },
          $({ $Bop($bop_f), $Aop($aop_f) => $inner_f }),+
        }

        #[cfg(test)]
        arithmetic_wrapper_common_tests! {
            { $Int, $Cnst, $Wrapper, $test_mod },
          $({ $Bop($bop_f), $Aop($aop_f) => $inner_f }),+
        }
    };
}

// Implements tests for ::core::ops implementations of $Wrapper.
#[cfg(test)]
macro_rules! arithmetic_wrapper_ops_tests {
    (  { $Wrapper:ident },
     $({ $Bop:ident($bop_f:ident), $Aop:ident($aop_f:ident) => $inner_f:ident }),+ $(,)?
    ) => {
        arithmetic_wrapper_binop_tests! {
            { $Wrapper },
          $({ $Bop($bop_f) => $inner_f }),+
        }

        aritmetic_wrapper_op_assign_tests! {
            { $Wrapper },
          $({ $Aop($aop_f) => $inner_f }),+
        }
    };
}

// Implements tests that are common to both signed and unsigned.
#[cfg(test)]
macro_rules! arithmetic_wrapper_common_tests {
    (  { $Int:ty, $Cnst:ident, $Wrapper:ident, $test_mod:ident },
     $({ $Bop:ident($bop_f:ident), $Aop:ident($aop_f:ident) => $inner_f:ident }),+ $(,)?
    ) => {
        #[cfg(test)]
        mod $test_mod {
            use super::*;

            type CnstTest = $Cnst<{ <$Int>::MIN + 1 }, { <$Int>::MAX - 1 }>;

            arithmetic_wrapper_common_traits_tests! {
                $Wrapper
            }

            arithmetic_wrapper_ops_tests! {
                { $Wrapper },
              $({ $Bop($bop_f), $Aop($aop_f) => $inner_f }),+
            }
        }
    };
}

// Implements tests for common trait implementations.
#[cfg(test)]
macro_rules! arithmetic_wrapper_common_traits_tests {
    ($Wrapper:ident) => {
        #[cfg(feature = "std")]
        #[test]
        fn fmt() {
            let wrapper = $Wrapper(CnstTest::default());

            // Display
            assert_eq!(format!("{}", wrapper), format!("{}", wrapper.0));
            // Debug
            assert_eq!(format!("{:?}", wrapper), format!("{:?}", wrapper.0));
            // Binary
            assert_eq!(format!("{:b}", wrapper), format!("{:b}", wrapper.0));
            // Octal
            assert_eq!(format!("{:o}", wrapper), format!("{:o}", wrapper.0));
            // LowerHex
            assert_eq!(format!("{:x}", wrapper), format!("{:x}", wrapper.0));
            // UpperHex
            assert_eq!(format!("{:X}", wrapper), format!("{:X}", wrapper.0));
        }
    };
}

// Verify that binary operators work as expected for arithmetic wrappers.
#[cfg(test)]
macro_rules! arithmetic_wrapper_binop_tests {
    (  { $Wrapper:ident },
     $({ $Bop:ident($bop_f:ident) => $inner_f:ident }),+ $(,)?
    ) => {$(
        mod $bop_f {
            use ::core::ops::$Bop;
            use super::*;

            #[test]
            fn copy_lhs_copy_rhs() {
                let mut cnst = CnstTest::default();
                let mut wrapper = $Wrapper(cnst);

                cnst = cnst.$inner_f(cnst.get());
                wrapper = <$Wrapper<_> as $Bop<_>>::$bop_f(wrapper, wrapper);

                assert_eq!(cnst.get(), wrapper.0.get());
            }

            #[test]
            fn copy_lhs_ref_rhs() {
                let mut cnst = CnstTest::default();
                let mut wrapper = $Wrapper(cnst);

                cnst = cnst.$inner_f(cnst.get());
                wrapper = <$Wrapper<_> as $Bop<_>>::$bop_f(wrapper, &wrapper);

                assert_eq!(cnst.get(), wrapper.0.get());
            }

            #[test]
            fn ref_lhs_copy_rhs() {
                let mut cnst = CnstTest::default();
                let mut wrapper = $Wrapper(cnst);

                cnst = cnst.$inner_f(cnst.get());
                wrapper = <&$Wrapper<_> as $Bop<_>>::$bop_f(&wrapper, wrapper);

                assert_eq!(cnst.get(), wrapper.0.get());
            }

            #[test]
            fn ref_lhs_ref_rhs() {
                let mut cnst = CnstTest::default();
                let mut wrapper = $Wrapper(cnst);

                cnst = cnst.$inner_f(cnst.get());
                wrapper = <&$Wrapper<_> as $Bop<_>>::$bop_f(&wrapper, &wrapper);

                assert_eq!(cnst.get(), wrapper.0.get());
            }
        }
    )+};
}

// Verify that self assigning operators work as expected for arithmetic wrappers.
#[cfg(test)]
macro_rules! aritmetic_wrapper_op_assign_tests {
    (  { $Wrapper:ident },
     $({ $Aop:ident($aop_f:ident) => $inner_f:ident }),+ $(,)?
    ) => {$(
        mod $aop_f {
            use ::core::ops::$Aop;
            use super::*;

            #[test]
            fn copy_rhs() {
                let mut cnst = CnstTest::default();
                let mut wrapper = $Wrapper(cnst);
                let rhs = wrapper;

                cnst = cnst.$inner_f(cnst.get());
                <$Wrapper<_> as $Aop<_>>::$aop_f(&mut wrapper, rhs);

                assert_eq!(cnst.get(), wrapper.0.get());
            }

            #[test]
            fn ref_rhs() {
                let mut cnst = CnstTest::default();
                let mut wrapper = $Wrapper(cnst);
                let rhs = wrapper;

                cnst = cnst.$inner_f(cnst.get());
                <$Wrapper<_> as $Aop<_>>::$aop_f(&mut wrapper, &rhs);

                assert_eq!(cnst.get(), wrapper.0.get());
            }

            #[test]
            fn prim_rhs() {
                let mut cnst = CnstTest::default();
                let mut wrapper = $Wrapper(cnst);
                let rhs = wrapper;

                cnst = cnst.$inner_f(cnst.get());
                <$Wrapper<_> as $Aop<_>>::$aop_f(&mut wrapper, rhs.0.get());

                assert_eq!(cnst.get(), wrapper.0.get());
            }

            #[test]
            fn prim_ref_rhs() {
                let mut cnst = CnstTest::default();
                let mut wrapper = $Wrapper(cnst);
                let rhs = wrapper;

                cnst = cnst.$inner_f(cnst.get());
                <$Wrapper<_> as $Aop<_>>::$aop_f(&mut wrapper, &rhs.0.get());

                assert_eq!(cnst.get(), wrapper.0.get());
            }
        }
    )+};
}
