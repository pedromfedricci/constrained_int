# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1] - (12. August, 2022)

### Added on 0.2.1

- range() API for `Constrained` types.
- Mixed integer operations for `Constrained` types ([#1]).
- Implements serde's `Serialize` and `Deserialize` traits for `Constrained` types, behind
  the `serde` feature ([#2]).

[#1]: https://github.com/pedromfedricci/constrained_int/pull/1
[#2]: https://github.com/pedromfedricci/constrained_int/pull/2

## [0.2.0] - (11. July, 2022)

### Added on 0.2.0

- Support for `no_std` environments.

### Changed on 0.2.0

- Bump `const_guard` to 0.1.3, which supports `no_std` environments.

## [0.1.2] - (10. July, 2022) - **Yanked**

### Added on 0.1.2

- Implements `From<MinError>` and `From<MaxError>` for `ConstrainedError` without
  requiring the `std` feature.

### Removed on 0.1.2

- Crate no longer imports thiserror crate.

## [0.1.1] - (08. July, 2022) - **Yanked**

### Added on 0.1.1

- ~~Support for `no_std` environments.~~ **Only on 0.2.0.**

## [0.1.0] - (07. July, 2022) - **Yanked**

### Added on 0.1.0

- Initial release.
