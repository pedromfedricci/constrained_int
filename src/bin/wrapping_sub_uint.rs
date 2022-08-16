use constrained_int::u8::ConstrainedU8;

// type CnstLarge = ConstrainedU8<{ u8::MIN }, { u8::MAX - 1 }>;
type CnstShort = ConstrainedU8<{ u8::MIN }, { u8::MIN + 1 }>;

fn main() {
    // let min = CnstLarge::new_min();
    // let _a = min.wrapping_sub(u8::MAX);

    let min = CnstShort::new_min();
    let _a = min.wrapping_sub(u8::MAX);
}
