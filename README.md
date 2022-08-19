# Integers that are constrained within inclusive ranges

[![License][license-image]](./LICENSE-APACHE)
[![Documentation][doc-image]][doc-link]
[![Crate][crate-image]][crate-link]
[![CI][ci-image]][ci-link]
[![codecov][codecov-image]][codecov-link]
![Safety][safety-image]
![No std][no_std-image]
![Maintenance][experimental-image]

`Constrained` types are represented simply as primitive integers, but their
values will **always** be contained by inclusive range bounds. The range is
defined at compile time, by assigning values to appropriate const generic
parameters. Constrained types provide fallible APIs for construction and
value assignment, they also implement wrapping, saturating, overflowing
and checked arithmetics for the range boundaries. See each desired type
documentation for more information.

The `constrained_int` crate relies on the [const_guards] crate to define compile
time constraints, which itself uses the incomplete [generic_const_exprs]
feature. Therefore, this crate can only be compile with nightly, and, more
importantly, must be considered as an **experimental** crate only.

This crate is `no_std` by default. See features section for more information.

## Install

```toml
# Cargo.toml

[dependencies]
constrained_int = "0.2"
```

## Example

```rust
use constrained_int::i8::{ConstrainedI8, ConstrainedI8Error};

// Lower bound = -5, upper bound = 10, default = -1.
type Constrained = ConstrainedI8<-5, 10, -1>;
type ConstrainedError = ConstrainedI8Error<-5, 10>;

fn main() -> Result<(), ConstrainedError> {
    // Gets the default value.
    let mut constrained = Constrained::default();
    assert_eq!(constrained.get(), -1);

    // Sets within inclusive range, succeeds.
    constrained.set(-5)?;
    assert_eq!(constrained.get(), -5);

    // Below lower bound, fails.
    assert_eq!(constrained.checked_sub(1), None);
    assert_eq!(constrained.get(), -5);

    // Saturates at the upper bound.
    constrained = constrained.saturating_add(100);
    assert_eq!(constrained.get(), 10);

    // Sets out of boundary, fails.
    assert!(constrained.set(11).is_err());

    // Wraps around the upper bound.
    constrained = constrained.wrapping_add(1);
    assert_eq!(constrained.get(), -5);

    Ok(())
}
```

## Documentation

This project documentation is hosted at [docs.rs][doc-link].

## Safety

This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in
100% safe Rust.

## Feature flags

This crate does not provide any default features. The features that can be
enabled are: `std` and `serde`.

### std

This crate does not link against the standard library by default, so it is
suitable for `no_std` environments. It does provide a `std` feature though,
that enables the standard library as a dependency. By enabling this crate's
`std` feature, these additional features are provided:

- All crate's error types will implement the `std::error::Error` trait.

If users already are importing the standard library on their crate, enabling
`std` feature comes at no additional cost.

### serde

The `serde` feature implements [serde]'s `Serialize` and `Deserialize` traits
for all `Constrained` types.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE] or <http://apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT] or <http://opensource.org/licenses/MIT>)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

## Code review

It is recommended to always use [cargo-crev] to verify the trustworthiness of
each of your dependencies, including this one.

[//]: # (general links)

[const_guards]: https://docs.rs/const_guards/latest/const_guards/
[generic_const_exprs]: https://github.com/rust-lang/rust/issues/76560
[serde]: https://serde.rs/
[cargo-crev]: https://github.com/crev-dev/cargo-crev
[doc-link]: https://docs.rs/constrained_int
[crate-link]: https://crates.io/crates/constrained_int
[codecov-link]: https://codecov.io/gh/pedromfedricci/constrained_int
[ci-link]: https://github.com/pedromfedricci/constrained_int/actions/workflows/ci.yaml
[LICENSE-APACHE]: ./LICENSE-APACHE
[LICENSE-MIT]: ./LICENSE-MIT

[//]: # (badges)

[safety-image]: https://img.shields.io/badge/unsafe-forbidden-success.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[ci-image]: https://github.com/pedromfedricci/constrained_int/actions/workflows/ci.yaml/badge.svg
[doc-image]: https://docs.rs/constrained_int/badge.svg
[crate-image]: https://img.shields.io/crates/v/constrained_int.svg
[codecov-image]: https://codecov.io/gh/pedromfedricci/constrained_int/branch/main/graph/badge.svg?token=5ZBKPBGE4P
[experimental-image]: https://img.shields.io/badge/maintenance-experimental-blue.svg
[no_std-image]: https://img.shields.io/badge/no__std-compatible-success.svg
