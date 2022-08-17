#![feature(mixed_integer_ops)]

#[macro_use]
mod bench_sub_unsigned;

#[cfg(cnst8bitonly)]
bench_overflowing_sub_unsigned_for! {
    { i8, u8, i8, ConstrainedI8 },
}

#[cfg(not(cnst8bitonly))]
bench_overflowing_sub_unsigned_for! {
    { i8, u8, i8, ConstrainedI8 },
    { i16, u16, i16, ConstrainedI16 },
    { i32, u32, i32, ConstrainedI32 },
    { i64, u64, i64, ConstrainedI64 },
    { i128, u128, i128, ConstrainedI128 },
    { isize, usize, isize, ConstrainedIsize }
}
