// Defines and implements `CnstGen`, used for `proptest` integration.
#[cfg(test)]
macro_rules! cnst_gen_def_impl {
    ($Int:ty, $int_md:ident, $mod_name:ident, $ty_path:path, $Ty:ident) => {
        #[cfg(test)]
        mod $mod_name {
            use ::core::ops::RangeInclusive;
            use ::proptest::num::$int_md::BinarySearch;
            use ::proptest::strategy::{NewTree, Strategy, ValueTree};
            use ::proptest::test_runner::TestRunner;
            use $ty_path::*;

            /// `CnstGen` is a `Constrained` type generator.
            // TODO: visibilty should be pub(super) or pub(crate).
            #[derive(Clone, Copy, Debug)]
            pub struct CnstGen<const MIN: $Int, const MAX: $Int, const DEF: $Int>;

            impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> CnstGen<MIN, MAX, DEF> {
                const RANGE: RangeInclusive<$Int> = MIN..=MAX;
            }

            impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> Strategy
                for CnstGen<MIN, MAX, DEF>
            {
                type Value = $Ty<MIN, MAX, DEF>;
                type Tree = CnstBinarySearch<MIN, MAX, DEF>;

                fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
                    Ok(CnstBinarySearch(Self::RANGE.new_tree(runner)?))
                }
            }

            // TODO: visibilty should be private.
            #[derive(Clone, Copy, Debug)]
            pub struct CnstBinarySearch<const MIN: $Int, const MAX: $Int, const DEF: $Int>(
                BinarySearch,
            );

            impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> ValueTree
                for CnstBinarySearch<MIN, MAX, DEF>
            {
                type Value = $Ty<MIN, MAX, DEF>;

                fn current(&self) -> Self::Value {
                    $Ty::<MIN, MAX, DEF>::new_unguarded(self.0.current()).unwrap()
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
    ($({ $Int:ty, $int_md:ident, $mod_name:ident }),+ $(,)*) => {$(
        #[cfg(test)]
        mod $mod_name {
            use ::core::ops::{Add, RangeInclusive, Sub};
            use ::proptest::strategy::{NewTree, Strategy, ValueTree};
            use ::proptest::test_runner::TestRunner;
            use ::proptest::num::$int_md::BinarySearch;

            /// `RhsGen` is a `Rhs` type generator.
            // TODO: visibilty should be pub(super) or pub(crate).
            #[derive(Clone, Copy, Debug)]
            pub struct RhsGen<const S: $Int, const E: $Int>;

            impl<const S: $Int, const E: $Int> RhsGen<S, E> {
                const RANGE: RangeInclusive<$Int> = S..=E;
            }

            impl<const S: $Int, const E: $Int> Strategy for RhsGen<S, E> {
                type Value = Rhs<S, E>;
                type Tree = RhsBinarySearch<S, E>;

                fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
                    Ok(RhsBinarySearch(Self::RANGE.new_tree(runner)?))
                }
            }

            // TODO: visibilty should be private.
            #[derive(Clone, Copy, Debug)]
            pub struct RhsBinarySearch<const S: $Int, const E: $Int>(BinarySearch);

            impl<const S: $Int, const E: $Int> ValueTree for RhsBinarySearch<S, E> {
                type Value = Rhs<S, E>;

                fn current(&self) -> Self::Value {
                    Rhs::try_from(self.0.current()).unwrap()
                }

                fn simplify(&mut self) -> bool {
                    self.0.simplify()
                }

                fn complicate(&mut self) -> bool {
                    self.0.complicate()
                }
            }

            /// `Rhs` is a bounded `rhs` for arithmetics operations.
            // TODO: visibilty should be pub(super) or pub(crate).
            #[derive(Clone, Copy, Debug)]
            pub struct Rhs<const S: $Int, const E: $Int>($Int);

            impl<const S: $Int, const E: $Int> Rhs<S, E> {
                // TODO: visibilty should be pub(super) or pub(crate).
                #[must_use]
                #[inline(always)]
                pub fn get(self) -> $Int {
                    self.0
                }
            }

            impl<const S: $Int, const E: $Int> Add<Rhs<S, E>> for $Int {
                type Output = $Int;

                #[must_use]
                #[inline(always)]
                fn add(self, rhs: Rhs<S, E>) -> Self::Output {
                    self + rhs.0
                }
            }

            impl<const S: $Int, const E: $Int> Sub<Rhs<S, E>> for $Int {
                type Output = $Int;

                #[must_use]
                #[inline(always)]
                fn sub(self, rhs: Rhs<S, E>) -> Self::Output {
                    self - rhs.0
                }
            }

            impl<const S: $Int, const E: $Int> Sub<$Int> for Rhs<S, E> {
                type Output = $Int;

                #[must_use]
                #[inline(always)]
                fn sub(self, rhs: $Int) -> Self::Output {
                    self.0 - rhs
                }
            }

            impl<const S: $Int, const E: $Int> TryFrom<$Int> for Rhs<S, E> {
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
    )+};
}
