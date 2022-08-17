#![allow(unused_macros)]

#[path = "../macros.rs"]
#[macro_use]
mod macros;

macro_rules! bench_sub_for {
    ($({ $UnsInt:ident, $uint_mod:ident, $Cnst:ident, $sub:ident }),+ $(,)*) => {$(
        mod $uint_mod {
            use constrained_int::$uint_mod::$Cnst;
            use criterion::{BenchmarkId, Criterion};

            pub fn primitive_overflow(c: &mut Criterion) {
                type CnstShort = $Cnst<{ $UnsInt::MIN }, { $UnsInt::MIN + 1 }>;
                type CnstLarge = $Cnst<{ $UnsInt::MIN }, { $UnsInt::MAX - 1 }>;

                let mut group = c.benchmark_group(overflowed!($UnsInt, stringify!($sub)));

                // Bench id format: Short ConstrainedUX/ux::MAX.
                group.bench_with_input(
                    BenchmarkId::new(short!($Cnst), max!($UnsInt)),
                    &$UnsInt::MAX,
                    |bench, rhs| {
                        let short = CnstShort::new_min();
                        bench.iter(|| short.$sub(*rhs));
                    },
                );

                // Bench id format: Large ConstrainedUX/ux::MAX.
                group.bench_with_input(
                    BenchmarkId::new(large!($Cnst), max!($UnsInt)),
                    &$UnsInt::MAX,
                    |bench, rhs| {
                        let large = CnstLarge::new_min();
                        bench.iter(|| large.$sub(*rhs));
                    },
                );

                group.bench_with_input(
                    BenchmarkId::new(stringify!($UnsInt), max!($UnsInt)),
                    &$UnsInt::MAX,
                    |bench, rhs| {
                        let prim = $UnsInt::MIN;
                        bench.iter(|| prim.$sub(*rhs));
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

// Generates wrapping_sub benches for unsigned integers.
macro_rules! bench_wrapping_sub_for {
    ($({ $UnsInt:ident, $uint_mod:ident, $Cnst:ident }),+ $(,)*) => {
        bench_sub_for! {
            $({ $UnsInt, $uint_mod, $Cnst, wrapping_sub }),+
        }
    };
}

// Generates overflowing_sub benches for unsigned integers.
macro_rules! bench_overflowing_sub_for {
    ($({ $UnsInt:ident, $uint_mod:ident, $Cnst:ident }),+ $(,)*) => {
        bench_sub_for! {
            $({ $UnsInt, $uint_mod, $Cnst, overflowing_sub }),+
        }
    };
}
