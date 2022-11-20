// Implementation and documentation values specific to signed integers.
macro_rules! arithmetic_wrapper_int_impl {
    ($SigInt:ty, $md:ident, $Cnst:ident, $Wrapper:ident, $min:literal..=$max:literal) => {
        impl<const MIN: $SigInt, const MAX: $SigInt, const DEF: $SigInt>
            $Wrapper<$Cnst<MIN, MAX, DEF>>
        {
            /// Returns a number representing sign of `self`.
            ///
            ///  - `0` if the number is zero
            ///  - `1` if the number is positive
            ///  - `-1` if the number is negative
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Cnst), ";")]
            #[doc = concat!("use constrained_int::", stringify!($Wrapper), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Cnst), "<", stringify!($min, $max), ">;")]
            ///
            #[doc = concat!("let mut value = ", stringify!($Wrapper), "(Constrained::new_min());")]
            /// assert_eq!(value.signum(), -1);
            ///
            #[doc = concat!("value = ", stringify!($Wrapper), "(Constrained::new_max());")]
            /// assert_eq!(value.signum(), 1);
            /// ```
            #[must_use = "this returns the result of the operation, without modifyind the original"]
            #[inline]
            pub const fn signum(self) -> $SigInt {
                self.0.signum()
            }

            /// Returns `true` if `self` is negative and `false` if the number is zero or
            /// positive.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Cnst), ";")]
            #[doc = concat!("use constrained_int::", stringify!($Wrapper), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Cnst), "<", stringify!($min, $max), ">;")]
            ///
            #[doc = concat!("let value = ", stringify!($Wrapper), "(Constrained::new_min());")]
            /// assert!(value.is_negative());
            /// ```
            #[inline]
            pub const fn is_negative(self) -> bool {
                self.0.is_negative()
            }

            /// Returns `true` if `self` is positive and `false` if the number is zero or
            /// negative.
            ///
            /// # Example
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Cnst), ";")]
            #[doc = concat!("use constrained_int::", stringify!($Wrapper), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Cnst), "<", stringify!($min, $max), ">;")]
            ///
            #[doc = concat!("let mut value = ", stringify!($Wrapper), "(Constrained::new_max());")]
            /// assert!(value.is_positive());
            /// ```
            #[inline]
            pub const fn is_positive(self) -> bool {
                self.0.is_positive()
            }
        }
    };
}

// Defines a arithmetic wrapper impls, tests and default doc values for signed integers.
macro_rules! arithmetic_wrapper_int_impl_for {
    (( $Wrapper:ident ), $({ $SigInt:ty, $md:ident, $Cnst:ident }),+ $(,)?) => {$(
        mod $md {
            use ::core::ops::{Add, AddAssign, Sub, SubAssign};
            use $crate::$md::$Cnst;
            use super::$Wrapper;

            arithmetic_wrapper_ops_impl! {
            //  ( sint, ConstrainedType, ArithmeticWrapperType ),
                ( $SigInt, $Cnst, $Wrapper ),
                {
                    wrapping_add,
                    wrapping_sub
                }
            }

            arithmetic_wrapper_int_impl! {
            //  sint, sint_mod, ConstrainedType, ArithmeticWrapperType, min..=max
                $SigInt, $md, $Cnst, $Wrapper, -127..=126
            }

            #[cfg(test)]
            mod tests_int_common {
                use super::*;

                arithmetic_wrapper_tests_common! {
                //  ( sint, ConstrainedType, ArithmeticWrapperType ),
                    ( $SigInt, $Cnst, $Wrapper ),
                    {
                        wrapping_add,
                        wrapping_sub
                    }
                }
            }

            #[cfg(test)]
            mod tests_int_specific {
                use super::*;

                arithmetic_wrapper_tests_int_specific! {
                //  sint, ConstrainedType, ArithmeticWrapperType
                    $SigInt, $Cnst, $Wrapper
                }
            }
        }
    )+};
}

#[cfg(test)]
macro_rules! arithmetic_wrapper_tests_int_specific {
    ($SigInt:ty, $Cnst:ident, $Wrapper:ident) => {
        #[test]
        fn signum() {
            let mut wrapper: $Wrapper<$Cnst<-1, 1, 0>>;

            wrapper = $Wrapper::default();
            assert_eq!(wrapper.signum(), 0);

            wrapper = $Wrapper($Cnst::new_min());
            assert_eq!(wrapper.signum(), -1);

            wrapper = $Wrapper($Cnst::new_max());
            assert_eq!(wrapper.signum(), 1);
        }

        #[test]
        fn is_positive() {
            let mut wrapper: $Wrapper<$Cnst<-1, 1, 0>>;

            wrapper = $Wrapper::default();
            assert!(!wrapper.is_positive());

            wrapper = $Wrapper($Cnst::new_min());
            assert!(!wrapper.is_positive());

            wrapper = $Wrapper($Cnst::new_max());
            assert!(wrapper.is_positive());
        }

        #[test]
        fn is_negative() {
            let mut wrapper: $Wrapper<$Cnst<-1, 1, 0>>;

            wrapper = $Wrapper::default();
            assert!(!wrapper.is_negative());

            wrapper = $Wrapper($Cnst::new_min());
            assert!(wrapper.is_negative());

            wrapper = $Wrapper($Cnst::new_max());
            assert!(!wrapper.is_negative());
        }
    };
}
