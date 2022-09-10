// implements binary operators "&T op U", "T op &U", "&T op &U"
// based on "T op U" where T and U are expected to be `Copy`able.
// Requires `const_trait_impl` features.
macro_rules! forward_ref_binop {
    // Const ops implementation.
    // This implementation is equivalent to the non-const version,
    // but with the additional `const` keyword.
    (impl<const $int:ty> const $imp:ident<$y:ident<$u:ident>>, $method:ident for $w:ident<$t:ident>) => {
        impl<'a, const MIN: $int, const MAX: $int, const DEF: $int> const
            $imp<$y<$u<MIN, MAX, DEF>>> for &'a $w<$t<MIN, MAX, DEF>>
        {
            type Output = <$w<$t<MIN, MAX, DEF>> as $imp<$y<$u<MIN, MAX, DEF>>>>::Output;

            #[inline]
            fn $method(
                self,
                rhs: $y<$u<MIN, MAX, DEF>>,
            ) -> <$w<$t<MIN, MAX, DEF>> as $imp<$y<$u<MIN, MAX, DEF>>>>::Output {
                $imp::$method(*self, rhs)
            }
        }

        impl<const MIN: $int, const MAX: $int, const DEF: $int> const $imp<&$y<$u<MIN, MAX, DEF>>>
            for $w<$t<MIN, MAX, DEF>>
        {
            type Output = <$w<$t<MIN, MAX, DEF>> as $imp<$y<$u<MIN, MAX, DEF>>>>::Output;

            #[inline]
            fn $method(
                self,
                rhs: &$y<$u<MIN, MAX, DEF>>,
            ) -> <$w<$t<MIN, MAX, DEF>> as $imp<$y<$u<MIN, MAX, DEF>>>>::Output {
                $imp::$method(self, *rhs)
            }
        }

        impl<const MIN: $int, const MAX: $int, const DEF: $int> const $imp<&$y<$u<MIN, MAX, DEF>>>
            for &$w<$t<MIN, MAX, DEF>>
        {
            type Output = <$w<$t<MIN, MAX, DEF>> as $imp<$y<$u<MIN, MAX, DEF>>>>::Output;

            #[inline]
            fn $method(
                self,
                rhs: &$y<$u<MIN, MAX, DEF>>,
            ) -> <$w<$t<MIN, MAX, DEF>> as $imp<$y<$u<MIN, MAX, DEF>>>>::Output {
                $imp::$method(*self, *rhs)
            }
        }
    };

    // Non-const ops implementation.
    (impl<const $int:ty> $imp:ident<$y:ident<$u:ident>>, $method:ident for $w:ident<$t:ident>) => {
        impl<'a, const MIN: $int, const MAX: $int, const DEF: $int> $imp<$y<$u<MIN, MAX, DEF>>>
            for &'a $w<$t<MIN, MAX, DEF>>
        {
            type Output = <$w<$t<MIN, MAX, DEF>> as $imp<$y<$u<MIN, MAX, DEF>>>>::Output;

            #[inline]
            fn $method(
                self,
                rhs: $y<$u<MIN, MAX, DEF>>,
            ) -> <$w<$t<MIN, MAX, DEF>> as $imp<$y<$u<MIN, MAX, DEF>>>>::Output {
                $imp::$method(*self, rhs)
            }
        }

        impl<const MIN: $int, const MAX: $int, const DEF: $int> $imp<&$y<$u<MIN, MAX, DEF>>>
            for $w<$t<MIN, MAX, DEF>>
        {
            type Output = <$w<$t<MIN, MAX, DEF>> as $imp<$y<$u<MIN, MAX, DEF>>>>::Output;

            #[inline]
            fn $method(
                self,
                rhs: &$y<$u<MIN, MAX, DEF>>,
            ) -> <$w<$t<MIN, MAX, DEF>> as $imp<$y<$u<MIN, MAX, DEF>>>>::Output {
                $imp::$method(self, *rhs)
            }
        }

        impl<const MIN: $int, const MAX: $int, const DEF: $int> $imp<&$y<$u<MIN, MAX, DEF>>>
            for &$w<$t<MIN, MAX, DEF>>
        {
            type Output = <$w<$t<MIN, MAX, DEF>> as $imp<$y<$u<MIN, MAX, DEF>>>>::Output;

            #[inline]
            fn $method(
                self,
                rhs: &$y<$u<MIN, MAX, DEF>>,
            ) -> <$w<$t<MIN, MAX, DEF>> as $imp<$y<$u<MIN, MAX, DEF>>>>::Output {
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
    (impl<const $int:ty> const $imp:ident<$y:ident<$u:ident>>, $method:ident for $w:ident<$t:ident>) => {
        impl<const MIN: $int, const MAX: $int, const DEF: $int> const $imp<&$y<$u<MIN, MAX, DEF>>>
            for $w<$t<MIN, MAX, DEF>>
        {
            #[inline]
            fn $method(&mut self, rhs: &$y<$u<MIN, MAX, DEF>>) {
                $imp::$method(self, *rhs);
            }
        }
    };

    // Const ops implementation.
    (impl<const $int:ty> const $imp:ident<$y:ty>, $method:ident for $w:ident<$t:ident>) => {
        impl<const MIN: $int, const MAX: $int, const DEF: $int> const $imp<&$y>
            for $w<$t<MIN, MAX, DEF>>
        {
            #[inline]
            fn $method(&mut self, rhs: &$y) {
                $imp::$method(self, *rhs);
            }
        }
    };

    // Non-cont ops implementation.
    (impl<const $int:ty> $imp:ident<$y:ident<$u:ident>>, $method:ident for $w:ident<$t:ident>) => {
        impl<const MIN: $int, const MAX: $int, const DEF: $int> $imp<&$y<$u<MIN, MAX, DEF>>>
            for $w<$t<MIN, MAX, DEF>>
        {
            #[inline]
            fn $method(&mut self, rhs: &$y<$u<MIN, MAX, DEF>>) {
                $imp::$method(self, *rhs);
            }
        }
    };

    // Non-cont ops implementation.
    (impl<const $int:ty> $imp:ident<$y:ty>, $method:ident for $w:ident<$t:ident>) => {
        impl<const MIN: $int, const MAX: $int, const DEF: $int> $imp<&$y>
            for $w<$t<MIN, MAX, DEF>>
        {
            #[inline]
            fn $method(&mut self, rhs: &$y) {
                $imp::$method(self, *rhs);
            }
        }
    };
}
