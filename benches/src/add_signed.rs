// Group benches for wrapping add APIs running against signed values.
#[macro_export]
#[doc(hidden)]
macro_rules! bench_add_signed {
    ($({ $Num:ident, $SigInt:ident, $num_mod:ident, $Cnst:ident, $add:ident }),+ $(,)*) => {
        use ::criterion::{criterion_group, criterion_main};

        $(
            mod $num_mod {
                $crate::overflowed_add_around_min! {
                    $Num, $SigInt, $num_mod, $Cnst, $add
                }

                $crate::overflowed_add_around_max! {
                    $Num, $SigInt, $num_mod, $Cnst, $add
                }
            }
        )+

        criterion_group! {
            benches,
            $(
                $num_mod::overflowed_add_around_min,
                $num_mod::overflowed_add_around_max,
            )+
        }

        criterion_main!(benches);
    };
}
