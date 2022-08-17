#![feature(mixed_integer_ops)]

use benches::bench_wrapping_add_signed;

bench_wrapping_add_signed! {
    { u8, i8, u8, ConstrainedU8 },
    { u16, i16, u16, ConstrainedU16 },
    { u32, i32, u32, ConstrainedU32 },
    { u64, i64, u64, ConstrainedU64 },
    { u128, i128, u128, ConstrainedU128 },
    { usize, isize, usize, ConstrainedUsize }
}
