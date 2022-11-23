// Implements common APIs and tests for the `Wrapping` type.
macro_rules! wrapping_common {
    ($Int:ty, $Cnst:ident, $test_mod:ident) => {
        arithmetic_wrapper_common! {
            { $Int, $Cnst, Wrapping, $test_mod },
            { Add(add), AddAssign(add_assign) => wrapping_add },
            { Sub(sub), SubAssign(sub_assign) => wrapping_sub },
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
