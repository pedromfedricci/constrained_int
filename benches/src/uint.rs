/// Generates `wrapping_add` benches for unsigned integers.
///
/// # Example
///
/// ```
/// benches::bench_uint_wrapping_add! {
///     { u8, u8, ConstrainedU8 },
/// }
/// ```
#[macro_export]
macro_rules! bench_uint_wrapping_add {
    ($({ $UnsInt:ident, $uint_mod:ident, $Cnst:ident }),+ $(,)*) => {
        $crate::bench_add_unsigned! {
            $({ $UnsInt, $UnsInt, $uint_mod, $Cnst, wrapping_add }),+
        }
    };
}

/// Generates `overflowing_add` benches for unsigned integers.
///
/// # Example
///
/// ```
/// benches::bench_uint_overflowing_add! {
///     { u8, u8, ConstrainedU8 },
/// }
/// ```
#[macro_export]
macro_rules! bench_uint_overflowing_add {
    ($({ $UnsInt:ident, $uint_mod:ident, $Cnst:ident }),+ $(,)*) => {
        $crate::bench_add_unsigned! {
            $({ $UnsInt, $UnsInt, $uint_mod, $Cnst, overflowing_add }),+
        }
    };
}

/// Generates `wrapping_sub` benches for unsigned intergers.
///
/// # Example
///
/// ```
/// benches::bench_uint_wrapping_sub! {
///     { u8, u8, ConstrainedU8 },
/// }
/// ```
#[macro_export]
macro_rules! bench_uint_wrapping_sub {
    ($({ $UnsInt:ident, $uint_mod:ident, $Cnst:ident }),+ $(,)*) => {
        $crate::bench_sub_unsigned! {
            $({ $UnsInt, $UnsInt, $uint_mod, $Cnst, wrapping_sub }),+
        }
    };
}

/// Generates `wrapping_sub` benches for unsigned intergers.
///
/// # Example
///
/// ```
/// benches::bench_uint_overflowing_sub! {
///     { u8, u8, ConstrainedU8 },
/// }
/// ```
#[macro_export]
macro_rules! bench_uint_overflowing_sub {
    ($({ $UnsInt:ident, $uint_mod:ident, $Cnst:ident }),+ $(,)*) => {
        $crate::bench_sub_unsigned! {
            $({ $UnsInt, $UnsInt, $uint_mod, $Cnst, overflowing_sub }),+
        }
    };
}

/// Generates `wrapping_add_signed` benches for unsigned integers.
///
/// # Example
///
/// ```
/// #![feature(mixed_integer_ops)]
///
/// benches::bench_wrapping_add_signed! {
///     { u8, i8, u8, ConstrainedU8 },
/// }
/// ```
#[macro_export]
macro_rules! bench_wrapping_add_signed {
    ($({ $UnsInt:ident, $SigInt:ident, $uint_mod:ident, $Cnst:ident }),+ $(,)*) => {
        $crate::bench_add_signed! {
            $({ $UnsInt, $SigInt, $uint_mod, $Cnst, wrapping_add_signed }),+
        }
    };
}

/// Generates `overflowing_add_signed` benches for unsigned integers.
///
/// # Example
///
/// ```
/// #![feature(mixed_integer_ops)]
///
/// benches::bench_overflowing_add_signed! {
///     { u8, i8, u8, ConstrainedU8 },
/// }
/// ```
#[macro_export]
macro_rules! bench_overflowing_add_signed {
    ($({ $UnsInt:ident, $SigInt:ident, $uint_mod:ident, $Cnst:ident }),+ $(,)*) => {
        $crate::bench_add_signed! {
            $({ $UnsInt, $SigInt, $uint_mod, $Cnst, overflowing_add_signed }),+
        }
    };
}
