#[macro_use]
mod bench_add;

#[cfg(cnst8bitonly)]
bench_wrapping_add_for! {
    { u8, u8, ConstrainedU8 },
}

#[cfg(not(cnst8bitonly))]
bench_wrapping_add_for! {
    { u8, u8, ConstrainedU8 },
    { u16, u16, ConstrainedU16 },
    { u32, u32, ConstrainedU32 },
    { u64, u64, ConstrainedU64 },
    { u128, u128, ConstrainedU128 },
    { usize, usize, ConstrainedUsize }
}
