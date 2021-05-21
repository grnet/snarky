# circuits

Provides the constraint-system.

## Overview

Builds on top of [`polynomials`]('../polynomials')

## Usage

```rust
use circuits::ConstraintSystem;

let (m, n, l) = (50, 40, 30);
let qap = ConstraintSystem::create_default(m, n, l).unwrap()
```

## Development

```commandline
cargo test [--release]
```




