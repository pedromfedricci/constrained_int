// Defines containers, errors, common impls and doc values for integers.
macro_rules! constrained_def_impl {
    (   $Int:ty, $md:ident, $Ty:ident, $Err:ident, $MinErr:ident, $MaxErr:ident,
        $min:literal..=$max:literal, ($l:literal, $h:literal)
    ) => {
        // This const function is used to enforce constraints for the range definition.
        // Relevant const generics are: `MIN`, `MAX`.
        // The constraints are:
        //     - `MAX` must be greater than `MIN`.
        // This ensures that `MIN` defines the lower inclusive bound and `MAX`
        // defines the upper inclusive bound. Also prevents range definitions
        // with single values.
        #[must_use]
        #[inline(always)]
        const fn guard_range<const MIN: $Int, const MAX: $Int>() -> bool {
            MIN < MAX
        }

        // This const function is used to enforce constraints for the default value.
        // Relevant const generics are: `MIN`, `MAX` and `DEF`.
        // The constraints are:
        //     - `DEF` must be equal to, or greater than `MIN`.
        //     - `DEF` must be equal to, or lower than `MAX`.
        // This ensures that DEF can't be out of range bounds.
        #[must_use]
        #[inline(always)]
        const fn guard_default<const MIN: $Int, const MAX: $Int, const DEF: $Int>() -> bool {
            DEF >= MIN && DEF <= MAX
        }

        // This const function is used to enforce constraints for wrapping arithmetics.
        // Relevant const generics are: `MIN`, `MAX`.
        // The constraints are:
        //     - `MAX` must be greater than `MIN`.
        //     - `MIN` must be greater than the type's `MIN`, **OR**
        //       `MAX` must be lower than the type's `MAX`.
        // This ensures that we can always represent the range's size using
        // a integer with the same length as the contained one, and also ensures the
        // range definition constraints of `guard_range`.
        #[must_use]
        #[inline(always)]
        const fn guard_arithmetics<const MIN: $Int, const MAX: $Int>() -> bool {
            ( MIN > <$Int>::MIN || MAX < <$Int>::MAX ) && guard_range::<MIN, MAX>()
        }

        // This const function is used to enforce constraints for the containers construction.
        // Relevant const generics are: `MIN`, `MAX` and `DEF`.
        // The constraints are:
        //     - `MAX` must be greater than `MIN`.
        //     - `MIN` must be greater than the type's `MIN`, **OR**
        //       `MAX` must be lower than the type's `MAX`.
        //     - `DEF` must be equal to, or greater than `MIN`.
        //     - `DEF` must be equal to, or lower than `MAX`.
        // This ensures that types can only be constructed when all constraints are satisfied.
        #[must_use]
        #[inline(always)]
        const fn guard_construction<const MIN: $Int, const MAX: $Int, const DEF: $Int>() -> bool {
            guard_arithmetics::<MIN, MAX>() && guard_default::<MIN, MAX, DEF>()
        }

        #[doc = concat!("An [`", stringify!($Int), "`] value that is constrained within an inclusive range.")]
        ///
        /// The range is defined at compile time, by assigning values to the parameters
        /// `MIN` and `MAX`. `MIN` indicates the lower **inclusive** bound of the range,
        /// while `MAX` indicateds the upper **inclusive** bound. The value will always
        /// be contained within the defined range once it's constructed.
        ///
        /// The condition `MAX` > `MIN` **must** be satified, or else the type can't
        /// be constructed.
        ///
        /// A default can be supplied by assigning a value to the parameter `DEF`. The
        /// default value must be contained by the range, meaning: `MIN` <= `DEF` <= `MAX`.
        /// Or else the type can't be constructed.
        ///
        /// It's also required that either `MIN` to be greater than the primitive's `MIN`
        /// or `MAX` to be lower than the primitive's `MAX`, or else the type can't be
        /// constructed.
        ///
        /// # Examples
        ///
        /// If the provided parameters satisfy the construction condition, associated
        /// constants and type constructors are acessible.
        /// ```
        #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
        ///
        #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
        ///
        #[doc = concat!("assert_eq!(Constrained::MIN, ", stringify!($min), ");")]
        #[doc = concat!("assert_eq!(Constrained::MAX, ", stringify!($max), ");")]
        #[doc = concat!("assert_eq!(Constrained::DEF, ", stringify!($min), ");")]
        ///
        #[doc = concat!("let constrained = Constrained::new(", stringify!($min), ")?;")]
        #[doc = concat!("assert_eq!(constrained.get(), ", stringify!($min), ");")]
        #[doc = concat!("# Ok::<(), constrained_int::", stringify!($md), "::", stringify!($Err),
            "<", stringify!($min, $max), ">>(())")]
        /// ```
        ///
        /// Associated constants and type constructors are guarded against parameters
        /// that violate the `MAX` > `MIN` condition.
        /// ```compile_fail
        #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
        ///
        /// // MIN greater or equal to MAX does not satisfy the construction condition.
        #[doc = concat!("type InvalidRange = ", stringify!($Ty), "<", stringify!($max, $min), ">;")]
        ///
        /// // None of these will compile for InvalidRange.
        /// let value = InvalidRange::MIN;
        /// let constrained = InvalidRange::default();
        /// let constrained = InvalidRange::min();
        /// /* ...other constructors */
        /// ```
        #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[repr(transparent)]
        pub struct $Ty<const MIN: $Int, const MAX: $Int, const DEF: $Int = MIN>($Int);

        // The `guard` protects this type's constructors by only implementing them for
        // generic parameter values that comply with the enforced conditions for construction.
        #[::const_guards::guard(<const MIN: $Int, const MAX: $Int, const DEF: $Int> { guard_construction::<MIN, MAX, DEF>() })]
        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> $Ty<MIN, MAX, DEF> {
            /// The minimum **inclusive** value that this type can hold.
            ///
            /// It's assigned the `MAX` parameter value. **Always** satisfies the
            /// condition: [`MIN`] < [`MAX`].
            ///
            #[doc = concat!("[`MIN`]: ", stringify!($Ty), "::MIN")]
            #[doc = concat!("[`MAX`]: ", stringify!($Ty), "::MAX")]
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            #[doc =concat!("assert_eq!(Constrained::MIN, ", stringify!($min), ");")]
            /// ```
            pub const MIN: $Int = MIN;

            /// The maximum **inclusive** value that this type can hold.
            ///
            /// It's assigned the `MIN` parameter value. **Always** satisfies the
            /// condition: [`MAX`] > [`MIN`].
            ///
            #[doc = concat!("[`MAX`]: ", stringify!($Ty), "::MAX")]
            #[doc = concat!("[`MIN`]: ", stringify!($Ty), "::MIN")]
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            #[doc =concat!("assert_eq!(Constrained::MAX, ", stringify!($max), ");")]
            /// ```
            pub const MAX: $Int = MAX;

            /// The initialized value when constructed with [`default()`].
            ///
            /// It's assigned the `DEF` parameter value.
            /// **Always** satisfies the condition: [`MIN`] <= [`DEF`] <= [`MAX`].
            ///
            /// [`default()`]: core::default::Default::default
            #[doc = concat!("[`MIN`]: ", stringify!($Ty), "::MIN")]
            #[doc = concat!("[`DEF`]: ", stringify!($Ty), "::DEF")]
            #[doc = concat!("[`MAX`]: ", stringify!($Ty), "::MAX")]
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max, $max), ">;")]
            ///
            #[doc =concat!("assert_eq!(Constrained::DEF, ", stringify!($max), ");")]
            /// ```
            ///
            pub const DEF: $Int = DEF;

            /// Creates a new instance with provided value, if it satifies the range's
            /// inclusive bounds. If the provided value is out of bounds, an error is
            /// returned, indicating which bound was violated.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            #[doc = concat!("let min = Constrained::new(", stringify!($min), ")?;")]
            #[doc = concat!("assert_eq!(min.get(), ", stringify!($min), ");")]
            ///
            #[doc = concat!("let max = Constrained::new(", stringify!($max), ")?;")]
            #[doc = concat!("assert_eq!(max.get(), ", stringify!($max), ");")]
            ///
            /// // Out of inclusive bounds.
            #[doc = concat!("assert!(Constrained::new(", stringify!($l), ").is_err());")]
            #[doc = concat!("assert!(Constrained::new(", stringify!($h), ").is_err());")]
            #[doc = concat!("# Ok::<(), constrained_int::", stringify!($md), "::", stringify!($Err),
                "<", stringify!($min, $max), ">>(())")]
            /// ```
            pub const fn new(value: $Int) -> Result<Self, $Err<MIN, MAX>> {
                Self::new_unguarded(value)
            }

            /// Creates a new instance with provided value, if it satifies the range's
            /// inclusive bounds. If provided value is out of bounds, the new instance
            /// is initialized with the value of the closest bound.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            /// // Below lower bound, so it constructs with lower bound value.
            #[doc = concat!("let min = Constrained::saturating_new(", stringify!($l), ");")]
            #[doc = concat!("assert_eq!(min.get(), ", stringify!($min), ");")]
            ///
            /// // Above upper bound, so it constructs with upper bound value.
            #[doc = concat!("let max = Constrained::saturating_new(", stringify!($h), ");")]
            #[doc = concat!("assert_eq!(max.get(), ", stringify!($max), ");")]
            /// ```
            #[must_use]
            pub const fn saturating_new(value: $Int) -> Self {
                Self::saturating_new_unguarded(value)
            }

            /// Creates a new instance with provided value, if it satifies the range's
            /// inclusive bounds. If provided value is out of bounds, a [`None`] is
            /// returned.
            ///
            /// [`None`]: core::option::Option::None
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            #[doc = concat!("let min = Constrained::checked_new(", stringify!($min), ").unwrap();")]
            #[doc = concat!("assert_eq!(min.get(), ", stringify!($min), ");")]
            ///
            #[doc = concat!("let max = Constrained::checked_new(", stringify!($max), ").unwrap();")]
            #[doc = concat!("assert_eq!(max.get(), ", stringify!($max), ");")]
            ///
            /// // Out of inclusive bounds.
            #[doc = concat!("assert_eq!(None, Constrained::checked_new(", stringify!($l), "));")]
            #[doc = concat!("assert_eq!(None, Constrained::checked_new(", stringify!($h), "));")]
            /// ```
            #[must_use]
            pub const fn checked_new(value: $Int) -> Option<Self> {
                Self::checked_new_unguarded(value)
            }

            /// Creates a new instance with the value defined by the range's lower bound.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            /// let min = Constrained::new_min();
            #[doc = concat!("assert_eq!(min.get(), ", stringify!($min), ");")]
            /// ```
            #[must_use]
            #[inline(always)]
            pub const fn new_min() -> Self {
                Self(MIN)
            }

            /// Creates a new instance with the value defined by the range's upper bound.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            /// let max = Constrained::new_max();
            #[doc = concat!("assert_eq!(max.get(), ", stringify!($max), ");")]
            /// ```
            #[must_use]
            #[inline(always)]
            pub const fn new_max() -> Self {
                Self(MAX)
            }

            /// Returns a [`RangeInclusive`] value corresponding to the type's bondaries.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            /// let range = Constrained::range();
            /// assert_eq!(Constrained::MIN, *range.start());
            /// assert_eq!(Constrained::MAX, *range.end());
            /// ```
            /// [`RangeInclusive`]: core::ops::RangeInclusive
            #[must_use]
            #[inline(always)]
            pub const fn range() -> ::core::ops::RangeInclusive<$Int> {
                ::core::ops::RangeInclusive::new(MIN, MAX)
            }
        }

        // Guard this constructor.
        #[::const_guards::guard(<const MIN: $Int, const MAX: $Int, const DEF: $Int> { guard_construction::<MIN, MAX, DEF>() })]
        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> Default for $Ty<MIN, MAX, DEF> {
            #[must_use]
            #[inline(always)]
            fn default() -> Self {
                Self(DEF)
            }
        }

        // Guard this constructor.
        #[::const_guards::guard(<const MIN: $Int, const MAX: $Int, const DEF: $Int> { guard_construction::<MIN, MAX, DEF>() })]
        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> TryFrom<$Int> for $Ty<MIN, MAX, DEF> {
            type Error = $Err<MIN, MAX>;

            fn try_from(value: $Int) -> Result<Self, Self::Error> {
                Self::new_unguarded(value)
            }
        }

        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> $Ty<MIN, MAX, DEF> {
            /// Checks if value is within the defined range, assuming that `MAX` < `MIN`
            /// is an impossible state.
            const fn in_range(value: $Int) -> Result<(), $Err<MIN, MAX>> {
                debug_assert!(!(MIN > MAX), "`MIN` can't be greater than `MAX`");

                if value > MAX {
                    Err($Err::greater())
                } else if value < MIN {
                    Err($Err::lower())
                } else {
                    Ok(())
                }
            }

            /// Unguarded private `new` constructor.
            // TODO: visibilty should be `pub(crate)`.
            const fn new_unguarded(value: $Int) -> Result<Self, $Err<MIN, MAX>> {
                // Can't use `?` operator on const fn yet:
                // https://github.com/rust-lang/rust/issues/74935.
                match Self::in_range(value) {
                    Ok(_) => Ok(Self(value)),
                    Err(err) => Err(err),
                }
            }

            /// Unguarded private `saturating` constructor.
            #[must_use]
            const fn saturating_new_unguarded(value: $Int) -> Self {
                match Self::in_range(value) {
                    Ok(_) => Self(value),
                    Err($Err::Greater(_)) => Self(MAX),
                    Err($Err::Lower(_)) => Self(MIN),
                }
            }

            /// Unguarded private `checked` constructor.
            #[must_use]
            const fn checked_new_unguarded(value: $Int) -> Option<Self> {
                match Self::in_range(value) {
                    Ok(_) => Some(Self(value)),
                    Err(_) => None,
                }
            }

            /// Sets the contained value, if it satifies the range's inclusive bounds. If
            /// the provided value is out of bounds, an error is returned, indicating
            /// which bound was violated.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            /// let mut constrained = Constrained::default();
            #[doc = concat!("assert_eq!(constrained.get(), ", stringify!($min), ");")]
            ///
            #[doc = concat!("constrained.set(", stringify!($max), ")?;")]
            #[doc = concat!("assert_eq!(constrained.get(), ", stringify!($max), ");")]
            ///
            /// // Out of inclusive bounds.
            #[doc = concat!("assert!(constrained.set(", stringify!($l), ").is_err());")]
            #[doc = concat!("assert!(constrained.set(", stringify!($h), ").is_err());")]
            ///
            #[doc = concat!("assert_eq!(constrained.get(), ", stringify!($max), ");")]
            #[doc = concat!("# Ok::<(), constrained_int::", stringify!($md), "::", stringify!($Err),
                "<", stringify!($min, $max), ">>(())")]
            /// ```
            pub fn set(&mut self, value: $Int) -> Result<(), $Err<MIN, MAX>> {
                Self::in_range(value)?;
                self.0 = value;
                Ok(())
            }

            /// Returns the value of the contained integer type.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Ty), ";")]
            ///
            #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
            ///
            /// let constrained = Constrained::default();
            #[doc = concat!("assert_eq!(constrained.get(), ", stringify!($min), ");")]
            /// ```
            #[must_use]
            #[inline(always)]
            pub const fn get(&self) -> $Int {
                self.0
            }

            /// This function is **not** part of the public API, it's subject to change
            /// without any prior notice.
            // TODO: remove and reuse new_unguarded as `pub(crate)`.
            // Unfortunate workaround.
            // Issue: https://github.com/Mari-W/const_guards/issues/2.
            #[doc(hidden)]
            #[cfg(any(test, feature = "serde"))]
            pub const fn __new(value: $Int) -> Result<Self, $Err<MIN, MAX>> {
                Self::new_unguarded(value)
            }
        }

        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> ::core::ops::RangeBounds<$Int> for $Ty<MIN, MAX, DEF> {
            #[must_use]
            #[inline(always)]
            fn start_bound(&self) -> ::core::ops::Bound<&$Int> {
                ::core::ops::Bound::Included(&MIN)
            }

            #[must_use]
            #[inline(always)]
            fn end_bound(&self) -> ::core::ops::Bound<&$Int> {
                ::core::ops::Bound::Included(&MAX)
            }
        }

        #[cfg(feature = "serde")]
        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> ::serde::Serialize for $Ty<MIN, MAX, DEF> {
            #[inline]
            fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                self.get().serialize(serializer)
            }
        }

        // Implements some ::core::fmt traits for `Constrained` types.
        constrained_fmt_impl! { Debug, Display, Binary, Octal, LowerHex, UpperHex for $Ty($Int) }

        #[doc = concat!("This error indicates that a [`", stringify!($Int), "`] value ")]
        /// violates the range's lower bound.
        ///
        /// If this crate's `std` feature is enabled, this error implements the standard
        /// library's `Error` trait.
        ///
        /// # Example
        ///
        /// ```
        #[doc = concat!("use constrained_int::", stringify!($md), "::{", stringify!($Ty), ", ", stringify!($Err) , "};")]
        ///
        #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
        ///
        /// // Lower than the lower bound.
        #[doc = concat!("match Constrained::new(", stringify!($l), ") {")]
        #[doc = concat!("    Err(", stringify!($Err), "::Lower(err))"," => drop(err),")]
        ///     _ => unreachable!(),
        /// }
        /// ```
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $MinErr<const MIN: $Int>(());

        impl<const MIN: $Int> $MinErr<MIN> {
            /// The minimum **inclusive** bound enforced by the range.
            pub const MIN: $Int = MIN;

            #[must_use]
            #[inline(always)]
            const fn new() -> Self {
                Self(())
            }
        }

        impl<const MIN: $Int> ::core::fmt::Display for $MinErr<MIN> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "value must be greater or equal to {MIN}")
            }
        }

        #[cfg(feature = "std")]
        impl<const MIN: $Int> ::std::error::Error for $MinErr<MIN> { }

        #[doc = concat!("This error indicates that a [`", stringify!($Int), "`] value ")]
        /// violates the range's upper bound.
        ///
        /// If this crate's `std` feature is enabled, this error implements the standard
        /// library's `Error` trait.
        ///
        /// # Example
        ///
        /// ```
        #[doc = concat!("use constrained_int::", stringify!($md), "::{", stringify!($Ty), ", ", stringify!($Err) , "};")]
        ///
        #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
        ///
        /// // Greater than the upper bound.
        #[doc = concat!("match Constrained::new(", stringify!($h), ") {")]
        #[doc = concat!("    Err(", stringify!($Err), "::Greater(err))"," => drop(err),")]
        ///     _ => unreachable!(),
        /// }
        /// ```
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $MaxErr<const MAX: $Int>(());

        impl<const MAX: $Int> $MaxErr<MAX> {
            /// The maximum **inclusive** bound enforced by the range.
            pub const MAX: $Int = MAX;

            #[must_use]
            #[inline(always)]
            const fn new() -> Self {
                Self(())
            }
        }

        impl<const MAX: $Int> ::core::fmt::Display for $MaxErr<MAX> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "value must be lower or equal to {MAX}")
            }
        }

        #[cfg(feature = "std")]
        impl<const MAX: $Int> ::std::error::Error for $MaxErr<MAX> { }

        #[doc = concat!("An error that indicates which range bound was violated by a [`", stringify!($Int), "`] value.")]
        ///
        #[doc = concat!("This error can be returned from fallible APIs for [`", stringify!($Ty), "`].")]
        ///
        #[doc = concat!("The [`Lower`] variant indicates that the value is lower than [`", stringify!($Ty), "::MIN`].")]
        ///
        #[doc = concat!("The [`Greater`] variant indicates that the value is greater than [`", stringify!($Ty), "::MAX`].")]
        ///
        /// If this crate's `std` feature is enabled, this error implements the standard
        /// library's `Error` trait.
        ///
        #[doc = concat!("[`Lower`]: ", stringify!($Err), "::Lower")]
        #[doc = concat!("[`Greater`]: ", stringify!($Err), "::Greater")]
        ///
        /// # Examples
        ///
        /// Variants can be pattern matched to identify which bound was violated.
        /// ```
        #[doc = concat!("use constrained_int::", stringify!($md), "::{", stringify!($Ty), ", ", stringify!($Err) , "};")]
        ///
        #[doc = concat!("type Constrained = ", stringify!($Ty), "<", stringify!($min, $max), ">;")]
        ///
        /// // First is below lower bound, second is above upper bound.
        #[doc = concat!("let results = [Constrained::new(", stringify!($l), "), Constrained::new(", stringify!($h), ")];")]
        /// for result in results {
        ///     match result {
        #[doc = concat!("        Err(", stringify!($Err), "::Lower(_)) => /*...*/ (),")]
        #[doc = concat!("        Err(", stringify!($Err), "::Greater(_)) => /*...*/ (),")]
        ///         _ => unreachable!(),
        ///     }
        /// }
        /// ```
        ///
        /// Associated constants are guarded against parameters that violate the
        /// `MAX` > `MIN` condition.
        /// ```compile_fail
        #[doc = concat!("use constrained_int::", stringify!($md), "::{", stringify!($Ty), ", ", stringify!($Err) , "};")]
        ///
        /// // MIN greater or equal to MAX is invalid.
        #[doc = concat!("type InvalidRange = ", stringify!($Ty), "<", stringify!($max, $min), ">;")]
        ///
        /// // None of these will compile for InvalidRange.
        /// let min = InvalidRange::MIN;
        /// let max = InvalidRange::MAX;
        /// ```
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum $Err<const MIN: $Int, const MAX: $Int> {
            // All variants hold a type that users can't construct themselves, preventing
            // this type from being directly constructed by them.
            //
            // This ensures that users can't create this type with parameters that don't
            // follow the condition `MAX` > `MIN`.
            #[doc = concat!("Indicates that the provided value is lower than [`", stringify!($Ty), "::MIN`].")]
            Lower($MinErr<MIN>),

            #[doc = concat!("Indicates that the provided value is greater than [`", stringify!($Ty), "::MAX`].")]
            Greater($MaxErr<MAX>),
        }

        #[::const_guards::guard(<const MIN: $Int, const MAX: $Int> { guard_range::<MIN, MAX>() })]
        impl<const MIN: $Int, const MAX: $Int> $Err<MIN, MAX> {
            /// The minimum **inclusive** bound enforced by the range.
            ///
            /// It's assigned the `MIN` parameter value. Always satisfies the condition:
            /// [`MIN`] < [`MAX`].
            ///
            #[doc = concat!("[`MIN`]: ", stringify!($Err), "::MIN")]
            #[doc = concat!("[`MAX`]: ", stringify!($Err), "::MAX")]
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Err), ";")]
            ///
            #[doc = concat!("type ConstrainedError = ", stringify!($Err), "<", stringify!($min, $max), ">;")]
            ///
            #[doc = concat!("assert_eq!(ConstrainedError::MIN,", stringify!($min) , ");")]
            /// ```
            pub const MIN: $Int = MIN;

            /// The maximum **inclusive** bound enforced by the range.
            ///
            /// It's assigned the `MAX` parameter value. Always satisfies the condition:
            /// [`MAX`] > [`MIN`].
            ///
            #[doc = concat!("[`MAX`]: ", stringify!($Err), "::MAX")]
            #[doc = concat!("[`MIN`]: ", stringify!($Err), "::MIN")]
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("use constrained_int::", stringify!($md), "::", stringify!($Err), ";")]
            ///
            #[doc = concat!("type ConstrainedError = ", stringify!($Err), "<", stringify!($min, $max), ">;")]
            ///
            #[doc = concat!("assert_eq!(ConstrainedError::MAX,", stringify!($max) , ");")]
            /// ```
            pub const MAX: $Int = MAX;
        }

        impl<const MIN: $Int, const MAX: $Int> $Err<MIN, MAX> {
            /// Returns `Lower` variant.
            #[must_use]
            #[inline(always)]
            const fn lower() -> Self {
                Self::Lower($MinErr::<MIN>::new())
            }

            /// Returns `Greater` variant.
            #[must_use]
            #[inline(always)]
            const fn greater() -> Self {
                Self::Greater($MaxErr::<MAX>::new())
            }
        }

        impl<const MIN: $Int, const MAX: $Int> From<$MinErr<MIN>> for $Err<MIN, MAX> {
            #[inline(always)]
            fn from(err: $MinErr<MIN>) -> Self {
                Self::Lower(err)
            }
        }

        impl<const MIN: $Int, const MAX: $Int> From<$MaxErr<MAX>> for $Err<MIN, MAX> {
            #[inline(always)]
            fn from(err: $MaxErr<MAX>) -> Self {
                Self::Greater(err)
            }
        }

        impl<const MIN: $Int, const MAX: $Int> ::core::fmt::Display for $Err<MIN, MAX> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    Self::Lower(err) => err.fmt(f),
                    Self::Greater(err) => err.fmt(f),
                }
            }
        }

        #[cfg(feature = "std")]
        impl<const MIN: $Int, const MAX: $Int> ::std::error::Error for $Err<MIN, MAX> { }
    };
}

