use serde_test::{assert_de_tokens_error, Token};
use std::fmt::{Display, Write};
use std::ops::RangeInclusive;

fn err<V, Idx>(value: V, prim: &str, range: RangeInclusive<Idx>) -> String
where
    V: Display,
    Idx: Display,
{
    let (min, max) = (range.start(), range.end());
    let mut err = String::from("invalid value: integer ");
    write!(&mut err, "`{value}`, expected a constrained {prim} value within {min}..={max}",)
        .unwrap();
    err
}

#[test]
fn deserialize_error_cnst_i8() {
    use constrained_int::i8::ConstrainedI8;
    type CnstMin = ConstrainedI8<-128, 126>;
    type CnstMax = ConstrainedI8<-127, 127>;

    let assert_min_err = assert_de_tokens_error::<CnstMin>;
    let assert_max_err = assert_de_tokens_error::<CnstMax>;

    // From signed.
    assert_min_err(&[Token::I8(127)], &err(127, "i8", CnstMin::range()));
    assert_min_err(&[Token::I16(-129)], &err(-129, "i8", CnstMin::range()));
    assert_min_err(&[Token::I16(127)], &err(127, "i8", CnstMin::range()));
    assert_min_err(&[Token::I32(-129)], &err(-129, "i8", CnstMin::range()));
    assert_min_err(&[Token::I32(127)], &err(127, "i8", CnstMin::range()));
    assert_min_err(&[Token::I64(-129)], &err(-129, "i8", CnstMin::range()));
    assert_min_err(&[Token::I64(127)], &err(127, "i8", CnstMin::range()));

    assert_max_err(&[Token::I8(-128)], &err(-128, "i8", CnstMax::range()));
    assert_max_err(&[Token::I16(-128)], &err(-128, "i8", CnstMax::range()));
    assert_max_err(&[Token::I16(128)], &err(128, "i8", CnstMax::range()));
    assert_max_err(&[Token::I32(-128)], &err(-128, "i8", CnstMax::range()));
    assert_max_err(&[Token::I32(128)], &err(128, "i8", CnstMax::range()));
    assert_max_err(&[Token::I64(-128)], &err(-128, "i8", CnstMax::range()));
    assert_max_err(&[Token::I64(128)], &err(128, "i8", CnstMax::range()));

    // From unsigned.
    assert_min_err(&[Token::U8(127)], &err(127, "i8", CnstMin::range()));
    assert_min_err(&[Token::U16(127)], &err(127, "i8", CnstMin::range()));
    assert_min_err(&[Token::U32(127)], &err(127, "i8", CnstMin::range()));
    assert_min_err(&[Token::U64(127)], &err(127, "i8", CnstMin::range()));

    assert_max_err(&[Token::U8(128)], &err(128, "i8", CnstMax::range()));
    assert_max_err(&[Token::U16(128)], &err(128, "i8", CnstMax::range()));
    assert_max_err(&[Token::U32(128)], &err(128, "i8", CnstMax::range()));
    assert_max_err(&[Token::U64(128)], &err(128, "i8", CnstMax::range()));
}

