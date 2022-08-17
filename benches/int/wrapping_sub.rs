#[macro_use]
mod bench_sub;

#[cfg(cnst8bitonly)]
bench_wrapping_sub_for! {
    { i8, i8, ConstrainedI8 },
}

#[cfg(not(cnst8bitonly))]
bench_wrapping_sub_for! {
    { i8, i8, ConstrainedI8 },
    { i16, i16, ConstrainedI16 },
    { i32, i32, ConstrainedI32 },
    { i64, i64, ConstrainedI64 },
    { i128, i128, ConstrainedI128 },
    { isize, isize, ConstrainedIsize }
}
