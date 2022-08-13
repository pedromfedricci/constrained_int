// API implementation and doc values specific to signed integers.
macro_rules! constrained_int_impl {
    (   $SigInt:ty, $UnsInt:ty, $md:ident, $Ty:ident, $Err:ident,
        $MinErr:ident, $MaxErr:ident, $min:literal..=$max:literal
    ) => {
        impl<const MIN: $SigInt, const MAX: $SigInt, const DEF: $SigInt> $Ty<MIN, MAX, DEF> {
            /// Saturating integer addition. Computes `self + rhs`, saturating the result
            /// at the range's inclusive bounds.
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

            /// Saturating addition with unsigned integer. Computes `self + rhs`, saturating
            /// the result at range's upper bound.
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
            #[doc = concat!("constrained = constrained.saturating_add_unsigned(1_", stringify!($UnsInt), ");")]
            #[doc = concat!("assert_eq!(constrained.get(), ", stringify!($max), ");")]
            /// ```
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn saturating_add_unsigned(self, rhs: $UnsInt) -> Self {
                let rhs = self.0.saturating_add_unsigned(rhs);
                Self::saturating_new_unguarded(rhs)
            }

            /// Saturating integer substraction. Computes self - rhs, saturating the result
            /// at the range's inclusive bounds.
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

            /// Saturating substraction with unsigned integer. Computes `self - rhs`,
            /// saturating the result at range's lower bound.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            /// let mut constrained = Constrained::new_min();
            /// // Saturates at the lower bound.
            #[doc = concat!("constrained = constrained.saturating_sub_unsigned(1_", stringify!($UnsInt), ");")]
            #[doc = concat!("assert_eq!(constrained.get(), ", stringify!($min), ");")]
            /// ```
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn saturating_sub_unsigned(self, rhs: $UnsInt) -> Self {
                let rhs = self.0.saturating_sub_unsigned(rhs);
                Self::saturating_new_unguarded(rhs)
            }

            /// Checked integer addition. Computes `self + rhs`, returning [`None`] if
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

            /// Checked addition with unsigned integer. Computes `self + rhs`, returning
            /// [`None`] if result is grater than the range's upper bound.
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
            #[doc = concat!("assert_eq!(constrained.checked_add_unsigned(1_", stringify!($UnsInt), "), None);")]
            /// ```
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn checked_add_unsigned(self, rhs: $UnsInt) -> Option<Self> {
                // Can't use `?` operator on const fn yet:
                // https://github.com/rust-lang/rust/issues/74935.
                match self.0.checked_add_unsigned(rhs) {
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

            /// Checked substraction with unsigned integer. Computes `self - rhs`, returning
            /// [`None`] if result is lower than the range's lower bound.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            /// let mut constrained = Constrained::new_min();
            /// // Above upper bound.
            #[doc = concat!("assert_eq!(constrained.checked_sub_unsigned(1_", stringify!($UnsInt), "), None);")]
            /// ```
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn checked_sub_unsigned(self, rhs: $UnsInt) -> Option<Self> {
                // Can't use `?` operator on const fn yet:
                // https://github.com/rust-lang/rust/issues/74935.
                match self.0.checked_sub_unsigned(rhs) {
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

            /// Fallible addition with unsigned. Computes `self + rhs`, returning an error
            /// if the result is greater than the range's upper bound.
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
            #[doc = concat!("assert!(constrained.try_add_unsigned(1_", stringify!($UnsInt), ").is_err());")]
            /// ```
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn try_add_unsigned(self, rhs: $UnsInt) -> Result<Self, $MaxErr<MAX>> {
                match self.checked_add_unsigned(rhs) {
                    Some(this) => Ok(this),
                    None => Err($MaxErr::new()),
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

            /// Fallible substraction with unsigned. Computes `self + rhs`, returning an
            /// error if the result is lower than the range's lower bound.
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
            #[doc = concat!("assert!(constrained.try_sub_unsigned(1_", stringify!($UnsInt), ").is_err());")]
            /// ```
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub const fn try_sub_unsigned(self, rhs: $UnsInt) -> Result<Self, $MinErr<MIN>> {
                match self.checked_sub_unsigned(rhs) {
                    Some(this) => Ok(this),
                    None => Err($MinErr::new()),
                }
            }

            /// Wrapping (modular) addition.
            ///
            /// Computes `self + rhs`, wrapping the result around the range's upper bound
            /// if the integer is positive, or at the range's lower bound if negative.
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
                    (wrapped, true) => Self::wrapped_add(wrapped, rhs.is_positive()),
                };

                self.wrapping_add(rhs)
            }

            /// Wrapping (modular) addition with unsigned integer. Computes `self + rhs`,
            /// wrapping around at the range's upper bound.
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
            #[doc = concat!("constrained = constrained.wrapping_add_unsigned(1_", stringify!($UnsInt), ");")]
            /// assert_eq!(constrained.get(), Constrained::MIN);
            /// ```
            #[must_use = "this returns the result of the operation, without modifyind the original"]
            pub const fn wrapping_add_unsigned(mut self, mut rhs: $UnsInt) -> Self {
                (self, rhs) = match self.0.overflowing_add_unsigned(rhs) {
                    (value, false) if value <= MAX => return Self(value),
                    (value, false) => return Self::wrap_around_max(value),
                    (wrapped, true) => Self::wrap_around_max_uns_over(wrapped),
                };

                self.wrapping_add_unsigned(rhs)
            }

            /// Wrapping (modular) substraction.
            ///
            /// Computes `self - rhs`, wrapping the result around the range's upper bound
            /// if the integer is negative, or at the range's lower bound if positive.
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
                    (wrapped, true) => Self::wrapped_sub(wrapped, rhs.is_positive()),
                };

                self.wrapping_sub(rhs)
            }

            /// Wrapping (modular) substraction with unsigned integer. Computes `self - rhs`,
            /// wrapping around at the range's lower bound.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            /// let mut constrained = Constrained::new_min();
            /// // Wraps around upper bound.
            #[doc = concat!("constrained = constrained.wrapping_sub_unsigned(1_", stringify!($UnsInt), ");")]
            /// assert_eq!(constrained.get(), Constrained::MAX);
            /// ```
            #[must_use = "this returns the result of the operation, without modifyind the original"]
            pub const fn wrapping_sub_unsigned(mut self, mut rhs: $UnsInt) -> Self {
                (self, rhs) = match self.0.overflowing_sub_unsigned(rhs) {
                    (value, false) if value >= MIN => return Self(value),
                    (value, false) => return Self::wrap_around_min(value),
                    (wrapped, true) => Self::wrap_around_min_uns_over(wrapped),
                };

                self.wrapping_sub_unsigned(rhs)
            }

            /// Wrapping (modular) addition, indicating if result was wrapped around.
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
            pub const fn overflowing_add(self, rhs: $SigInt) -> (Self, bool) {
                match self.0.overflowing_add(rhs) {
                    (value, false) if value >= MIN && value <= MAX => (Self(value), false),
                    (value, false) if value > MAX => (Self::wrap_around_max(value), true),
                    (value, false) => (Self::wrap_around_min(value), true),
                    (wrapped, true) => (Self::overflowed_add(wrapped, rhs.is_positive()), true),
                }
            }

            /// Wrapping (modular) addition with unsigned integer, indicating if result
            /// was wrapped around.
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
            #[doc = concat!("(constrained, wrapped) = constrained.overflowing_add_unsigned(1_", stringify!($UnsInt), ");")]
            /// assert_eq!(constrained.get(), Constrained::MIN);
            /// assert_eq!(wrapped, true);
            /// ```
            #[must_use = "this returns the result of the operation, without modifyind the original"]
            pub const fn overflowing_add_unsigned(self, rhs: $UnsInt) -> (Self, bool) {
                match self.0.overflowing_add_unsigned(rhs) {
                    (value, false) if value <= MAX => (Self(value), false),
                    (value, false) => (Self::wrap_around_max(value), true),
                    (wrapped, true) => (Self::overflowed_add_unsigned(wrapped), true),
                }
            }

            /// Wrapping (modular) substraction, indicating if result was wrapped around.
            ///
            /// Computes `self - rhs`, wrapping the result around the range's upper
            /// bound if the integer is negative, or at the range's lower bound if
            /// positive. If a wrapping substraction would have occurred, then the boolean
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
            pub const fn overflowing_sub(self, rhs: $SigInt) -> (Self, bool) {
                match self.0.overflowing_sub(rhs) {
                    (value, false) if value >= MIN && value <= MAX => (Self(value), false),
                    (value, false) if value < MIN => (Self::wrap_around_min(value), true),
                    (value, false) => (Self::wrap_around_max(value), true),
                    (wrapped, true) => (Self::overflowed_sub(wrapped, rhs.is_positive()), true),
                }
            }

            /// Wrapping (modular) substraction with unsigned integer, indicating if result
            /// was wrapped around.
            ///
            /// Computes `self - rhs`, wrapping the result around the range's lower
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
            /// let mut constrained = Constrained::new_min();
            /// let mut wrapped: bool;
            ///
            /// // Wraps around the upper bound, the boolean is set to `true`.
            #[doc = concat!("(constrained, wrapped) = constrained.overflowing_sub_unsigned(1_", stringify!($UnsInt), ");")]
            /// assert_eq!(constrained.get(), Constrained::MAX);
            /// assert_eq!(wrapped, true);
            /// ```
            #[must_use = "this returns the result of the operation, without modifyind the original"]
            pub const fn overflowing_sub_unsigned(self, rhs: $UnsInt) -> (Self, bool) {
                match self.0.overflowing_sub_unsigned(rhs) {
                    (value, false) if value >= MIN => (Self(value), false),
                    (value, false) => (Self::wrap_around_min(value), true),
                    (wrapped, true) => (Self::overflowed_sub_unsigned(wrapped), true),
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

            /// Wraps the value around the range's upper bound.
            ///
            /// Caller must ensure that `value` is greater than `MAX`, or else there will
            /// be an unexpected overflow.
            #[must_use]
            const fn wrap_around_max(mut value: $SigInt) -> Self {
                debug_assert!(value > MAX, "value must be greater than `MAX`");
                let offset = Self::remainder(<$SigInt>::abs_diff(MAX, value) - 1);
                // Can't overflow since `MIN + x % range_size()` is at most equal to `MAX`.
                value = MIN + offset as $SigInt;
                Self(value)
            }

            /// Wraps the value around the range's lower bound.
            ///
            /// Caller must ensure that `value` is lower than `MIN`, or else there will
            /// be an unexpected overflow.
            #[must_use]
            const fn wrap_around_min(mut value: $SigInt) -> Self {
                debug_assert!(value < MIN, "value must be lower than `MIN`");
                let offset = Self::remainder(<$SigInt>::abs_diff(MIN, value) - 1);
                // Can't overflow since `MAX - x % range_size()` is at least equal to `MIN`.
                value = MAX - offset as $SigInt;
                Self(value)
            }

            /// Computes the value to wrap around the range's upper bound from the value
            /// returned by a overflowed signed operation.
            ///
            /// Wraps `<$SigInt>::MAX` around the range's upper bound and returns the
            /// result as the first value. Computes the remaining from the wrapped value
            /// and returns it as the second value.
            ///
            /// Caller must ensure that `value` is lower than 0, or else there will be
            /// a unexpected overflow.
            #[must_use]
            const fn wrap_around_max_over(mut value: $SigInt) -> (Self, $SigInt) {
                debug_assert!(value < 0, "value must lower than 0");
                value = <$SigInt>::abs_diff(<$SigInt>::MIN, value) as $SigInt;
                // TODO: No conditional compilation based on constexpr evaluation yet.
                if <$SigInt>::MAX > MAX {
                    (Self::wrap_around_max(<$SigInt>::MAX), Self::remainder_signed(value + 1))
                } else {
                    (Self(MIN), Self::remainder_signed(value))
                }
            }

            /// Computes the value to wrap around the range's upper bound from the value
            /// returned by a overflowed unsigned operation.
            ///
            /// Wraps `<$SigInt>::MAX` around the range's upper bound and returns the
            /// result as the first value. Computes the remaining from the wrapped value
            /// and returns it in the second position.
            ///
            /// Caller must ensure that `value` is lower than `<$SigInt>::MAX`, or else
            /// there will be a unexpected overflow.
            #[must_use]
            const fn wrap_around_max_uns_over(value: $SigInt) -> (Self, $UnsInt) {
                debug_assert!(value < <$SigInt>::MAX,
                    concat!("value must be lower than ", stringify!($SigInt), "::MAX")
                );
                let value = <$SigInt>::abs_diff(<$SigInt>::MIN, value);
                // TODO: No conditional compilation based on constexpr evaluation yet.
                if <$SigInt>::MAX > MAX {
                    (Self::wrap_around_max(<$SigInt>::MAX), Self::remainder(value + 1))
                } else {
                    (Self(MIN), Self::remainder(value))
                }
            }

            /// Computes the value to wrap around the range's upper bound from the value
            /// returned by a overflowed signed operation.
            ///
            /// Wraps `<$SigInt>::MIN` around the range's lower bound and returns the
            /// result as the first value. Computes the remaining from the wrapped value
            /// and returns it in the second position.
            ///
            /// Caller must ensure that `value` is greater than 0, or else there will be
            /// an unexpected overflow.
            #[must_use]
            const fn wrap_around_min_over(mut value: $SigInt) -> (Self, $SigInt) {
                debug_assert!(value > 0, "value must greater than 0");
                value = <$SigInt>::abs_diff(<$SigInt>::MAX, value) as $SigInt;
                // TODO: No conditional compilation based on constexpr evaluation yet.
                if <$SigInt>::MIN < MIN {
                    (Self::wrap_around_min(<$SigInt>::MIN), Self::remainder_signed(value + 1))
                } else {
                    (Self(MAX), Self::remainder_signed(value))
                }
            }

            /// Computes the value to wrap around the range's lower bound from the value
            /// returned by a overflowed unsigned operation.
            ///
            /// Wraps `<$SigInt>::MIN` around the range's lower bound and returns the
            /// result as the first value. Computes the remaining from the wrapped value
            /// and returns it in the second position.
            ///
            /// Caller must ensure that `value` is greater than <$SigInt>::MIN, or else
            /// there will be an unexpected overflow.
            #[must_use]
            const fn wrap_around_min_uns_over(value: $SigInt) -> (Self, $UnsInt) {
                debug_assert!(value > <$SigInt>::MIN,
                    concat!("value must be greater than ", stringify!($SigInt), "::MIN")
                );
                let value = <$SigInt>::abs_diff(<$SigInt>::MAX, value);
                // TODO: No conditional compilation based on constexpr evaluation yet.
                if <$SigInt>::MIN < MIN {
                    (Self::wrap_around_min(<$SigInt>::MIN), Self::remainder(value + 1))
                } else {
                    (Self(MAX), Self::remainder(value))
                }
            }

            /// Returns `Self` and `rhs` for next `wrapping_add` call.
            ///
            /// `wrapped`: overflowed integer.
            /// `is_pos`: must be `true` if `rhs` was positive, `false` otherwise.
            #[must_use]
            const fn wrapped_add(wrapped: $SigInt, is_pos: bool) -> (Self, $SigInt) {
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
            const fn wrapped_sub(wrapped: $SigInt, is_pos: bool) -> (Self, $SigInt) {
                if is_pos {
                    Self::wrap_around_min_over(wrapped)
                } else {
                    let (this, rhs) = Self::wrap_around_max_over(wrapped);
                    (this, -rhs)
                }
            }

            /// Handles overflowed `overflowing_add` calls from inner integer.
            ///
            /// `wrapped`: overflowed integer.
            /// `is_pos`: must be `true` if `rhs` was positive, `false` otherwise.
            #[must_use]
            const fn overflowed_add(wrapped: $SigInt, is_pos: bool) -> Self {
                let (this, rhs) = Self::wrapped_add(wrapped, is_pos);
                this.wrapping_add(rhs)
            }

            /// Handles overflowed `overflowing_sub` calls from inner integer.
            ///
            /// `wrapped`: overflowed integer.
            /// `is_pos`: must be `true` if `rhs` was positive, `false` otherwise.
            #[must_use]
            const fn overflowed_sub(wrapped: $SigInt, is_pos: bool) -> Self {
                let (this, rhs) = Self::wrapped_sub(wrapped, is_pos);
                this.wrapping_sub(rhs)
            }

            /// Handles overflowed `overflowing_add_unsigned` calls from inner integer.
            ///
            /// `wrapped`: overflowed integer.
            /// `is_pos`: must be `true` if `rhs` was positive, `false` otherwise.
            #[must_use]
            const fn overflowed_add_unsigned(wrapped: $SigInt) -> Self {
                let (this, rhs) = Self::wrap_around_max_uns_over(wrapped);
                this.wrapping_add_unsigned(rhs)
            }

            /// Handles overflowed `overflowing_sub_unsigned` calls from inner integer.
            ///
            /// `wrapped`: overflowed integer.
            /// `is_pos`: must be `true` if `rhs` was positive, `false` otherwise.
            #[must_use]
            const fn overflowed_sub_unsigned(wrapped: $SigInt) -> Self {
                let (this, rhs) = Self::wrap_around_min_uns_over(wrapped);
                this.wrapping_sub_unsigned(rhs)
            }

            /// Computes the remainder of `value` by the range's size.
            #[must_use]
            const fn remainder(value: $UnsInt) -> $UnsInt {
                value % Self::range_size()
            }

            /// Computes the remainder of signed `value` by the range's size.
            ///
            /// Caller must ensure that `value` is a positive value, or else there will
            /// be a unexpected overflow.
            #[must_use]
            const fn remainder_signed(value: $SigInt) -> $SigInt {
                debug_assert!(value >= 0, "value must be greater or equal to 0");
                let value = value as $UnsInt % Self::range_size();
                value as $SigInt
            }

            /// Returns the range size.
            #[must_use]
            const fn range_size() -> $UnsInt {
                // Can't overflow since construction is guarded against `MAX ==
                // <$SigInt>::MAX` AND `MIN == <$SigInt>::MIN` at the same time.
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
         $Ty:ident, $Err:ident, $MinErr:ident, $MaxErr:ident }),+ $(,)*
    ) => {$(
        #[doc = concat!("Container and Error types for a range constrained [`prim@", stringify!($SigInt), "`].")]
        pub mod $sint_md {
            constrained_def_impl! {
            //  sint, sint_mod, TypeName, ErrorName, MinErrorName, MaxErrorName, min..=max, (min-1, max+1)
                $SigInt, $sint_md, $Ty, $Err, $MinErr, $MaxErr, -127..=126, (-128, 127)
            }

            constrained_int_impl! {
            //  sint, uint, sint_mod, TypeName, ErrorName, MinErrorName, MaxErrorName, min..=max
                $SigInt, $UnsInt, $sint_md, $Ty, $Err, $MinErr, $MaxErr, -127..=126
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
                //  sint, uint, sint_mod, uint_mod, ty_mod_path, TypeName, ErrorName, MinErrorName, MaxErrorName
                    $SigInt, $UnsInt, $sint_md, $uint_md, super, $Ty, $Err, $MinErr, $MaxErr
                }
            }
        }
    )+};
}

// Implements all signed integer specific tests.
#[cfg(test)]
macro_rules! tests_int {
    (   $SigInt:ty, $UnsInt:ty, $sint_md:ident, $uint_md:ident, $ty_path:path,
        $Ty:ident, $Err:ident, $MinErr:ident, $MaxErr:ident
    ) => {
        use crate::proptest::$sint_md::{CnstGen, Rhs as SigRhs, RhsGen as SigRhsGen};
        use crate::proptest::$uint_md::{Rhs as UnsRhs, RhsGen as UnsRhsGen};
        use ::core::fmt::Debug;
        use $ty_path::{$Err, $MaxErr, $MinErr, $Ty};

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

        fn assert_add_unsigned_bounded<
            const MIN: $SigInt,
            const MAX: $SigInt,
            const DEF: $SigInt,
        >(
            rhs: UnsRhs<{ 0 }, { $Ty::<MIN, MAX, DEF>::range_size() - 1 }>,
            add: impl Fn($Ty<MIN, MAX, DEF>, $UnsInt) -> $Ty<MIN, MAX, DEF>,
        ) {
            let mut cnst = $Ty(MIN);
            cnst = add(cnst, rhs.get());
            assert_eq!(cnst.get(), MIN.checked_add_unsigned(rhs.get()).unwrap());
        }

        fn assert_sub_unsigned_bounded<
            const MIN: $SigInt,
            const MAX: $SigInt,
            const DEF: $SigInt,
        >(
            rhs: UnsRhs<{ 0 }, { $Ty::<MIN, MAX, DEF>::range_size() - 1 }>,
            sub: impl Fn($Ty<MIN, MAX, DEF>, $UnsInt) -> $Ty<MIN, MAX, DEF>,
        ) {
            let mut cnst = $Ty(MAX);
            cnst = sub(cnst, rhs.get());
            assert_eq!(cnst.get(), MAX.checked_sub_unsigned(rhs.get()).unwrap());
        }

        fn assert_unsigned_unbounded<
            const MIN: $SigInt,
            const MAX: $SigInt,
            const DEF: $SigInt,
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

        fn assert_wrapping_add_unsigned_unbounded<
            const MIN: $SigInt,
            const MAX: $SigInt,
            const DEF: $SigInt,
        >(
            rhs: UnsRhs<{ 1 }, { $Ty::<MIN, MAX, DEF>::range_size() }>,
            add: impl Fn($Ty<MIN, MAX, DEF>, $UnsInt) -> $Ty<MIN, MAX, DEF>,
        ) {
            let mut cnst = $Ty(MAX);
            cnst = add(cnst, rhs.get());
            assert_eq!(cnst.get(), MIN.checked_add_unsigned(rhs - 1).unwrap());
        }

        fn assert_wrapping_sub_unsigned_unbounded<
            const MIN: $SigInt,
            const MAX: $SigInt,
            const DEF: $SigInt,
        >(
            rhs: UnsRhs<{ 1 }, { $Ty::<MIN, MAX, DEF>::range_size() }>,
            sub: impl Fn($Ty<MIN, MAX, DEF>, $UnsInt) -> $Ty<MIN, MAX, DEF>,
        ) {
            let mut cnst = $Ty(MIN);
            cnst = sub(cnst, rhs.get());
            assert_eq!(cnst.get(), MAX.checked_sub_unsigned(rhs - 1).unwrap());
        }

        fn assert_wrapping_unsigned_range_size<
            const MIN: $SigInt,
            const MAX: $SigInt,
            const DEF: $SigInt,
        >(
            mut cnst: $Ty<MIN, MAX, DEF>,
            wrap: impl Fn($Ty<MIN, MAX, DEF>, $UnsInt) -> $Ty<MIN, MAX, DEF>,
        ) {
            let inner = cnst.get();
            cnst = wrap(cnst, $Ty::<MIN, MAX, DEF>::range_size());
            assert_eq!(cnst.get(), inner);
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
        //  { ErrorName, MinErrorName, MaxErrorName },
            { $Err, $MinErr, $MaxErr },
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
                fn checked_add_unsigned_bounded(rhs in UnsRhsGen) {
                    assert_add_unsigned_bounded(rhs, |cnst: Cnst, rhs| {
                        cnst.checked_add_unsigned(rhs).expect("expected `checked_add_unsigned` to succeed")
                    });
                }

                #[test]
                fn try_add_unsigned_bounded(rhs in UnsRhsGen) {
                    assert_add_unsigned_bounded(rhs, |cnst: Cnst, rhs| {
                        cnst.try_add_unsigned(rhs).expect("expected `try_add_unsigned` to succeed")
                    });
                }

                #[test]
                fn saturating_add_unsigned_bounded(rhs in UnsRhsGen) {
                    assert_add_unsigned_bounded(rhs, Cnst::saturating_add_unsigned);
                }

                #[test]
                fn wrapping_add_unsigned_bounded(rhs in UnsRhsGen) {
                    assert_add_unsigned_bounded(rhs, Cnst::wrapping_add_unsigned);
                }

                #[test]
                fn overflowing_add_unsigned_bounded(rhs in UnsRhsGen) {
                    assert_add_unsigned_bounded(rhs, |cnst: Cnst, rhs| {
                        let (cnst, overflowed) = cnst.overflowing_add_unsigned(rhs);
                        assert!(!overflowed, "expected `overflowing_add_unsigned` to not overflow");
                        cnst
                    });
                }

                #[test]
                fn checked_sub_unsigned_bounded(rhs in UnsRhsGen) {
                    assert_sub_unsigned_bounded(rhs, |cnst: Cnst, rhs| {
                        cnst.checked_sub_unsigned(rhs).expect("expected `checked_sub_unsigned` to succeed")
                    })
                }

                #[test]
                fn try_sub_unsigned_bounded(rhs in UnsRhsGen) {
                    assert_sub_unsigned_bounded(rhs, |cnst: Cnst, rhs| {
                        cnst.try_sub_unsigned(rhs).expect("expected `try_sub_unsigned` to succeed")
                    });
                }

                #[test]
                fn saturating_sub_unsigned_bounded(rhs in UnsRhsGen) {
                    assert_sub_unsigned_bounded(rhs, Cnst::saturating_sub_unsigned);
                }

                #[test]
                fn wrapping_sub_unsigned_bounded(rhs in UnsRhsGen) {
                    assert_sub_unsigned_bounded(rhs, Cnst::wrapping_sub_unsigned);
                }

                #[test]
                fn overflowing_sub_unsigned_bounded(rhs in UnsRhsGen) {
                    assert_sub_unsigned_bounded(rhs, |cnst: Cnst, rhs| {
                        let (cnst, overflowed) = cnst.overflowing_sub_unsigned(rhs);
                        assert!(!overflowed, "expected `overflowing_sub_unsigned` to not overflow");
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
                fn checked_add_unsigned_unbounded((cnst, rhs) in (CnstGen, UnsRhsGen)) {
                    assert_unsigned_unbounded(cnst, rhs, None, Cnst::checked_add_unsigned);
                }

                #[test]
                fn try_add_unsigned_unbounded((cnst, rhs) in (CnstGen, UnsRhsGen)) {
                    assert_unsigned_unbounded(cnst, rhs, Err(MaxErr::new()), Cnst::try_add_unsigned);
                }

                #[test]
                fn saturating_add_unsigned_unbounded((cnst, rhs) in (CnstGen, UnsRhsGen)) {
                    assert_unsigned_unbounded(cnst, rhs, Cnst::new_max(), Cnst::saturating_add_unsigned);
                }

                #[test]
                fn checked_sub_unsigned_unbounded((cnst, rhs) in (CnstGen, UnsRhsGen)) {
                    assert_unsigned_unbounded(cnst, rhs, None, Cnst::checked_sub_unsigned);
                }

                #[test]
                fn try_sub_unsigned_unbounded((cnst, rhs) in (CnstGen, UnsRhsGen)) {
                    assert_unsigned_unbounded(cnst, rhs, Err(MinErr::new()), Cnst::try_sub_unsigned);
                }

                #[test]
                fn saturating_sub_unsigned_unbounded((cnst, rhs) in (CnstGen, UnsRhsGen)) {
                    assert_unsigned_unbounded(cnst, rhs, Cnst::new_min(), Cnst::saturating_sub_unsigned);
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

                #[test]
                fn wrapping_add_unsigned_unbounded(rhs in UnsRhsGen) {
                    assert_wrapping_add_unsigned_unbounded(rhs, Cnst::wrapping_add_unsigned);
                }

                #[test]
                fn overflowing_add_unsigned_unbounded(rhs in UnsRhsGen) {
                    assert_wrapping_add_unsigned_unbounded(rhs, |cnst: Cnst, rhs| {
                        let (cnst, overflowed) = cnst.overflowing_add_unsigned(rhs);
                        assert!(overflowed, "expected `overflowing_add_unsigned` to overflow");
                        cnst
                    });
                }

                #[test]
                fn wrapping_sub_unsigned_unbounded(rhs in UnsRhsGen) {
                    assert_wrapping_sub_unsigned_unbounded(rhs, Cnst::wrapping_sub_unsigned);
                }

                #[test]
                fn overflowing_sub_unsigned_unbounded(rhs in UnsRhsGen) {
                    assert_wrapping_sub_unsigned_unbounded(rhs, |cnst: Cnst, rhs| {
                        let (cnst, overflowed) = cnst.overflowing_sub_unsigned(rhs);
                        assert!(overflowed, "expected `overflowing_sub_unsigned` to overflow");
                        cnst
                    });
                }

                #[test]
                fn wrapping_add_unsigned_range_size(cnst in CnstGen) {
                    assert_wrapping_unsigned_range_size(cnst, Cnst::wrapping_add_unsigned);
                }

                #[test]
                fn overflowing_add_unsigned_range_size(cnst in CnstGen) {
                    assert_wrapping_unsigned_range_size(cnst, |cnst: Cnst, rsize| {
                        let (cnst, overflowed) = cnst.overflowing_add_unsigned(rsize);
                        assert!(overflowed, "expected `overflowing_add` to overflow");
                        cnst
                    });
                }

                #[test]
                fn wrapping_sub_unsigned_range_size(cnst in CnstGen) {
                    assert_wrapping_unsigned_range_size(cnst, Cnst::wrapping_sub_unsigned);
                }

                #[test]
                fn overflowing_sub_unsigned_range_size(cnst in CnstGen) {
                    assert_wrapping_unsigned_range_size(cnst, |cnst: Cnst, rsize| {
                        let (cnst, overflowed) = cnst.overflowing_sub_unsigned(rsize);
                        assert!(overflowed, "expected `overflowing_sub` to overflow");
                        cnst
                    });
                }
            }
        }
    )+};
}
