// API implementation and doc values especific to unsigned integers.
macro_rules! constrained_uint_impl {
    ($UnsInt:ty, $md:ident, $Ty:ident, $MinErr:ident, $MaxErr:ident, $min:literal..=$max:literal) => {
        impl<const MIN: $UnsInt, const MAX: $UnsInt, const DEF: $UnsInt> $Ty<MIN, MAX, DEF> {
            /// Saturating integer addition. Computes self + rhs, saturating the result
            /// at range's inclusive upper bound.
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
                // Can't use `unwrap_or` because it is not `const`.
                match self.checked_add(rhs) {
                    Some(this) => this,
                    None => Self(MAX),
                }
            }

            /// Saturating integer substraction. Computes self - rhs, saturating the
            /// result at range's inclusive lower bound.
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
                // Can't use `unwrap_or` because it is not `const`.
                match self.checked_sub(rhs) {
                    Some(this) => this,
                    None => Self(MIN),
                }
            }

            /// Checked integer addition. Computes self + rhs, returning [`None`] if
            /// result is greater than range's inclusive upper bound.
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

            /// Checked integer substraction. Computes `self - rhs`, returning [`None`] if
            /// result is lower than range's inclusive lower bound.
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
            /// the result is greater than the range's inclusive upper bound.
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
                match self.checked_add(rhs) {
                    Some(this) => Ok(this),
                    None => Err($MaxErr(())),
                }
            }

            /// Fallible integer substraction. Computes `self + rhs`, returning an error
            /// if the result is lower than the range's inclusive lower bound.
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
                match self.checked_sub(rhs) {
                    Some(this) => Ok(this),
                    None => Err($MinErr(())),
                }
            }

            /// Wrapping (modular) addition. Computes `self + rhs`, wrapping around at
            /// the upper inclusive range bound.
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

            /// Wrapping (modular) substraction. Computes `self - rhs`, wrapping around
            /// at the lower inclusive range bound.
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

            /// Calculates `self` + `rhs`, indicating if result was wrapped around.
            ///
            /// Returns a tuple of the addition along with a boolean indicating whether
            /// the result was wrapped around the upper incluside range bound. If an
            /// wrapping addition would have occurred then the wrapped value is returned.
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
            pub const fn overflowing_add(mut self, mut rhs: $UnsInt) -> (Self, bool) {
                match self.0.overflowing_add(rhs) {
                    (value, false) if value <= MAX => (Self(value), false),
                    (value, false) => (Self::wrap_around_max(value), true),
                    (wrapped, true) => {
                        (self, rhs) = Self::wrap_around_max_over(wrapped);
                        (self.wrapping_add(rhs), true)
                    }
                }
            }

            /// Calculates `self` - `rhs`, indicating if result was wrapped around.
            ///
            /// Returns a tuple of the substraction along with a boolean indicating
            /// whether the result was wrapped around the lower incluside range bound.
            /// If an wrapping substraction would have occurred then the wrapped value
            /// is returned.
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
            pub const fn overflowing_sub(mut self, mut rhs: $UnsInt) -> (Self, bool) {
                match self.0.overflowing_sub(rhs) {
                    (value, false) if value >= MIN => (Self(value), false),
                    (value, false) => (Self::wrap_around_min(value), true),
                    (wrapped, true) => {
                        (self, rhs) = Self::wrap_around_min_over(wrapped);
                        (self.wrapping_sub(rhs), true)
                    }
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
                value = MIN + (value - MAX - 1) % Self::range_size();
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
                value = MAX - (MIN - value - 1) % Self::range_size();
                Self(value)
            }

            /// Computes the value to wrap around the range upper bound from a integer
            /// overflow.
            ///
            /// Wraps `<$UnsInt>::MAX` around the range upper bound and returns the result
            /// as the first value. Computes the remaining from provided overflowed
            /// integer and returns it as the second value.
            ///
            /// Caller must ensure that `value` is lower than `<$UnsInt>::MAX`, or else
            /// there will be an unexpected overflow.
            #[must_use]
            const fn wrap_around_max_over(value: $UnsInt) -> (Self, $UnsInt) {
                debug_assert!(value < <$UnsInt>::MAX, "value must lower than <$UnsInt>::MAX");
                // No conditional compilation based on constexpr evaluation unfortunately.
                if <$UnsInt>::MAX > MAX {
                    (Self::wrap_around_max(<$UnsInt>::MAX), value + 1)
                } else {
                    (Self(MIN), value)
                }
            }

            /// Computes the value to wrap around the range lower bound from a integer
            /// overflow.
            ///
            /// Wraps `<$UnsInt>::MIN` around the range lower bound and returns the result
            /// as the first value. Computes the remaining from provided overflowed
            /// integer and returns it as the second value.
            ///
            /// Caller must ensure that `value` is greater than `<$UnsInt>::MIN`, or else
            /// there will be an unexpected overflow.
            #[must_use]
            const fn wrap_around_min_over(mut value: $UnsInt) -> (Self, $UnsInt) {
                debug_assert!(value > <$UnsInt>::MIN, "value must greater than <$UnsInt>::MIN");
                value = <$UnsInt>::MAX - value;
                // No conditional compilation based on constexpr evaluation unfortunately.
                if <$UnsInt>::MIN < MIN {
                    (Self::wrap_around_min(<$UnsInt>::MIN), value + 1)
                } else {
                    (Self(MAX), value)
                }
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
        }
    };
}

