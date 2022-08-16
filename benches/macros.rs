// For large integers, it's impractical to use the actual MAX value
// as part of the benchmark id, so just stringify it like `prim`::MAX.
macro_rules! max {
    ($Num:ident) => {
        concat!(stringify!($Num), "::MAX")
    };
}

// For large integers, it's impractical to use the actual MIN value
// as part of the benchmark id, so just stringify it like `prim`::MIN.
#[allow(unused_macros)]
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

// Name a benchmark with a unique combination within group.
macro_rules! short {
    ($Cnst:ident) => {
        name!($Cnst, "Short")
    };
}

// Name a benchmark with a unique combination. within group.
macro_rules! large {
    ($Cnst:ident) => {
        name!($Cnst, "Large")
    };
}

// Group for wrapping_add with specific setup.
macro_rules! group {
    ($bits:literal, $func:literal, $detail:expr) => {
        concat!($bits, ": ", $func, ": ", $detail)
    };
}

// Group for wrapping_add that does overflow the primitive integer.
macro_rules! overflowed {
    ($Num:ident, $bits:literal, $func:literal) => {
        group!($bits, $func, concat!(stringify!($Num), " overflow"))
    };
}
