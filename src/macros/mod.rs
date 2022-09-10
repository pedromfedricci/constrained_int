// Import:
// - `constrained_def_impl!`.
#[macro_use]
mod common;

// Import:
// - `constrained_uint_def_impl!`.
//
// Required:
// - `constrained_def_impl!`.
#[macro_use]
mod uint;

// Import:
// - `constrained_int_def_impl!`.
//
// Required:
// - `constrained_def_impl!`.
#[macro_use]
mod int;

// Import:
// - `forward_ref_binop!`.
// - `forward_ref_op_assign!`.
#[macro_use]
mod ops;
