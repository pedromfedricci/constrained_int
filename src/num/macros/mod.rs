// Import:
// - `arithmetic_wrapper_serde_impl!`.
// - `arithmetic_wrapper_fmt_impl!`.
// - `arithmetic_wrapper_ops_impl!`.
// - `arithmetic_wrapper_tests_common!`.
#[macro_use]
mod common;

// Import:
// - `arithmetic_wrapper_int_impl!`.
//
// Required:
// - `arithmetic_wrapper_ops_impl!`.
// - `arithmetic_wrapper_tests_common!`.
#[macro_use]
mod int;

#[macro_use]
mod wrapping;

#[macro_use]
mod saturating;
