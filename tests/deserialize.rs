use serde_test::{assert_de_tokens, Token};

#[test]
fn bounded_value_cnst_i8() {
    use constrained_int::i8::ConstrainedI8;
    type CnstMin = ConstrainedI8<{ i8::MIN }, { i8::MAX - 1 }>;
    type CnstMax = ConstrainedI8<{ i8::MIN + 1 }, { i8::MAX }>;

    let assert_min = |v, t| assert_de_tokens(&CnstMin::new(v).unwrap(), t);
    let assert_max = |v, t| assert_de_tokens(&CnstMax::new(v).unwrap(), t);

    // From signed.
    assert_min(-128, &[Token::I8(-128)]);
    assert_min(-128, &[Token::I16(-128)]);
    assert_min(-128, &[Token::I32(-128)]);
    assert_min(-128, &[Token::I64(-128)]);

    assert_max(127, &[Token::I8(127)]);
    assert_max(127, &[Token::I16(127)]);
    assert_max(127, &[Token::I32(127)]);
    assert_max(127, &[Token::I64(127)]);

    // From unsigned.
    assert_min(0, &[Token::U8(0)]);
    assert_min(0, &[Token::U16(0)]);
    assert_min(0, &[Token::U32(0)]);
    assert_min(0, &[Token::U64(0)]);

    assert_max(127, &[Token::U8(127)]);
    assert_max(127, &[Token::U16(127)]);
    assert_max(127, &[Token::U32(127)]);
    assert_max(127, &[Token::U64(127)]);
}

#[test]
fn bounded_value_cnst_i16() {
    use constrained_int::i16::ConstrainedI16;
    type CnstMin = ConstrainedI16<{ i16::MIN }, { i16::MAX - 1 }>;
    type CnstMax = ConstrainedI16<{ i16::MIN + 1 }, { i16::MAX }>;

    let assert_min = |v, t| assert_de_tokens(&CnstMin::new(v).unwrap(), t);
    let assert_max = |v, t| assert_de_tokens(&CnstMax::new(v).unwrap(), t);

    // From signed.
    assert_min(-128, &[Token::I8(-128)]);
    assert_min(-32768, &[Token::I16(-32768)]);
    assert_min(-32768, &[Token::I32(-32768)]);
    assert_min(-32768, &[Token::I64(-32768)]);

    assert_max(127, &[Token::I8(127)]);
    assert_max(32767, &[Token::I16(32767)]);
    assert_max(32767, &[Token::I32(32767)]);
    assert_max(32767, &[Token::I64(32767)]);

    // From unsigned.
    assert_min(0, &[Token::U8(0)]);
    assert_min(0, &[Token::U16(0)]);
    assert_min(0, &[Token::U32(0)]);
    assert_min(0, &[Token::U64(0)]);

    assert_max(255, &[Token::U8(255)]);
    assert_max(32767, &[Token::U16(32767)]);
    assert_max(32767, &[Token::U32(32767)]);
    assert_max(32767, &[Token::U64(32767)]);
}

#[test]
fn bounded_value_cnst_i32() {
    use constrained_int::i32::ConstrainedI32;
    type CnstMin = ConstrainedI32<{ i32::MIN }, { i32::MAX - 1 }>;
    type CnstMax = ConstrainedI32<{ i32::MIN + 1 }, { i32::MAX }>;

    let assert_min = |v, t| assert_de_tokens(&CnstMin::new(v).unwrap(), t);
    let assert_max = |v, t| assert_de_tokens(&CnstMax::new(v).unwrap(), t);

    // From signed.
    assert_min(-128, &[Token::I8(-128)]);
    assert_min(-32768, &[Token::I16(-32768)]);
    assert_min(-2147483648, &[Token::I32(-2147483648)]);
    assert_min(-2147483648, &[Token::I64(-2147483648)]);

    assert_max(127, &[Token::I8(127)]);
    assert_max(32767, &[Token::I16(32767)]);
    assert_max(2147483647, &[Token::I32(2147483647)]);
    assert_max(2147483647, &[Token::I64(2147483647)]);

    // From unsigned.
    assert_min(0, &[Token::U8(0)]);
    assert_min(0, &[Token::U16(0)]);
    assert_min(0, &[Token::U32(0)]);
    assert_min(0, &[Token::U64(0)]);

    assert_max(255, &[Token::U8(255)]);
    assert_max(65535, &[Token::U16(65535)]);
    assert_max(2147483647, &[Token::U32(2147483647)]);
    assert_max(2147483647, &[Token::U64(2147483647)]);
}

