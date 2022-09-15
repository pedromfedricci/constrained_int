// implements binary operators "&T op U", "T op &U", "&T op &U"
// based on "T op U" where T and U are expected to be `Copy`able.
// Requires `const_trait_impl` features.
macro_rules! forward_ref_binop {
    // Const ops implementation.
    // This implementation is equivalent to the non-const version,
    // but with the additional `const` keyword.
    (impl$(<$(const $c:ident: $i:ty),+>)? const $imp:ident<$u:ty>, $method:ident for $t:ty) => {
        impl$(<'a, $(const $c: $i,)+>)? const $imp<$u> for &'a $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, rhs: $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, rhs)
            }
        }

        impl$(<$(const $c: $i,)+>)? const $imp<&$u> for $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, rhs: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$method(self, *rhs)
            }
        }

        impl$(<$(const $c: $i,)+>)? const $imp<&$u> for &$t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, rhs: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, *rhs)
            }
        }
    };

    // Non-const ops implementation.
    (impl$(<$(const $c:ident: $i:ty),+>)? $imp:ident<$u:ty>, $method:ident for $t:ty) => {
        impl$(<'a, $(const $c: $i,)+>)? $imp<$u> for &'a $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, rhs: $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, rhs)
            }
        }

        impl$(<$(const $c: $i,)+>)? $imp<&$u> for $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, rhs: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$method(self, *rhs)
            }
        }

        impl$(<$(const $c: $i,)+>)? $imp<&$u> for &$t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, rhs: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, *rhs)
            }
        }
    };
}

// implements "T op= &U", based on "T op= U"
// where U is expected to be `Copy`able.
// Requires `const_trait_impl` and `const_mut_refs` features.
macro_rules! forward_ref_op_assign {
    // Const ops implementation.
    // This implementation is equivalent to the non-const version,
    // but with the additional `const` keyword.
    (impl$(<$(const $c:ident: $i:ty),+>)? const $imp:ident<$u:ty>, $method:ident for $t:ty) => {
        impl$(<$(const $c: $i,)+>)? const $imp<&$u> for $t {
            #[inline]
            fn $method(&mut self, rhs: &$u) {
                $imp::$method(self, *rhs);
            }
        }
    };

    // Non-cont ops implementation.
    (impl$(<$(const $c:ident: $i:ty),+>)? $imp:ident<$u:ty>, $method:ident for $t:ty) => {
        impl$(<$(const $c: $i,)+>)? $imp<&$u> for $t {
            #[inline]
            fn $method(&mut self, rhs: &$u) {
                $imp::$method(self, *rhs);
            }
        }
    };
}
