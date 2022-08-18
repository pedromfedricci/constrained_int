use benches::bench_uint_wrapping_add;

bench_uint_wrapping_add! {
    { u8, u8, ConstrainedU8 },
    { u16, u16, ConstrainedU16 },
    { u32, u32, ConstrainedU32 },
    { u64, u64, ConstrainedU64 },
    { u128, u128, ConstrainedU128 },
    { usize, usize, ConstrainedUsize }
}
