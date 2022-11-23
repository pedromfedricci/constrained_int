macro_rules! saturating_common {
    ($Int:ty, $Cnst:ident, $test_mod:ident) => {
        arithmetic_wrapper_common! {
            { $Int, $Cnst, Saturating, $test_mod },
            { Add(add), AddAssign(add_assign) => saturating_add },
            { Sub(sub), SubAssign(sub_assign) => saturating_sub },
        }
    };
}

macro_rules! saturating_int {
    ($({ $SigInt:ty, $md:ident, $Cnst:ident }),+ $(,)?) => {$(
        mod $md {
            use $crate::$md::$Cnst;
            use super::Saturating;

            saturating_common! {
                $SigInt, $Cnst, tests_int_common
            }

            arithmetic_wrapper_int_specific! {
                $SigInt, $md, $Cnst, Saturating
            }
        }
    )+};
}

macro_rules! saturating_uint {
    ($({ $UnsInt:ty, $md:ident, $Cnst:ident }),+ $(,)?) => {$(
        mod $md {
            use $crate::$md::$Cnst;
            use super::Saturating;

            saturating_common! {
                $UnsInt, $Cnst, test_uint_common
            }
        }
    )+};
}