// Defines mods, containers, errors, impls, tests and default doc values for unsigned integers.
macro_rules! constrained_uint_def_impl {
    ($({ $UnsInt:ty, $md:ident, $Ty:ident, $Err:ident, $MinErr:ident, $MaxErr:ident }),+ $(,)*) => {$(
        #[doc = concat!("Container and Error types for a range constrained [`prim@", stringify!($UnsInt), "`].")]
        pub mod $md {
            constrained_def_impl! {
            //  uint, mod_name, TypeName, ErrorName, MinErrorName, MaxErrorName, min..=max, (min-1, max+1)
                $UnsInt, $md, $Ty, $Err, $MinErr, $MaxErr, 1..=254, (0, 255)
            }

            constrained_uint_impl! {
            //  uint, mod_name, TypeName, MinErrorName, MaxErrorName, min..=max
                $UnsInt, $md, $Ty, $MinErr, $MaxErr, 1..=254
            }

            #[cfg(test)]
            mod tests_uint_common {
                tests_common! {
                //  uint, mod_path, TypeName, ErrorName, MinErrorName, MaxErrorName
                    $UnsInt, super, $Ty, $Err, $MinErr, $MaxErr
                }
            }

            #[cfg(test)]
            mod tests_uint {
                tests_uint! {
                //  uint, mod_path, TypeName, MinErrorName, MaxErrorName
                    $UnsInt, super, $Ty, $MinErr, $MaxErr
                }
            }
        }
    )+};
}

