# protocol

This is the core crate, exposing also the user interface.

## Usage

```rust
use circuits::ConstraintSystem;
use protocol::{SRS, Trapdoor, BatchProof, Phase};
use protocol;

let (m, n, l) = (5, 4, 3);
let qap = ConstraintSystem::create_default(m, n, l).unwrap();

let (mut srs, trp) = SRS::setup_with_random_trapdoor(&qap);

let mut batch = BatchProof::initiate();
protocol::update(&qap, &mut srs, &mut batch, Phase::ONE);
protocol::update(&qap, &mut srs, &mut batch, Phase::TWO);

let result = protocol::verify(&qap, &srs, &batch);
assert!(bool::from(result));
println!("{:?}", result);
```

See also [`examples/flow.rs`](./examples/flow.rs).

## Development

```commandline
cargo test [--release]
```
