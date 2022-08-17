#![allow(unused_macros)]

#[path = "../macros.rs"]
#[macro_use]
mod macros;

macro_rules! bench_add_for {
    ($({ $SigInt:ident, $int_mod:ident, $Cnst:ident, $add:ident }),+ $(,)*) => {$(
        mod $int_mod {
            use constrained_int::$int_mod::$Cnst;
            use criterion::{BenchmarkId, Criterion};

            pub fn primitive_overflow(c: &mut Criterion) {
                type CnstShortMin = $Cnst<{ $SigInt::MIN }, { $SigInt::MIN + 1 }>;
                type CnstLargeMin = $Cnst<{ $SigInt::MIN }, { $SigInt::MAX - 1 }>;

                type CnstShortMax = $Cnst<{ $SigInt::MAX - 1 }, { $SigInt::MAX }>;
                type CnstLargeMax = $Cnst<{ $SigInt::MIN + 1 }, { $SigInt::MAX }>;

                let mut group = c.benchmark_group(overflowed!($SigInt, stringify!($add)));

                // Bench id format: Short ConstrainedIX/ix::MAX.
                group.bench_with_input(
                    BenchmarkId::new(short!($Cnst), max!($SigInt)),
                    &$SigInt::MAX,
                    |bench, rhs| {
                        let short = CnstShortMax::new_max();
                        bench.iter(|| short.$add(*rhs));
                    },
                );

                // Bench id format: Short ConstrainedIX/ix::MIN.
                group.bench_with_input(
                    BenchmarkId::new(short!($Cnst), min!($SigInt)),
                    &$SigInt::MIN,
                    |bench, rhs| {
                        let short = CnstShortMin::new_min();
                        bench.iter(|| short.$add(*rhs));
                    },
                );

                // Bench id format: Large ConstrainedIX/ix::MAX.
                group.bench_with_input(
                    BenchmarkId::new(large!($Cnst), max!($SigInt)),
                    &$SigInt::MAX,
                    |bench, rhs| {
                        let large = CnstLargeMax::new_max();
                        bench.iter(|| large.$add(*rhs));
                    },
                );

                // Bench id format: Large ConstrainedIX/ix::MIN.
                group.bench_with_input(
                    BenchmarkId::new(large!($Cnst), min!($SigInt)),
                    &$SigInt::MIN,
                    |bench, rhs| {
                        let large = CnstLargeMin::new_min();
                        bench.iter(|| large.$add(*rhs));
                    },
                );

                group.bench_with_input(
                    BenchmarkId::new(stringify!($SigInt), max!($SigInt)),
                    &$SigInt::MAX,
                    |bench, rhs| {
                        let prim = $SigInt::MAX;
                        bench.iter(|| prim.$add(*rhs));
                    },
                );

                group.bench_with_input(
                    BenchmarkId::new(stringify!($SigInt), min!($SigInt)),
                    &$SigInt::MIN,
                    |bench, rhs| {
                        let prim = $SigInt::MIN;
                        bench.iter(|| prim.$add(*rhs));
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

// Generates wrapping_add benches for signed integers.
macro_rules! bench_wrapping_add_for {
    ($({ $SigInt:ident, $int_mod:ident, $Cnst:ident }),+ $(,)*) => {
        bench_add_for! {
            $({ $SigInt, $int_mod, $Cnst, wrapping_add }),+
        }
    };
}

// Generates overflowing_add benches for signed integers.
macro_rules! bench_overflowing_add_for {
    ($({ $SigInt:ident, $int_mod:ident, $Cnst:ident }),+ $(,)*) => {
        bench_add_for! {
            $({ $SigInt, $int_mod, $Cnst, overflowing_add }),+
        }
    };
}
