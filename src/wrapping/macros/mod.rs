// Import:
// - `wrapping_fmt_impl!`.
// - `wrapping_ops_impl!`.
// - `wrapping_tests_common!`.
#[macro_use]
mod common;

// Import:
// - `wrapping_uint_impl_for!`.
//
// Required:
// - `wrapping_ops_impl!`.
// - `wrapping_tests_common!`.
#[macro_use]
mod uint;

// Import:
// - `wrapping_int_impl_for!`.
//
// Required:
// - `wrapping_ops_impl!`.
// - `wrapping_tests_common!`.
#[macro_use]
mod int;
