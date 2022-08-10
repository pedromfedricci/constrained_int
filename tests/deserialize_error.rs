use serde_test::{assert_de_tokens_error, Token};
use std::fmt::{Display, Write};
use std::ops::RangeInclusive;

fn val_err<V, Idx>(value: V, prim: &str, range: RangeInclusive<Idx>) -> String
where
    V: Display,
    Idx: Display,
{
    let (min, max) = (range.start(), range.end());
    let mut val_err = String::from("invalid value: integer ");
    write!(&mut val_err, "`{value}`, expected a constrained {prim} value within {min}..={max}",)
        .unwrap();
    val_err
}

#[test]
fn unbounded_value_cnst_i8() {
    use constrained_int::i8::ConstrainedI8;
    type CnstMin = ConstrainedI8<-128, 126>;
    type CnstMax = ConstrainedI8<-127, 127>;

    let min_err = assert_de_tokens_error::<CnstMin>;
    let max_err = assert_de_tokens_error::<CnstMax>;

    // From signed.
    min_err(&[Token::I8(127)], &val_err(127, "i8", CnstMin::range()));
    min_err(&[Token::I16(-129)], &val_err(-129, "i8", CnstMin::range()));
    min_err(&[Token::I16(127)], &val_err(127, "i8", CnstMin::range()));
    min_err(&[Token::I32(-129)], &val_err(-129, "i8", CnstMin::range()));
    min_err(&[Token::I32(127)], &val_err(127, "i8", CnstMin::range()));
    min_err(&[Token::I64(-129)], &val_err(-129, "i8", CnstMin::range()));
    min_err(&[Token::I64(127)], &val_err(127, "i8", CnstMin::range()));

    max_err(&[Token::I8(-128)], &val_err(-128, "i8", CnstMax::range()));
    max_err(&[Token::I16(-128)], &val_err(-128, "i8", CnstMax::range()));
    max_err(&[Token::I16(128)], &val_err(128, "i8", CnstMax::range()));
    max_err(&[Token::I32(-128)], &val_err(-128, "i8", CnstMax::range()));
    max_err(&[Token::I32(128)], &val_err(128, "i8", CnstMax::range()));
    max_err(&[Token::I64(-128)], &val_err(-128, "i8", CnstMax::range()));
    max_err(&[Token::I64(128)], &val_err(128, "i8", CnstMax::range()));

    // From unsigned.
    min_err(&[Token::U8(127)], &val_err(127, "i8", CnstMin::range()));
    min_err(&[Token::U16(127)], &val_err(127, "i8", CnstMin::range()));
    min_err(&[Token::U32(127)], &val_err(127, "i8", CnstMin::range()));
    min_err(&[Token::U64(127)], &val_err(127, "i8", CnstMin::range()));

    max_err(&[Token::U8(128)], &val_err(128, "i8", CnstMax::range()));
    max_err(&[Token::U16(128)], &val_err(128, "i8", CnstMax::range()));
    max_err(&[Token::U32(128)], &val_err(128, "i8", CnstMax::range()));
    max_err(&[Token::U64(128)], &val_err(128, "i8", CnstMax::range()));
}

