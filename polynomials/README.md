# polynomials

Provides univariate polynomials for constraint-systems.

## Overview

A thin wrapper around
[arkworks-rs/algebra/poly](https://github.com/arkworks-rs/algebra/tree/master/poly)

## Usage

```rust
use polynomials::Univariate;
use backend::scalar;

let poly = Univariate::create_from_u64(&vec![1, 2, 3, 4, 5]);
let x = scalar!(666);
let res = poly.evaluate(&x);
```

See also [`examples/main.rs`](./examples/main.rs).

## Development

```commandline
cargo test [--release]
```
