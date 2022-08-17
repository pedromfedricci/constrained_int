#[macro_export]
#[doc(hidden)]
macro_rules! __bench_add_int {
    ($({ $SigInt:ident, $int_mod:ident, $Cnst:ident, $add:ident }),+ $(,)*) => {
        use ::criterion::{criterion_group, criterion_main};

        $(mod $int_mod {
            use ::constrained_int::$int_mod::$Cnst;
            use ::criterion::{BenchmarkId, Criterion};

            pub fn primitive_overflow(c: &mut Criterion) {
                type CnstShortMin = $Cnst<{ $SigInt::MIN }, { $SigInt::MIN + 1 }>;
                type CnstLargeMin = $Cnst<{ $SigInt::MIN }, { $SigInt::MAX - 1 }>;

                type CnstShortMax = $Cnst<{ $SigInt::MAX - 1 }, { $SigInt::MAX }>;
                type CnstLargeMax = $Cnst<{ $SigInt::MIN + 1 }, { $SigInt::MAX }>;

                let mut group = c.benchmark_group($crate::overflowed!($SigInt, $add));

                // Bench id format: Short ConstrainedIX/ix::MAX.
                group.bench_with_input(
                    BenchmarkId::new($crate::short!($Cnst), $crate::max!($SigInt)),
                    &$SigInt::MAX,
                    |bench, rhs| {
                        let short = CnstShortMax::new_max();
                        bench.iter(|| short.$add(*rhs));
                    },
                );

                // Bench id format: Short ConstrainedIX/ix::MIN.
                group.bench_with_input(
                    BenchmarkId::new($crate::short!($Cnst), $crate::min!($SigInt)),
                    &$SigInt::MIN,
                    |bench, rhs| {
                        let short = CnstShortMin::new_min();
                        bench.iter(|| short.$add(*rhs));
                    },
                );

                // Bench id format: Large ConstrainedIX/ix::MAX.
                group.bench_with_input(
                    BenchmarkId::new($crate::large!($Cnst), $crate::max!($SigInt)),
                    &$SigInt::MAX,
                    |bench, rhs| {
                        let large = CnstLargeMax::new_max();
                        bench.iter(|| large.$add(*rhs));
                    },
                );

                // Bench id format: Large ConstrainedIX/ix::MIN.
                group.bench_with_input(
                    BenchmarkId::new($crate::large!($Cnst), $crate::min!($SigInt)),
                    &$SigInt::MIN,
                    |bench, rhs| {
                        let large = CnstLargeMin::new_min();
                        bench.iter(|| large.$add(*rhs));
                    },
                );

                group.bench_with_input(
                    BenchmarkId::new(stringify!($SigInt), $crate::max!($SigInt)),
                    &$SigInt::MAX,
                    |bench, rhs| {
                        let prim = $SigInt::MAX;
                        bench.iter(|| prim.$add(*rhs));
                    },
                );

                group.bench_with_input(
                    BenchmarkId::new(stringify!($SigInt),$crate:: min!($SigInt)),
                    &$SigInt::MIN,
                    |bench, rhs| {
                        let prim = $SigInt::MIN;
                        bench.iter(|| prim.$add(*rhs));
                    },
                );

                group.finish();
            }
        })+

        criterion_group! {
            benches,
            $($int_mod::primitive_overflow,)+
        }

        criterion_main!(benches);
    };
}

// Generates wrapping_add benches for signed integers.
#[macro_export]
macro_rules! bench_wrapping_add_int {
    ($({ $SigInt:ident, $int_mod:ident, $Cnst:ident }),+ $(,)*) => {
        $crate::__bench_add_int! {
            $({ $SigInt, $int_mod, $Cnst, wrapping_add }),+
        }
    };
}

// Generates overflowing_add benches for signed integers.
#[macro_export]
macro_rules! bench_overflowing_add_int {
    ($({ $SigInt:ident, $int_mod:ident, $Cnst:ident }),+ $(,)*) => {
        $crate::__bench_add_int! {
            $({ $SigInt, $int_mod, $Cnst, overflowing_add }),+
        }
    };
}
