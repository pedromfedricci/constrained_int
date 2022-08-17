#[macro_export]
#[doc(hidden)]
macro_rules! __bench_sub_unsigned {
    ($({ $SigInt:ident, $UnsInt:ident, $int_mod:ident, $Cnst:ident, $sub:ident }),+ $(,)*) => {
        use ::criterion::{criterion_group, criterion_main};

        $(mod $int_mod {
            use ::constrained_int::$int_mod::$Cnst;
            use ::criterion::{BenchmarkId, Criterion};

            pub fn primitive_overflow(c: &mut Criterion) {
                type CnstShort = $Cnst<{ $SigInt::MIN }, { $SigInt::MIN + 1 }>;
                type CnstLarge = $Cnst<{ $SigInt::MIN }, { $SigInt::MAX - 1 }>;

                let mut group = c.benchmark_group($crate::overflowed!($SigInt, $sub));

                // Bench id format: Short ConstrainedIX/ux::MAX.
                group.bench_with_input(
                    BenchmarkId::new($crate::short!($Cnst), $crate::max!($UnsInt)),
                    &$UnsInt::MAX,
                    |bench, rhs| {
                        let short = CnstShort::new_max();
                        bench.iter(|| short.$sub(*rhs));
                    },
                );

                // Bench id format: Large ConstrainedIX/ux::MAX.
                group.bench_with_input(
                    BenchmarkId::new($crate::large!($Cnst), $crate::max!($UnsInt)),
                    &$UnsInt::MAX,
                    |bench, rhs| {
                        let large = CnstLarge::new_max();
                        bench.iter(|| large.$sub(*rhs));
                    },
                );

                group.bench_with_input(
                    BenchmarkId::new(stringify!($SigInt), $crate::max!($UnsInt)),
                    &$UnsInt::MAX,
                    |bench, rhs| {
                        let prim = $SigInt::MAX;
                        bench.iter(|| prim.$sub(*rhs));
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

// Generates wrapping_sub_unsigned benches.
#[macro_export]
macro_rules! bench_wrapping_sub_unsigned {
    ($({ $SigInt:ident, $UnsInt:ident, $int_mod:ident, $Cnst:ident }),+ $(,)*) => {
        $crate::__bench_sub_unsigned! {
            $({ $SigInt, $UnsInt, $int_mod, $Cnst, wrapping_sub_unsigned }),+
        }
    };
}

// Generates overflowing_sub_unsigned benches.
#[macro_export]
macro_rules! bench_overflowing_sub_unsigned {
    ($({ $SigInt:ident, $UnsInt:ident, $int_mod:ident, $Cnst:ident }),+ $(,)*) => {
        $crate::__bench_sub_unsigned! {
            $({ $SigInt, $UnsInt, $int_mod, $Cnst, overflowing_sub_unsigned }),+
        }
    };
}
