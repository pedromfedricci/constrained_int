// Import all macros.
#[macro_use]
mod macros;

#[cfg(any(cnst8bitonly, not(cnst8bitonly)))]
constrained_deserialize_impl! {
    u8, u8, ConstrainedU8, deserialize_u8,
    num_as_self_uint!(u8, u8:visit_u8);
    uint_to_self!(u8, u16:visit_u16 u32:visit_u32 u64:visit_u64);
    int_to_uint!(u8, i8:visit_i8 i16:visit_i16 i32:visit_i32 i64:visit_i64);
}

#[cfg(not(cnst8bitonly))]
constrained_deserialize_impl! {
    u16, u16, ConstrainedU16, deserialize_u16,
    num_as_self_uint!(u16, u8:visit_u8 u16:visit_u16);
    uint_to_self!(u16, u32:visit_u32 u64:visit_u64);
    int_to_uint!(u16, i8:visit_i8 i16:visit_i16 i32:visit_i32 i64:visit_i64);
}

#[cfg(not(cnst8bitonly))]
constrained_deserialize_impl! {
    u32, u32, ConstrainedU32, deserialize_u32,
    num_as_self_uint!(u32, u8:visit_u8 u16:visit_u16 u32:visit_u32);
    uint_to_self!(u32, u64:visit_u64);
    int_to_uint!(u32, i8:visit_i8 i16:visit_i16 i32:visit_i32 i64:visit_i64);
}

#[cfg(not(cnst8bitonly))]
constrained_deserialize_impl! {
    u64, u64, ConstrainedU64, deserialize_u64,
    num_as_self_uint!(u64, u8:visit_u8 u16:visit_u16 u32:visit_u32 u64:visit_u64);
    int_to_uint!(u64, i8:visit_i8 i16:visit_i16 i32:visit_i32 i64:visit_i64);
}

#[cfg(not(cnst8bitonly))]
constrained_deserialize_impl! {
    usize, usize, ConstrainedUsize, deserialize_u64,
    num_as_self_uint!(usize, u8:visit_u8 u16:visit_u16);
    uint_to_self!(usize, u32:visit_u32 u64:visit_u64);
    int_to_uint!(usize, i8:visit_i8 i16:visit_i16 i32:visit_i32 i64:visit_i64);
}

#[cfg(any(cnst8bitonly, not(cnst8bitonly)))]
constrained_deserialize_impl! {
    i8, i8, ConstrainedI8, deserialize_i8,
    num_as_self_int!(i8, i8:visit_i8);
    int_to_int!(i8, i16:visit_i16 i32:visit_i32 i64:visit_i64);
    uint_to_self!(i8, u8:visit_u8 u16:visit_u16 u32:visit_u32 u64:visit_u64);
}

#[cfg(not(cnst8bitonly))]
constrained_deserialize_impl! {
    i16, i16, ConstrainedI16, deserialize_i16,
    num_as_self_int!(i16, i8:visit_i8  i16:visit_i16);
    int_to_int!(i16, i32:visit_i32 i64:visit_i64);
    uint_to_self!(i16, u8:visit_u8 u16:visit_u16 u32:visit_u32 u64:visit_u64);
}

#[cfg(not(cnst8bitonly))]
constrained_deserialize_impl! {
    i32, i32, ConstrainedI32, deserialize_i32,
    num_as_self_int!(i32, i8:visit_i8  i16:visit_i16 i32:visit_i32);
    int_to_int!(i32, i64:visit_i64);
    uint_to_self!(i32, u8:visit_u8 u16:visit_u16 u32:visit_u32 u64:visit_u64);
}

#[cfg(not(cnst8bitonly))]
constrained_deserialize_impl! {
    i64, i64, ConstrainedI64, deserialize_i64,
    num_as_self_int!(i64, i8:visit_i8  i16:visit_i16 i32:visit_i32 i64:visit_i64);
    uint_to_self!(i64, u8:visit_u8 u16:visit_u16 u32:visit_u32 u64:visit_u64);
}

#[cfg(not(cnst8bitonly))]
constrained_deserialize_impl! {
    isize, isize, ConstrainedIsize, deserialize_i64,
    num_as_self_int!(isize, i8:visit_i8  i16:visit_i16);
    int_to_int!(isize, i32:visit_i32 i64:visit_i64);
    uint_to_self!(isize, u8:visit_u8 u16:visit_u16 u32:visit_u32 u64:visit_u64);
}
