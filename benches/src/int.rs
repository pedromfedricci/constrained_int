/// Generates `wrapping_add` benches for signed integers.
///
/// # Example
///
/// ```
/// benches::bench_int_wrapping_add! {
///     { i8, i8, ConstrainedI8 },
/// }
/// ```
#[macro_export]
macro_rules! bench_int_wrapping_add {
    ($({ $SigInt:ident, $int_mod:ident, $Cnst:ident }),+ $(,)*) => {
        $crate::bench_add_signed! {
            $({ $SigInt, $SigInt, $int_mod, $Cnst, wrapping_add }),+
        }
    };
}

/// Generates `overflowing_add` benches for signed integers.
///
/// # Example
///
/// ```
/// benches::bench_int_overflowing_add! {
///     { i8, i8, ConstrainedI8 },
/// }
/// ```
#[macro_export]
macro_rules! bench_int_overflowing_add {
    ($({ $SigInt:ident, $int_mod:ident, $Cnst:ident }),+ $(,)*) => {
        $crate::bench_add_signed! {
            $({ $SigInt, $SigInt, $int_mod, $Cnst, overflowing_add }),+
        }
    };
}

/// Generates `wrapping_sub` benches for signed integers.
///
/// # Example
///
/// ```
/// benches::bench_int_wrapping_sub! {
///     { i8, i8, ConstrainedI8 },
/// }
/// ```
#[macro_export]
macro_rules! bench_int_wrapping_sub {
    ($({ $SigInt:ident, $int_mod:ident, $Cnst:ident }),+ $(,)*) => {
        $crate::bench_sub_signed! {
            $({ $SigInt, $SigInt, $int_mod, $Cnst, wrapping_sub }),+
        }
    };
}

/// Generates `overflowing_sub` benches for signed integers.
///
/// # Example
///
/// ```
/// benches::bench_int_overflowing_sub! {
///     { i8, i8, ConstrainedI8 },
/// }
/// ```
#[macro_export]
macro_rules! bench_int_overflowing_sub {
    ($({ $SigInt:ident, $int_mod:ident, $Cnst:ident }),+ $(,)*) => {
        $crate::bench_sub_signed! {
            $({ $SigInt, $SigInt, $int_mod, $Cnst, overflowing_sub }),+
        }
    };
}

/// Generates `wrapping_add_unsigned` benches for signed integers.
///
/// # Example
///
/// ```
/// #![feature(mixed_integer_ops)]
///
/// benches::bench_wrapping_add_unsigned! {
///     { i8, u8, i8, ConstrainedI8 },
/// }
/// ```
#[macro_export]
macro_rules! bench_wrapping_add_unsigned {
    ($({ $SigInt:ident, $UnsInt:ident, $int_mod:ident, $Cnst:ident }),+ $(,)*) => {
        $crate::bench_add_unsigned! {
            $({ $SigInt, $UnsInt, $int_mod, $Cnst, wrapping_add_unsigned }),+
        }
    };
}

/// Generates `overflowing_add_unsigned` benches for signed integers.
///
/// # Example
///
/// ```
/// #![feature(mixed_integer_ops)]
///
/// benches::bench_overflowing_add_unsigned! {
///     { i8, u8, i8, ConstrainedI8 },
/// }
/// ```
#[macro_export]
macro_rules! bench_overflowing_add_unsigned {
    ($({ $SigInt:ident, $UnsInt:ident, $int_mod:ident, $Cnst:ident }),+ $(,)*) => {
        $crate::bench_add_unsigned! {
            $({ $SigInt, $UnsInt, $int_mod, $Cnst, overflowing_add_unsigned }),+
        }
    };
}

/// Generates `wrapping_sub_unsigned` benches for signed integers.
///
/// # Example
///
/// ```
/// #![feature(mixed_integer_ops)]
///
/// benches::bench_wrapping_sub_unsigned! {
///     { i8, u8, i8, ConstrainedI8 },
/// }
/// ```
#[macro_export]
macro_rules! bench_wrapping_sub_unsigned {
    ($({ $SigInt:ident, $UnsInt:ident, $int_mod:ident, $Cnst:ident }),+ $(,)*) => {
        $crate::bench_sub_unsigned! {
            $({ $SigInt, $UnsInt, $int_mod, $Cnst, wrapping_sub_unsigned }),+
        }
    };
}

/// Generates `overflowing_sub_unsigned` benches for signed integers.
///
/// # Example
///
/// ```
/// #![feature(mixed_integer_ops)]
///
/// benches::bench_overflowing_sub_unsigned! {
///     { i8, u8, i8, ConstrainedI8 },
/// }
/// ```
#[macro_export]
macro_rules! bench_overflowing_sub_unsigned {
    ($({ $SigInt:ident, $UnsInt:ident, $int_mod:ident, $Cnst:ident }),+ $(,)*) => {
        $crate::bench_sub_unsigned! {
            $({ $SigInt, $UnsInt, $int_mod, $Cnst, overflowing_sub_unsigned }),+
        }
    };
}
