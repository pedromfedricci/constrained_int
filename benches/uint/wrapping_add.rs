#[path = "../macros.rs"]
#[macro_use]
mod macros;

macro_rules! bench_wrapping_add_for {
    ($({ $UnsInt:ident, $uint_mod:ident, $Cnst:ident, $bits:literal }),+ $(,)*) => {$(
        mod $uint_mod {
            use constrained_int::$uint_mod::$Cnst;
            use criterion::{BenchmarkId, Criterion};

            pub fn primitive_overflow(c: &mut Criterion) {
                type CnstShort = $Cnst<{ $UnsInt::MAX - 1 }, { $UnsInt::MAX }>;
                type CnstLarge = $Cnst<{ $UnsInt::MIN + 1 }, { $UnsInt::MAX }>;

                let mut group = c.benchmark_group(overflowed!($UnsInt, $bits, "wrapping_add"));

                // Bench id format: Short ConstrainedUX/ux::MAX.
                group.bench_with_input(
                    BenchmarkId::new(short!($Cnst), max!($UnsInt)),
                    &$UnsInt::MAX,
                    |bench, rhs| {
                        let short = CnstShort::new_max();
                        bench.iter(|| short.wrapping_add(*rhs));
                    },
                );

                // Bench id format: Large ConstrainedUX/ux::MAX.
                group.bench_with_input(
                    BenchmarkId::new(large!($Cnst), max!($UnsInt)),
                    &$UnsInt::MAX,
                    |bench, rhs| {
                        let large = CnstLarge::new_max();
                        bench.iter(|| large.wrapping_add(*rhs));
                    },
                );

                group.bench_with_input(
                    BenchmarkId::new(stringify!($UnsInt), max!($UnsInt)),
                    &$UnsInt::MAX,
                    |bench, rhs| {
                        let prim = $UnsInt::MAX;
                        bench.iter(|| prim.wrapping_add(*rhs));
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

#[cfg(cnst8bitonly)]
bench_wrapping_add_for! {
    { u8, u8, ConstrainedU8, "8-bit unsigned" },
}

#[cfg(not(cnst8bitonly))]
bench_wrapping_add_for! {
    { u8, u8, ConstrainedU8, "8-bit unsigned" },
    { u16, u16, ConstrainedU16, "16-bit unsigned" },
    { u32, u32, ConstrainedU32, "32-bit unsigned" },
    { u64, u64, ConstrainedU64, "64-bit unsigned" },
    { u128, u128, ConstrainedU128, "128-bit unsigned" },
    { usize, usize, ConstrainedUsize, "pointer-sized unsigned" }
}