#[test]
fn unbounded_value_cnst_i16() {
    use constrained_int::i16::ConstrainedI16;
    type CnstMin = ConstrainedI16<-32768, 32766>;
    type CnstMax = ConstrainedI16<-32767, 32767>;

    let min_err = assert_de_tokens_error::<CnstMin>;
    let max_err = assert_de_tokens_error::<CnstMax>;

    // From signed.
    min_err(&[Token::I16(32767)], &val_err(32767, "i16", CnstMin::range()));
    min_err(&[Token::I32(-32769)], &val_err(-32769, "i16", CnstMin::range()));
    min_err(&[Token::I32(32767)], &val_err(32767, "i16", CnstMin::range()));
    min_err(&[Token::I64(-32769)], &val_err(-32769, "i16", CnstMin::range()));
    min_err(&[Token::I64(32767)], &val_err(32767, "i16", CnstMin::range()));

    max_err(&[Token::I16(-32768)], &val_err(-32768, "i16", CnstMax::range()));
    max_err(&[Token::I32(-32768)], &val_err(-32768, "i16", CnstMax::range()));
    max_err(&[Token::I32(32768)], &val_err(32768, "i16", CnstMax::range()));
    max_err(&[Token::I64(-32768)], &val_err(-32768, "i16", CnstMax::range()));
    max_err(&[Token::I64(32768)], &val_err(32768, "i16", CnstMax::range()));

    // From unsigned.
    min_err(&[Token::U16(32767)], &val_err(32767, "i16", CnstMin::range()));
    min_err(&[Token::U32(32767)], &val_err(32767, "i16", CnstMin::range()));
    min_err(&[Token::U64(32767)], &val_err(32767, "i16", CnstMin::range()));

    max_err(&[Token::U16(32768)], &val_err(32768, "i16", CnstMax::range()));
    max_err(&[Token::U32(32768)], &val_err(32768, "i16", CnstMax::range()));
    max_err(&[Token::U64(32768)], &val_err(32768, "i16", CnstMax::range()));
}

#[test]
fn unbounded_value_cnst_i32() {
    use constrained_int::i32::ConstrainedI32;
    type CnstMin = ConstrainedI32<-2147483648, 2147483646>;
    type CnstMax = ConstrainedI32<-2147483647, 2147483647>;

    let min_err = assert_de_tokens_error::<CnstMin>;
    let max_err = assert_de_tokens_error::<CnstMax>;

    // From signed.
    min_err(&[Token::I32(2147483647)], &val_err(2147483647, "i32", CnstMin::range()));
    min_err(&[Token::I64(-2147483649)], &val_err(-2147483649_i64, "i32", CnstMin::range()));
    min_err(&[Token::I64(2147483647)], &val_err(2147483647, "i32", CnstMin::range()));

    max_err(&[Token::I32(-2147483648)], &val_err(-2147483648, "i32", CnstMax::range()));
    max_err(&[Token::I64(-2147483648)], &val_err(-2147483648, "i32", CnstMax::range()));
    max_err(&[Token::I64(2147483648)], &val_err(2147483648_i64, "i32", CnstMax::range()));

    // From unsigned.
    min_err(&[Token::U32(2147483647)], &val_err(2147483647, "i32", CnstMin::range()));
    min_err(&[Token::U64(2147483647)], &val_err(2147483647, "i32", CnstMin::range()));

    max_err(&[Token::U32(2147483648)], &val_err(2147483648_u32, "i32", CnstMax::range()));
    max_err(&[Token::U64(2147483648)], &val_err(2147483648_u64, "i32", CnstMax::range()));
}

#[test]
fn unbounded_value_cnst_i64() {
    use constrained_int::i64::ConstrainedI64;
    type CnstMin = ConstrainedI64<-9223372036854775808, 9223372036854775806>;
    type CnstMax = ConstrainedI64<-9223372036854775807, 9223372036854775807>;

    let min_err = assert_de_tokens_error::<CnstMin>;
    let max_err = assert_de_tokens_error::<CnstMax>;

    // From signed.
    min_err(
        &[Token::I64(9223372036854775807)],
        &val_err(9223372036854775807_i64, "i64", CnstMin::range()),
    );

    max_err(
        &[Token::I64(-9223372036854775808)],
        &val_err(-9223372036854775808_i64, "i64", CnstMax::range()),
    );

    // From unsigned.
    min_err(
        &[Token::U64(9223372036854775807)],
        &val_err(9223372036854775807_u64, "i64", CnstMin::range()),
    );

    max_err(
        &[Token::U64(9223372036854775808)],
        &val_err(9223372036854775808_u64, "i64", CnstMax::range()),
    );
}