// Implements all unsigned integer specific tests.
#[cfg(test)]
macro_rules! tests_uint {
    ($UnsInt:ty, $ty_path:path, $Ty:ident, $MinErr:ident, $MaxErr:ident) => {
        use ::core::fmt::Debug;
        use ::core::ops::RangeInclusive;
        use $ty_path::*;

        // For testing purposes:
        //
        // Defines a type for arithmetic tests that **will not** overflow the inner integer.
        const MIN: $UnsInt = (<$UnsInt>::MAX / 5) * 2;
        const MAX: $UnsInt = (<$UnsInt>::MAX / 5) * 3;
        type ConstrainedEx = $Ty<{ MIN }, { MAX }>;
        // The range size must be a least 3.
        sa::const_assert!(ConstrainedEx::range_size() >= 3);
        // Must be able to wrap `range_size() - 2` around `MIN` without overflowing the inner integer.
        sa::const_assert!(<$UnsInt>::MIN < (ConstrainedEx::MIN - ConstrainedEx::range_size() - 2));
        // Must be able to wrap `range_size() + 2` around `MAX` without overflowing the inner integer.
        sa::const_assert!(<$UnsInt>::MAX > (ConstrainedEx::MAX + ConstrainedEx::range_size() + 2));

        const DEF_WRAP_RANGE: RangeInclusive<$UnsInt> = RangeInclusive::new(1, 2);
        const OFF_WRAP_RANGE: RangeInclusive<$UnsInt> = RangeInclusive::new(2, 3);

        // For tests that evaluate correct range wrapping while operations on the
        // inner integers have overflowed.
        //
        // Verify with the same lower bound as inner integer type for wrapping operations.
        type ConstrainedMin = $Ty<{ <$UnsInt>::MIN }, { <$UnsInt>::MAX - 1 }>;
        // Verify with the same upper bound as inner integer type for wrapping operations.
        type ConstrainedMax = $Ty<{ <$UnsInt>::MIN + 1 }, { <$UnsInt>::MAX }>;

        fn assert_add_bounded<F: Fn(ConstrainedEx, $UnsInt) -> ConstrainedEx>(succeed: F) {
            let mut constrained: ConstrainedEx;

            for value in 0..=2 {
                constrained = ConstrainedEx::new_min();
                constrained = succeed(constrained, value);
                assert_eq!(constrained.get(), ConstrainedEx::MIN + value);
            }

            for value in 1..=2 {
                constrained = ConstrainedEx::new_min();
                constrained = succeed(constrained, ConstrainedEx::range_size() - value);
                assert_eq!(constrained.get(), ConstrainedEx::MAX - value + 1);
            }
        }

        #[test]
        fn checked_add_bounded() {
            assert_add_bounded(|cnst, value| cnst.checked_add(value).unwrap());
        }

        #[test]
        fn try_add_bounded() {
            assert_add_bounded(|cnst, value| cnst.try_add(value).unwrap());
        }

        #[test]
        fn saturating_add_bounded() {
            assert_add_bounded(ConstrainedEx::saturating_add);
        }

        #[test]
        fn wrapping_add_bounded() {
            assert_add_bounded(ConstrainedEx::wrapping_add);
        }

        #[test]
        fn overflowing_add_bounded() {
            assert_add_bounded(|cnst, value| {
                let (cnst, overflowed) = cnst.overflowing_add(value);
                assert!(!overflowed);
                cnst
            })
        }

        fn assert_add_unbounded<T: Eq + Debug, F: Fn(ConstrainedEx, $UnsInt) -> T>(
            fail: F,
            failed: T,
        ) {
            let mut constrained: ConstrainedEx;

            for value in 0..=2 {
                constrained = ConstrainedEx::new_min();
                let res = fail(constrained, ConstrainedEx::range_size() + value);
                assert_eq!(res, failed);
            }

            constrained = ConstrainedEx::new_max();
            assert_eq!(fail(constrained, <$UnsInt>::MAX), failed);
        }

        #[test]
        fn checked_add_unbounded() {
            assert_add_unbounded(ConstrainedEx::checked_add, None);
        }

        #[test]
        fn try_add_unbounded() {
            assert_add_unbounded(ConstrainedEx::try_add, Err($MaxErr(())));
        }

        #[test]
        fn saturating_add_unbounded() {
            assert_add_unbounded(ConstrainedEx::saturating_add, $Ty(ConstrainedEx::MAX));
        }

        fn assert_sub_bounded<F: Fn(ConstrainedEx, $UnsInt) -> ConstrainedEx>(succeed: F) {
            let mut constrained: ConstrainedEx;

            for value in 0..=2 {
                constrained = ConstrainedEx::new_max();
                constrained = succeed(constrained, value);
                assert_eq!(constrained.get(), ConstrainedEx::MAX - value);
            }

            for value in 1..=2 {
                constrained = ConstrainedEx::new_max();
                constrained = succeed(constrained, ConstrainedEx::range_size() - value);
                assert_eq!(constrained.get(), ConstrainedEx::MIN + value - 1);
            }
        }

        #[test]
        fn checked_sub_bounded() {
            assert_sub_bounded(|cnst, value| cnst.checked_sub(value).unwrap());
        }

        #[test]
        fn try_sub_bounded() {
            assert_sub_bounded(|cnst, value| cnst.try_sub(value).unwrap());
        }

        #[test]
        fn saturating_sub_bounded() {
            assert_sub_bounded(ConstrainedEx::saturating_sub);
        }

        #[test]
        fn wrapping_sub_bounded() {
            assert_sub_bounded(ConstrainedEx::wrapping_sub);
        }

        #[test]
        fn overflowing_sub_bounded() {
            assert_sub_bounded(|cnst, value| {
                let (cnst, overflowed) = cnst.overflowing_sub(value);
                assert!(!overflowed);
                cnst
            })
        }

        fn assert_sub_unbounded<T: Eq + Debug, F: Fn(ConstrainedEx, $UnsInt) -> T>(
            fail: F,
            failed: T,
        ) {
            let mut constrained: ConstrainedEx;

            for value in 0..=2 {
                constrained = ConstrainedEx::new_max();
                let res = fail(constrained, ConstrainedEx::range_size() + value);
                assert_eq!(res, failed);
            }

            constrained = ConstrainedEx::new_min();
            assert_eq!(fail(constrained, <$UnsInt>::MAX), failed);
        }

        #[test]
        fn checked_sub_unbounded() {
            assert_sub_unbounded(ConstrainedEx::checked_sub, None);
        }

        #[test]
        fn try_sub_unbounded() {
            assert_sub_unbounded(ConstrainedEx::try_sub, Err($MinErr(())));
        }

        #[test]
        fn saturating_sub_unbounded() {
            assert_sub_unbounded(ConstrainedEx::saturating_sub, $Ty(ConstrainedEx::MIN));
        }

        fn assert_wrapping_add_wraps<
            F: Fn($Ty<MIN, MAX, DEF>, $UnsInt) -> $Ty<MIN, MAX, DEF>,
            const MIN: $UnsInt,
            const MAX: $UnsInt,
            const DEF: $UnsInt,
        >(
            wrap: F,
            range: RangeInclusive<$UnsInt>,
        ) {
            let mut constrained: $Ty<MIN, MAX, DEF>;

            for value in range {
                constrained = $Ty(MAX);
                constrained = wrap(constrained, value);
                assert_eq!(constrained.get(), MIN + value - 1);
            }

            for value in 0..=2 {
                constrained = $Ty(MAX);
                constrained = wrap(constrained, $Ty::<MIN, MAX, DEF>::range_size() - value);
                assert_eq!(constrained.get(), MAX - value);
            }

            for value in 1..=2 {
                constrained = $Ty(MAX);
                constrained = wrap(constrained, $Ty::<MIN, MAX, DEF>::range_size());
                constrained = wrap(constrained, value);
                assert_eq!(constrained.get(), MIN + value - 1);
            }
        }

        #[test]
        fn wrapping_add_wraps() {
            assert_wrapping_add_wraps(ConstrainedEx::wrapping_add, DEF_WRAP_RANGE);
        }

        #[test]
        fn overflowing_add_wraps() {
            assert_wrapping_add_wraps(
                |cnst: ConstrainedEx, value| {
                    let (cnst, overflowed) = cnst.overflowing_add(value);
                    assert!(overflowed);
                    cnst
                },
                DEF_WRAP_RANGE,
            )
        }

        #[test]
        fn wrapping_add_wraps_inner_overflow_min() {
            assert_wrapping_add_wraps(ConstrainedMin::wrapping_add, OFF_WRAP_RANGE);
        }

        #[test]
        fn overflowing_add_wraps_inner_overflow_min() {
            assert_wrapping_add_wraps(
                |cnst: ConstrainedMin, value| {
                    let (cnst, overflowed) = cnst.overflowing_add(value);
                    assert!(overflowed);
                    cnst
                },
                OFF_WRAP_RANGE,
            );
        }

        #[test]
        fn wrapping_add_wraps_inner_overflow_max() {
            assert_wrapping_add_wraps(ConstrainedMax::wrapping_add, DEF_WRAP_RANGE);
        }

        #[test]
        fn overflowing_add_wraps_inner_overflow_max() {
            assert_wrapping_add_wraps(
                |cnst: ConstrainedMax, value| {
                    let (cnst, overflowed) = cnst.overflowing_add(value);
                    assert!(overflowed);
                    cnst
                },
                DEF_WRAP_RANGE,
            );
        }

        fn assert_wrapping_sub_wraps<
            F: Fn($Ty<MIN, MAX, DEF>, $UnsInt) -> $Ty<MIN, MAX, DEF>,
            const MIN: $UnsInt,
            const MAX: $UnsInt,
            const DEF: $UnsInt,
        >(
            wrap: F,
            range: RangeInclusive<$UnsInt>,
        ) {
            let mut constrained: $Ty<MIN, MAX, DEF>;

            for value in range {
                constrained = $Ty(MIN);
                constrained = wrap(constrained, value);
                assert_eq!(constrained.get(), MAX - value + 1);
            }

            for value in 0..=2 {
                constrained = $Ty(MIN);
                constrained = wrap(constrained, $Ty::<MIN, MAX, DEF>::range_size() - value);
                assert_eq!(constrained.get(), MIN + value);
            }

            for value in 1..=2 {
                constrained = $Ty(MIN);
                constrained = wrap(constrained, $Ty::<MIN, MAX, DEF>::range_size());
                constrained = wrap(constrained, value);
                assert_eq!(constrained.get(), MAX - value + 1);
            }
        }

        #[test]
        fn wrapping_sub_wraps() {
            assert_wrapping_sub_wraps(ConstrainedEx::wrapping_sub, DEF_WRAP_RANGE);
        }

        #[test]
        fn overflowing_sub_wraps() {
            assert_wrapping_sub_wraps(
                |cnst: ConstrainedEx, value| {
                    let (cnst, overflowed) = cnst.overflowing_sub(value);
                    assert!(overflowed);
                    cnst
                },
                DEF_WRAP_RANGE,
            )
        }

        #[test]
        fn wrapping_sub_wraps_inner_overflow_min() {
            assert_wrapping_sub_wraps(ConstrainedMin::wrapping_sub, DEF_WRAP_RANGE);
        }

        #[test]
        fn overflowing_sub_wraps_inner_overflow_min() {
            assert_wrapping_sub_wraps(
                |cnst: ConstrainedMin, value| {
                    let (cnst, overflowed) = cnst.overflowing_sub(value);
                    assert!(overflowed);
                    cnst
                },
                DEF_WRAP_RANGE,
            );
        }

        #[test]
        fn wrapping_sub_wraps_inner_overflow_max() {
            assert_wrapping_sub_wraps(ConstrainedMax::wrapping_sub, OFF_WRAP_RANGE);
        }

        #[test]
        fn overflowing_sub_wraps_inner_overflow_max() {
            assert_wrapping_sub_wraps(
                |cnst: ConstrainedMax, value| {
                    let (cnst, overflowed) = cnst.overflowing_sub(value);
                    assert!(overflowed);
                    cnst
                },
                OFF_WRAP_RANGE,
            );
        }
    };
}