#[test]
fn bounded_value_cnst_i64() {
    use constrained_int::i64::ConstrainedI64;
    type CnstMin = ConstrainedI64<{ i64::MIN }, { i64::MAX - 1 }>;
    type CnstMax = ConstrainedI64<{ i64::MIN + 1 }, { i64::MAX }>;

    let assert_min = |v, t| assert_de_tokens(&CnstMin::new(v).unwrap(), t);
    let assert_max = |v, t| assert_de_tokens(&CnstMax::new(v).unwrap(), t);

    // From signed.
    assert_min(-128, &[Token::I8(-128)]);
    assert_min(-32768, &[Token::I16(-32768)]);
    assert_min(-2147483648, &[Token::I32(-2147483648)]);
    assert_min(-9223372036854775808, &[Token::I64(-9223372036854775808)]);

    assert_max(127, &[Token::I8(127)]);
    assert_max(32767, &[Token::I16(32767)]);
    assert_max(2147483647, &[Token::I32(2147483647)]);
    assert_max(9223372036854775807, &[Token::I64(9223372036854775807)]);

    // From unsigned.
    assert_min(0, &[Token::U8(0)]);
    assert_min(0, &[Token::U16(0)]);
    assert_min(0, &[Token::U32(0)]);
    assert_min(0, &[Token::U64(0)]);

    assert_max(255, &[Token::U8(255)]);
    assert_max(65535, &[Token::U16(65535)]);
    assert_max(4294967295, &[Token::U32(4294967295)]);
    assert_max(9223372036854775807, &[Token::U64(9223372036854775807)]);
}

#[test]
fn bounded_value_cnst_isize() {
    use constrained_int::isize::ConstrainedIsize;
    type Cnst = ConstrainedIsize<-8, 8>;

    let assert = |v, t| assert_de_tokens(&Cnst::new(v).unwrap(), t);

    // From signed.
    assert(-8, &[Token::I8(-8)]);
    assert(-8, &[Token::I16(-8)]);
    assert(-8, &[Token::I32(-8)]);
    assert(-8, &[Token::I64(-8)]);

    assert(8, &[Token::I8(8)]);
    assert(8, &[Token::I16(8)]);
    assert(8, &[Token::I32(8)]);
    assert(8, &[Token::I64(8)]);

    // From unsigned.
    assert(0, &[Token::U8(0)]);
    assert(0, &[Token::U16(0)]);
    assert(0, &[Token::U32(0)]);
    assert(0, &[Token::U64(0)]);

    assert(8, &[Token::U8(8)]);
    assert(8, &[Token::U16(8)]);
    assert(8, &[Token::U32(8)]);
    assert(8, &[Token::U64(8)]);
}

#[test]
fn bounded_value_cnst_i128() {
    use constrained_int::i128::ConstrainedI128;
    type CnstMin = ConstrainedI128<{ i128::MIN }, { i128::MAX - 1 }>;
    type CnstMax = ConstrainedI128<{ i128::MIN + 1 }, { i128::MAX }>;

    let assert_min = |v, t| assert_de_tokens(&CnstMin::new(v).unwrap(), t);
    let assert_max = |v, t| assert_de_tokens(&CnstMax::new(v).unwrap(), t);

    // From signed.
    assert_min(-128, &[Token::I8(-128)]);
    assert_min(-32768, &[Token::I16(-32768)]);
    assert_min(-2147483648, &[Token::I32(-2147483648)]);
    assert_min(-9223372036854775808, &[Token::I64(-9223372036854775808)]);

    assert_max(127, &[Token::I8(127)]);
    assert_max(32767, &[Token::I16(32767)]);
    assert_max(2147483647, &[Token::I32(2147483647)]);
    assert_max(9223372036854775807, &[Token::I64(9223372036854775807)]);

    // From unsigned.
    assert_min(0, &[Token::U8(0)]);
    assert_min(0, &[Token::U16(0)]);
    assert_min(0, &[Token::U32(0)]);
    assert_min(0, &[Token::U64(0)]);

    assert_max(255, &[Token::U8(255)]);
    assert_max(65535, &[Token::U16(65535)]);
    assert_max(4294967295, &[Token::U32(4294967295)]);
    assert_max(18446744073709551615, &[Token::U64(18446744073709551615)]);
}

