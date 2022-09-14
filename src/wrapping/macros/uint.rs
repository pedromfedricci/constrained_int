// Defines Wrapping impls, tests and default doc values for unsigned integers.
macro_rules! wrapping_uint_impl_for {
    ($({ $UnsInt:ty, $md:ident, $Cnst:ident }),+ $(,)?) => {$(
        mod $md {
            use ::core::ops::{Add, AddAssign, Sub, SubAssign};
            use $crate::$md::$Cnst;
            use $crate::wrapping::Wrapping;

            wrapping_ops_impl! { $UnsInt, $Cnst }

            #[cfg(test)]
            mod tests_uint_common {
                use super::*;

                wrapping_tests_common! { $UnsInt, $Cnst }
            }
        }
    )+};
}
