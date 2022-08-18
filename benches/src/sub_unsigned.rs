#[macro_export]
macro_rules! bench_sub_unsigned {
    ($({ $Num:ident, $UnsInt:ident, $num_mod:ident, $Cnst:ident, $sub:ident }),+ $(,)*) => {
        use ::criterion::{criterion_group, criterion_main};

        $(
            mod $num_mod {
                $crate::overflowing_sub_around_min! {
                    $Num, $UnsInt, $num_mod, $Cnst, $sub
                }
            }
        )+

        criterion_group! {
            benches,
            $($num_mod::overflowing_sub_around_min,)+
        }

        criterion_main!(benches);
    };
}
