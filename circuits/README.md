# circuits

Provides the constraint-system.

## Overview

Builds on top of [`polynomials`]('../polynomials')

## Usage

```rust
use circuits::QAP;

let (m, n, l) = (50, 40, 30);
let qap = QAP::create_default(m, n, l).unwrap()
```

## Development

```commandline
cargo test [--release]
```




