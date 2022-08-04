// Defines and implements `CnstGen`, used for `proptest` integration.
#[cfg(test)]
macro_rules! cnst_gen_def_impl {
    ($Int:ty, $int_md:ident, $Ty:ident, $mod_name:ident, $Gen:ident) => {
        #[cfg(test)]
        mod $mod_name {
            use ::core::ops::RangeInclusive;
            use ::proptest::num::$int_md::BinarySearch;
            use ::proptest::strategy::{NewTree, Strategy, ValueTree};
            use ::proptest::test_runner::TestRunner;
            use crate::$int_md::*;

            /// `CnstGen` is a `Constrained` type generator.
            // TODO: visibilty should be `pub(crate)`.
            #[derive(Clone, Copy, Debug)]
            pub struct $Gen<const MIN: $Int, const MAX: $Int, const DEF: $Int>;

            impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> $Gen<MIN, MAX, DEF> {
                const RANGE: RangeInclusive<$Int> = MIN..=MAX;
            }

            impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> Strategy
                for $Gen<MIN, MAX, DEF>
            {
                type Value = $Ty<MIN, MAX, DEF>;
                type Tree = CnstBinarySearch<MIN, MAX, DEF>;

                fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
                    Ok(CnstBinarySearch(Self::RANGE.new_tree(runner)?))
                }
            }

            // TODO: visibilty should be `pub(crate)`.
            #[derive(Clone, Copy, Debug)]
            pub struct CnstBinarySearch<const MIN: $Int, const MAX: $Int, const DEF: $Int>(
                BinarySearch,
            );

            impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> ValueTree
                for CnstBinarySearch<MIN, MAX, DEF>
            {
                type Value = $Ty<MIN, MAX, DEF>;

                fn current(&self) -> Self::Value {
                    $Ty::<MIN, MAX, DEF>::__new(self.0.current()).unwrap()
                }

                fn simplify(&mut self) -> bool {
                    self.0.simplify()
                }

                fn complicate(&mut self) -> bool {
                    self.0.complicate()
                }
            }
        }
    };
}

// Defines and implements `RhsGen` and `Rhs`, used for `proptest` integration.
#[cfg(test)]
macro_rules! rhs_gen_def_impl {
    ($Int:ty, $int_md:ident, $mod_name:ident, $Gen:ident, $Rhs:ident) => {
        #[cfg(test)]
        mod $mod_name {
            use ::core::ops::{Add, RangeInclusive, Sub};
            use ::proptest::num::$int_md::BinarySearch;
            use ::proptest::strategy::{NewTree, Strategy, ValueTree};
            use ::proptest::test_runner::TestRunner;

            /// `$Gen` is a `Rhs` type generator.
            // TODO: visibilty should be `pub(crate)`.
            #[derive(Clone, Copy, Debug)]
            pub struct $Gen<const S: $Int, const E: $Int>;

            impl<const S: $Int, const E: $Int> $Gen<S, E> {
                const RANGE: RangeInclusive<$Int> = S..=E;
            }

            impl<const S: $Int, const E: $Int> Strategy for $Gen<S, E> {
                type Value = $Rhs<S, E>;
                type Tree = RhsBinarySearch<S, E>;

                fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
                    Ok(RhsBinarySearch(Self::RANGE.new_tree(runner)?))
                }
            }

            // TODO: visibilty should be `pub(crate)`.
            #[derive(Clone, Copy, Debug)]
            pub struct RhsBinarySearch<const S: $Int, const E: $Int>(BinarySearch);

            impl<const S: $Int, const E: $Int> ValueTree for RhsBinarySearch<S, E> {
                type Value = $Rhs<S, E>;

                fn current(&self) -> Self::Value {
                    $Rhs::try_from(self.0.current()).unwrap()
                }

                fn simplify(&mut self) -> bool {
                    self.0.simplify()
                }

                fn complicate(&mut self) -> bool {
                    self.0.complicate()
                }
            }

            /// `Rhs` is a bounded `rhs` for arithmetics operations.
            // TODO: visibilty should be `pub(crate)`.
            #[derive(Clone, Copy, Debug)]
            pub struct $Rhs<const S: $Int, const E: $Int>($Int);

            impl<const S: $Int, const E: $Int> $Rhs<S, E> {
                // TODO: visibilty should be `pub(crate)`.
                #[must_use]
                #[inline(always)]
                pub const fn get(self) -> $Int {
                    self.0
                }
            }

            impl<const S: $Int, const E: $Int> Add<$Rhs<S, E>> for $Int {
                type Output = $Int;

                #[must_use]
                #[inline(always)]
                fn add(self, rhs: $Rhs<S, E>) -> Self::Output {
                    self + rhs.0
                }
            }

            impl<const S: $Int, const E: $Int> Sub<$Rhs<S, E>> for $Int {
                type Output = $Int;

                #[must_use]
                #[inline(always)]
                fn sub(self, rhs: $Rhs<S, E>) -> Self::Output {
                    self - rhs.0
                }
            }

            impl<const S: $Int, const E: $Int> Sub<$Int> for $Rhs<S, E> {
                type Output = $Int;

                #[must_use]
                #[inline(always)]
                fn sub(self, rhs: $Int) -> Self::Output {
                    self.0 - rhs
                }
            }

            impl<const S: $Int, const E: $Int> TryFrom<$Int> for $Rhs<S, E> {
                type Error = &'static str;

                fn try_from(num: $Int) -> Result<Self, Self::Error> {
                    if num >= S && num <= E {
                        Ok(Self(num))
                    } else {
                        Err("value is not contained within Rhs's range")
                    }
                }
            }
        }
    };
}

// Defines and implemets strategies for unsigned types.
#[cfg(test)]
macro_rules! strategies_uint_def_impl {
    ($({ $UnsInt:ty, $uint_md:ident, $Ty:ident }),+ $(,)*) => {$(
        #[cfg(test)]
        // TODO: visibility should be `pub(crate)`.
        pub mod $uint_md {
            cnst_gen_def_impl! {
                $UnsInt, $uint_md, $Ty, cnst, CnstGen
            }
            // TODO: visibility should be `pub(crate)`.
            pub use cnst::*;

            rhs_gen_def_impl! {
                $UnsInt, $uint_md, rhs, RhsGen, Rhs
            }
            // TODO: visibility should be `pub(crate)`.
            pub use rhs::*;
        }
    )+};
}

// Defines and implemets strategies for signed types.
#[cfg(test)]
macro_rules! strategies_int_def_impl {
    ($({ $SigInt:ty, $UnsInt:ty, $sint_md:ident, $Ty:ident }),+ $(,)*) => {$(
        #[cfg(test)]
        // TODO: visibility should be `pub(crate)`.
        pub mod $sint_md {
            cnst_gen_def_impl! {
                $SigInt, $sint_md, $Ty, cnst, CnstGen
            }
            // TODO: visibility should be `pub(crate)`.
            pub use cnst::*;

            rhs_gen_def_impl! {
                $SigInt, $sint_md, rhs, RhsGen, Rhs
            }
            // TODO: visibility should be `pub(crate)`.
            pub use rhs::*;

            impl<const S: $SigInt, const E: $SigInt> Rhs<S, E> {
                // TODO: visibility should be `pub(crate)`.
                pub const fn unsigned(self) -> $UnsInt {
                    self.get() as $UnsInt
                }
            }

            impl<const S: $SigInt, const E: $SigInt> ::core::ops::Neg for Rhs<S, E> {
                type Output = $SigInt;

                fn neg(self) -> Self::Output {
                    -self.get()
                }
            }
        }
    )+};
}
