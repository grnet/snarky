# backend

Wrapper of the bilinear-pairing library.
Currently around [`bls12_381`](https://docs.rs/bls12_381/0.4.0/bls12_381/).

## Overview

Purpose of this crate is to expose the following elliptic-curve agnostic macros, 
so that the backend can easily be unplugged in the future.

- `scalar`: field element from primitive numerical value
- `zero`: zero field element
- `one`: unit field element
- `genG1`: generator of G1
- `genG2`: generator of G2
- `zeroG1`: G1 neutral element
- `zeroG2`: G2 neutral element
- `contained_in_group`: checks that the input lies on the group curve
- `bytes1`: byte representation of G1 elements
- `bytes2`: byte representation of G2 elements
- `ct_eq`: constant-time equality check between algebraic entities
- `ct_ne`: constant-time inequality check between algebraic entities
- `inv`: field element inversion
- `pow`: field element raising to power
- `add1`: G1 group operation
- `add2`: G2 group operation
- `smul1`: scalar multiplication over G1
- `smul2`: scalar multiplication over G2
- `pair`: the bilinear pairing
- `hashG1`: hash byte sequence as G1 element

## Usage

See [`examples/main.rs`](./examples/main.rs)

## Development

```commandline
cargo test [--release]
```
