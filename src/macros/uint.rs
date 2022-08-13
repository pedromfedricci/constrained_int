// API implementation and doc values specific to unsigned integers.
macro_rules! constrained_uint_impl {
    (   $UnsInt:ty, $SigInt:ty, $md:ident, $Ty:ident, $Err:ident,
        $MinErr:ident, $MaxErr:ident, $min:literal..=$max:literal
    ) => {
        impl<const MIN: $UnsInt, const MAX: $UnsInt, const DEF: $UnsInt> $Ty<MIN, MAX, DEF> {
            /// Saturating integer addition. Computes `self + rhs`, saturating the result
            /// at range's upper bound.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            /// let mut constrained = Constrained::new_max();
            /// // Saturates at upper bound.
            /// constrained = constrained.saturating_add(1);
            #[doc = concat!("assert_eq!(constrained.get(), ", stringify!($max), ");")]
            /// ```
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn saturating_add(self, rhs: $UnsInt) -> Self {
                // TODO: Can't use `unwrap_or` because it is not `const`.
                // Tracking issue: https://github.com/rust-lang/rust/issues/91930.
                match self.checked_add(rhs) {
                    Some(this) => this,
                    None => Self(MAX),
                }
            }

            /// Saturating addition with a signed integer.
            ///
            /// Computes `self + rhs`, saturating the result at the range's upper bound
            /// if the integer is positive, or at the range's lower bound if negative.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            /// let mut constrained = Constrained::new_min();
            /// // Saturates at upper bound.
            /// constrained = constrained.saturating_add_signed(-1);
            #[doc = concat!("assert_eq!(constrained.get(), ", stringify!($min), ");")]
            /// ```
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn saturating_add_signed(self, rhs: $SigInt) -> Self {
                let rhs = self.0.saturating_add_signed(rhs);
                Self::saturating_new_unguarded(rhs)
            }

            /// Saturating integer substraction. Computes `self - rhs`, saturating the
            /// result at range's lower bound.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            /// let mut constrained = Constrained::new_min();
            /// // Saturates at lower bound.
            /// constrained = constrained.saturating_sub(1);
            #[doc = concat!("assert_eq!(constrained.get(), ", stringify!($min), ");")]
            /// ```
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn saturating_sub(self, rhs: $UnsInt) -> Self {
                // TODO: Can't use `unwrap_or` because it is not `const`.
                // Tracking issue: https://github.com/rust-lang/rust/issues/91930.
                match self.checked_sub(rhs) {
                    Some(this) => this,
                    None => Self(MIN),
                }
            }

            /// Checked integer addition. Computes `self + rhs`, returning [`None`] if
            /// result is greater than range's upper bound.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            /// let constrained = Constrained::new_max();
            /// // Above upper bound.
            /// assert_eq!(constrained.checked_add(1), None);
            /// ```
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn checked_add(self, rhs: $UnsInt) -> Option<Self> {
                match self.0.checked_add(rhs) {
                    Some(value) if value <= MAX => Some(Self(value)),
                    _ => None,
                }
            }

            /// Checked addition with a signed integer. Computes `self + rhs`, returning
            /// [`None`] if result is out of the range's inclusive bounds.
            ///
            /// # Example
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            /// let constrained = Constrained::new_min();
            /// // Below lower bound.
            /// assert_eq!(constrained.checked_add_signed(-1), None);
            /// ```
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn checked_add_signed(self, rhs: $SigInt) -> Option<Self> {
                // TODO: Can't use `?` operator on const fn yet:
                // https://github.com/rust-lang/rust/issues/74935.
                match self.0.checked_add_signed(rhs) {
                    Some(value) => Self::checked_new_unguarded(value),
                    None => None,
                }
            }

            /// Checked integer substraction. Computes `self - rhs`, returning [`None`] if
            /// result is lower than range's lower bound.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            /// let constrained = Constrained::new_min();
            /// // Below lower bound.
            /// assert_eq!(constrained.checked_sub(1), None);
            /// ```
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn checked_sub(self, rhs: $UnsInt) -> Option<Self> {
                match self.0.checked_sub(rhs) {
                    Some(value) if value >= MIN => Some(Self(value)),
                    _ => None,
                }
            }

            /// Fallible integer addition. Computes `self + rhs`, returning an error if
            /// the result is greater than the range's upper bound.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            /// let constrained = Constrained::new_max();
            /// // Above upper bound.
            /// assert!(constrained.try_add(1).is_err());
            /// ```
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn try_add(self, rhs: $UnsInt) -> Result<Self, $MaxErr<MAX>> {
                // TODO: Can't use `ok_or` because it is not `const`.
                // Tracking issue: https://github.com/rust-lang/rust/issues/91930.
                match self.checked_add(rhs) {
                    Some(this) => Ok(this),
                    None => Err($MaxErr::<MAX>::new()),
                }
            }

            /// Fallible addition with signed integer. Computes `self + rhs`, returning
            /// an error if the result is out of the range's inclusive bounds.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            /// let constrained = Constrained::new_min();
            /// // Below upper bound.
            /// assert!(constrained.try_add_signed(-1).is_err());
            /// ```
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn try_add_signed(self, rhs: $SigInt) -> Result<Self, $Err<MIN, MAX>> {
                match self.checked_add_signed(rhs) {
                    Some(this) => Ok(this),
                    None if rhs.is_positive() => Err($Err::greater()),
                    None => Err($Err::lower()),
                }
            }

            /// Fallible integer substraction. Computes `self + rhs`, returning an error
            /// if the result is lower than the range's lower bound.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            /// let constrained = Constrained::new_min();
            /// // Below lower bound.
            /// assert!(constrained.try_sub(1).is_err());
            /// ```
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn try_sub(self, rhs: $UnsInt) -> Result<Self, $MinErr<MIN>> {
                // TODO: Can't use `ok_or` because it is not `const`.
                // Tracking issue: https://github.com/rust-lang/rust/issues/91930.
                match self.checked_sub(rhs) {
                    Some(this) => Ok(this),
                    None => Err($MinErr::<MIN>::new()),
                }
            }

            /// Wrapping (modular) addition. Computes `self + rhs`, wrapping around at
            /// the range's upper bound.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            /// let mut constrained = Constrained::new_max();
            /// // Wraps around the upper bound.
            /// constrained = constrained.wrapping_add(1);
            /// assert_eq!(constrained.get(), Constrained::MIN);
            /// ```
            #[must_use = "this returns the result of the operation, without modifyind the original"]
            pub const fn wrapping_add(mut self, mut rhs: $UnsInt) -> Self {
                (self, rhs) = match self.0.overflowing_add(rhs) {
                    (value, false) if value <= MAX => return Self(value),
                    (value, false) => return Self::wrap_around_max(value),
                    (wrapped, true) => Self::wrap_around_max_over(wrapped),
                };

                self.wrapping_add(rhs)
            }

            /// Wrapping (modular) addition with signed integer.
            ///
            /// Computes `self + rhs`, wrapping the result around the range's upper
            /// bound if the integer is positive, or at the range's lower bound if
            /// negative.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            /// let mut constrained = Constrained::new_min();
            /// // Wraps around the upper bound.
            /// constrained = constrained.wrapping_add_signed(-1);
            /// assert_eq!(constrained.get(), Constrained::MAX);
            /// ```
            #[must_use = "this returns the result of the operation, without modifyind the original"]
            pub const fn wrapping_add_signed(self, rhs: $SigInt) -> Self {
                match self.0.overflowing_add_signed(rhs) {
                    (value, false) if value >= MIN && value <= MAX => Self(value),
                    (value, false) if value > MAX => Self::wrap_around_max(value),
                    (value, false) => Self::wrap_around_min(value),
                    (wrapped, true) => Self::overflowed_add_signed(wrapped, rhs.is_positive()),
                }
            }

            /// Wrapping (modular) substraction. Computes `self - rhs`, wrapping around
            /// at the range's lower bound.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            /// let mut constrained = Constrained::new_min();
            /// // Wraps around the lower bound.
            /// constrained = constrained.wrapping_sub(1);
            /// assert_eq!(constrained.get(), Constrained::MAX);
            /// ```
            #[must_use = "this returns the result of the operation, without modifyind the original"]
            pub const fn wrapping_sub(mut self, mut rhs: $UnsInt) -> Self {
                (self, rhs) = match self.0.overflowing_sub(rhs) {
                    (value, false) if value >= MIN => return Self(value),
                    (value, false) => return Self::wrap_around_min(value),
                    (wrapped, true) => Self::wrap_around_min_over(wrapped),
                };

                self.wrapping_sub(rhs)
            }

            /// Wrapping (modular) addition, indicating if result was wrapped around.
            ///
            /// Computes `self + rhs`, wrapping the result around the range's upper
            /// bound. If a wrapping addition would have occurred, then the boolean
            /// is set to `true`, else to `false`.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            /// let mut constrained = Constrained::new_max();
            /// let mut wrapped: bool;
            ///
            /// // Wraps around the upper bound, the boolean is set to `true`.
            /// (constrained, wrapped) = constrained.overflowing_add(1);
            /// assert_eq!(constrained.get(), Constrained::MIN);
            /// assert_eq!(wrapped, true);
            /// ```
            #[must_use = "this returns the result of the operation, without modifyind the original"]
            pub const fn overflowing_add(self, rhs: $UnsInt) -> (Self, bool) {
                match self.0.overflowing_add(rhs) {
                    (value, false) if value <= MAX => (Self(value), false),
                    (value, false) => (Self::wrap_around_max(value), true),
                    (wrapped, true) => (Self::overflowed_add(wrapped), true)
                }
            }

            /// Wrapping (modular) addition with signed integer, indicating if result
            /// was wrapped around.
            ///
            /// Computes `self + rhs`, wrapping the result around the range's upper
            /// bound if the integer is positive, or at the range's lower bound if
            /// negative. If a wrapping addition would have occurred, then the boolean
            /// is set to `true`, else to `false`.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            /// let mut constrained = Constrained::new_min();
            /// let mut wrapped: bool;
            ///
            /// // Wraps around the lower bound, the boolean is set to `true`.
            /// (constrained, wrapped) = constrained.overflowing_add_signed(-1);
            /// assert_eq!(constrained.get(), Constrained::MAX);
            /// assert_eq!(wrapped, true);
            /// ```
            #[must_use = "this returns the result of the operation, without modifyind the original"]
            pub const fn overflowing_add_signed(self, rhs: $SigInt) -> (Self, bool) {
                match self.0.overflowing_add_signed(rhs) {
                    (value, false) if value >= MIN && value <= MAX => (Self(value), false),
                    (value, false) if value > MAX => (Self::wrap_around_max(value), true),
                    (value, false) => (Self::wrap_around_min(value), true),
                    (wrapped, true) => (Self::overflowed_add_signed(wrapped, rhs.is_positive()), true),
                }
            }

            /// Wrapping (modular) substraction, indicating if result was wrapped around.
            ///
            /// Computes `self - rhs`, wrapping the result around the range's lower
            /// bound. If a wrapping substraction would have occurred, then the boolean
            /// is set to `true`, else to `false`.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            /// let mut constrained = Constrained::new_min();
            /// let mut wrapped: bool;
            ///
            /// // Wraps around the lower bound, the boolean is set to `true`.
            /// (constrained, wrapped) = constrained.overflowing_sub(1);
            /// assert_eq!(constrained.get(), Constrained::MAX);
            /// assert_eq!(wrapped, true);
            /// ```
            #[must_use = "this returns the result of the operation, without modifyind the original"]
            pub const fn overflowing_sub(self, rhs: $UnsInt) -> (Self, bool) {
                match self.0.overflowing_sub(rhs) {
                    (value, false) if value >= MIN => (Self(value), false),
                    (value, false) => (Self::wrap_around_min(value), true),
                    (wrapped, true) => (Self::overflowed_sub(wrapped), true)
                }
            }

            /// Wraps the value around the range upper bound.
            ///
            /// Caller must ensure that `value` is greater than `MAX`, or else there will
            /// be an unexpected overflow.
            #[must_use]
            const fn wrap_around_max(mut value: $UnsInt) -> Self {
                debug_assert!(value > MAX, "value must be greater than `MAX`");
                // Can't overflow since `MIN + x % range_size()` is at most equal to `MAX`.
                value = MIN + Self::remainder(value - MAX - 1);
                Self(value)
            }

            /// Wraps the value around the range lower bound.
            ///
            /// Caller must ensure that `value` is lower than `MIN`, or else there will
            /// be an unexpected overflow.
            #[must_use]
            const fn wrap_around_min(mut value: $UnsInt) -> Self {
                debug_assert!(value < MIN, "value must be lower than `MIN`");
                // Can't overflow since `MAX - x % range_size()` is at least equal to `MIN`.
                value = MAX - Self::remainder(MIN - value - 1);
                Self(value)
            }

            /// Computes the value to wrap around the range's upper bound from the value
            /// returned by a overflowed unsigned operation.
            ///
            /// Wraps `<$UnsInt>::MAX` around the range's upper bound and returns the
            /// result as the first value. Computes the remaining from wrapped value
            /// integer and returns it as the second value.
            ///
            /// Caller must ensure that `value` is lower than `<$UnsInt>::MAX`, or else
            /// there will be an unexpected overflow.
            #[must_use]
            const fn wrap_around_max_over(value: $UnsInt) -> (Self, $UnsInt) {
                debug_assert!(value < <$UnsInt>::MAX,
                    concat!("value must be lower than ", stringify!($UnsInt), "::MAX")
                );
                // No conditional compilation based on constexpr evaluation yet.
                if <$UnsInt>::MAX > MAX {
                    (Self::wrap_around_max(<$UnsInt>::MAX), Self::remainder(value + 1))
                } else {
                    (Self(MIN), Self::remainder(value))
                }
            }

            /// Computes the value to wrap around the range's lower bound from the value
            /// returned by a overflowed unsigned operation.
            ///
            /// Wraps `<$UnsInt>::MIN` around the range's lower bound and returns the
            /// result as the first value. Computes the remaining from the wrapped value
            /// and returns it in the second position.
            ///
            /// Caller must ensure that `value` is greater than 0, or else there will be
            /// an unexpected overflow.
            #[must_use]
            const fn wrap_around_min_over(mut value: $UnsInt) -> (Self, $UnsInt) {
                debug_assert!(value > <$UnsInt>::MIN, "value must greater than 0");
                value = <$UnsInt>::MAX - value;
                // No conditional compilation based on constexpr evaluation yet.
                if <$UnsInt>::MIN < MIN {
                    (Self::wrap_around_min(<$UnsInt>::MIN), Self::remainder(value + 1))
                } else {
                    (Self(MAX), Self::remainder(value))
                }
            }

            /// Handles overflowed `overflowing_add` calls from inner integer.
            #[must_use]
            const fn overflowed_add(wrapped: $UnsInt) -> Self {
                let (this, rhs) = Self::wrap_around_max_over(wrapped);
                this.wrapping_add(rhs)
            }

            /// Handles overflowed `overflowing_sub` calls from inner integer.
            #[must_use]
            const fn overflowed_sub(wrapped: $UnsInt) -> Self {
                let (this, rhs) = Self::wrap_around_min_over(wrapped);
                this.wrapping_sub(rhs)
            }

            /// Handles overflowed `overflowing_add_signed` calls from inner integer.
            ///
            /// `wrapped`: overflowed integer.
            /// `is_pos`: must be `true` if `rhs` was positive, `false` otherwise.
            #[must_use]
            const fn overflowed_add_signed(wrapped: $UnsInt, is_pos: bool) -> Self {
                if is_pos {
                    Self::overflowed_add(wrapped)
                } else {
                    Self::overflowed_sub(wrapped)
                }
            }

            /// Computes the remainder of `value` by the range's size.
            #[must_use]
            const fn remainder(value: $UnsInt) -> $UnsInt {
                value % Self::range_size()
            }

            /// Returns the range size.
            #[must_use]
            const fn range_size() -> $UnsInt {
                // Can't overflow since construction is guarded against `MAX ==
                // <$UnsInt>::MAX` AND `MIN == <$UnsInt>::MIN` at the same time, and
                // `MIN` can't be greater than `MAX`.
                debug_assert!(guard_arithmetics::<MIN, MAX>(), "invalid range");
                MAX - MIN + 1
            }

            /// Returns the range size as $SigInt, truncating at <$SigInt>::MAX.
            #[cfg(test)]
            #[must_use]
            const fn range_size_signed() -> $SigInt {
                let range_size = Self::range_size();
                if range_size > <$SigInt>::MAX as $UnsInt {
                    <$SigInt>::MAX
                } else {
                    range_size as $SigInt
                }
            }
        }
    };
}