#[test]
fn deserialize_error_cnst_i16() {
    use constrained_int::i16::ConstrainedI16;
    type CnstMin = ConstrainedI16<-32768, 32766>;
    type CnstMax = ConstrainedI16<-32767, 32767>;

    let assert_min_err = assert_de_tokens_error::<CnstMin>;
    let assert_max_err = assert_de_tokens_error::<CnstMax>;

    // From signed.
    assert_min_err(&[Token::I16(32767)], &err(32767, "i16", CnstMin::range()));
    assert_min_err(&[Token::I32(-32769)], &err(-32769, "i16", CnstMin::range()));
    assert_min_err(&[Token::I32(32767)], &err(32767, "i16", CnstMin::range()));
    assert_min_err(&[Token::I64(-32769)], &err(-32769, "i16", CnstMin::range()));
    assert_min_err(&[Token::I64(32767)], &err(32767, "i16", CnstMin::range()));

    assert_max_err(&[Token::I16(-32768)], &err(-32768, "i16", CnstMax::range()));
    assert_max_err(&[Token::I32(-32768)], &err(-32768, "i16", CnstMax::range()));
    assert_max_err(&[Token::I32(32768)], &err(32768, "i16", CnstMax::range()));
    assert_max_err(&[Token::I64(-32768)], &err(-32768, "i16", CnstMax::range()));
    assert_max_err(&[Token::I64(32768)], &err(32768, "i16", CnstMax::range()));

    // From unsigned.
    assert_min_err(&[Token::U16(32767)], &err(32767, "i16", CnstMin::range()));
    assert_min_err(&[Token::U32(32767)], &err(32767, "i16", CnstMin::range()));
    assert_min_err(&[Token::U64(32767)], &err(32767, "i16", CnstMin::range()));

    assert_max_err(&[Token::U16(32768)], &err(32768, "i16", CnstMax::range()));
    assert_max_err(&[Token::U32(32768)], &err(32768, "i16", CnstMax::range()));
    assert_max_err(&[Token::U64(32768)], &err(32768, "i16", CnstMax::range()));
}

#[test]
fn deserialize_error_cnst_i32() {
    use constrained_int::i32::ConstrainedI32;
    type CnstMin = ConstrainedI32<-2147483648, 2147483646>;
    type CnstMax = ConstrainedI32<-2147483647, 2147483647>;

    let assert_min_err = assert_de_tokens_error::<CnstMin>;
    let assert_max_err = assert_de_tokens_error::<CnstMax>;

    // From signed.
    assert_min_err(&[Token::I32(2147483647)], &err(2147483647, "i32", CnstMin::range()));
    assert_min_err(&[Token::I64(-2147483649)], &err(-2147483649_i64, "i32", CnstMin::range()));
    assert_min_err(&[Token::I64(2147483647)], &err(2147483647, "i32", CnstMin::range()));

    assert_max_err(&[Token::I32(-2147483648)], &err(-2147483648, "i32", CnstMax::range()));
    assert_max_err(&[Token::I64(-2147483648)], &err(-2147483648, "i32", CnstMax::range()));
    assert_max_err(&[Token::I64(2147483648)], &err(2147483648_i64, "i32", CnstMax::range()));

    // From unsigned.
    assert_min_err(&[Token::U32(2147483647)], &err(2147483647, "i32", CnstMin::range()));
    assert_min_err(&[Token::U64(2147483647)], &err(2147483647, "i32", CnstMin::range()));

    assert_max_err(&[Token::U32(2147483648)], &err(2147483648_u32, "i32", CnstMax::range()));
    assert_max_err(&[Token::U64(2147483648)], &err(2147483648_u64, "i32", CnstMax::range()));
}

#[test]
fn deserialize_error_cnst_i64() {
    use constrained_int::i64::ConstrainedI64;
    type CnstMin = ConstrainedI64<-9223372036854775808, 9223372036854775806>;
    type CnstMax = ConstrainedI64<-9223372036854775807, 9223372036854775807>;

    let assert_min_err = assert_de_tokens_error::<CnstMin>;
    let assert_max_err = assert_de_tokens_error::<CnstMax>;

    // From signed.
    assert_min_err(
        &[Token::I64(9223372036854775807)],
        &err(9223372036854775807_i64, "i64", CnstMin::range()),
    );

    assert_max_err(
        &[Token::I64(-9223372036854775808)],
        &err(-9223372036854775808_i64, "i64", CnstMax::range()),
    );

    // From unsigned.
    assert_min_err(
        &[Token::U64(9223372036854775807)],
        &err(9223372036854775807_u64, "i64", CnstMin::range()),
    );

    assert_max_err(
        &[Token::U64(9223372036854775808)],
        &err(9223372036854775808_u64, "i64", CnstMax::range()),
    );
}

