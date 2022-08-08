macro_rules! constrained_deserialize_impl {
    ($Int:ty, $int_md:ident, $Ty:ident, $deserialize:ident, $($method:ident!($Inner:ty, $($Arg:ty : $visit:ident)*);)*) => {
        #[cfg(feature = "serde")]
        impl<'de, const MIN: $Int, const MAX: $Int, const DEF: $Int> ::serde::Deserialize<'de>
            for crate::$int_md::$Ty<MIN, MAX, DEF>
        {
            fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                use ::serde::de::{Error as DesError, Visitor, Unexpected};
                use ::core::fmt::{Formatter, Result as FmtResult};
                // use crate::$int_md::guard_construction;

                struct ConstrainedVisitor<const MIN: $Int, const MAX: $Int, const DEF: $Int>;

                // TODO: reuse modules's guard instead.
                // Unfortunate workaround.
                // Issue: https://github.com/Mari-W/const_guards/issues/2.
                #[inline(always)]
                const fn guard_construction<const MIN: $Int, const MAX: $Int, const DEF: $Int>() -> bool {
                    (MIN < MAX) && (DEF >= MIN && DEF <= MAX)  && (MIN > <$Int>::MIN || MAX < <$Int>::MAX)
                }

                impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> ConstrainedVisitor<MIN, MAX, DEF> {
                    fn guard_construction<E: DesError>(&self) -> Result<(), E> {
                        if guard_construction::<MIN, MAX, DEF>() {
                            Ok(())
                        } else {
                            Err(E::invalid_type(Unexpected::Other("invalid range definition"), self))
                        }
                    }
                }

                impl<const MIN: $Int, const MAX: $Int, const DEF: $Int> Visitor<'_>
                    for ConstrainedVisitor<MIN, MAX, DEF>
                {
                    type Value = crate::$int_md::$Ty<MIN, MAX, DEF>;

                    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                        write!(f, "a constrained {} value within {MIN}..={MAX}", stringify!($Int))
                    }

                    $($($method!($Inner, $Arg : $visit);)*)*
                }

                deserializer.$deserialize(ConstrainedVisitor)
            }
        }
    };
}

// Equivalent to serde's `num_self!` and `num_as_self!` but for uint only.
macro_rules! num_as_self_uint {
    ($UnsInt:ty, $Arg:ty : $visit:ident) => {
        fn $visit<E: DesError>(self, v: $Arg) -> Result<Self::Value, E> {
            self.guard_construction()?;
            Self::Value::__new(v as $UnsInt)
                .map_err(|_| E::invalid_value(Unexpected::Unsigned(v as u64), &self))
        }
    };
}

// Equivalent to serde's `num_self!` and `num_as_self!` but for sint only.
macro_rules! num_as_self_int {
    ($SigInt:ty, $Arg:ty : $visit:ident) => {
        fn $visit<E: DesError>(self, v: $Arg) -> Result<Self::Value, E> {
            self.guard_construction()?;
            Self::Value::__new(v as $SigInt)
                .map_err(|_| E::invalid_value(Unexpected::Signed(v as i64), &self))
        }
    };
}

macro_rules! uint_to_self {
    ($UnsInt:ty, $Arg:ty : $visit:ident) => {
        fn $visit<E: DesError>(self, v: $Arg) -> Result<Self::Value, E> {
            self.guard_construction()?;

            if v as u64 <= <$UnsInt>::MAX as u64 {
                if let Ok(value) = Self::Value::__new(v as $UnsInt) {
                    return Ok(value);
                }
            }
            Err(E::invalid_value(Unexpected::Unsigned(v as u64), &self))
        }
    };
}

macro_rules! int_to_int {
    ($SigInt:ty, $Arg:ty : $visit:ident) => {
        fn $visit<E: DesError>(self, v: $Arg) -> Result<Self::Value, E> {
            self.guard_construction()?;

            if <$SigInt>::MIN as i64 <= v as i64 && v as i64 <= <$SigInt>::MAX as i64 {
                if let Ok(value) = Self::Value::__new(v as $SigInt) {
                    return Ok(value);
                }
            }
            Err(E::invalid_value(Unexpected::Signed(v as i64), &self))
        }
    };
}

macro_rules! int_to_uint {
    ($UnsInt:ty, $Arg:ty : $visit:ident) => {
        fn $visit<E: DesError>(self, v: $Arg) -> Result<Self::Value, E> {
            self.guard_construction()?;

            if 0 < v && v as u64 <= <$UnsInt>::MAX as u64 {
                Self::Value::__new(v as $UnsInt)
                    .map_err(|_| E::invalid_value(Unexpected::Unsigned(v as u64), &self))
            } else {
                Err(E::invalid_value(Unexpected::Signed(v as i64), &self))
            }
        }
    };
}
