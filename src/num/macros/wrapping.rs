// Implements common APIs and tests for the `Wrapping` type.
macro_rules! wrapping_common {
    ($Int:ty, $Cnst:ident, $test_mod:ident) => {
        arithmetic_wrapper_common! {
            { $Int, $Cnst, Wrapping, $test_mod },
            { Add(add), AddAssign(add_assign) => wrapping_add },
            { Sub(sub), SubAssign(sub_assign) => wrapping_sub },
        }

        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> core::iter::Sum
            for Wrapping<$Cnst<MIN, MAX, DEF>>
        {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Wrapping($Cnst::zero()), |acc, elem| acc + elem)
            }
        }

        impl<'a, const MIN: $Int, const MAX: $Int, const DEF: $Int>
            core::iter::Sum<&'a Wrapping<$Cnst<MIN, MAX, DEF>>> for Wrapping<$Cnst<MIN, MAX, DEF>>
        {
            fn sum<I: Iterator<Item = &'a Wrapping<$Cnst<MIN, MAX, DEF>>>>(iter: I) -> Self {
                iter.fold(Wrapping($Cnst::zero()), |acc, elem| acc + *elem)
            }
        }

        #[cfg(test)]
        mod test_common {
            use super::$Cnst;
            use super::Wrapping;

            type WrappingSumTest = Wrapping<$Cnst<0, 1>>;
            const WRAPPING_SUM_TEST: [WrappingSumTest; 2] =
                [Wrapping($Cnst::new_max()), Wrapping($Cnst::new_max())];

            #[test]
            fn sum_impl_for_wrapping() {
                let sum: WrappingSumTest = WRAPPING_SUM_TEST.into_iter().sum();
                assert_eq!(sum.0.get(), 0);
            }

            #[test]
            fn sum_impl_for_wrapping_ref() {
                let sum: WrappingSumTest = WRAPPING_SUM_TEST.iter().sum();
                assert_eq!(sum.0.get(), 0);
            }
        }
    };
}

// Implements common and signed specific APIs and tests for the `Wrapping` type.
macro_rules! wrapping_int {
    ($({ $SigInt:ty, $md:ident, $Cnst:ident }),+ $(,)?) => {$(
        mod $md {
            use $crate::$md::$Cnst;
            use super::Wrapping;

            wrapping_common! {
                $SigInt, $Cnst, tests_int_common
            }

            arithmetic_wrapper_int_specific! {
                $SigInt, $md, $Cnst, Wrapping
            }
        }
    )+};
}

// Implements common and unsigned specific APIs and tests for the `Wrapping` type.
macro_rules! wrapping_uint {
    ($({ $UnsInt:ty, $md:ident, $Cnst:ident }),+ $(,)?) => {$(
        mod $md {
            use $crate::$md::$Cnst;
            use super::Wrapping;

            wrapping_common! {
                $UnsInt, $Cnst, test_uint_common
            }
        }
    )+};
}
