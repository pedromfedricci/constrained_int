macro_rules! constrained_deserialize_impl {
    ($Num:ty, $num_mod:ident, $Cnst:ident, $deserialize:ident, $($method:ident!($Inner:ty, $($Visit:ty : $visit:ident)*);)*) => {
        #[cfg(feature = "serde")]
        impl<'de, const MIN: $Num, const MAX: $Num, const DEF: $Num> ::serde::Deserialize<'de>
            for crate::$num_mod::$Cnst<MIN, MAX, DEF>
        {
            fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                use ::serde::de::{Error as DesError, Visitor, Unexpected};
                use ::core::fmt::{Formatter, Result as FmtResult};
                // use crate::$num_mod::guard_construction;

                struct ConstrainedVisitor<const MIN: $Num, const MAX: $Num, const DEF: $Num>;

                // TODO: reuse modules's guard instead.
                // Unfortunate workaround.
                // Issue: https://github.com/Mari-W/const_guards/issues/2.
                #[inline(always)]
                const fn guard_construction<const MIN: $Num, const MAX: $Num, const DEF: $Num>() -> bool {
                    (MIN < MAX) && (DEF >= MIN && DEF <= MAX)  && (MIN > <$Num>::MIN || MAX < <$Num>::MAX)
                }

                impl<const MIN: $Num, const MAX: $Num, const DEF: $Num> ConstrainedVisitor<MIN, MAX, DEF> {
                    fn guard_construction<E: DesError>(&self) -> Result<(), E> {
                        if guard_construction::<MIN, MAX, DEF>() {
                            Ok(())
                        } else {
                            Err(E::invalid_type(Unexpected::Other("invalid range definition"), self))
                        }
                    }
                }

                impl<const MIN: $Num, const MAX: $Num, const DEF: $Num> Visitor<'_>
                    for ConstrainedVisitor<MIN, MAX, DEF>
                {
                    type Value = crate::$num_mod::$Cnst<MIN, MAX, DEF>;

                    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                        write!(f, "a constrained {} value within {MIN}..={MAX}", stringify!($Num))
                    }

                    $($($method!($Inner, $Visit : $visit);)*)*
                }

                deserializer.$deserialize(ConstrainedVisitor)
            }
        }
    };
}

// Equivalent to serde's `num_self!` and `num_as_self!` but for uint only.
macro_rules! num_as_self_uint {
    ($UnsInner:ty, $UnsInt:ty : $visit:ident) => {
        fn $visit<E: DesError>(self, v: $UnsInt) -> Result<Self::Value, E> {
            self.guard_construction()?;
            Self::Value::__new(v as $UnsInner)
                .map_err(|_| E::invalid_value(Unexpected::Unsigned(v as u64), &self))
        }
    };
}

// Equivalent to serde's `num_self!` and `num_as_self!` but for sint only.
macro_rules! num_as_self_int {
    ($SigInner:ty, $SigInt:ty : $visit:ident) => {
        fn $visit<E: DesError>(self, v: $SigInt) -> Result<Self::Value, E> {
            self.guard_construction()?;
            Self::Value::__new(v as $SigInner)
                .map_err(|_| E::invalid_value(Unexpected::Signed(v as i64), &self))
        }
    };
}

macro_rules! uint_to_self {
    ($Inner:ty, $UnsInt:ty : $visit:ident) => {
        fn $visit<E: DesError>(self, v: $UnsInt) -> Result<Self::Value, E> {
            self.guard_construction()?;

            if v as u64 <= <$Inner>::MAX as u64 {
                if let Ok(value) = Self::Value::__new(v as $Inner) {
                    return Ok(value);
                }
            }
            Err(E::invalid_value(Unexpected::Unsigned(v as u64), &self))
        }
    };
}

macro_rules! int_to_int {
    ($SigInner:ty, $SigInt:ty : $visit:ident) => {
        fn $visit<E: DesError>(self, v: $SigInt) -> Result<Self::Value, E> {
            self.guard_construction()?;

            if <$SigInner>::MIN as i64 <= v as i64 && v as i64 <= <$SigInner>::MAX as i64 {
                if let Ok(value) = Self::Value::__new(v as $SigInner) {
                    return Ok(value);
                }
            }
            Err(E::invalid_value(Unexpected::Signed(v as i64), &self))
        }
    };
}

macro_rules! int_to_uint {
    ($UnsInner:ty, $SigInt:ty : $visit:ident) => {
        fn $visit<E: DesError>(self, v: $SigInt) -> Result<Self::Value, E> {
            self.guard_construction()?;

            if 0 < v && v as u64 <= <$UnsInner>::MAX as u64 {
                if let Ok(value) = Self::Value::__new(v as $UnsInner) {
                    return Ok(value);
                }
            }
            Err(E::invalid_value(Unexpected::Signed(v as i64), &self))
        }
    };
}
