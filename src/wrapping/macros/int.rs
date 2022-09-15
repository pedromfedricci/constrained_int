// API implementation and doc values specific to signed integers.
macro_rules! wrapping_int_impl {
    ($SigInt:ty, $md:ident, $Cnst:ident, $min:literal..=$max:literal) => {
        impl<const MIN: $SigInt, const MAX: $SigInt, const DEF: $SigInt>
            Wrapping<$Cnst<MIN, MAX, DEF>>
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
            /// use constrained_int::wrapping::Wrapping;
            ///
            #[doc = concat!("type Constrained = ", stringify!($Cnst), "<", stringify!($min, $max), ">;")]
            ///
            /// let mut wrapped = Wrapping(Constrained::new_min());
            /// assert_eq!(wrapped.signum(), -1);
            ///
            /// wrapped = Wrapping(Constrained::new_max());
            /// assert_eq!(wrapped.signum(), 1);
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
            /// use constrained_int::wrapping::Wrapping;
            ///
            #[doc = concat!("type Constrained = ", stringify!($Cnst), "<", stringify!($min, $max), ">;")]
            ///
            /// let mut wrapped = Wrapping(Constrained::new_min());
            /// assert!(wrapped.is_negative());
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
            /// use constrained_int::wrapping::Wrapping;
            ///
            #[doc = concat!("type Constrained = ", stringify!($Cnst), "<", stringify!($min, $max), ">;")]
            ///
            /// let mut wrapped = Wrapping(Constrained::new_max());
            /// assert!(wrapped.is_positive());
            /// ```
            #[inline]
            pub const fn is_positive(self) -> bool {
                self.0.is_positive()
            }
        }
    };
}

// Defines Wrapping impls, tests and default doc values for signed integers.
macro_rules! wrapping_int_impl_for {
    ($({ $SigInt:ty, $md:ident, $Cnst:ident }),+ $(,)?) => {$(
        mod $md {
            use ::core::ops::{Add, AddAssign, Sub, SubAssign};
            use $crate::$md::$Cnst;
            use $crate::wrapping::Wrapping;

            wrapping_ops_impl! { $SigInt, $Cnst }
            wrapping_int_impl! { $SigInt, $md, $Cnst, -127..=126 }

            #[cfg(test)]
            mod tests_int_common {
                use super::*;

                wrapping_tests_common! { $SigInt, $Cnst }
            }

            #[cfg(test)]
            mod tests_int_specific {
                use super::*;

                wrapping_tests_int_specific! { $SigInt, $Cnst }
            }
        }
    )+};
}

#[cfg(test)]
macro_rules! wrapping_tests_int_specific {
    ($SigInt:ty, $Cnst:ident) => {
        #[test]
        fn signum() {
            let mut wrapping: Wrapping<$Cnst<-1, 1, 0>>;

            wrapping = Wrapping::default();
            assert_eq!(wrapping.signum(), 0);

            wrapping = Wrapping($Cnst::new_min());
            assert_eq!(wrapping.signum(), -1);

            wrapping = Wrapping($Cnst::new_max());
            assert_eq!(wrapping.signum(), 1);
        }

        #[test]
        fn is_positive() {
            let mut wrapping: Wrapping<$Cnst<-1, 1, 0>>;

            wrapping = Wrapping::default();
            assert!(!wrapping.is_positive());

            wrapping = Wrapping($Cnst::new_min());
            assert!(!wrapping.is_positive());

            wrapping = Wrapping($Cnst::new_max());
            assert!(wrapping.is_positive());
        }

        #[test]
        fn is_negative() {
            let mut wrapping: Wrapping<$Cnst<-1, 1, 0>>;

            wrapping = Wrapping::default();
            assert!(!wrapping.is_negative());

            wrapping = Wrapping($Cnst::new_min());
            assert!(wrapping.is_negative());

            wrapping = Wrapping($Cnst::new_max());
            assert!(!wrapping.is_negative());
        }
    };
}
