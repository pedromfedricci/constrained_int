#![allow(unused_macros)]

#[path = "../macros.rs"]
#[macro_use]
mod macros;

macro_rules! bench_add_unsigned_for {
    ($({ $SigInt:ident, $UnsInt:ident, $int_mod:ident, $Cnst:ident, $add:ident }),+ $(,)*) => {$(
        mod $int_mod {
            use constrained_int::$int_mod::$Cnst;
            use criterion::{BenchmarkId, Criterion};

            pub fn primitive_overflow(c: &mut Criterion) {
                type CnstShort = $Cnst<{ $SigInt::MAX - 1 }, { $SigInt::MAX }>;
                type CnstLarge = $Cnst<{ $SigInt::MIN + 1 }, { $SigInt::MAX }>;

                let mut group = c.benchmark_group(overflowed!($SigInt, stringify!($add)));

                // Bench id format: Short ConstrainedIX/ux::MAX.
                group.bench_with_input(
                    BenchmarkId::new(short!($Cnst), max!($UnsInt)),
                    &$UnsInt::MAX,
                    |bench, rhs| {
                        let short = CnstShort::new_max();
                        bench.iter(|| short.$add(*rhs));
                    },
                );

                // Bench id format: Large ConstrainedIX/ux::MAX.
                group.bench_with_input(
                    BenchmarkId::new(large!($Cnst), max!($UnsInt)),
                    &$UnsInt::MAX,
                    |bench, rhs| {
                        let large = CnstLarge::new_max();
                        bench.iter(|| large.$add(*rhs));
                    },
                );

                group.bench_with_input(
                    BenchmarkId::new(stringify!($SigInt), max!($UnsInt)),
                    &$UnsInt::MAX,
                    |bench, rhs| {
                        let prim = $SigInt::MAX;
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

// Generates wrapping_add_unsigned benches.
macro_rules! bench_wrapping_add_unsigned_for {
    ($({ $SigInt:ident, $UnsInt:ident, $int_mod:ident, $Cnst:ident }),+ $(,)*) => {
        bench_add_unsigned_for! {
            $({ $SigInt, $UnsInt, $int_mod, $Cnst, wrapping_add_unsigned }),+
        }
    };
}

// Generates overflowing_add_unsigned benches.
macro_rules! bench_overflowing_add_unsigned_for {
    ($({ $SigInt:ident, $UnsInt:ident, $int_mod:ident, $Cnst:ident }),+ $(,)*) => {
        bench_add_unsigned_for! {
            $({ $SigInt, $UnsInt, $int_mod, $Cnst, overflowing_add_unsigned }),+
        }
    };
}
