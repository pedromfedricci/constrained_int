#[macro_export]
#[doc(hidden)]
macro_rules! __bench_sub_uint {
    ($({ $UnsInt:ident, $uint_mod:ident, $Cnst:ident, $sub:ident }),+ $(,)*) => {
        use ::criterion::{criterion_group, criterion_main};

        $(mod $uint_mod {
            use ::constrained_int::$uint_mod::$Cnst;
            use ::criterion::{BenchmarkId, Criterion};

            pub fn primitive_overflow(c: &mut Criterion) {
                type CnstShort = $Cnst<{ $UnsInt::MIN }, { $UnsInt::MIN + 1 }>;
                type CnstLarge = $Cnst<{ $UnsInt::MIN }, { $UnsInt::MAX - 1 }>;

                let mut group = c.benchmark_group($crate::overflowed!($UnsInt, $sub));

                // Bench id format: Short ConstrainedUX/ux::MAX.
                group.bench_with_input(
                    BenchmarkId::new($crate::short!($Cnst), $crate::max!($UnsInt)),
                    &$UnsInt::MAX,
                    |bench, rhs| {
                        let short = CnstShort::new_min();
                        bench.iter(|| short.$sub(*rhs));
                    },
                );

                // Bench id format: Large ConstrainedUX/ux::MAX.
                group.bench_with_input(
                    BenchmarkId::new($crate::large!($Cnst), $crate::max!($UnsInt)),
                    &$UnsInt::MAX,
                    |bench, rhs| {
                        let large = CnstLarge::new_min();
                        bench.iter(|| large.$sub(*rhs));
                    },
                );

                group.bench_with_input(
                    BenchmarkId::new(stringify!($UnsInt), $crate::max!($UnsInt)),
                    &$UnsInt::MAX,
                    |bench, rhs| {
                        let prim = $UnsInt::MIN;
                        bench.iter(|| prim.$sub(*rhs));
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

// Generates wrapping_sub benches for unsigned integers.
#[macro_export]
macro_rules! bench_wrapping_sub_uint {
    ($({ $UnsInt:ident, $uint_mod:ident, $Cnst:ident }),+ $(,)*) => {
        $crate::__bench_sub_uint! {
            $({ $UnsInt, $uint_mod, $Cnst, wrapping_sub }),+
        }
    };
}

// Generates overflowing_sub benches for unsigned integers.
#[macro_export]
macro_rules! bench_overflowing_sub_uint {
    ($({ $UnsInt:ident, $uint_mod:ident, $Cnst:ident }),+ $(,)*) => {
        $crate::__bench_sub_uint! {
            $({ $UnsInt, $uint_mod, $Cnst, overflowing_sub }),+
        }
    };
}
