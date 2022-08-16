#![feature(mixed_integer_ops)]

#[path = "../macros.rs"]
#[macro_use]
mod macros;

macro_rules! bench_wrapping_sub_unsigned_for {
    ($({ $SigInt:ident, $UnsInt:ident, $int_mod:ident, $Cnst:ident, $bits:literal }),+ $(,)*) => {$(
        mod $int_mod {
            use constrained_int::$int_mod::$Cnst;
            use criterion::{BenchmarkId, Criterion};

            pub fn primitive_overflow(c: &mut Criterion) {
                type CnstShort = $Cnst<{ $SigInt::MIN }, { $SigInt::MIN + 1 }>;
                type CnstLarge = $Cnst<{ $SigInt::MIN }, { $SigInt::MAX - 1 }>;

                let mut group = c.benchmark_group(overflowed!($SigInt, $bits, "wrapping_sub_unsigned"));

                // Bench id format: Short ConstrainedIX/ux::MAX.
                group.bench_with_input(
                    BenchmarkId::new(short!($Cnst), max!($UnsInt)),
                    &$UnsInt::MAX,
                    |bench, rhs| {
                        let short = CnstShort::new_max();
                        bench.iter(|| short.wrapping_sub_unsigned(*rhs));
                    },
                );

                // Bench id format: Large ConstrainedIX/ux::MAX.
                group.bench_with_input(
                    BenchmarkId::new(large!($Cnst), max!($UnsInt)),
                    &$UnsInt::MAX,
                    |bench, rhs| {
                        let large = CnstLarge::new_max();
                        bench.iter(|| large.wrapping_sub_unsigned(*rhs));
                    },
                );

                group.bench_with_input(
                    BenchmarkId::new(stringify!($SigInt), max!($UnsInt)),
                    &$UnsInt::MAX,
                    |bench, rhs| {
                        let prim = $SigInt::MAX;
                        bench.iter(|| prim.wrapping_sub_unsigned(*rhs));
                    },
                );

                group.finish();
            }
        })+

        use criterion::{criterion_group, criterion_main};

        criterion_group! {
            benches,
            $($int_mod::primitive_overflow,)+
        }

        criterion_main!(benches);
    };
}

#[cfg(cnst8bitonly)]
bench_wrapping_sub_unsigned_for! {
    { i8, u8, i8, ConstrainedI8, "8-bit signed" },
}

#[cfg(not(cnst8bitonly))]
bench_wrapping_sub_unsigned_for! {
    { i8, u8, i8, ConstrainedI8, "8-bit signed" },
    { i16, u16, i16, ConstrainedI16, "16-bit signed" },
    { i32, u32, i32, ConstrainedI32, "32-bit signed" },
    { i64, u64, i64, ConstrainedI64, "64-bit signed" },
    { i128, u128, i128, ConstrainedI128, "128-bit signed" },
    { isize, usize, isize, ConstrainedIsize, "pointer-sized signed" }
}
