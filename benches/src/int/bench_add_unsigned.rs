#[macro_export]
#[doc(hidden)]
macro_rules! __bench_add_unsigned {
    ($({ $SigInt:ident, $UnsInt:ident, $int_mod:ident, $Cnst:ident, $add:ident }),+ $(,)*) => {
        use ::criterion::{criterion_group, criterion_main};

        $(mod $int_mod {
            use ::constrained_int::$int_mod::$Cnst;
            use ::criterion::{BenchmarkId, Criterion};

            pub fn primitive_overflow(c: &mut Criterion) {
                type CnstShort = $Cnst<{ $SigInt::MAX - 1 }, { $SigInt::MAX }>;
                type CnstLarge = $Cnst<{ $SigInt::MIN + 1 }, { $SigInt::MAX }>;

                let mut group = c.benchmark_group($crate::overflowed!($SigInt, $add));

                // Bench id format: Short ConstrainedIX/ux::MAX.
                group.bench_with_input(
                    BenchmarkId::new($crate::short!($Cnst), $crate::max!($UnsInt)),
                    &$UnsInt::MAX,
                    |bench, rhs| {
                        let short = CnstShort::new_max();
                        bench.iter(|| short.$add(*rhs));
                    },
                );

                // Bench id format: Large ConstrainedIX/ux::MAX.
                group.bench_with_input(
                    BenchmarkId::new($crate::large!($Cnst), $crate::max!($UnsInt)),
                    &$UnsInt::MAX,
                    |bench, rhs| {
                        let large = CnstLarge::new_max();
                        bench.iter(|| large.$add(*rhs));
                    },
                );

                group.bench_with_input(
                    BenchmarkId::new(stringify!($SigInt), $crate::max!($UnsInt)),
                    &$UnsInt::MAX,
                    |bench, rhs| {
                        let prim = $SigInt::MAX;
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

// Generates wrapping_add_unsigned benches.
#[macro_export]
macro_rules! bench_wrapping_add_unsigned {
    ($({ $SigInt:ident, $UnsInt:ident, $int_mod:ident, $Cnst:ident }),+ $(,)*) => {
        $crate::__bench_add_unsigned! {
            $({ $SigInt, $UnsInt, $int_mod, $Cnst, wrapping_add_unsigned }),+
        }
    };
}

// Generates overflowing_add_unsigned benches.
#[macro_export]
macro_rules! bench_overflowing_add_unsigned {
    ($({ $SigInt:ident, $UnsInt:ident, $int_mod:ident, $Cnst:ident }),+ $(,)*) => {
        $crate::__bench_add_unsigned! {
            $({ $SigInt, $UnsInt, $int_mod, $Cnst, overflowing_add_unsigned }),+
        }
    };
}