#[test]
fn deserialize_error_cnst_u8() {
    use constrained_int::u8::ConstrainedU8;
    type CnstMin = ConstrainedU8<0, 254>;
    type CnstMax = ConstrainedU8<1, 255>;

    let assert_min_err = assert_de_tokens_error::<CnstMin>;
    let assert_max_err = assert_de_tokens_error::<CnstMax>;

    // From signed.
    assert_min_err(&[Token::I8(-1)], &err(-1, "u8", CnstMin::range()));
    assert_min_err(&[Token::I16(-1)], &err(-1, "u8", CnstMin::range()));
    assert_min_err(&[Token::I16(255)], &err(255, "u8", CnstMin::range()));
    assert_min_err(&[Token::I32(-1)], &err(-1, "u8", CnstMin::range()));
    assert_min_err(&[Token::I32(255)], &err(255, "u8", CnstMin::range()));
    assert_min_err(&[Token::I64(-1)], &err(-1, "u8", CnstMin::range()));
    assert_min_err(&[Token::I64(255)], &err(255, "u8", CnstMin::range()));

    assert_max_err(&[Token::I8(0)], &err(0, "u8", CnstMax::range()));
    assert_max_err(&[Token::I16(0)], &err(0, "u8", CnstMax::range()));
    assert_max_err(&[Token::I16(256)], &err(256, "u8", CnstMax::range()));
    assert_max_err(&[Token::I32(0)], &err(0, "u8", CnstMax::range()));
    assert_max_err(&[Token::I32(256)], &err(256, "u8", CnstMax::range()));
    assert_max_err(&[Token::I32(0)], &err(0, "u8", CnstMax::range()));
    assert_max_err(&[Token::I64(256)], &err(256, "u8", CnstMax::range()));

    // From unsigned.
    assert_min_err(&[Token::U8(255)], &err(255, "u8", CnstMin::range()));
    assert_min_err(&[Token::U16(255)], &err(255, "u8", CnstMin::range()));
    assert_min_err(&[Token::U32(255)], &err(255, "u8", CnstMin::range()));
    assert_min_err(&[Token::U64(255)], &err(255, "u8", CnstMin::range()));

    assert_max_err(&[Token::U16(256)], &err(256, "u8", CnstMax::range()));
    assert_max_err(&[Token::U32(256)], &err(256, "u8", CnstMax::range()));
    assert_max_err(&[Token::U64(256)], &err(256, "u8", CnstMax::range()));
}

#[test]
fn deserialize_error_cnst_u16() {
    use constrained_int::u16::ConstrainedU16;
    type CnstMin = ConstrainedU16<0, 65534>;
    type CnstMax = ConstrainedU16<1, 65535>;

    let assert_min_err = assert_de_tokens_error::<CnstMin>;
    let assert_max_err = assert_de_tokens_error::<CnstMax>;

    // From signed.
    assert_min_err(&[Token::I8(-1)], &err(-1, "u16", CnstMin::range()));
    assert_min_err(&[Token::I16(-1)], &err(-1, "u16", CnstMin::range()));
    assert_min_err(&[Token::I32(-1)], &err(-1, "u16", CnstMin::range()));
    assert_min_err(&[Token::I32(65535)], &err(65535, "u16", CnstMin::range()));
    assert_min_err(&[Token::I64(-1)], &err(-1, "u16", CnstMin::range()));
    assert_min_err(&[Token::I64(65535)], &err(65535, "u16", CnstMin::range()));

    assert_max_err(&[Token::I8(0)], &err(0, "u16", CnstMax::range()));
    assert_max_err(&[Token::I16(0)], &err(0, "u16", CnstMax::range()));
    assert_max_err(&[Token::I32(0)], &err(0, "u16", CnstMax::range()));
    assert_max_err(&[Token::I32(65536)], &err(65536, "u16", CnstMax::range()));
    assert_max_err(&[Token::I64(0)], &err(0, "u16", CnstMax::range()));
    assert_max_err(&[Token::I64(65536)], &err(65536, "u16", CnstMax::range()));

    // From unsigned.
    assert_min_err(&[Token::U16(65535)], &err(65535, "u16", CnstMin::range()));
    assert_min_err(&[Token::U32(65535)], &err(65535, "u16", CnstMin::range()));
    assert_min_err(&[Token::U64(65535)], &err(65535, "u16", CnstMin::range()));

    assert_max_err(&[Token::U32(65536)], &err(65536, "u16", CnstMax::range()));
    assert_max_err(&[Token::U64(65536)], &err(65536, "u16", CnstMax::range()));
}