#[test]
fn unbounded_value_cnst_u8() {
    use constrained_int::u8::ConstrainedU8;
    type CnstMin = ConstrainedU8<0, 254>;
    type CnstMax = ConstrainedU8<1, 255>;

    let min_err = assert_de_tokens_error::<CnstMin>;
    let max_err = assert_de_tokens_error::<CnstMax>;

    // From signed.
    min_err(&[Token::I8(-1)], &val_err(-1, "u8", CnstMin::range()));
    min_err(&[Token::I16(-1)], &val_err(-1, "u8", CnstMin::range()));
    min_err(&[Token::I16(255)], &val_err(255, "u8", CnstMin::range()));
    min_err(&[Token::I32(-1)], &val_err(-1, "u8", CnstMin::range()));
    min_err(&[Token::I32(255)], &val_err(255, "u8", CnstMin::range()));
    min_err(&[Token::I64(-1)], &val_err(-1, "u8", CnstMin::range()));
    min_err(&[Token::I64(255)], &val_err(255, "u8", CnstMin::range()));

    max_err(&[Token::I8(0)], &val_err(0, "u8", CnstMax::range()));
    max_err(&[Token::I16(0)], &val_err(0, "u8", CnstMax::range()));
    max_err(&[Token::I16(256)], &val_err(256, "u8", CnstMax::range()));
    max_err(&[Token::I32(0)], &val_err(0, "u8", CnstMax::range()));
    max_err(&[Token::I32(256)], &val_err(256, "u8", CnstMax::range()));
    max_err(&[Token::I32(0)], &val_err(0, "u8", CnstMax::range()));
    max_err(&[Token::I64(256)], &val_err(256, "u8", CnstMax::range()));

    // From unsigned.
    min_err(&[Token::U8(255)], &val_err(255, "u8", CnstMin::range()));
    min_err(&[Token::U16(255)], &val_err(255, "u8", CnstMin::range()));
    min_err(&[Token::U32(255)], &val_err(255, "u8", CnstMin::range()));
    min_err(&[Token::U64(255)], &val_err(255, "u8", CnstMin::range()));

    max_err(&[Token::U16(256)], &val_err(256, "u8", CnstMax::range()));
    max_err(&[Token::U32(256)], &val_err(256, "u8", CnstMax::range()));
    max_err(&[Token::U64(256)], &val_err(256, "u8", CnstMax::range()));
}

#[test]
fn unbounded_value_cnst_u16() {
    use constrained_int::u16::ConstrainedU16;
    type CnstMin = ConstrainedU16<0, 65534>;
    type CnstMax = ConstrainedU16<1, 65535>;

    let min_err = assert_de_tokens_error::<CnstMin>;
    let max_err = assert_de_tokens_error::<CnstMax>;

    // From signed.
    min_err(&[Token::I8(-1)], &val_err(-1, "u16", CnstMin::range()));
    min_err(&[Token::I16(-1)], &val_err(-1, "u16", CnstMin::range()));
    min_err(&[Token::I32(-1)], &val_err(-1, "u16", CnstMin::range()));
    min_err(&[Token::I32(65535)], &val_err(65535, "u16", CnstMin::range()));
    min_err(&[Token::I64(-1)], &val_err(-1, "u16", CnstMin::range()));
    min_err(&[Token::I64(65535)], &val_err(65535, "u16", CnstMin::range()));

    max_err(&[Token::I8(0)], &val_err(0, "u16", CnstMax::range()));
    max_err(&[Token::I16(0)], &val_err(0, "u16", CnstMax::range()));
    max_err(&[Token::I32(0)], &val_err(0, "u16", CnstMax::range()));
    max_err(&[Token::I32(65536)], &val_err(65536, "u16", CnstMax::range()));
    max_err(&[Token::I64(0)], &val_err(0, "u16", CnstMax::range()));
    max_err(&[Token::I64(65536)], &val_err(65536, "u16", CnstMax::range()));

    // From unsigned.
    min_err(&[Token::U16(65535)], &val_err(65535, "u16", CnstMin::range()));
    min_err(&[Token::U32(65535)], &val_err(65535, "u16", CnstMin::range()));
    min_err(&[Token::U64(65535)], &val_err(65535, "u16", CnstMin::range()));

    max_err(&[Token::U32(65536)], &val_err(65536, "u16", CnstMax::range()));
    max_err(&[Token::U64(65536)], &val_err(65536, "u16", CnstMax::range()));
}

