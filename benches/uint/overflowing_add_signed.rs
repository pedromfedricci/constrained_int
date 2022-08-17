#![feature(mixed_integer_ops)]

#[macro_use]
mod bench_add_signed;

#[cfg(cnst8bitonly)]
bench_overflowing_add_signed_for! {
    { u8, i8, u8, ConstrainedU8 },
}

#[cfg(not(cnst8bitonly))]
bench_overflowing_add_signed_for! {
    { u8, i8, u8, ConstrainedU8 },
    { u16, i16, u16, ConstrainedU16 },
    { u32, i32, u32, ConstrainedU32 },
    { u64, i64, u64, ConstrainedU64 },
    { u128, i128, u128, ConstrainedU128 },
    { usize, isize, usize, ConstrainedUsize }
}
