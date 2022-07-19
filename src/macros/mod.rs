// Import `constrained_def_impl!` macro.
#[macro_use]
mod common;

// Import `constrained_uint_def_impl!` macro.
// Requires `constrained_def_impl!` macro.
#[macro_use]
mod uint;

// Import `constrained_int_def_impl!` macro.
// Requires `constrained_def_impl!` macro.
#[macro_use]
mod int;
