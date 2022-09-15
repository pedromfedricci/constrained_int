use serde_test::{assert_ser_tokens, Token};

#[test]
fn constrained_usizes() {
    use constrained_int::u8::ConstrainedU8;
    assert_ser_tokens(&ConstrainedU8::<0, 1>::default(), &[Token::U8(0)]);

    use constrained_int::u16::ConstrainedU16;
    assert_ser_tokens(&ConstrainedU16::<0, 1>::default(), &[Token::U16(0)]);

    use constrained_int::u32::ConstrainedU32;
    assert_ser_tokens(&ConstrainedU32::<0, 1>::default(), &[Token::U32(0)]);

    use constrained_int::u64::ConstrainedU64;
    assert_ser_tokens(&ConstrainedU64::<0, 1>::default(), &[Token::U64(0)]);
}

#[test]
fn constrained_isizes() {
    use constrained_int::i8::ConstrainedI8;
    assert_ser_tokens(&ConstrainedI8::<-1, 1>::default(), &[Token::I8(-1)]);

    use constrained_int::i16::ConstrainedI16;
    assert_ser_tokens(&ConstrainedI16::<-1, 1>::default(), &[Token::I16(-1)]);

    use constrained_int::i32::ConstrainedI32;
    assert_ser_tokens(&ConstrainedI32::<-1, 1>::default(), &[Token::I32(-1)]);

    use constrained_int::i64::ConstrainedI64;
    assert_ser_tokens(&ConstrainedI64::<-1, 1>::default(), &[Token::I64(-1)]);
}

#[test]
fn wrapping() {
    use constrained_int::wrapping::Wrapping;

    use constrained_int::u8::ConstrainedU8;
    type WrappedU8 = Wrapping<ConstrainedU8<0, 254>>;
    assert_ser_tokens(&WrappedU8::default(), &[Token::U8(0)]);

    use constrained_int::i16::ConstrainedI16;
    type WrappedI16 = Wrapping<ConstrainedI16<-128, 126>>;
    assert_ser_tokens(&WrappedI16::default(), &[Token::I16(-128)]);
}
