# Benchmarks for the constrained_int crate

This crate implements benchmarks for all `Constrained` types' wrapping APIs,
using [Critetion].

For signed types, the tested APIs and their corresponding benches are:

| API (int)                | Benchmark Group          |
|--------------------------|--------------------------|
| wrapping_add             | int_wrapping_add         |
| wrapping_sub             | int_wrapping_sub         |
| overflowing_add          | int_overflowing_add      |
| overflowing_sub          | int_overflowing_sub      |
| wrapping_add_unsigned    | wrapping_add_unsigned    |
| wrapping_sub_unsigned    | wrapping_sub_unsigned    |
| overflowing_add_unsigned | overflowing_add_unsigned |
| overflowing_sub_unsigned | overflowing_sub_unsigned |

For unsigned types, the tested APIs and their corresponding benches are:

| API (uint)             | Benchmark Group        |
|------------------------|------------------------|
| wrapping_add           | uint_wrapping_add      |
| wrapping_sub           | uint_wrapping_sub      |
| overflowing_add        | uint_overflowing_add   |
| overflowing_sub        | uint_overflowing_sub   |
| wrapping_add_signed    | wrapping_add_signed    |
| overflowing_add_signed | overflowing_add_signed |

## Running benches

Choose one of the benchmark groups and run it as the following examples:

```bash
cargo bench --package benches --bench uint_wrapping_add
cargo bench --package benches --bench overflowing_add_signed
```

## Reports access

Benchmark results are stored at `target/criterion` as html files, the main index is at
`target/criterion/report/index.html`. For more information about selecting specific
benchmarks or general CLI options see the [book].

[//]: # (general links)

[Critetion]: https://crates.io/crates/criterion
[book]: https://bheisler.github.io/criterion.rs/book/user_guide/command_line_options.html
