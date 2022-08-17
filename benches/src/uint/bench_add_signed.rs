#[macro_export]
#[doc(hidden)]
macro_rules! __bench_add_signed {
    ($({ $UnsInt:ident, $SigInt:ident, $uint_mod:ident, $Cnst:ident, $add:ident }),+ $(,)*) => {
        use ::criterion::{criterion_group, criterion_main};

        $(mod $uint_mod {
            use ::constrained_int::$uint_mod::$Cnst;
            use ::criterion::{BenchmarkId, Criterion};

            pub fn primitive_overflow(c: &mut Criterion) {
                type CnstShortMin = $Cnst<{ $UnsInt::MIN }, { $UnsInt::MIN + 1 }>;
                type CnstLargeMin = $Cnst<{ $UnsInt::MIN }, { $UnsInt::MAX - 1 }>;

                type CnstShortMax = $Cnst<{ $UnsInt::MAX - 1 }, { $UnsInt::MAX }>;
                type CnstLargeMax = $Cnst<{ $UnsInt::MIN + 1 }, { $UnsInt::MAX }>;

                let mut group = c.benchmark_group($crate::overflowed!($UnsInt, $add));

                // Bench id format: Short ConstrainedUX/ix::MAX.
                group.bench_with_input(
                    BenchmarkId::new($crate::short!($Cnst), $crate::max!($SigInt)),
                    &$SigInt::MAX,
                    |bench, rhs| {
                        let short = CnstShortMax::new_max();
                        bench.iter(|| short.$add(*rhs));
                    },
                );

                // Bench id format: Short ConstrainedUX/ix::MIN.
                group.bench_with_input(
                    BenchmarkId::new($crate::short!($Cnst), $crate::min!($SigInt)),
                    &$SigInt::MIN,
                    |bench, rhs| {
                        let short = CnstShortMin::new_min();
                        bench.iter(|| short.$add(*rhs));
                    },
                );

                // Bench id format: Large ConstrainedUX/ix::MAX.
                group.bench_with_input(
                    BenchmarkId::new($crate::large!($Cnst), $crate::max!($SigInt)),
                    &$SigInt::MAX,
                    |bench, rhs| {
                        let large = CnstLargeMax::new_max();
                        bench.iter(|| large.$add(*rhs));
                    },
                );

                // Bench id format: Large ConstrainedUX/ix::MIN.
                group.bench_with_input(
                    BenchmarkId::new($crate::large!($Cnst), $crate::min!($SigInt)),
                    &$SigInt::MIN,
                    |bench, rhs| {
                        let large = CnstLargeMin::new_min();
                        bench.iter(|| large.$add(*rhs));
                    },
                );

                group.bench_with_input(
                    BenchmarkId::new(stringify!($UnsInt), $crate::max!($SigInt)),
                    &$SigInt::MAX,
                    |bench, rhs| {
                        let prim = $UnsInt::MAX;
                        bench.iter(|| prim.$add(*rhs));
                    },
                );

                group.bench_with_input(
                    BenchmarkId::new(stringify!($UnsInt), $crate::min!($SigInt)),
                    &$SigInt::MIN,
                    |bench, rhs| {
                        let prim = $UnsInt::MIN;
                        bench.iter(|| prim.$add(*rhs));
                    },
                );

                group.finish();
            }
        })+

        criterion_group! {
            benches,
            $($uint_mod::primitive_overflow,)+
        }

        criterion_main!(benches);
    };
}

// Generates wrapping_add_signed benches.
#[macro_export]
macro_rules! bench_wrapping_add_signed {
    ($({ $UnsInt:ident, $SigInt:ident, $uint_mod:ident, $Cnst:ident }),+ $(,)*) => {
        $crate::__bench_add_signed! {
            $({ $UnsInt, $SigInt, $uint_mod, $Cnst, wrapping_add_signed }),+
        }
    };
}

#[macro_export]
// Generates overflowing_add_signed benches.
macro_rules! bench_overflowing_add_signed {
    ($({ $UnsInt:ident, $SigInt:ident, $uint_mod:ident, $Cnst:ident }),+ $(,)*) => {
        $crate::__bench_add_signed! {
            $({ $UnsInt, $SigInt, $uint_mod, $Cnst, overflowing_add_signed }),+
        }
    };
}
