// Group benches for wrapping sub APIs running against unsigned values.
#[macro_export]
#[doc(hidden)]
macro_rules! bench_sub_unsigned {
    ($({ $Num:ident, $UnsInt:ident, $num_mod:ident, $Cnst:ident, $sub:ident }),+ $(,)*) => {
        use ::criterion::{criterion_group, criterion_main};

        $(
            mod $num_mod {
                $crate::overflowed_sub_around_min! {
                    $Num, $UnsInt, $num_mod, $Cnst, $sub
                }
            }
        )+

        criterion_group! {
            benches,
            $($num_mod::overflowed_sub_around_min,)+
        }

        criterion_main!(benches);
    };
}
