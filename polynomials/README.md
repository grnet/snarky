# polynomials

Provides univariate polynomials for constraint-systems.

## Overview

Polynomials evaluation proveesa by Horner's rule, without taking into account
potential sparsity. If the current backend
([`bls12_381`](https://docs.rs/bls12_381/0.4.0/bls12_381/))
is replaced by one providing sparse polynomials, make sure to use that utility and
omit the present crate from the project.

## Usage

```rust
use polynomials::Univariate;
use backend::scalar;

let poly = Univariate::create_from_u64(&vec![1, 2, 3, 4, 5]);
let x = scalar!(666);
let res = poly.evaluate(&x).unwrap();
```

## Development

```commandline
cargo test [--release]
```