#[test]
fn deserialize_error_cnst_u32() {
    use constrained_int::u32::ConstrainedU32;
    type CnstMin = ConstrainedU32<0, 4294967294>;
    type CnstMax = ConstrainedU32<1, 4294967295>;

    let assert_min_err = assert_de_tokens_error::<CnstMin>;
    let assert_max_err = assert_de_tokens_error::<CnstMax>;

    // From signed.
    assert_min_err(&[Token::I8(-1)], &err(-1, "u32", CnstMin::range()));
    assert_min_err(&[Token::I16(-1)], &err(-1, "u32", CnstMin::range()));
    assert_min_err(&[Token::I32(-1)], &err(-1, "u32", CnstMin::range()));
    assert_min_err(&[Token::I64(-1)], &err(-1, "u32", CnstMin::range()));
    assert_min_err(&[Token::I64(4294967295)], &err(4294967295_i64, "u32", CnstMin::range()));

    assert_max_err(&[Token::I8(0)], &err(0, "u32", CnstMax::range()));
    assert_max_err(&[Token::I16(0)], &err(0, "u32", CnstMax::range()));
    assert_max_err(&[Token::I32(0)], &err(0, "u32", CnstMax::range()));
    assert_max_err(&[Token::I64(0)], &err(0, "u32", CnstMax::range()));
    assert_max_err(&[Token::I64(4294967296)], &err(4294967296_i64, "u32", CnstMax::range()));

    // From unsigned.
    assert_min_err(&[Token::U32(4294967295)], &err(4294967295_u32, "u32", CnstMin::range()));
    assert_min_err(&[Token::U64(4294967295)], &err(4294967295_u64, "u32", CnstMin::range()));

    assert_max_err(&[Token::U64(4294967296)], &err(4294967296_u64, "u32", CnstMax::range()));
}

#[test]
fn deserialize_error_cnst_u64() {
    use constrained_int::u64::ConstrainedU64;
    type CnstMin = ConstrainedU64<0, 18446744073709551614>;
    type CnstMax = ConstrainedU64<1, 18446744073709551615>;

    let assert_min_err = assert_de_tokens_error::<CnstMin>;
    let assert_max_err = assert_de_tokens_error::<CnstMax>;

    // From signed.
    assert_min_err(&[Token::I8(-1)], &err(-1, "u64", CnstMin::range()));
    assert_min_err(&[Token::I16(-1)], &err(-1, "u64", CnstMin::range()));
    assert_min_err(&[Token::I32(-1)], &err(-1, "u64", CnstMin::range()));
    assert_min_err(&[Token::I64(-1)], &err(-1, "u64", CnstMin::range()));
    assert_min_err(&[Token::I64(-1)], &err(-1, "u64", CnstMin::range()));

    assert_max_err(&[Token::I8(0)], &err(0, "u64", CnstMax::range()));
    assert_max_err(&[Token::I16(0)], &err(0, "u64", CnstMax::range()));
    assert_max_err(&[Token::I32(0)], &err(0, "u64", CnstMax::range()));
    assert_max_err(&[Token::I64(0)], &err(0, "u64", CnstMax::range()));
    assert_max_err(&[Token::I64(0)], &err(0, "u64", CnstMax::range()));
}
