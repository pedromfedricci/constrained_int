// API implementation and doc values especific to signed integers.
macro_rules! constrained_int_impl {
    ($SigInt:ty, $UnsInt:ty, $md:ident, $Ty:ident, $Err:ident, $min:literal..=$max:literal) => {
        impl<const MIN: $SigInt, const MAX: $SigInt, const DEF: $SigInt> $Ty<MIN, MAX, DEF> {
            /// Saturating integer addition. Computes self + rhs, saturating the result
            /// at either of the range bounds.
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
            ///
            /// constrained = Constrained::new_min();
            /// // Saturates at lower bound.
            /// constrained = constrained.saturating_add(-1);
            #[doc = concat!("assert_eq!(constrained.get(), ", stringify!($min), ");")]
            /// ```
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn saturating_add(self, mut rhs: $SigInt) -> Self {
                rhs = self.0.saturating_add(rhs);
                Self::saturating_new_unguarded(rhs)
            }

            /// Saturating integer substraction. Computes self - rhs, saturating the
            /// result at either of the range bounds.
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
            ///
            /// constrained = Constrained::new_max();
            /// // Saturates at upper bound.
            /// constrained = constrained.saturating_sub(-1);
            #[doc = concat!("assert_eq!(constrained.get(), ", stringify!($max), ");")]
            /// ```
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn saturating_sub(self, mut rhs: $SigInt) -> Self {
                rhs = self.0.saturating_sub(rhs);
                Self::saturating_new_unguarded(rhs)
            }

            /// Checked integer addition. Computes self + rhs, returning [`None`] if
            /// result is out of the range's inclusive bounds.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            /// let mut constrained = Constrained::new_max();
            /// // Above upper bound.
            /// assert_eq!(constrained.checked_add(1), None);
            ///
            /// constrained = Constrained::new_min();
            /// // Below lower bound.
            /// assert_eq!(constrained.checked_add(-1), None);
            /// ```
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn checked_add(self, rhs: $SigInt) -> Option<Self> {
                // Can't use `?` operator on const fn yet:
                // https://github.com/rust-lang/rust/issues/74935.
                match self.0.checked_add(rhs) {
                    Some(value) => Self::checked_new_unguarded(value),
                    None => None,
                }
            }

            /// Checked integer substraction. Computes self - rhs, returning [`None`] if
            /// result is out of the range's inclusive bounds.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            /// let mut constrained = Constrained::new_min();
            /// // Below lower bound.
            /// assert_eq!(constrained.checked_sub(1), None);
            ///
            /// let mut constrained = Constrained::new_max();
            /// // Above upper bound.
            /// assert_eq!(constrained.checked_sub(-1), None);
            /// ```
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn checked_sub(self, rhs: $SigInt) -> Option<Self> {
                // Can't use `?` operator on const fn yet:
                // https://github.com/rust-lang/rust/issues/74935.
                match self.0.checked_sub(rhs) {
                    Some(value) => Self::checked_new_unguarded(value),
                    None => None,
                }
            }

            /// Fallible integer addition. Computes `self + rhs`, returning an error
            /// if the result is out of the range's inclusive bounds.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            /// let mut constrained = Constrained::new_max();
            /// // Above upper bound.
            /// assert!(constrained.try_add(1).is_err());
            ///
            /// constrained = Constrained::new_min();
            /// // Below lower bound.
            /// assert!(constrained.try_add(-1).is_err());
            /// ```
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn try_add(self, rhs: $SigInt) -> Result<Self, $Err<MIN, MAX>> {
                match self.checked_add(rhs) {
                    Some(this) => Ok(this),
                    None if rhs.is_positive() => Err($Err::greater()),
                    None => Err($Err::lower()),
                }
            }

            /// Fallible integer substraction. Computes `self + rhs`, returning an error
            /// if the result is out of the range's inclusive bounds.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            /// let mut constrained = Constrained::new_min();
            /// // Below lower bound.
            /// assert!(constrained.try_sub(1).is_err());
            ///
            /// constrained = Constrained::new_max();
            /// // Above upper bound.
            /// assert!(constrained.try_sub(-1).is_err());
            /// ```
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn try_sub(self, rhs: $SigInt) -> Result<Self, $Err<MIN, MAX>> {
                match self.checked_sub(rhs) {
                    Some(this) => Ok(this),
                    None if rhs.is_positive() => Err($Err::lower()),
                    None => Err($Err::greater()),
                }
            }

            /// Wrapping (modular) addition. Computes `self + rhs`, wrapping around at
            /// the boundary of the range.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            /// let mut constrained = Constrained::new_max();
            /// // Wraps around upper bound.
            /// constrained = constrained.wrapping_add(1);
            /// assert_eq!(constrained.get(), Constrained::MIN);
            ///
            /// constrained = Constrained::new_min();
            /// // Wraps around lower bound.
            /// constrained = constrained.wrapping_add(-1);
            /// assert_eq!(constrained.get(), Constrained::MAX);
            /// ```
            #[must_use = "this returns the result of the operation, without modifyind the original"]
            pub const fn wrapping_add(mut self, mut rhs: $SigInt) -> Self {
                (self, rhs) = match self.0.overflowing_add(rhs) {
                    (value, false) if value >= MIN && value <= MAX => return Self(value),
                    (value, false) if value > MAX => return Self::wrap_around_max(value),
                    (value, false) => return Self::wrap_around_min(value),
                    (wrapped, true) => Self::overflowed_add(wrapped, rhs.is_positive()),
                };

                self.wrapping_add(rhs)
            }

            /// Wrapping (modular) substraction. Computes `self - rhs`, wrapping around
            /// at the boundary of the range.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            /// let mut constrained = Constrained::new_min();
            /// // Wraps around lower bound.
            /// constrained = constrained.wrapping_sub(1);
            /// assert_eq!(constrained.get(), Constrained::MAX);
            ///
            /// constrained = Constrained::new_max();
            /// // Wraps around upper bound.
            /// constrained = constrained.wrapping_sub(-1);
            /// assert_eq!(constrained.get(), Constrained::MIN);
            /// ```
            #[must_use = "this returns the result of the operation, without modifyind the original"]
            pub const fn wrapping_sub(mut self, mut rhs: $SigInt) -> Self {
                (self, rhs) = match self.0.overflowing_sub(rhs) {
                    (value, false) if value >= MIN && value <= MAX => return Self(value),
                    (value, false) if value < MIN => return Self::wrap_around_min(value),
                    (value, false) => return Self::wrap_around_max(value),
                    (wrapped, true) => Self::overflowed_sub(wrapped, rhs.is_positive()),
                };

                self.wrapping_sub(rhs)
            }

            /// Calculates `self` + `rhs`, indicating if result was wrapped around.
            ///
            /// Returns a tuple of the addition along with a boolean indicating whether
            /// the result was wrapped around either of the range bounds. If a range
            /// overflow would have occurred then the wrapped value is returned.
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
            /// (constrained, wrapped) = constrained.overflowing_add(2);
            /// assert_eq!(constrained.get(), Constrained::MIN + 1);
            /// assert_eq!(wrapped, true);
            ///
            /// // Addition is within range bounds, the boolean is set to `false`.
            /// (constrained, wrapped) = constrained.overflowing_add(-1);
            /// assert_eq!(constrained.get(), Constrained::MIN);
            /// assert_eq!(wrapped, false);
            /// ```
            #[must_use = "this returns the result of the operation, without modifyind the original"]
            pub const fn overflowing_add(mut self, mut rhs: $SigInt) -> (Self, bool) {
                match self.0.overflowing_add(rhs) {
                    (value, false) if value >= MIN && value <= MAX => (Self(value), false),
                    (value, false) if value > MAX => (Self::wrap_around_max(value), true),
                    (value, false) => (Self::wrap_around_min(value), true),
                    (wrapped, true) => {
                        (self, rhs) = Self::overflowed_add(wrapped, rhs.is_positive());
                        (self.wrapping_add(rhs), true)
                    }
                }
            }

            /// Calculates `self` - `rhs`, indicating if result was wrapped around.
            ///
            /// Returns a tuple of the substraction along with a boolean indicating
            /// whether the result was wrapped around either of the range bounds. If a
            /// range overflow would have occurred then the wrapped value is returned.
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
            /// (constrained, wrapped) = constrained.overflowing_sub(2);
            /// assert_eq!(constrained.get(), Constrained::MAX - 1);
            /// assert_eq!(wrapped, true);
            ///
            /// // Substraction is within range bounds, the boolean is set to `false`.
            /// (constrained, wrapped) = constrained.overflowing_sub(-1);
            /// assert_eq!(constrained.get(), Constrained::MAX);
            /// assert_eq!(wrapped, false);
            /// ```
            #[must_use = "this returns the result of the operation, without modifyind the original"]
            pub const fn overflowing_sub(mut self, mut rhs: $SigInt) -> (Self, bool) {
                match self.0.overflowing_sub(rhs) {
                    (value, false) if value >= MIN && value <= MAX => (Self(value), false),
                    (value, false) if value < MIN => (Self::wrap_around_min(value), true),
                    (value, false) => (Self::wrap_around_max(value), true),
                    (wrapped, true) => {
                        (self, rhs) = Self::overflowed_sub(wrapped, rhs.is_positive());
                        (self.wrapping_sub(rhs), true)
                    }
                }
            }

            /// Returns a number representing sign of `self`.
            ///
            ///  - `0` if the number is zero
            ///  - `1` if the number is positive
            ///  - `-1` if the number is negative
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            /// let mut constrained = Constrained::new_min();
            /// assert_eq!(constrained.signum(), -1);
            ///
            /// constrained = Constrained::new_max();
            /// assert_eq!(constrained.signum(), 1);
            ///
            /// constrained.set(0).unwrap();
            /// assert_eq!(constrained.signum(), 0);
            /// ```
            #[must_use = "this returns the result of the operation, without modifyind the original"]
            #[inline(always)]
            pub const fn signum(self) -> $SigInt {
                self.0.signum()
            }

            /// Returns `true` if `self` is negative and `false` if the number is zero or
            /// positive.
            ///
            /// # Example
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            /// let mut constrained = Constrained::new_min();
            /// assert!(constrained.is_negative());
            /// ```
            #[inline(always)]
            pub const fn is_negative(self) -> bool {
                self.0.is_negative()
            }

            /// Returns `true` if `self` is positive and `false` if the number is zero or
            /// negative.
            ///
            /// # Example
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            /// let mut constrained = Constrained::new_max();
            /// assert!(constrained.is_positive());
            /// ```
            #[inline(always)]
            pub const fn is_positive(self) -> bool {
                self.0.is_positive()
            }

            #[doc = concat!("Checked absolute value. Computes `", stringify!($SigInt), "::abs()`, ")]
            /// returning None if it's greater than `MAX`.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<-10, 8>;")]
            /// let mut constrained = Constrained::new(-5).unwrap();
            /// // Lower than `MAX`.
            /// constrained = constrained.checked_abs().unwrap();
            /// assert_eq!(constrained.get(), 5);
            ///
            /// // Greater than `MAX`.
            /// constrained.set(-9).unwrap();
            /// assert_eq!(constrained.checked_abs(), None);
            /// ```
            #[must_use = "this returns the result of the operation, without modifyind the original"]
            pub const fn checked_abs(self) -> Option<Self> {
                match self.0.checked_abs() {
                    Some(value) if value <= MAX => Some(Self(value)),
                    _ => None,
                }
            }

            /// Wraps the value around the range upper bound.
            ///
            /// Caller must ensure that `value` is greater from `MAX`, or else there will
            /// be an unexpected overflow.
            #[must_use]
            const fn wrap_around_max(mut value: $SigInt) -> Self {
                debug_assert!(value > MAX, "value must be greater than `MAX`");
                let offset = (<$SigInt>::abs_diff(MAX, value) - 1) % Self::range_size();
                // Can't overflow since `MIN + x % range_size()` is at most equal to `MAX`.
                value = MIN + offset as $SigInt;
                Self(value)
            }

            /// Wraps the value around the range lower bound.
            ///
            /// Caller must ensure that `value` is lower from `MIN`, or else there will
            /// be an unexpected overflow.
            #[must_use]
            const fn wrap_around_min(mut value: $SigInt) -> Self {
                debug_assert!(value < MIN, "value must be lower than `MIN`");
                let offset = (<$SigInt>::abs_diff(MIN, value) - 1) % Self::range_size();
                // Can't overflow since `MAX - x % range_size()` is at least equal to `MIN`.
                value = MAX - offset as $SigInt;
                Self(value)
            }

            /// Computes the value to wrap around the range upper bound from a integer
            /// overflow.
            ///
            /// Wraps `<$SigInt>::MAX` around the range upper bound and returns the result
            /// as the first value. Computes the remaining from provided overflowed
            /// integer and returns it as the second value.
            ///
            /// Caller must ensure that `value` is lower than `<$SigInt>::MAX`, or else
            /// there will be an unexpected overflow.
            #[must_use]
            const fn wrap_around_max_over(mut value: $SigInt) -> (Self, $SigInt) {
                debug_assert!(value < <$SigInt>::MAX, "value must lower than <$SigInt>::MAX");
                value = <$SigInt>::abs_diff(<$SigInt>::MIN, value) as $SigInt;
                // No conditional compilation based on constexpr evaluation unfortunately.
                if <$SigInt>::MAX > MAX {
                    (Self::wrap_around_max(<$SigInt>::MAX), value + 1)
                } else {
                    (Self(MIN), value)
                }
            }

            /// Computes the value to wrap around the range lower bound from a integer
            /// overflow.
            ///
            /// Wraps `<$SigInt>::MIN` around the range lower bound and returns the result
            /// as the first value. Computes the remaining from provided overflowed
            /// integer and returns it as the second value.
            ///
            /// Caller must ensure that `value` is greater than `<$SigInt>::MIN`, or else
            /// there will be an unexpected overflow.
            #[must_use]
            const fn wrap_around_min_over(mut value: $SigInt) -> (Self, $SigInt) {
                debug_assert!(value > <$SigInt>::MIN, "value must greater than <$SigInt>::MIN");
                value = <$SigInt>::abs_diff(<$SigInt>::MAX, value) as $SigInt;
                // No conditional compilation based on constexpr evaluation unfortunately.
                if <$SigInt>::MIN < MIN {
                    (Self::wrap_around_min(<$SigInt>::MIN), value + 1)
                } else {
                    (Self(MAX), value)
                }
            }

            /// Returns `Self` and `rhs` for next `wrapping_add` call.
            ///
            /// `wrapped`: overflowed integer.
            /// `is_pos`: must be `true` if `rhs` was positive, `false` otherwise.
            #[must_use]
            const fn overflowed_add(wrapped: $SigInt, is_pos: bool) -> (Self, $SigInt) {
                if is_pos {
                    Self::wrap_around_max_over(wrapped)
                } else {
                    let (this, rhs) = Self::wrap_around_min_over(wrapped);
                    (this, -rhs)
                }
            }

            /// Returns `Self` and `rhs` for next `wrapping_sub` call.
            ///
            /// `wrapped`: overflowed integer.
            /// `is_pos`: must be `true` if `rhs` was positive, `false` otherwise.
            #[must_use]
            const fn overflowed_sub(wrapped: $SigInt, is_pos: bool) -> (Self, $SigInt) {
                if is_pos {
                    Self::wrap_around_min_over(wrapped)
                } else {
                    let (this, rhs) = Self::wrap_around_max_over(wrapped);
                    (this, -rhs)
                }
            }

            /// Returns the range size.
            #[must_use]
            const fn range_size() -> $UnsInt {
                // Can't overflow since construction is guarded against `MAX ==
                // <$SigInt>::MAX` AND `MIN == <$SigInt>::MIN` at the same time,
                // and `MIN` can't be greater than `MAX`.
                debug_assert!(guard_arithmetics::<MIN, MAX>(), "invalid range");
                <$SigInt>::abs_diff(MIN, MAX) + 1
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

// Defines mods, containers, errors and impls, tests and default doc values for unsigned integers.
macro_rules! constrained_int_def_impl {
    ($({ $SigInt:ty, $UnsInt:ty, $sint_md:ident, $uint_md:ident,
        $Ty:ident, $Err:ident, $MinErr:ident, $MaxErr:ident }),+ $(,)*) =>
    {$(
        #[doc = concat!("Container and Error types for a range constrained [`prim@", stringify!($SigInt), "`].")]
        pub mod $sint_md {
            constrained_def_impl! {
            //  sint, sint_mod, TypeName, ErrorName, MinErrorName, MaxErrorName, min..=max, (min-1, max+1)
                $SigInt, $sint_md, $Ty, $Err, $MinErr, $MaxErr, -127..=126, (-128, 127)
            }

            constrained_int_impl! {
            //  sint, uint, sint_mod, TypeName, ErrorName, min..=max
                $SigInt, $UnsInt, $sint_md, $Ty, $Err, -127..=126
            }

            #[cfg(test)]
            mod tests_int_common {
                tests_common! {
                //  sint, ty_mod_path, TypeName, ErrorName, MinErrorName, MaxErrorName
                    $SigInt, super, $Ty, $Err, $MinErr, $MaxErr
                }
            }

            #[cfg(test)]
            mod tests_int_specific {
                tests_int! {
                //  sint, uint, sint_mod, uint_mod, ty_mod_path, TypeName, ErrorName
                    $SigInt, $UnsInt, $sint_md, $uint_md, super, $Ty, $Err
                }
            }
        }
    )+};
}

// Implements all signed integer specific tests.
#[cfg(test)]
macro_rules! tests_int {
    ($SigInt:ty, $UnsInt:ty, $sint_md:ident, $uint_md:ident, $ty_path:path, $Ty:ident, $Err:ident) => {
        use ::core::fmt::Debug;
        use $ty_path::{$Err, $Ty};

        rhs_gen_def_impl! { { $SigInt, $sint_md, sig_rhs }, }
        use sig_rhs::{Rhs as SigRhs, RhsGen as SigRhsGen};

        #[test]
        fn signum() {
            let mut constrained: $Ty<-1, 1, 0>;

            constrained = $Ty::default();
            assert_eq!(constrained.signum(), 0);

            constrained = $Ty::new_min();
            assert_eq!(constrained.signum(), -1);

            constrained = $Ty::new_max();
            assert_eq!(constrained.signum(), 1);
        }

        #[test]
        fn is_positive() {
            let mut constrained: $Ty<-1, 1, 0>;

            constrained = $Ty::default();
            assert!(!constrained.is_positive());

            constrained = $Ty::new_min();
            assert!(!constrained.is_positive());

            constrained = $Ty::new_max();
            assert!(constrained.is_positive());
        }

        #[test]
        fn is_negative() {
            let mut constrained: $Ty<-1, 1, 0>;

            constrained = $Ty::default();
            assert!(!constrained.is_negative());

            constrained = $Ty::new_min();
            assert!(constrained.is_negative());

            constrained = $Ty::new_max();
            assert!(!constrained.is_negative());
        }

        #[test]
        fn checked_abs() {
            let constrained = $Ty::<{ <$SigInt>::MIN }, 1>::new_min();
            assert_eq!(constrained.checked_abs(), None);

            let constrained = $Ty::<-1, 1>::new_min();
            assert_eq!(constrained.checked_abs(), Some($Ty(1)));

            let constrained = $Ty::<-2, 1>::new_min();
            assert_eq!(constrained.checked_abs(), None);
        }

        #[cfg(test)]
        impl<const S: $SigInt, const E: $SigInt> ::core::ops::Neg for SigRhs<S, E> {
            type Output = $SigInt;

            fn neg(self) -> Self::Output {
                -self.get()
            }
        }

        fn assert_add_bounded<const MIN: $SigInt, const MAX: $SigInt, const DEF: $SigInt>(
            rhs: SigRhs<{ 0 }, { $Ty::<MIN, MAX, DEF>::range_size_signed() - 1 }>,
            add: impl Fn($Ty<MIN, MAX, DEF>, $SigInt) -> $Ty<MIN, MAX, DEF>,
        ) {
            let mut cnst = $Ty(MIN);
            cnst = add(cnst, rhs.get());
            assert_eq!(cnst.get(), MIN + rhs);

            let mut cnst = $Ty(MAX);
            cnst = add(cnst, -rhs);
            assert_eq!(cnst.get(), MAX - rhs);
        }

        fn assert_sub_bounded<const MIN: $SigInt, const MAX: $SigInt, const DEF: $SigInt>(
            rhs: SigRhs<{ 0 }, { $Ty::<MIN, MAX, DEF>::range_size_signed() - 1 }>,
            sub: impl Fn($Ty<MIN, MAX, DEF>, $SigInt) -> $Ty<MIN, MAX, DEF>,
        ) {
            let mut cnst = $Ty(MIN);
            cnst = sub(cnst, -rhs);
            assert_eq!(cnst.get(), MIN + rhs);

            let mut cnst = $Ty(MAX);
            cnst = sub(cnst, rhs.get());
            assert_eq!(cnst.get(), MAX - rhs);
        }

        fn assert_add_unbounded<
            const MIN: $SigInt,
            const MAX: $SigInt,
            const DEF: $SigInt,
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

        fn assert_sub_unbounded<
            const MIN: $SigInt,
            const MAX: $SigInt,
            const DEF: $SigInt,
            T: Eq + Debug,
        >(
            rhs: SigRhs<{ 1 }, { <$SigInt>::MAX }>,
            expected: (T, T),
            op: impl Fn($Ty<MIN, MAX, DEF>, $SigInt) -> T,
        ) {
            let cnst = $Ty(MIN);
            let returned = op(cnst, rhs.get());
            assert_eq!(returned, expected.0);

            let cnst = $Ty(MAX);
            let returned = op(cnst, -rhs);
            assert_eq!(returned, expected.1);
        }

        fn assert_wrapping_add_unbounded<
            const MIN: $SigInt,
            const MAX: $SigInt,
            const DEF: $SigInt,
        >(
            rhs: SigRhs<{ 1 }, { $Ty::<MIN, MAX, DEF>::range_size_signed() }>,
            add: impl Fn($Ty<MIN, MAX, DEF>, $SigInt) -> $Ty<MIN, MAX, DEF>,
        ) {
            let mut cnst = $Ty(MAX);
            cnst = add(cnst, rhs.get());
            assert_eq!(cnst.get(), MIN + (rhs - 1));

            let mut cnst = $Ty(MIN);
            cnst = add(cnst, -rhs);
            assert_eq!(cnst.get(), MAX - (rhs - 1));
        }

        fn assert_wrapping_sub_unbounded<
            const MIN: $SigInt,
            const MAX: $SigInt,
            const DEF: $SigInt,
        >(
            rhs: SigRhs<{ 1 }, { $Ty::<MIN, MAX, DEF>::range_size_signed() }>,
            sub: impl Fn($Ty<MIN, MAX, DEF>, $SigInt) -> $Ty<MIN, MAX, DEF>,
        ) {
            let mut cnst = $Ty(MIN);
            cnst = sub(cnst, rhs.get());
            assert_eq!(cnst.get(), MAX - (rhs - 1));

            let mut cnst = $Ty(MAX);
            cnst = sub(cnst, -rhs);
            assert_eq!(cnst.get(), MIN + (rhs - 1));
        }

        // For testing purposes:
        //
        // Verify arithmetics for a range definition with the same lower bound as the
        // inner integer.
        type ConstrainedMin = $Ty<{ <$SigInt>::MIN }, { <$SigInt>::MAX - 1 }>;
        // Verify arithmetics for a range definition with the same upper bound as the
        // inner integer.
        type ConstrainedMax = $Ty<{ <$SigInt>::MIN + 1 }, { <$SigInt>::MAX }>;

        impl_int_prop_tests_for! {
        //  { int, uint, ErrorName },
            { $SigInt, $UnsInt, $Err },
        //  { module_name, TypeName },+
            { min, ConstrainedMin },
            { max, ConstrainedMax },
        }
    };
}

// Implement property tests for arithemtic operations for a number of parameterized
// `Constrained` types, e.g: `Constrainedi8<-10, 21, 0>`;
#[cfg(test)]
macro_rules! impl_int_prop_tests_for {
    ({ $SigInt:ty, $UnsInt:ty, $Err:ident }, $({ $md:ident, $Ty:ty }),+ $(,)*) => {$(
        #[cfg(test)]
        mod $md {
            use ::proptest::proptest;
            use super::*;

            type Cnst = $Ty;
            type CnstErr = $Err<{ Cnst::MIN }, { Cnst::MAX }>;

            proptest! {
                #[test]
                fn checked_add_bounded(rhs in SigRhsGen) {
                    assert_add_bounded(rhs, |cnst: Cnst, rhs| {
                        cnst.checked_add(rhs).expect("expected `checked_add` to succeed")
                    });
                }

                #[test]
                fn try_add_bounded(rhs in SigRhsGen) {
                    assert_add_bounded(rhs, |cnst: Cnst, rhs| {
                        cnst.try_add(rhs).expect("expected `try_add` to succeed")
                    });
                }

                #[test]
                fn saturating_add_bounded(rhs in SigRhsGen) {
                    assert_add_bounded(rhs, Cnst::saturating_add);
                }

                #[test]
                fn wrapping_add_bounded(rhs in SigRhsGen) {
                    assert_add_bounded(rhs, Cnst::wrapping_add);
                }

                #[test]
                fn overflowing_add_bounded(rhs in SigRhsGen) {
                    assert_add_bounded(rhs, |cnst: Cnst, rhs| {
                        let (cnst, overflowed) = cnst.overflowing_add(rhs);
                        assert!(!overflowed, "expected `overflowing_add` to not overflow");
                        cnst
                    });
                }

                #[test]
                fn checked_sub_bounded(rhs in SigRhsGen) {
                    assert_sub_bounded(rhs, |cnst: Cnst, rhs| {
                        cnst.checked_sub(rhs).expect("expected `checked_sub` to succeed")
                    });
                }

                #[test]
                fn try_sub_bounded(rhs in SigRhsGen) {
                    assert_sub_bounded(rhs, |cnst: Cnst, rhs| {
                        cnst.try_sub(rhs).expect("expected `try_sub` to succeed")
                    });
                }

                #[test]
                fn saturating_sub_bounded(rhs in SigRhsGen) {
                    assert_sub_bounded(rhs, Cnst::saturating_sub);
                }

                #[test]
                fn wrapping_sub_bounded(rhs in SigRhsGen) {
                    assert_sub_bounded(rhs, Cnst::wrapping_sub);
                }

                #[test]
                fn overflowing_sub_bounded(rhs in SigRhsGen) {
                    assert_sub_bounded(rhs, |cnst: Cnst, rhs| {
                        let (cnst, overflowed) = cnst.overflowing_sub(rhs);
                        assert!(!overflowed, "expected `overflowing_sub` to not overflow");
                        cnst
                    });
                }

                #[test]
                fn checked_add_unbounded(rhs in SigRhsGen) {
                    assert_add_unbounded(rhs, (None, None), Cnst::checked_add);
                }

                #[test]
                fn try_add_unbounded(rhs in SigRhsGen) {
                    assert_add_unbounded(
                        rhs,
                        (Err(CnstErr::greater()), Err(CnstErr::lower())),
                        Cnst::try_add
                    );
                }

                #[test]
                fn saturating_add_unbounded(rhs in SigRhsGen) {
                    assert_add_unbounded(
                        rhs,
                        (Cnst::new_max(), Cnst::new_min()),
                        Cnst::saturating_add
                    );
                }

                #[test]
                fn checked_sub_unbounded(rhs in SigRhsGen) {
                    assert_sub_unbounded(rhs, (None, None), Cnst::checked_sub);
                }

                #[test]
                fn try_sub_unbounded(rhs in SigRhsGen) {
                    assert_sub_unbounded(
                        rhs,
                        (Err(CnstErr::lower()), Err(CnstErr::greater())),
                        Cnst::try_sub
                    );
                }

                #[test]
                fn saturating_sub_unbounded(rhs in SigRhsGen) {
                    assert_sub_unbounded(
                        rhs,
                        (Cnst::new_min(), Cnst::new_max()),
                        Cnst::saturating_sub
                    );
                }

                #[test]
                fn wrapping_add_unbounded(rhs in SigRhsGen) {
                    assert_wrapping_add_unbounded(rhs, Cnst::wrapping_add);
                }

                #[test]
                fn overflowing_add_unbounded(rhs in SigRhsGen) {
                    assert_wrapping_add_unbounded(rhs, |cnst: Cnst, rhs| {
                        let (cnst, overflowed) = cnst.overflowing_add(rhs);
                        assert!(overflowed, "expected `overflowing_add` to overflow");
                        cnst
                    });
                }

                #[test]
                fn wrapping_sub_unbounded(rhs in SigRhsGen) {
                    assert_wrapping_sub_unbounded(rhs, Cnst::wrapping_sub);
                }

                #[test]
                fn overflowing_sub_unbounded(rhs in SigRhsGen) {
                    assert_wrapping_sub_unbounded(rhs, |cnst: Cnst, rhs| {
                        let (cnst, overflowed) = cnst.overflowing_sub(rhs);
                        assert!(overflowed, "expected `overflowing_sub` to overflow");
                        cnst
                    });
                }
            }
        }
    )+};
}
