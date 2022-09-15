// Import:
// - `strategies_uint_def_impl!`.
// - `strategies_int_def_impl!`.
#[cfg(test)]
#[macro_use]
mod macros;

strategies_uint_def_impl! {
    { u8, u8, ConstrainedU8 },
    { u16, u16, ConstrainedU16 },
    { u32, u32, ConstrainedU32 },
    { u64, u64, ConstrainedU64 },
    { u128, u128, ConstrainedU128 },
    { usize, usize, ConstrainedUsize },
}

strategies_int_def_impl! {
    { i8, u8, i8, ConstrainedI8 },
    { i16, u16, i16, ConstrainedI16 },
    { i32, u32, i32, ConstrainedI32 },
    { i64, u64, i64, ConstrainedI64 },
    { i128, u128, i128, ConstrainedI128 },
    { isize, usize, isize, ConstrainedIsize },
}
