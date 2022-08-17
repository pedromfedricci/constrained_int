#![allow(unused_macros)]

// For large integers, it's impractical to use the actual MAX value
// as part of the benchmark id, so just stringify it like `prim`::MAX.
macro_rules! max {
    ($Num:ident) => {
        concat!(stringify!($Num), "::MAX")
    };
}

// For large integers, it's impractical to use the actual MIN value
// as part of the benchmark id, so just stringify it like `prim`::MIN.
macro_rules! min {
    ($Num:ident) => {
        concat!(stringify!($Num), "::MIN")
    };
}

// Name a benchmark, without a parameter.
macro_rules! name {
    ($Cnst:ident, $size:literal) => {
        concat!($size, " ", stringify!($Cnst))
    };
}

// Name a benchmark with a Constrained type and a `Short` suffix.
macro_rules! short {
    ($Cnst:ident) => {
        name!($Cnst, "Short")
    };
}

// Name a benchmark with a Constrained type and a `Large` suffix.
macro_rules! large {
    ($Cnst:ident) => {
        name!($Cnst, "Large")
    };
}

// Name a group for a wrapping API with a unique combination of
// integer, function name and detail.
macro_rules! group {
    ($num:expr, $func:expr, $detail:expr) => {
        concat!($num, ": ", $func, ": ", $detail)
    };
}

// Name a group for a wrapping API that does overflow the inner integer.
macro_rules! overflowed {
    ($Num:ty, $func:expr) => {
        group!(stringify!($Num), $func, "arithmetic overflow")
    };
}
