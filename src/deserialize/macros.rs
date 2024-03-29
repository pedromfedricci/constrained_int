// Implements `serde::Deserialize` for `Constrained` types, checking construction
// constraints at runtime. Deserialization in almost identical to serde's impls for
// the std primitive-like types.
macro_rules! constrained_deserialize_impl {
    (   $Num:ty, $num_mod:ident, $Cnst:ident, $deserialize:ident,
        $($method:ident!($Inner:ty, $($Visit:ty : $visit:ident)*);)*
    ) => {
        impl<'de, const MIN: $Num, const MAX: $Num, const DEF: $Num> ::serde::Deserialize<'de>
            for $crate::$num_mod::$Cnst<MIN, MAX, DEF>
        where
            $crate::Constraints<{ $crate::$num_mod::guard_construction::<MIN, MAX, DEF>() }>: $crate::Guard,
        {
            fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                use ::serde::de::{Error as DesError, Visitor, Unexpected};
                use ::core::fmt::{Formatter, Result as FmtResult};

                struct ConstrainedVisitor<const MIN: $Num, const MAX: $Num, const DEF: $Num>;

                impl<const MIN: $Num, const MAX: $Num, const DEF: $Num> Visitor<'_>
                    for ConstrainedVisitor<MIN, MAX, DEF>
                where
                    $crate::Constraints<{ $crate::$num_mod::guard_construction::<MIN, MAX, DEF>() }>: $crate::Guard,
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

// Equivalent to serde's `num_self!` and `num_as_self!` but for uint vistors only.
macro_rules! num_as_self_uint {
    ($Inner:ty, $UnsInt:ty : $visit:ident) => {
        fn $visit<E: DesError>(self, v: $UnsInt) -> Result<Self::Value, E> {
            let err = |_| E::invalid_value(Unexpected::Unsigned(v as u64), &self);
            Self::Value::new(v as $Inner).map_err(err)
        }
    };
}

// Equivalent to serde's `num_self!` and `num_as_self!` but for int visitors only.
macro_rules! num_as_self_int {
    ($Inner:ty, $SigInt:ty : $visit:ident) => {
        fn $visit<E: DesError>(self, v: $SigInt) -> Result<Self::Value, E> {
            let err = |_| E::invalid_value(Unexpected::Signed(v as i64), &self);
            Self::Value::new(v as $Inner).map_err(err)
        }
    };
}

// Casts a uint visit to inner's type, if representable. Then constructs the
// container if the range definition is valid, and the value is within range.
macro_rules! uint_to_self {
    ($Inner:ty, $UnsInt:ty : $visit:ident) => {
        fn $visit<E: DesError>(self, v: $UnsInt) -> Result<Self::Value, E> {
            if v as u64 <= <$Inner>::MAX as u64 {
                if let Ok(value) = Self::Value::new(v as $Inner) {
                    return Ok(value);
                }
            }
            Err(E::invalid_value(Unexpected::Unsigned(v as u64), &self))
        }
    };
}

// Casts a int visit to inner's (int) type, if representable. Then constructs the
// container if the range definition is valid, and the value is within range.
macro_rules! int_to_int {
    ($SigInner:ty, $SigInt:ty : $visit:ident) => {
        fn $visit<E: DesError>(self, v: $SigInt) -> Result<Self::Value, E> {
            if <$SigInner>::MIN as i64 <= v as i64 && v as i64 <= <$SigInner>::MAX as i64 {
                if let Ok(value) = Self::Value::new(v as $SigInner) {
                    return Ok(value);
                }
            }
            Err(E::invalid_value(Unexpected::Signed(v as i64), &self))
        }
    };
}

// Casts a int visit to inner's (uint) type, if representable. Then constructs the
// container if the range definiton is valid, and the value is within range.
macro_rules! int_to_uint {
    ($UnsInner:ty, $SigInt:ty : $visit:ident) => {
        fn $visit<E: DesError>(self, v: $SigInt) -> Result<Self::Value, E> {
            if 0 <= v && v as u64 <= <$UnsInner>::MAX as u64 {
                if let Ok(value) = Self::Value::new(v as $UnsInner) {
                    return Ok(value);
                }
            }
            Err(E::invalid_value(Unexpected::Signed(v as i64), &self))
        }
    };
}

// Casts a 128bit visit to inner's (128bit) type, if representable. Then constructs the
// container if the range definiton is valid, and the value is within range.
macro_rules! num_128 {
    ($Inner:ty, $Visit:ty : $visit:ident) => {
        fn $visit<E: DesError>(self, v: $Visit) -> Result<Self::Value, E> {
            if v as i128 >= <$Inner>::MIN as i128 && v as u128 <= <$Inner>::MAX as u128 {
                if let Ok(value) = Self::Value::new(v as $Inner) {
                    return Ok(value);
                }
            }
            Err(E::invalid_value(Unexpected::Other(stringify!($Visit)), &self))
        }
    };
}
