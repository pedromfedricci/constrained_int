#![feature(mixed_integer_ops)]

use benches::bench_overflowing_add_unsigned;

bench_overflowing_add_unsigned! {
    { i8, u8, i8, ConstrainedI8 },
    { i16, u16, i16, ConstrainedI16 },
    { i32, u32, i32, ConstrainedI32 },
    { i64, u64, i64, ConstrainedI64 },
    { i128, u128, i128, ConstrainedI128 },
    { isize, usize, isize, ConstrainedIsize }
}
