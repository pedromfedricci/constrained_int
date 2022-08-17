// For large integers, it's impractical to use the actual MAX value
// as part of the benchmark id, so just stringify it like `prim`::MAX.
#[macro_export]
macro_rules! max {
    ($Num:ident) => {
        concat!(stringify!($Num), "::MAX")
    };
}

// For large integers, it's impractical to use the actual MIN value
// as part of the benchmark id, so just stringify it like `prim`::MIN.
#[macro_export]
macro_rules! min {
    ($Num:ident) => {
        concat!(stringify!($Num), "::MIN")
    };
}

// Name a benchmark, without a parameter.
#[macro_export]
macro_rules! name {
    ($Cnst:ident, $size:literal) => {
        concat!($size, " ", stringify!($Cnst))
    };
}

// Name a benchmark with a Constrained type and a `Short` suffix.
#[macro_export]
macro_rules! short {
    ($Cnst:ident) => {
        $crate::name!($Cnst, "Short")
    };
}

// Name a benchmark with a Constrained type and a `Large` suffix.
#[macro_export]
macro_rules! large {
    ($Cnst:ident) => {
        $crate::name!($Cnst, "Large")
    };
}

// Name a group for a wrapping API with a unique combination of
// integer, function name and detail.
#[macro_export]
macro_rules! group {
    ($num:expr, $func:expr, $detail:expr) => {
        concat!($num, ": ", $func, ": ", $detail)
    };
}

// Name a group for a wrapping API that does overflow the inner integer.
#[macro_export]
macro_rules! overflowed {
    ($Num:ty, $func:ident) => {
        $crate::group!(stringify!($Num), stringify!($func), "arithmetic overflow")
    };
}
