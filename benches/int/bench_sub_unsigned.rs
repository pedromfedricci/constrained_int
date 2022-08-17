#![allow(unused_macros)]

#[path = "../macros.rs"]
#[macro_use]
mod macros;

macro_rules! bench_sub_unsigned_for {
    ($({ $SigInt:ident, $UnsInt:ident, $int_mod:ident, $Cnst:ident, $sub:ident }),+ $(,)*) => {$(
        mod $int_mod {
            use constrained_int::$int_mod::$Cnst;
            use criterion::{BenchmarkId, Criterion};

            pub fn primitive_overflow(c: &mut Criterion) {
                type CnstShort = $Cnst<{ $SigInt::MIN }, { $SigInt::MIN + 1 }>;
                type CnstLarge = $Cnst<{ $SigInt::MIN }, { $SigInt::MAX - 1 }>;

                let mut group = c.benchmark_group(overflowed!($SigInt, stringify!($sub)));

                // Bench id format: Short ConstrainedIX/ux::MAX.
                group.bench_with_input(
                    BenchmarkId::new(short!($Cnst), max!($UnsInt)),
                    &$UnsInt::MAX,
                    |bench, rhs| {
                        let short = CnstShort::new_max();
                        bench.iter(|| short.$sub(*rhs));
                    },
                );

                // Bench id format: Large ConstrainedIX/ux::MAX.
                group.bench_with_input(
                    BenchmarkId::new(large!($Cnst), max!($UnsInt)),
                    &$UnsInt::MAX,
                    |bench, rhs| {
                        let large = CnstLarge::new_max();
                        bench.iter(|| large.$sub(*rhs));
                    },
                );

                group.bench_with_input(
                    BenchmarkId::new(stringify!($SigInt), max!($UnsInt)),
                    &$UnsInt::MAX,
                    |bench, rhs| {
                        let prim = $SigInt::MAX;
                        bench.iter(|| prim.$sub(*rhs));
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

// Generates wrapping_sub_unsigned benches.
macro_rules! bench_wrapping_sub_unsigned_for {
    ($({ $SigInt:ident, $UnsInt:ident, $int_mod:ident, $Cnst:ident }),+ $(,)*) => {
        bench_sub_unsigned_for! {
            $({ $SigInt, $UnsInt, $int_mod, $Cnst, wrapping_sub_unsigned }),+
        }
    };
}

// Generates overflowing_sub_unsigned benches.
macro_rules! bench_overflowing_sub_unsigned_for {
    ($({ $SigInt:ident, $UnsInt:ident, $int_mod:ident, $Cnst:ident }),+ $(,)*) => {
        bench_sub_unsigned_for! {
            $({ $SigInt, $UnsInt, $int_mod, $Cnst, overflowing_sub_unsigned }),+
        }
    };
}