#[test]
fn unbounded_value_cnst_u32() {
    use constrained_int::u32::ConstrainedU32;
    type CnstMin = ConstrainedU32<0, 4294967294>;
    type CnstMax = ConstrainedU32<1, 4294967295>;

    let min_err = assert_de_tokens_error::<CnstMin>;
    let max_err = assert_de_tokens_error::<CnstMax>;

    // From signed.
    min_err(&[Token::I8(-1)], &val_err(-1, "u32", CnstMin::range()));
    min_err(&[Token::I16(-1)], &val_err(-1, "u32", CnstMin::range()));
    min_err(&[Token::I32(-1)], &val_err(-1, "u32", CnstMin::range()));
    min_err(&[Token::I64(-1)], &val_err(-1, "u32", CnstMin::range()));
    min_err(&[Token::I64(4294967295)], &val_err(4294967295_i64, "u32", CnstMin::range()));

    max_err(&[Token::I8(0)], &val_err(0, "u32", CnstMax::range()));
    max_err(&[Token::I16(0)], &val_err(0, "u32", CnstMax::range()));
    max_err(&[Token::I32(0)], &val_err(0, "u32", CnstMax::range()));
    max_err(&[Token::I64(0)], &val_err(0, "u32", CnstMax::range()));
    max_err(&[Token::I64(4294967296)], &val_err(4294967296_i64, "u32", CnstMax::range()));

    // From unsigned.
    min_err(&[Token::U32(4294967295)], &val_err(4294967295_u32, "u32", CnstMin::range()));
    min_err(&[Token::U64(4294967295)], &val_err(4294967295_u64, "u32", CnstMin::range()));

    max_err(&[Token::U64(4294967296)], &val_err(4294967296_u64, "u32", CnstMax::range()));
}

#[test]
fn unbounded_value_cnst_u64() {
    use constrained_int::u64::ConstrainedU64;
    type CnstMin = ConstrainedU64<0, 18446744073709551614>;
    type CnstMax = ConstrainedU64<1, 18446744073709551615>;

    let min_err = assert_de_tokens_error::<CnstMin>;
    let max_err = assert_de_tokens_error::<CnstMax>;

    // From signed.
    min_err(&[Token::I8(-1)], &val_err(-1, "u64", CnstMin::range()));
    min_err(&[Token::I16(-1)], &val_err(-1, "u64", CnstMin::range()));
    min_err(&[Token::I32(-1)], &val_err(-1, "u64", CnstMin::range()));
    min_err(&[Token::I64(-1)], &val_err(-1, "u64", CnstMin::range()));
    min_err(&[Token::I64(-1)], &val_err(-1, "u64", CnstMin::range()));

    max_err(&[Token::I8(0)], &val_err(0, "u64", CnstMax::range()));
    max_err(&[Token::I16(0)], &val_err(0, "u64", CnstMax::range()));
    max_err(&[Token::I32(0)], &val_err(0, "u64", CnstMax::range()));
    max_err(&[Token::I64(0)], &val_err(0, "u64", CnstMax::range()));
    max_err(&[Token::I64(0)], &val_err(0, "u64", CnstMax::range()));
}

macro_rules! range_err {
    ($Cnst:ident) => {
        concat!(
            // serde's invalid_type.
            "invalid type: ",
            // user defined.
            stringify!($Cnst),
            "<MIN, MAX, DEF>",
            // serde's expecting.
            ", expected ",
            // user defined.
            "MIN, MAX and DEF to comply with construction constraints"
        )
    };
}

