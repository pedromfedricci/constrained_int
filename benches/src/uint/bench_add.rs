#[macro_export]
#[doc(hidden)]
macro_rules! __bench_add_uint {
    ($({ $UnsInt:ident, $uint_mod:ident, $Cnst:ident, $add:ident }),+ $(,)*) => {
        use ::criterion::{criterion_group, criterion_main};

        $(mod $uint_mod {
            use ::constrained_int::$uint_mod::$Cnst;
            use ::criterion::{BenchmarkId, Criterion};

            pub fn primitive_overflow(c: &mut Criterion) {
                type CnstShort = $Cnst<{ $UnsInt::MAX - 1 }, { $UnsInt::MAX }>;
                type CnstLarge = $Cnst<{ $UnsInt::MIN + 1 }, { $UnsInt::MAX }>;

                let mut group = c.benchmark_group($crate::overflowed!($UnsInt, $add));

                // Bench id format: Short ConstrainedUX/ux::MAX.
                group.bench_with_input(
                    BenchmarkId::new($crate::short!($Cnst), $crate::max!($UnsInt)),
                    &$UnsInt::MAX,
                    |bench, rhs| {
                        let short = CnstShort::new_max();
                        bench.iter(|| short.$add(*rhs));
                    },
                );

                // Bench id format: Large ConstrainedUX/ux::MAX.
                group.bench_with_input(
                    BenchmarkId::new($crate::large!($Cnst), $crate::max!($UnsInt)),
                    &$UnsInt::MAX,
                    |bench, rhs| {
                        let large = CnstLarge::new_max();
                        bench.iter(|| large.$add(*rhs));
                    },
                );

                group.bench_with_input(
                    BenchmarkId::new(stringify!($UnsInt), $crate::max!($UnsInt)),
                    &$UnsInt::MAX,
                    |bench, rhs| {
                        let prim = $UnsInt::MAX;
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

// Generates wrapping_add benches for unsigned integers.
#[macro_export]
macro_rules! bench_wrapping_add_uint {
    ($({ $UnsInt:ident, $uint_mod:ident, $Cnst:ident }),+ $(,)*) => {
        $crate::__bench_add_uint! {
            $({ $UnsInt, $uint_mod, $Cnst, wrapping_add }),+
        }
    };
}

// Generates overflowing_add benches for unsigned integers.
#[macro_export]
macro_rules! bench_overflowing_add_uint {
    ($({ $UnsInt:ident, $uint_mod:ident, $Cnst:ident }),+ $(,)*) => {
        $crate::__bench_add_uint! {
            $({ $UnsInt, $uint_mod, $Cnst, overflowing_add }),+
        }
    };
}
