#[macro_export]
macro_rules! bench_sub_signed {
    ($({ $Num:ident, $SigInt:ident, $num_mod:ident, $Cnst:ident, $sub:ident }),+ $(,)*) => {
        use ::criterion::{criterion_group, criterion_main};

        $(
            mod $num_mod {
                $crate::overflowing_sub_around_min! {
                    $Num, $SigInt, $num_mod, $Cnst, $sub
                }

                $crate::overflowing_sub_around_max! {
                    $Num, $SigInt, $num_mod, $Cnst, $sub
                }
            }
        )+

        criterion_group! {
            benches,
            $(
                $num_mod::overflowing_sub_around_min,
                $num_mod::overflowing_sub_around_max,
            )+
        }

        criterion_main!(benches);
    };
}