// Defines mods, containers, errors, impls, tests and default doc values for unsigned integers.
macro_rules! constrained_uint_def_impl {
    ($({ $UnsInt:ty, $SigInt:ty, $uint_md:ident, $sint_md:ident,
         $Ty:ident, $Err:ident, $MinErr:ident, $MaxErr:ident }),+ $(,)*
    ) => {$(
        #[doc = concat!("Container and Error types for a range constrained [`prim@", stringify!($UnsInt), "`].")]
        pub mod $uint_md {
            constrained_def_impl! {
            //  uint, uint_mod, TypeName, ErrorName, MinErrorName, MaxErrorName, min..=max, (min-1, max+1)
                $UnsInt, $uint_md, $Ty, $Err, $MinErr, $MaxErr, 1..=254, (0, 255)
            }

            constrained_uint_impl! {
            //  uint, int, uint_mod, TypeName, ErroName, MinErrorName, MaxErrorName, min..=max
                $UnsInt, $SigInt, $uint_md, $Ty, $Err, $MinErr, $MaxErr, 1..=254
            }

            #[cfg(test)]
            mod tests_uint_common {
                tests_common! {
                //  uint, ty_mod_path, TypeName, ErrorName, MinErrorName, MaxErrorName
                    $UnsInt, super, $Ty, $Err, $MinErr, $MaxErr
                }
            }

            #[cfg(test)]
            mod tests_uint_specific {
                tests_uint! {
                //  uint, sint, uint_mod, sint_mod, ty_mod_path, TypeName, ErrorName, MinErrorName, MaxErrorName
                    $UnsInt, $SigInt, $uint_md, $sint_md, super, $Ty, $Err, $MinErr, $MaxErr
                }
            }
        }
    )+};
}

