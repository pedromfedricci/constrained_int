//! Integers that are constrained within inclusive ranges.
//!
//! `Constrained` types are represented simply as primitive integers, but their
//! values will **always** be contained by inclusive range bounds. The range is
//! defined at compile time, by assigning values to appropriate const generic
//! parameters. Constrained types provide fallible APIs for construction and
//! value assignment, they also implement wrapping, saturating, overflowing
//! and checked arithmetics for the range boundaries. See each desired type
//! documentation for more information.
//!
//! The `constrained_int` crate relies on the [`const_guards`] crate to define
//! compile time constraints, which itself uses the incomplete [`generic_const_exprs`]
//! feature. Therefore, this crate can only be compile with nightly and, more
//! importantly, must be considered as an **experimental** crate only.
//!
//! This crate is `no_std` by default. See features section for more information.
//!
//! ## Example
//!
//! ```
//! use constrained_int::i8::{ConstrainedI8, ConstrainedI8Error};
//!
//! // Lower bound = -5, upper bound = 10, default = -1.
//! type Constrained = ConstrainedI8<-5, 10, -1>;
//! type ConstrainedError = ConstrainedI8Error<-5, 10>;
//!
//! // Gets user defined default value.
//! let mut constrained = Constrained::default();
//! assert_eq!(constrained.get(), -1);
//!
//! // Sets within inclusive range, succeeds.
//! constrained.set(-5)?;
//! assert_eq!(constrained.get(), -5);
//!
//! // Below lower bound.
//! assert_eq!(constrained.checked_sub(1), None);
//! assert_eq!(constrained.get(), -5);
//!
//! // Saturates at upper bound.
//! constrained = constrained.saturating_add(100);
//! assert_eq!(constrained.get(), 10);
//!
//! // Sets out of bound, fails.
//! assert!(constrained.set(11).is_err());
//!
//! // Wraps around the upper bound.
//! constrained = constrained.wrapping_add(1);
//! assert_eq!(constrained.get(), -5);
//! # Ok::<(), constrained_int::i8::ConstrainedI8Error<-5, 10>>(())
//! ```
//!
//! ## Feature flags
//!
//! This crate does not link against the standard library by default, so it is
//! suitable for `no_std` environments.. It does provide a `std` feature though,
//! that enables the standard library as a dependency. By  enabling this crate's
//! `std` feature, these additional features are provided:
//!   - All crate's error types will implement the `std::error::Error` trait.
//! If users already are importing the standard library on their crate, enabling
//! `std` feature comes at no additional cost.
//!
//! [`generic_const_exprs`]: https://github.com/rust-lang/rust/issues/76560

// No raw pointers here, maybe in another castle.
#![forbid(unsafe_code)]
//
// `std` feature will import std as a dependency.
#![cfg_attr(not(feature = "std"), no_std)]
//
// The `const_guards` dependency relies on `generic_const_exprs`.
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
//
// Tracking issue for `doc_auto_cfg` feature:
// https://github.com/rust-lang/rust/issues/43781.
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
//
// rustdoc lints.
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

#[cfg(test)]
extern crate static_assertions as sa;

// Import `constrained_uint_def_impl!` macro.
// Import `constrained_int_def_impl!` macro.
#[macro_use]
mod macros;

// Define mods, containers, errors, tests and impls for unsigned integers with default
// values for doc examples.
constrained_uint_def_impl! {
//  { uint, mod_name, TypeName, ErrorName, MinErrorName, MaxErrorName },
    { u8, u8, ConstrainedU8, ConstrainedU8Error, MinU8Error, MaxU8Error },
    { u16, u16, ConstrainedU16, ConstrainedU16Error, MinU16Error, MaxU16Error },
    { u32, u32, ConstrainedU32, ConstrainedU32Error, MinU32Error, MaxU32Error },
    { u64, u64, ConstrainedU64, ConstrainedU64Error, MinU64Error, Max64Error },
    { u128, u128, ConstrainedU128, ConstrainedU128Error, Min128Error, Max128Error },
    { usize, usize, ConstrainedUsize, ConstrainedUsizeError, MinUsizeError, MaxUsizeError },
}

// Define mods, containers, errors, tests and impls for signed integers with default
// values for doc examples.
constrained_int_def_impl! {
//  { int, uint, mod_name, TypeName, ErrorName, MinErrorName, MaxErrorName },
    { i8, u8, i8, ConstrainedI8, ConstrainedI8Error, MinI8Error, MaxI8Error },
    { i16, u16, i16, ConstrainedI16, ConstrainedI16Error, MinI16Error, MaxI16Error },
    { i32, u32, i32, ConstrainedI32, ConstrainedI32Error, MinI32Error, MaxI32Error },
    { i64, u64, i64, ConstrainedI64, ConstrainedI64Error, MinI64Error, MaxI64Error },
    { i128, u128, i128, ConstrainedI128, ConstrainedI128Error, MinI128Error, MaxI128Error },
    { isize, usize, isize, ConstrainedIsize, ConstrainedIsizeError, MinIsizeError, MaxIsizeError },
}
