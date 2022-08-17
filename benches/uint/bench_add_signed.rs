#![allow(unused_macros)]

#[path = "../macros.rs"]
#[macro_use]
mod macros;

macro_rules! bench_add_for {
    ($({ $UnsInt:ident, $SigInt:ident, $uint_mod:ident, $Cnst:ident, $add:ident }),+ $(,)*) => {$(
        mod $uint_mod {
            use constrained_int::$uint_mod::$Cnst;
            use criterion::{BenchmarkId, Criterion};

            pub fn primitive_overflow(c: &mut Criterion) {
                type CnstShortMin = $Cnst<{ $UnsInt::MIN }, { $UnsInt::MIN + 1 }>;
                type CnstLargeMin = $Cnst<{ $UnsInt::MIN }, { $UnsInt::MAX - 1 }>;

                type CnstShortMax = $Cnst<{ $UnsInt::MAX - 1 }, { $UnsInt::MAX }>;
                type CnstLargeMax = $Cnst<{ $UnsInt::MIN + 1 }, { $UnsInt::MAX }>;

                let mut group = c.benchmark_group(overflowed!($UnsInt, stringify!($add)));

                // Bench id format: Short ConstrainedUX/ix::MAX.
                group.bench_with_input(
                    BenchmarkId::new(short!($Cnst), max!($SigInt)),
                    &$SigInt::MAX,
                    |bench, rhs| {
                        let short = CnstShortMax::new_max();
                        bench.iter(|| short.$add(*rhs));
                    },
                );

                // Bench id format: Short ConstrainedUX/ix::MIN.
                group.bench_with_input(
                    BenchmarkId::new(short!($Cnst), min!($SigInt)),
                    &$SigInt::MIN,
                    |bench, rhs| {
                        let short = CnstShortMin::new_min();
                        bench.iter(|| short.$add(*rhs));
                    },
                );

                // Bench id format: Large ConstrainedUX/ix::MAX.
                group.bench_with_input(
                    BenchmarkId::new(large!($Cnst), max!($SigInt)),
                    &$SigInt::MAX,
                    |bench, rhs| {
                        let large = CnstLargeMax::new_max();
                        bench.iter(|| large.$add(*rhs));
                    },
                );

                // Bench id format: Large ConstrainedUX/ix::MIN.
                group.bench_with_input(
                    BenchmarkId::new(large!($Cnst), min!($SigInt)),
                    &$SigInt::MIN,
                    |bench, rhs| {
                        let large = CnstLargeMin::new_min();
                        bench.iter(|| large.$add(*rhs));
                    },
                );

                group.bench_with_input(
                    BenchmarkId::new(stringify!($UnsInt), max!($SigInt)),
                    &$SigInt::MAX,
                    |bench, rhs| {
                        let prim = $UnsInt::MAX;
                        bench.iter(|| prim.$add(*rhs));
                    },
                );

                group.bench_with_input(
                    BenchmarkId::new(stringify!($UnsInt), min!($SigInt)),
                    &$SigInt::MIN,
                    |bench, rhs| {
                        let prim = $UnsInt::MIN;
                        bench.iter(|| prim.$add(*rhs));
                    },
                );

                group.finish();
            }
        })+

        use criterion::{criterion_group, criterion_main};

        criterion_group! {
            benches,
            $($uint_mod::primitive_overflow,)+
        }

        criterion_main!(benches);
    };
}

// Generates wrapping_add_signed benches.
macro_rules! bench_wrapping_add_signed_for {
    ($({ $UnsInt:ident, $SigInt:ident, $uint_mod:ident, $Cnst:ident }),+ $(,)*) => {
        bench_add_for! {
            $({ $UnsInt, $SigInt, $uint_mod, $Cnst, wrapping_add_signed }),+
        }
    };
}

// Generates overflowing_add_signed benches.
macro_rules! bench_overflowing_add_signed_for {
    ($({ $UnsInt:ident, $SigInt:ident, $uint_mod:ident, $Cnst:ident }),+ $(,)*) => {
        bench_add_for! {
            $({ $UnsInt, $SigInt, $uint_mod, $Cnst, overflowing_add_signed }),+
        }
    };
}