#[test]
fn bounded_value_cnst_u8() {
    use constrained_int::u8::ConstrainedU8;
    type CnstMin = ConstrainedU8<{ u8::MIN }, { u8::MAX - 1 }>;
    type CnstMax = ConstrainedU8<{ u8::MIN + 1 }, { u8::MAX }>;

    let assert_min = |v, t| assert_de_tokens(&CnstMin::new(v).unwrap(), t);
    let assert_max = |v, t| assert_de_tokens(&CnstMax::new(v).unwrap(), t);

    // From signed.
    assert_min(0, &[Token::I8(0)]);
    assert_min(0, &[Token::I16(0)]);
    assert_min(0, &[Token::I32(0)]);
    assert_min(0, &[Token::I64(0)]);

    assert_max(127, &[Token::I8(127)]);
    assert_max(255, &[Token::I16(255)]);
    assert_max(255, &[Token::I32(255)]);
    assert_max(255, &[Token::I64(255)]);

    // From unsigned.
    assert_min(0, &[Token::U8(0)]);
    assert_min(0, &[Token::U16(0)]);
    assert_min(0, &[Token::U32(0)]);
    assert_min(0, &[Token::U64(0)]);

    assert_max(255, &[Token::U8(255)]);
    assert_max(255, &[Token::U16(255)]);
    assert_max(255, &[Token::U32(255)]);
    assert_max(255, &[Token::U64(255)]);
}

#[test]
fn bounded_value_cnst_u16() {
    use constrained_int::u16::ConstrainedU16;
    type CnstMin = ConstrainedU16<{ u16::MIN }, { u16::MAX - 1 }>;
    type CnstMax = ConstrainedU16<{ u16::MIN + 1 }, { u16::MAX }>;

    let assert_min = |v, t| assert_de_tokens(&CnstMin::new(v).unwrap(), t);
    let assert_max = |v, t| assert_de_tokens(&CnstMax::new(v).unwrap(), t);

    // From signed.
    assert_min(0, &[Token::I8(0)]);
    assert_min(0, &[Token::I16(0)]);
    assert_min(0, &[Token::I32(0)]);
    assert_min(0, &[Token::I64(0)]);

    assert_max(127, &[Token::I8(127)]);
    assert_max(32767, &[Token::I16(32767)]);
    assert_max(65535, &[Token::I32(65535)]);
    assert_max(65535, &[Token::I64(65535)]);

    // From unsigned.
    assert_min(0, &[Token::U8(0)]);
    assert_min(0, &[Token::U16(0)]);
    assert_min(0, &[Token::U32(0)]);
    assert_min(0, &[Token::U64(0)]);

    assert_max(255, &[Token::U8(255)]);
    assert_max(65535, &[Token::U16(65535)]);
    assert_max(65535, &[Token::U32(65535)]);
    assert_max(65535, &[Token::U64(65535)]);
}

#[test]
fn bounded_value_cnst_u32() {
    use constrained_int::u32::ConstrainedU32;
    type CnstMin = ConstrainedU32<{ u32::MIN }, { u32::MAX - 1 }>;
    type CnstMax = ConstrainedU32<{ u32::MIN + 1 }, { u32::MAX }>;

    let assert_min = |v, t| assert_de_tokens(&CnstMin::new(v).unwrap(), t);
    let assert_max = |v, t| assert_de_tokens(&CnstMax::new(v).unwrap(), t);

    // From signed.
    assert_min(0, &[Token::I8(0)]);
    assert_min(0, &[Token::I16(0)]);
    assert_min(0, &[Token::I32(0)]);
    assert_min(0, &[Token::I64(0)]);

    assert_max(127, &[Token::I8(127)]);
    assert_max(32767, &[Token::I16(32767)]);
    assert_max(2147483647, &[Token::I32(2147483647)]);
    assert_max(4294967295, &[Token::I64(4294967295)]);

    // From unsigned.
    assert_min(0, &[Token::U8(0)]);
    assert_min(0, &[Token::U16(0)]);
    assert_min(0, &[Token::U32(0)]);
    assert_min(0, &[Token::U64(0)]);

    assert_max(255, &[Token::U8(255)]);
    assert_max(65535, &[Token::U16(65535)]);
    assert_max(4294967295, &[Token::U32(4294967295)]);
    assert_max(4294967295, &[Token::U64(4294967295)]);
}

