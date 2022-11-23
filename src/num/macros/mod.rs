// Import:
// - `arithmetic_wrapper_common!`.
#[macro_use]
mod common;

// Import:
// - `arithmetic_wrapper_int_specific!`.
#[macro_use]
mod int;

// Import:
// - `wrapping_int!`.
// - `wrapping_uint!`.
//
// Required:
// - `arithmetic_wrapper_common!`.
// - `arithmetic_wrapper_int_specific!`.
#[macro_use]
mod wrapping;

// Import:
// - `saturating_int!`.
// - `saturating_uint!`.
//
// Required:
// - `arithmetic_wrapper_common!`.
// - `arithmetic_wrapper_int_specific!`.
#[macro_use]
mod saturating;
