// Import:
// - `arithmetic_wrapper_common_impl!`.
// - `arithmetic_wrapper_uint_impl_for!`.
// - `arithmetic_wrapper_int_impl_for!`.
#[macro_use]
mod macros;

// Requires:
// - all macros.
mod saturating;
mod wrapping;

pub use saturating::Saturating;
pub use wrapping::Wrapping;
