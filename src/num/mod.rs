// Import:
// - `arithmetic_wrapper_traits_impl!`.
// - `wrapping_int!`.
// - `wrapping_uint!`.
// - `saturating_int!`.
// - `saturating_uint!`.
#[macro_use]
mod macros;

// Requires:
// - `arithmetic_wrapper_traits_impl!`.
// - `wrapping_int!`.
// - `wrapping_uint!`.
mod wrapping;
pub use wrapping::Wrapping;

// Requires:
// - `arithmetic_wrapper_traits_impl!`.
// - `saturating_int!`.
// - `saturating_uint!`.
mod saturating;
pub use saturating::Saturating;
