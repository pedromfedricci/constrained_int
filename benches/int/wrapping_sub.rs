#[path = "../macros.rs"]
#[macro_use]
mod macros;

macro_rules! bench_wrapping_sub_for {
    ($({ $SigInt:ident, $int_mod:ident, $Cnst:ident, $bits:literal }),+ $(,)*) => {$(
        mod $int_mod {
            use constrained_int::$int_mod::$Cnst;
            use criterion::{BenchmarkId, Criterion};

            pub fn primitive_overflow(c: &mut Criterion) {
                type CnstShortMin = $Cnst<{ $SigInt::MIN }, { $SigInt::MIN + 1 }>;
                type CnstLargeMin = $Cnst<{ $SigInt::MIN }, { $SigInt::MAX - 1 }>;

                type CnstShortMax = $Cnst<{ $SigInt::MAX - 1 }, { $SigInt::MAX }>;
                type CnstLargeMax = $Cnst<{ $SigInt::MIN + 1 }, { $SigInt::MAX }>;

                let mut group = c.benchmark_group(overflowed!($SigInt, $bits, "wrapping_sub"));

                // Bench id format: Short ConstrainedIX/ix::MAX.
                group.bench_with_input(
                    BenchmarkId::new(short!($Cnst), max!($SigInt)),
                    &$SigInt::MAX,
                    |bench, rhs| {
                        let short = CnstShortMin::new_min();
                        bench.iter(|| short.wrapping_sub(*rhs));
                    },
                );

                // Bench id format: Short ConstrainedIX/ix::MIN.
                group.bench_with_input(
                    BenchmarkId::new(short!($Cnst), min!($SigInt)),
                    &$SigInt::MIN,
                    |bench, rhs| {
                        let short = CnstShortMax::new_max();
                        bench.iter(|| short.wrapping_sub(*rhs));
                    },
                );

                // Bench id format: Large ConstrainedIX/ix::MAX.
                group.bench_with_input(
                    BenchmarkId::new(large!($Cnst), max!($SigInt)),
                    &$SigInt::MAX,
                    |bench, rhs| {
                        let large = CnstLargeMin::new_min();
                        bench.iter(|| large.wrapping_sub(*rhs));
                    },
                );

                // Bench id format: Large ConstrainedIX/ix::MIN.
                group.bench_with_input(
                    BenchmarkId::new(large!($Cnst), min!($SigInt)),
                    &$SigInt::MIN,
                    |bench, rhs| {
                        let large = CnstLargeMax::new_max();
                        bench.iter(|| large.wrapping_sub(*rhs));
                    },
                );

                group.bench_with_input(
                    BenchmarkId::new(stringify!($SigInt), max!($SigInt)),
                    &$SigInt::MAX,
                    |bench, rhs| {
                        let prim = $SigInt::MIN;
                        bench.iter(|| prim.wrapping_sub(*rhs));
                    },
                );

                group.bench_with_input(
                    BenchmarkId::new(stringify!($SigInt), min!($SigInt)),
                    &$SigInt::MIN,
                    |bench, rhs| {
                        let prim = $SigInt::MAX;
                        bench.iter(|| prim.wrapping_sub(*rhs));
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
bench_wrapping_sub_for! {
    { i8, i8, ConstrainedI8, "8-bit signed" },
}

#[cfg(not(cnst8bitonly))]
bench_wrapping_sub_for! {
    { i8, i8, ConstrainedI8, "8-bit signed" },
    { i16, i16, ConstrainedI16, "16-bit signed" },
    { i32, i32, ConstrainedI32, "32-bit signed" },
    { i64, i64, ConstrainedI64, "64-bit signed" },
    { i128, i128, ConstrainedI128, "128-bit signed" },
    { isize, isize, ConstrainedIsize, "pointer-sized signed" }
}
