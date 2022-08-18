// Group benches for wrapping add APIs running against unsigned values.
#[macro_export]
#[doc(hidden)]
macro_rules! bench_add_unsigned {
    ($({ $Num:ident, $UnsInt:ident, $num_mod:ident, $Cnst:ident, $add:ident }),+ $(,)*) => {
        use ::criterion::{criterion_group, criterion_main};

        $(
            mod $num_mod {
                $crate::overflowed_add_around_max! {
                    $Num, $UnsInt, $num_mod, $Cnst, $add
                }
            }
        )+

        criterion_group! {
            benches,
            $($num_mod::overflowed_add_around_max,)+
        }

        criterion_main!(benches);
    };
}