// Implements all unsigned integer specific tests.
#[cfg(test)]
macro_rules! tests_uint {
    (   $UnsInt:ty, $SigInt:ty, $uint_md:ident, $sint_md:ident, $ty_path:path,
        $Ty:ident, $Err:ident, $MinErr:ident, $MaxErr:ident
    ) => {
        use crate::proptest::$sint_md::{Rhs as SigRhs, RhsGen as SigRhsGen};
        use crate::proptest::$uint_md::{CnstGen, Rhs as UnsRhs, RhsGen as UnsRhsGen};
        use ::core::fmt::Debug;
        use $ty_path::{$Err, $MaxErr, $MinErr, $Ty};

        fn assert_add_bounded<const MIN: $UnsInt, const MAX: $UnsInt, const DEF: $UnsInt>(
            rhs: UnsRhs<{ 0 }, { $Ty::<MIN, MAX, DEF>::range_size() - 1 }>,
            add: impl Fn($Ty<MIN, MAX, DEF>, $UnsInt) -> $Ty<MIN, MAX, DEF>,
        ) {
            let mut cnst = $Ty(MIN);
            cnst = add(cnst, rhs.get());
            assert_eq!(cnst.get(), MIN + rhs);
        }

        fn assert_sub_bounded<const MIN: $UnsInt, const MAX: $UnsInt, const DEF: $UnsInt>(
            rhs: UnsRhs<{ 0 }, { $Ty::<MIN, MAX, DEF>::range_size() - 1 }>,
            sub: impl Fn($Ty<MIN, MAX, DEF>, $UnsInt) -> $Ty<MIN, MAX, DEF>,
        ) {
            let mut cnst = $Ty(MAX);
            cnst = sub(cnst, rhs.get());
            assert_eq!(cnst.get(), MAX - rhs);
        }

        fn assert_unbounded<
            const MIN: $UnsInt,
            const MAX: $UnsInt,
            const DEF: $UnsInt,
            T: Eq + Debug,
        >(
            cnst: $Ty<MIN, MAX, DEF>,
            rhs: UnsRhs<{ $Ty::<MIN, MAX, DEF>::range_size() }, { <$UnsInt>::MAX }>,
            expected: T,
            op: impl Fn($Ty<MIN, MAX, DEF>, $UnsInt) -> T,
        ) {
            let returned = op(cnst, rhs.get());
            assert_eq!(returned, expected);
        }

        fn assert_wrapping_add_unbounded<
            const MIN: $UnsInt,
            const MAX: $UnsInt,
            const DEF: $UnsInt,
        >(
            rhs: UnsRhs<{ 1 }, { $Ty::<MIN, MAX, DEF>::range_size() }>,
            add: impl Fn($Ty<MIN, MAX, DEF>, $UnsInt) -> $Ty<MIN, MAX, DEF>,
        ) {
            let mut cnst = $Ty(MAX);
            cnst = add(cnst, rhs.get());
            assert_eq!(cnst.get(), MIN + (rhs - 1));
        }

        fn assert_wrapping_sub_unbounded<
            const MIN: $UnsInt,
            const MAX: $UnsInt,
            const DEF: $UnsInt,
        >(
            rhs: UnsRhs<{ 1 }, { $Ty::<MIN, MAX, DEF>::range_size() }>,
            sub: impl Fn($Ty<MIN, MAX, DEF>, $UnsInt) -> $Ty<MIN, MAX, DEF>,
        ) {
            let mut cnst = $Ty(MIN);
            cnst = sub(cnst, rhs.get());
            assert_eq!(cnst.get(), MAX - (rhs - 1));
        }

        fn assert_wrapping_range_size<
            const MIN: $UnsInt,
            const MAX: $UnsInt,
            const DEF: $UnsInt,
        >(
            mut cnst: $Ty<MIN, MAX, DEF>,
            wrap: impl Fn($Ty<MIN, MAX, DEF>, $UnsInt) -> $Ty<MIN, MAX, DEF>,
        ) {
            let inner = cnst.get();
            cnst = wrap(cnst, $Ty::<MIN, MAX, DEF>::range_size());
            assert_eq!(cnst.get(), inner);
        }

        fn assert_add_signed_bounded<const MIN: $UnsInt, const MAX: $UnsInt, const DEF: $UnsInt>(
            rhs: SigRhs<{ 0 }, { $Ty::<MIN, MAX, DEF>::range_size_signed() - 1 }>,
            add: impl Fn($Ty<MIN, MAX, DEF>, $SigInt) -> $Ty<MIN, MAX, DEF>,
        ) {
            let mut cnst = $Ty(MIN);
            cnst = add(cnst, rhs.get());
            assert_eq!(cnst.get(), MIN + rhs.unsigned());

            let mut cnst = $Ty(MAX);
            cnst = add(cnst, -rhs);
            assert_eq!(cnst.get(), MAX - rhs.unsigned());
        }

        fn assert_add_signed_unbounded<
            const MIN: $UnsInt,
            const MAX: $UnsInt,
            const DEF: $UnsInt,
            T: Eq + Debug,
        >(
            rhs: SigRhs<{ 1 }, { <$SigInt>::MAX }>,
            expected: (T, T),
            op: impl Fn($Ty<MIN, MAX, DEF>, $SigInt) -> T,
        ) {
            let cnst = $Ty(MAX);
            let returned = op(cnst, rhs.get());
            assert_eq!(returned, expected.0);

            let cnst = $Ty(MIN);
            let returned = op(cnst, -rhs);
            assert_eq!(returned, expected.1);
        }

        fn assert_wrapping_add_signed_unbounded<
            const MIN: $UnsInt,
            const MAX: $UnsInt,
            const DEF: $UnsInt,
        >(
            rhs: SigRhs<{ 1 }, { $Ty::<MIN, MAX, DEF>::range_size_signed() }>,
            add: impl Fn($Ty<MIN, MAX, DEF>, $SigInt) -> $Ty<MIN, MAX, DEF>,
        ) {
            let mut cnst = $Ty(MAX);
            cnst = add(cnst, rhs.get());
            assert_eq!(cnst.get(), MIN + (rhs.unsigned() - 1));

            let mut cnst = $Ty(MIN);
            cnst = add(cnst, -rhs);
            assert_eq!(cnst.get(), MAX - (rhs.unsigned() - 1));
        }

        // For testing purposes:
        //
        // Verify arithmetics for a range definition with the same lower bound as the
        // inner integer.
        type ConstrainedMin = $Ty<{ <$UnsInt>::MIN }, { <$UnsInt>::MAX - 1 }>;
        // Verify arithmetics for a range definition with the same upper bound as the
        // inner integer.
        type ConstrainedMax = $Ty<{ <$UnsInt>::MIN + 1 }, { <$UnsInt>::MAX }>;

        impl_uint_prop_tests_for! {
        //  { ErrorName, MinErrorName, MaxErrorName },
            { $Err, $MinErr, $MaxErr },
        //  { module_name, TypeName },+
            { min, ConstrainedMin },
            { max, ConstrainedMax },
        }
    };
}

