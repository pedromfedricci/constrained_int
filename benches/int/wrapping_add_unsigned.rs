#![feature(mixed_integer_ops)]

#[path = "../macros.rs"]
#[macro_use]
mod macros;

macro_rules! bench_wrapping_add_unsigned_for {
    ($({ $SigInt:ident, $UnsInt:ident, $int_mod:ident, $Cnst:ident }),+ $(,)*) => {$(
        mod $int_mod {
            use constrained_int::$int_mod::$Cnst;
            use criterion::{BenchmarkId, Criterion};

            pub fn primitive_overflow(c: &mut Criterion) {
                type CnstShort = $Cnst<{ $SigInt::MAX - 1 }, { $SigInt::MAX }>;
                type CnstLarge = $Cnst<{ $SigInt::MIN + 1 }, { $SigInt::MAX }>;

                let mut group = c.benchmark_group(overflowed!($SigInt, "wrapping_add_unsigned"));

                // Bench id format: Short ConstrainedIX/ux::MAX.
                group.bench_with_input(
                    BenchmarkId::new(short!($Cnst), max!($UnsInt)),
                    &$UnsInt::MAX,
                    |bench, rhs| {
                        let short = CnstShort::new_max();
                        bench.iter(|| short.wrapping_add_unsigned(*rhs));
                    },
                );

                // Bench id format: Large ConstrainedIX/ux::MAX.
                group.bench_with_input(
                    BenchmarkId::new(large!($Cnst), max!($UnsInt)),
                    &$UnsInt::MAX,
                    |bench, rhs| {
                        let large = CnstLarge::new_max();
                        bench.iter(|| large.wrapping_add_unsigned(*rhs));
                    },
                );

                group.bench_with_input(
                    BenchmarkId::new(stringify!($SigInt), max!($UnsInt)),
                    &$UnsInt::MAX,
                    |bench, rhs| {
                        let prim = $SigInt::MAX;
                        bench.iter(|| prim.wrapping_add_unsigned(*rhs));
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
bench_wrapping_add_unsigned_for! {
    { i8, u8, i8, ConstrainedI8 },
}

#[cfg(not(cnst8bitonly))]
bench_wrapping_add_unsigned_for! {
    { i8, u8, i8, ConstrainedI8 },
    { i16, u16, i16, ConstrainedI16 },
    { i32, u32, i32, ConstrainedI32 },
    { i64, u64, i64, ConstrainedI64 },
    { i128, u128, i128, ConstrainedI128 },
    { isize, usize, isize, ConstrainedIsize }
}