// Implemets ::core::fmt traits for containers.
macro_rules! constrained_fmt_impl {
    ($($Trait:ident),+ for $Ty:ident($Int:ty)) => {$(
        impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> ::core::fmt::$Trait for $Ty<MIN, MAX, DEF> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                self.get().fmt(f)
            }
        }
    )+};
}

// Implements tests that are common to both signed and unsigned.
#[cfg(test)]
macro_rules! tests_common {
    ($Int:ty, $ty_path:path, $Ty:ident, $Err:ident, $MinErr:ident, $MaxErr:ident) => {
        use $ty_path::*;

        type CnstTest = $Ty<{ <$Int>::MIN + 1 }, { <$Int>::MAX - 1 }>;
        const GREATER_MAX: $Int = <$Int>::MAX;
        const LOWER_MIN: $Int = <$Int>::MIN;

        #[test]
        fn new_bounded() {
            let mut constrained: CnstTest;

            constrained = CnstTest::new(CnstTest::MIN).expect("in range value");
            assert_eq!(constrained.get(), CnstTest::MIN);

            constrained = CnstTest::new(CnstTest::MAX).expect("in range value");
            assert_eq!(constrained.get(), CnstTest::MAX);
        }

        #[test]
        fn new_unbounded() {
            let err = CnstTest::new(LOWER_MIN).expect_err("expected value lower than `MIN`");
            assert_eq!(err, $Err::lower());

            let err = CnstTest::new(GREATER_MAX).expect_err("expected value greater than `MAX`");
            assert_eq!(err, $Err::greater());
        }

        #[test]
        fn saturating_new_bounded() {
            let mut constrained: CnstTest;

            constrained = CnstTest::saturating_new(CnstTest::MIN);
            assert_eq!(constrained.get(), CnstTest::MIN);

            constrained = CnstTest::saturating_new(CnstTest::MAX);
            assert_eq!(constrained.get(), CnstTest::MAX);
        }

        #[test]
        fn saturating_new_unbounded() {
            let mut constrained: CnstTest;

            constrained = CnstTest::saturating_new(LOWER_MIN);
            assert_eq!(constrained.get(), CnstTest::MIN);

            constrained = CnstTest::saturating_new(GREATER_MAX);
            assert_eq!(constrained.get(), CnstTest::MAX);
        }

        #[test]
        fn checked_new_bounded() {
            let mut constrained: CnstTest;

            constrained = CnstTest::checked_new(CnstTest::MIN).expect("in range value");
            assert_eq!(constrained.get(), CnstTest::MIN);

            constrained = CnstTest::checked_new(CnstTest::MAX).expect("in range value");
            assert_eq!(constrained.get(), CnstTest::MAX);
        }

        #[test]
        fn checked_new_unbounded() {
            assert_eq!(CnstTest::checked_new(LOWER_MIN), None);

            assert_eq!(CnstTest::checked_new(GREATER_MAX), None);
        }

        #[test]
        fn constrained_set_bounded() {
            let mut constrained = CnstTest::default();

            constrained.set(CnstTest::MIN).expect("in range value");
            assert_eq!(constrained.get(), CnstTest::MIN);

            constrained.set(CnstTest::MAX).expect("in range value");
            assert_eq!(constrained.get(), CnstTest::MAX);
        }

        #[test]
        fn constrained_set_unbounded() {
            let mut constrained = CnstTest::default();

            let err = constrained.set(LOWER_MIN).expect_err("expected value lower than `MIN`");
            assert_eq!(err, $Err::lower());

            let err = constrained.set(GREATER_MAX).expect_err("expected value greater than `MAX`");
            assert_eq!(err, $Err::greater())
        }

        #[test]
        fn constrained_default() {
            assert_eq!($Ty::<1, 3, 2>::default().get(), 2);
            assert_eq!(CnstTest::default().get(), CnstTest::MIN);
        }

        #[test]
        fn constrained_new_min_and_max() {
            assert_eq!(CnstTest::new_min().get(), CnstTest::MIN);
            assert_eq!(CnstTest::new_max().get(), CnstTest::MAX);
        }

        #[test]
        fn constrained_associated_consts() {
            type Constrained = $Ty<{ <$Int>::MIN }, { <$Int>::MAX - 1 }, { 0 }>;
            assert_eq!(Constrained::MIN, <$Int>::MIN);
            assert_eq!(Constrained::MAX, <$Int>::MAX - 1);
            assert_eq!(Constrained::DEF, 0);
        }

        #[test]
        fn constrained_range_bounds() {
            use ::core::ops::{Bound, RangeBounds};
            let constrained = CnstTest::default();

            let start_bound = constrained.start_bound();
            assert_eq!(start_bound, Bound::Included(&CnstTest::MIN));

            let end_bound = constrained.end_bound();
            assert_eq!(end_bound, Bound::Included(&CnstTest::MAX));
        }

        #[test]
        fn constrained_range_inclusive() {
            let range = CnstTest::range();
            assert_eq!(CnstTest::MIN, *range.start());
            assert_eq!(CnstTest::MAX, *range.end());
        }

        #[test]
        fn constrained_try_from_bounded() {
            let cnst = CnstTest::try_from(CnstTest::MIN).expect("expected in range value");
            assert_eq!(cnst.get(), CnstTest::MIN);

            let cnst = CnstTest::try_from(CnstTest::MAX).expect("expected in range value");
            assert_eq!(cnst.get(), CnstTest::MAX);
        }

        #[test]
        fn constrained_try_from_unbounded() {
            let err = CnstTest::try_from(LOWER_MIN).expect_err("expected value lower than `MIN`");
            assert_eq!(err, $Err::lower());

            let err =
                CnstTest::try_from(GREATER_MAX).expect_err("expected value greater than `MAX`");
            assert_eq!(err, $Err::greater());
        }

        #[cfg(feature = "std")]
        #[test]
        fn constrained_fmt_impl() {
            let cnst = CnstTest::default();
            // Debug
            assert_eq!(format!("{:?}", cnst), format!("{:?}", cnst.0));
            // Display
            assert_eq!(format!("{}", cnst), format!("{}", cnst.0));
            // Binary
            assert_eq!(format!("{:b}", cnst), format!("{:b}", cnst.0));
            // Octal
            assert_eq!(format!("{:o}", cnst), format!("{:o}", cnst.0));
            // LowerHex
            assert_eq!(format!("{:x}", cnst), format!("{:x}", cnst.0));
            // UpperHex
            assert_eq!(format!("{:X}", cnst), format!("{:X}", cnst.0));
        }

        #[test]
        fn variant_errs_associated_const() {
            type TestMin = $MinErr<{ <$Int>::MIN }>;
            assert_eq!(TestMin::MIN, <$Int>::MIN);

            type TestMax = $MaxErr<{ <$Int>::MAX }>;
            assert_eq!(TestMax::MAX, <$Int>::MAX);
        }

        #[test]
        fn err_associated_consts() {
            type TestError = $Err<{ <$Int>::MIN }, { <$Int>::MAX - 1 }>;
            assert_eq!(TestError::MIN, <$Int>::MIN);
            assert_eq!(TestError::MAX, <$Int>::MAX - 1);
        }

        #[test]
        fn from_variants_types_for_err() {
            type TestError = $Err<{ <$Int>::MIN }, { <$Int>::MAX - 1 }>;
            type TestMinErr = $MinErr<{ <$Int>::MIN }>;
            type TestMaxErr = $MaxErr<{ <$Int>::MAX - 1 }>;

            let min_err: TestMinErr = $MinErr(());
            let err: TestError = $Err::from(min_err);
            assert_eq!(err, $Err::Lower(min_err));

            let max_err: TestMaxErr = $MaxErr(());
            let err: TestError = $Err::from(max_err);
            assert_eq!(err, $Err::Greater(max_err));
        }

        #[cfg(feature = "std")]
        #[test]
        fn err_transparent_display() {
            type TestError = $Err<{ <$Int>::MIN }, { <$Int>::MAX - 1 }>;
            type TestMinErr = $MinErr<{ <$Int>::MIN }>;
            type TestMaxErr = $MaxErr<{ <$Int>::MAX - 1 }>;

            let min_err: TestMinErr = $MinErr(());
            let err: TestError = $Err::lower();
            assert_eq!(err.to_string(), min_err.to_string());

            let max_err: TestMaxErr = $MaxErr(());
            let err: TestError = $Err::greater();
            assert_eq!(err.to_string(), max_err.to_string());
        }

        #[cfg(feature = "std")]
        #[test]
        fn min_err_display_impl() {
            type TestMin = $MinErr<{ <$Int>::MIN }>;

            let min_err: TestMin = $MinErr(());
            assert_eq!(
                min_err.to_string(),
                format!("value must be greater or equal to {}", TestMin::MIN)
            );
        }

        #[cfg(feature = "std")]
        #[test]
        fn max_err_display_impl() {
            type TestMax = $MaxErr<{ <$Int>::MAX }>;

            let max_err: TestMax = $MaxErr(());
            assert_eq!(
                max_err.to_string(),
                format!("value must be lower or equal to {}", TestMax::MAX)
            );
        }
    };
}
