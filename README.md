# rust-appro-eq
[![crates.io badge](https://img.shields.io/crates/v/appro-eq.svg)](https://crates.io/crates/appro-eq)
[![Build Status](https://travis-ci.org/chalharu/rust-appro-eq.svg?branch=master)](https://travis-ci.org/chalharu/rust-appro-eq)
[![docs.rs](https://docs.rs/appro-eq/badge.svg)](https://docs.rs/appro_eq)
[![Coverage Status](https://coveralls.io/repos/github/chalharu/rust-appro-eq/badge.svg?branch=master)](https://coveralls.io/github/chalharu/rust-appro-eq?branch=master)

Implementing the `ApproEq` traits, Can asserts that the two expressions are approximately equal to each other.

## How-to Use
See the [crate documentation](https://docs.rs/appro-eq/) for more details.

### Examples

```rust
assert_appro_eq!(1f64, 1.5f64, 0.6f64); // does not panic
assert_appro_eq!(0f64, 1e-12f64); // does not panic
```

```rust:should_panic
assert_appro_eq!(1f64, 2f64); // panics
```

### Optional Features

- **`complex`** - Implement `ApproEq` traits for `num_complex::Complex`. This adds a dependency on the `num-complex` crate.

- **`rational`** - Implement `ApproEq` traits for `num_rational::Ratio`. This adds a dependency on the `num-rational` crate.

- **`ndarray`** - Implement `ApproEq` traits for `ndarray::ArrayBase`. This adds a dependency on the `ndarray` crate.

- **`i128`** - Implement `ApproEq` traits for `i128` and `u128`. **Available only on Rust nightly channel.**
