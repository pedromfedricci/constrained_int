#[macro_export]
macro_rules! overflowing_sub_around_min {
    ($Num:ident, $Rhs:ident, $num_mod:ident, $Cnst:ident, $sub:ident) => {
        pub fn overflowing_sub_around_min(c: &mut ::criterion::Criterion) {
            use ::constrained_int::$num_mod::$Cnst;
            use ::criterion::BenchmarkId;

            type CnstShortMin = $Cnst<{ $Num::MIN }, { $Num::MIN + 1 }>;
            type CnstLargeMin = $Cnst<{ $Num::MIN }, { $Num::MAX - 1 }>;

            let mut group = c.benchmark_group($crate::overflowed!($Num, $sub));

            group.bench_with_input(
                BenchmarkId::new($crate::short!($Cnst), $crate::max!($Rhs)),
                &$Rhs::MAX,
                |bench, rhs| {
                    let short = CnstShortMin::new_min();
                    bench.iter(|| short.$sub(*rhs));
                },
            );

            group.bench_with_input(
                BenchmarkId::new($crate::large!($Cnst), $crate::max!($Rhs)),
                &$Rhs::MAX,
                |bench, rhs| {
                    let large = CnstLargeMin::new_min();
                    bench.iter(|| large.$sub(*rhs));
                },
            );

            group.bench_with_input(
                BenchmarkId::new(stringify!($Num), $crate::max!($Rhs)),
                &$Rhs::MAX,
                |bench, rhs| {
                    let prim = $Num::MIN;
                    bench.iter(|| prim.$sub(*rhs));
                },
            );

            group.finish();
        }
    };
}

#[macro_export]
macro_rules! overflowing_add_around_max {
    ($Num:ident, $Rhs:ident, $num_mod:ident, $Cnst:ident, $add:ident) => {
        pub fn overflowing_add_around_max(c: &mut ::criterion::Criterion) {
            use ::constrained_int::$num_mod::$Cnst;
            use ::criterion::BenchmarkId;

            type CnstShortMax = $Cnst<{ $Num::MAX - 1 }, { $Num::MAX }>;
            type CnstLargeMax = $Cnst<{ $Num::MIN + 1 }, { $Num::MAX }>;

            let mut group = c.benchmark_group($crate::overflowed!($Num, $add));

            group.bench_with_input(
                BenchmarkId::new($crate::short!($Cnst), $crate::max!($Rhs)),
                &$Rhs::MAX,
                |bench, rhs| {
                    let short = CnstShortMax::new_max();
                    bench.iter(|| short.$add(*rhs));
                },
            );

            group.bench_with_input(
                BenchmarkId::new($crate::large!($Cnst), $crate::max!($Rhs)),
                &$Rhs::MAX,
                |bench, rhs| {
                    let large = CnstLargeMax::new_max();
                    bench.iter(|| large.$add(*rhs));
                },
            );

            group.bench_with_input(
                BenchmarkId::new(stringify!($Num), $crate::max!($Rhs)),
                &$Rhs::MAX,
                |bench, rhs| {
                    let prim = $Num::MAX;
                    bench.iter(|| prim.$add(*rhs));
                },
            );

            group.finish();
        }
    };
}

#[macro_export]
macro_rules! overflowing_add_around_min {
    ($Num:ident, $Rhs:ident, $num_mod:ident, $Cnst:ident, $add:ident) => {
        pub fn overflowing_add_around_min(c: &mut ::criterion::Criterion) {
            use ::constrained_int::$num_mod::$Cnst;
            use ::criterion::BenchmarkId;

            type CnstShortMin = $Cnst<{ $Num::MIN }, { $Num::MIN + 1 }>;
            type CnstLargeMin = $Cnst<{ $Num::MIN }, { $Num::MAX - 1 }>;

            let mut group = c.benchmark_group($crate::overflowed!($Num, $add));

            group.bench_with_input(
                BenchmarkId::new($crate::short!($Cnst), $crate::min!($Rhs)),
                &$Rhs::MIN,
                |bench, rhs| {
                    let short = CnstShortMin::new_min();
                    bench.iter(|| short.$add(*rhs));
                },
            );

            group.bench_with_input(
                BenchmarkId::new($crate::large!($Cnst), $crate::min!($Rhs)),
                &$Rhs::MIN,
                |bench, rhs| {
                    let large = CnstLargeMin::new_min();
                    bench.iter(|| large.$add(*rhs));
                },
            );

            group.bench_with_input(
                BenchmarkId::new(stringify!($Num), $crate::min!($Rhs)),
                &$Rhs::MIN,
                |bench, rhs| {
                    let prim = $Num::MIN;
                    bench.iter(|| prim.$add(*rhs));
                },
            );

            group.finish();
        }
    };
}

#[macro_export]
macro_rules! overflowing_sub_around_max {
    ($Num:ident, $Rhs:ident, $num_mod:ident, $Cnst:ident, $sub:ident) => {
        pub fn overflowing_sub_around_max(c: &mut ::criterion::Criterion) {
            use ::constrained_int::$num_mod::$Cnst;
            use ::criterion::BenchmarkId;

            type CnstShortMax = $Cnst<{ $Num::MAX - 1 }, { $Num::MAX }>;
            type CnstLargeMax = $Cnst<{ $Num::MIN + 1 }, { $Num::MAX }>;

            let mut group = c.benchmark_group($crate::overflowed!($Num, $sub));

            group.bench_with_input(
                BenchmarkId::new($crate::short!($Cnst), $crate::min!($Rhs)),
                &$Rhs::MIN,
                |bench, rhs| {
                    let short = CnstShortMax::new_max();
                    bench.iter(|| short.$sub(*rhs));
                },
            );

            group.bench_with_input(
                BenchmarkId::new($crate::large!($Cnst), $crate::min!($Rhs)),
                &$Rhs::MIN,
                |bench, rhs| {
                    let large = CnstLargeMax::new_max();
                    bench.iter(|| large.$sub(*rhs));
                },
            );

            group.bench_with_input(
                BenchmarkId::new(stringify!($Num), $crate::min!($Rhs)),
                &$Rhs::MIN,
                |bench, rhs| {
                    let prim = $Num::MAX;
                    bench.iter(|| prim.$sub(*rhs));
                },
            );

            group.finish();
        }
    };
}