// Implement property tests for arithemtic operations for a number of
// **parameterized** `Constrained` types, e.g: `ConstrainedU8<0, 100, 0>`;
#[cfg(test)]
macro_rules! impl_uint_prop_tests_for {
    ({ $Err:ident, $MinErr:ident, $MaxErr:ident }, $({ $md:ident, $Ty:ty }),+ $(,)*) => {$(
        #[cfg(test)]
        mod $md {
            use ::proptest::proptest;
            use super::*;

            type Cnst = $Ty;
            type MinErr = $MinErr<{ Cnst::MIN }>;
            type MaxErr = $MaxErr<{ Cnst::MAX }>;
            type CnstErr = $Err<{ Cnst::MIN }, { Cnst::MAX }>;

            proptest! {
                #[test]
                fn checked_add_bounded(rhs in UnsRhsGen) {
                    assert_add_bounded(rhs, |cnst: Cnst, rhs| {
                        cnst.checked_add(rhs).expect("expected `checked_add` to succeed")
                    });
                }

                #[test]
                fn try_add_bounded(rhs in UnsRhsGen) {
                    assert_add_bounded(rhs, |cnst: Cnst, rhs| {
                        cnst.try_add(rhs).expect("expected `try_add` to succeed")
                    });
                }

                #[test]
                fn saturating_add_bounded(rhs in UnsRhsGen) {
                    assert_add_bounded(rhs, Cnst::saturating_add);
                }

                #[test]
                fn wrapping_add_bounded(rhs in UnsRhsGen) {
                    assert_add_bounded(rhs, Cnst::wrapping_add);
                }

                #[test]
                fn overflowing_add_bounded(rhs in UnsRhsGen) {
                    assert_add_bounded(rhs, |cnst: Cnst, rhs| {
                        let (cnst, overflowed) = cnst.overflowing_add(rhs);
                        assert!(!overflowed, "expected `overflowing_add` to not overflow");
                        cnst
                    });
                }

                #[test]
                fn checked_add_signed_bounded(rhs in SigRhsGen) {
                    assert_add_signed_bounded(rhs, |cnst: Cnst, rhs| {
                        cnst.checked_add_signed(rhs).expect("expected `checked_add_signed` to succeed")
                    })
                }

                #[test]
                fn try_add_signed_bounded(rhs in SigRhsGen) {
                    assert_add_signed_bounded(rhs, |cnst: Cnst, rhs| {
                        cnst.try_add_signed(rhs).expect("expected `try_add_signed` to succeed")
                    })
                }

                #[test]
                fn saturating_add_signed_bounded(rhs in SigRhsGen) {
                    assert_add_signed_bounded(rhs, Cnst::saturating_add_signed);
                }

                #[test]
                fn wrapping_add_signed_bounded(rhs in SigRhsGen) {
                    assert_add_signed_bounded(rhs, Cnst::wrapping_add_signed);
                }

                #[test]
                fn overflowing_add_signed_bounded(rhs in SigRhsGen) {
                    assert_add_signed_bounded(rhs, |cnst: Cnst, rhs| {
                        let (cnst, overflowed) = cnst.overflowing_add_signed(rhs);
                        assert!(!overflowed, "expected `overflowing_add_signed` to not overflow");
                        cnst
                    });
                }

                #[test]
                fn checked_sub_bounded(rhs in UnsRhsGen) {
                    assert_sub_bounded(rhs, |cnst: Cnst, rhs| {
                        cnst.checked_sub(rhs).expect("expected `checked_sub` to succeed")
                    });
                }

                #[test]
                fn try_sub_bounded(rhs in UnsRhsGen) {
                    assert_sub_bounded(rhs, |cnst: Cnst, rhs| {
                        cnst.try_sub(rhs).expect("expected `try_sub` to succeed")
                    });
                }

                #[test]
                fn saturating_sub_bounded(rhs in UnsRhsGen) {
                    assert_sub_bounded(rhs, Cnst::saturating_sub);
                }

                #[test]
                fn wrapping_sub_bounded(rhs in UnsRhsGen) {
                    assert_sub_bounded(rhs, Cnst::wrapping_sub);
                }

                #[test]
                fn overflowing_sub_bounded(rhs in UnsRhsGen) {
                    assert_sub_bounded(rhs, |cnst: Cnst, rhs| {
                        let (cnst, overflowed) = cnst.overflowing_sub(rhs);
                        assert!(!overflowed, "expected `overflowing_sub` to not overflow");
                        cnst
                    });
                }

                #[test]
                fn checked_add_unbounded((cnst, rhs) in (CnstGen, UnsRhsGen)) {
                    assert_unbounded(cnst, rhs, None, Cnst::checked_add);
                }

                #[test]
                fn try_add_unbounded((cnst, rhs) in (CnstGen, UnsRhsGen)) {
                    assert_unbounded(cnst, rhs, Err(MaxErr::new()), Cnst::try_add);
                }

                #[test]
                fn saturating_add_unbounded((cnst, rhs) in (CnstGen, UnsRhsGen)) {
                    assert_unbounded(cnst, rhs, Cnst::new_max(), Cnst::saturating_add);
                }

                #[test]
                fn checked_sub_unbounded((cnst, rhs) in (CnstGen, UnsRhsGen)) {
                    assert_unbounded(cnst, rhs, None, Cnst::checked_sub);
                }

                #[test]
                fn try_sub_unbounded((cnst, rhs) in (CnstGen, UnsRhsGen)) {
                    assert_unbounded(cnst, rhs, Err(MinErr::new()), Cnst::try_sub);
                }

                #[test]
                fn saturating_sub_unbounded((cnst, rhs) in (CnstGen, UnsRhsGen)) {
                    assert_unbounded(cnst, rhs, Cnst::new_min(), Cnst::saturating_sub);
                }

                #[test]
                fn checked_add_signed_unbounded(rhs in SigRhsGen) {
                    assert_add_signed_unbounded(rhs, (None, None), Cnst::checked_add_signed);
                }

                #[test]
                fn try_add_signed_unbounded(rhs in SigRhsGen) {
                    assert_add_signed_unbounded(
                        rhs,
                        (Err(CnstErr::greater()), Err(CnstErr::lower())),
                        Cnst::try_add_signed
                    );
                }

                #[test]
                fn saturating_add_signed_unbounded(rhs in SigRhsGen) {
                    assert_add_signed_unbounded(
                        rhs,
                        (Cnst::new_max(), Cnst::new_min()),
                        Cnst::saturating_add_signed
                    );
                }

                #[test]
                fn wrapping_add_unbounded(rhs in UnsRhsGen) {
                    assert_wrapping_add_unbounded(rhs, Cnst::wrapping_add);
                }

                #[test]
                fn overflowing_add_unbounded(rhs in UnsRhsGen) {
                    assert_wrapping_add_unbounded(rhs, |cnst: Cnst, rhs| {
                        let (cnst, overflowed) = cnst.overflowing_add(rhs);
                        assert!(overflowed, "expected `overflowing_add` to overflow");
                        cnst
                    });
                }

                #[test]
                fn wrapping_add_signed_unbounded(rhs in SigRhsGen) {
                    assert_wrapping_add_signed_unbounded(rhs, Cnst::wrapping_add_signed);
                }

                #[test]
                fn overflowing_add_signed_unbounded(rhs in SigRhsGen) {
                    assert_wrapping_add_signed_unbounded(rhs, |cnst: Cnst, rhs| {
                        let (cnst, overflowed) = cnst.overflowing_add_signed(rhs);
                        assert!(overflowed, "expected `overflowing_add_signed` to overflow");
                        cnst
                    });
                }

                #[test]
                fn wrapping_sub_unbounded(rhs in UnsRhsGen) {
                    assert_wrapping_sub_unbounded(rhs, Cnst::wrapping_sub);
                }

                #[test]
                fn overflowing_sub_unbounded(rhs in UnsRhsGen) {
                    assert_wrapping_sub_unbounded(rhs, |cnst: Cnst, rhs| {
                        let (cnst, overflowed) = cnst.overflowing_sub(rhs);
                        assert!(overflowed, "expected `overflowing_sub` to overflow");
                        cnst
                    });
                }

                #[test]
                fn wrapping_add_range_size(cnst in CnstGen) {
                    assert_wrapping_range_size(cnst, Cnst::wrapping_add);
                }

                #[test]
                fn overflowing_add_range_size(cnst in CnstGen) {
                    assert_wrapping_range_size(cnst, |cnst: Cnst, rsize| {
                        let (cnst, overflowed) = cnst.overflowing_add(rsize);
                        assert!(overflowed, "expected `overflowing_add` to overflow");
                        cnst
                    });
                }

                #[test]
                fn wrapping_sub_range_size(cnst in CnstGen) {
                    assert_wrapping_range_size(cnst, Cnst::wrapping_sub);
                }

                #[test]
                fn overflowing_sub_range_size(cnst in CnstGen) {
                    assert_wrapping_range_size(cnst, |cnst: Cnst, rsize| {
                        let (cnst, overflowed) = cnst.overflowing_sub(rsize);
                        assert!(overflowed, "expected `overflowing_sub` to overflow");
                        cnst
                    });
                }
            }
        }
    )+};
}