#[test]
fn bounded_value_cnst_u64() {
    use constrained_int::u64::ConstrainedU64;
    type CnstMin = ConstrainedU64<{ u64::MIN }, { u64::MAX - 1 }>;
    type CnstMax = ConstrainedU64<{ u64::MIN + 1 }, { u64::MAX }>;

    let assert_min = |v, t| assert_de_tokens(&CnstMin::new(v).unwrap(), t);
    let assert_max = |v, t| assert_de_tokens(&CnstMax::new(v).unwrap(), t);

    // From signed.
    assert_min(0, &[Token::I8(0)]);
    assert_min(0, &[Token::I16(0)]);
    assert_min(0, &[Token::I32(0)]);
    assert_min(0, &[Token::I64(0)]);

    assert_max(127, &[Token::I8(127)]);
    assert_max(32767, &[Token::I16(32767)]);
    assert_max(2147483647, &[Token::I32(2147483647)]);
    assert_max(9223372036854775807, &[Token::I64(9223372036854775807)]);

    // From unsigned.
    assert_min(0, &[Token::U8(0)]);
    assert_min(0, &[Token::U16(0)]);
    assert_min(0, &[Token::U32(0)]);
    assert_min(0, &[Token::U64(0)]);

    assert_max(255, &[Token::U8(255)]);
    assert_max(65535, &[Token::U16(65535)]);
    assert_max(4294967295, &[Token::U32(4294967295)]);
    assert_max(18446744073709551615, &[Token::U64(18446744073709551615)]);
}

#[test]
fn bounded_value_cnst_usize() {
    use constrained_int::usize::ConstrainedUsize;
    type Cnst = ConstrainedUsize<0, 8>;

    let assert = |v, t| assert_de_tokens(&Cnst::new(v).unwrap(), t);

    // From signed.
    assert(0, &[Token::I8(0)]);
    assert(0, &[Token::I16(0)]);
    assert(0, &[Token::I32(0)]);
    assert(0, &[Token::I64(0)]);

    assert(8, &[Token::I8(8)]);
    assert(8, &[Token::I16(8)]);
    assert(8, &[Token::I32(8)]);
    assert(8, &[Token::I64(8)]);

    // From unsigned.
    assert(0, &[Token::U8(0)]);
    assert(0, &[Token::U16(0)]);
    assert(0, &[Token::U32(0)]);
    assert(0, &[Token::U64(0)]);

    assert(8, &[Token::U8(8)]);
    assert(8, &[Token::U16(8)]);
    assert(8, &[Token::U32(8)]);
    assert(8, &[Token::U64(8)]);
}

#[test]
fn bounded_value_cnst_u128() {
    use constrained_int::u128::ConstrainedU128;
    type CnstMin = ConstrainedU128<{ u128::MIN }, { u128::MAX - 1 }>;
    type CnstMax = ConstrainedU128<{ u128::MIN + 1 }, { u128::MAX }>;

    let assert_min = |v, t| assert_de_tokens(&CnstMin::new(v).unwrap(), t);
    let assert_max = |v, t| assert_de_tokens(&CnstMax::new(v).unwrap(), t);

    // From signed.
    assert_min(0, &[Token::I8(0)]);
    assert_min(0, &[Token::I16(0)]);
    assert_min(0, &[Token::I32(0)]);
    assert_min(0, &[Token::I64(0)]);

    assert_max(127, &[Token::I8(127)]);
    assert_max(32767, &[Token::I16(32767)]);
    assert_max(2147483647, &[Token::I32(2147483647)]);
    assert_max(9223372036854775807, &[Token::I64(9223372036854775807)]);

    // From unsigned.
    assert_min(0, &[Token::U8(0)]);
    assert_min(0, &[Token::U16(0)]);
    assert_min(0, &[Token::U32(0)]);
    assert_min(0, &[Token::U64(0)]);

    assert_max(255, &[Token::U8(255)]);
    assert_max(65535, &[Token::U16(65535)]);
    assert_max(4294967295, &[Token::U32(4294967295)]);
    assert_max(18446744073709551615, &[Token::U64(18446744073709551615)]);
}