macro_rules! impl_invalid_range_test_for {
    ($({ $Num:ident, $num_mod:ident, $Cnst:ident, $test:ident }),+ $(,)*) => {$(
        #[test]
        fn $test() {
            use constrained_int::$num_mod::$Cnst;
            type MinMax = $Cnst<{ $Num::MIN }, { $Num::MAX }>;
            type MinEqMax = $Cnst<0, 0>;
            type MinGtMax = $Cnst<1, 0>;
            type DefOut = $Cnst<0, 1 , 2>;

            let minmax_err = assert_de_tokens_error::<MinMax>;
            let mineqmax_err = assert_de_tokens_error::<MinEqMax>;
            let mingtmax_err = assert_de_tokens_error::<MinGtMax>;
            let defout_err = assert_de_tokens_error::<DefOut>;

            minmax_err(&[Token::I8(0)], range_err!($Cnst));
            minmax_err(&[Token::I16(0)], range_err!($Cnst));
            minmax_err(&[Token::I32(0)], range_err!($Cnst));
            minmax_err(&[Token::I64(0)], range_err!($Cnst));
            minmax_err(&[Token::U8(0)], range_err!($Cnst));
            minmax_err(&[Token::U16(0)], range_err!($Cnst));
            minmax_err(&[Token::U32(0)], range_err!($Cnst));
            minmax_err(&[Token::U64(0)], range_err!($Cnst));

            mineqmax_err(&[Token::I8(0)], range_err!($Cnst));
            mineqmax_err(&[Token::I16(0)], range_err!($Cnst));
            mineqmax_err(&[Token::I32(0)], range_err!($Cnst));
            mineqmax_err(&[Token::I64(0)], range_err!($Cnst));
            mineqmax_err(&[Token::U8(0)], range_err!($Cnst));
            mineqmax_err(&[Token::U16(0)], range_err!($Cnst));
            mineqmax_err(&[Token::U32(0)], range_err!($Cnst));
            mineqmax_err(&[Token::U64(0)], range_err!($Cnst));

            mingtmax_err(&[Token::I8(0)], range_err!($Cnst));
            mingtmax_err(&[Token::I16(0)], range_err!($Cnst));
            mingtmax_err(&[Token::I32(0)], range_err!($Cnst));
            mingtmax_err(&[Token::I64(0)], range_err!($Cnst));
            mingtmax_err(&[Token::U8(0)], range_err!($Cnst));
            mingtmax_err(&[Token::U16(0)], range_err!($Cnst));
            mingtmax_err(&[Token::U32(0)], range_err!($Cnst));
            mingtmax_err(&[Token::U64(0)], range_err!($Cnst));

            defout_err(&[Token::I8(0)], range_err!($Cnst));
            defout_err(&[Token::I16(0)], range_err!($Cnst));
            defout_err(&[Token::I32(0)], range_err!($Cnst));
            defout_err(&[Token::I64(0)], range_err!($Cnst));
            defout_err(&[Token::U8(0)], range_err!($Cnst));
            defout_err(&[Token::U16(0)], range_err!($Cnst));
            defout_err(&[Token::U32(0)], range_err!($Cnst));
            defout_err(&[Token::U64(0)], range_err!($Cnst));
        }
    )+};
}

impl_invalid_range_test_for! {
    { u8, u8, ConstrainedU8, invalid_range_cnst_u8 },
    { u16, u16, ConstrainedU16, invalid_range_cnst_u16 },
    { u32, u32, ConstrainedU32, invalid_range_cnst_u32 },
    { u64, u64, ConstrainedU64, invalid_range_cnst_u64 },
    { u128, u128, ConstrainedU128, invalid_range_cnst_u128 },
    { usize, usize, ConstrainedUsize, invalid_range_cnst_usize },

    { i8, i8, ConstrainedI8, invalid_range_cnst_i8 },
    { i16, i16, ConstrainedI16, invalid_range_cnst_i16 },
    { i32, i32, ConstrainedI32, invalid_range_cnst_i32 },
    { i64, i64, ConstrainedI64, invalid_range_cnst_i64 },
    { i128, i128, ConstrainedI128, invalid_range_cnst_i128 },
    { isize, isize, ConstrainedIsize, invalid_range_cnst_isize },
}
