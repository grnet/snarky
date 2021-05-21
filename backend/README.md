# backend

Wrapper of the bilinear-pairing library in use; currently around
[arkworks-rs/curves/bls12_381](https://github.com/arkworks-rs/curves/tree/master/bls12_381).

## Overview

Purpose of this crate is to expose the following elliptic-curve agnostic macros,
so that multiple backends can easily be pluggable.


## Demo

```commandline
cargo run --examples main [--release]
```

## Development

```commandline
cargo test [--release]
```

## Usage

See also [`examples/main.rs`](./examples/main.rs).

```rust
use backend::*
```

This will make available the following macros:

### `scalar`

Field element from primitive numerical value.

```rust
let seven = scalar!(7u64);
```

### `zero`

Zero field element.

```rust
let zero = zero!();
assert!(scalar!(0u64), zero)
```

### `one`

Unit field element.

```rust
let one = one!();
assert!(scalar!(1u64), one)
```

### `genG1`

Generator of G1.

```rust
let G = genG1!();
```

### `genG2`

Generator of G2.

```rust
let H = genG2!();
```

### `zeroG1`

G1 zero element (additive neutral).

```rust
let zero1 = zeroG1!();
```

### `zeroG2`

G2 zero element (additive neutral).

```rust
let zero2 = zeroG2!();
```

### `unit`

GT unit element (multiplicative neutral).

```rust
let unit = unit!();
```

### `contained_in_group`

Checks that the provided element lies on the appropriate curve.

```rust
let G = genG1!();
assert!(contained_in_group!(G));
```

### `bytes1`

Byte representation of G1 elements

```rust
let G = genG1!();
let export = bytes1!(G);
```

### `bytes2`

Byte representation of G2 elements

```rust
let H = genG2!();
let export = bytes2!(G);
```

### `ct_eq`

Constant-time equality check between algebraic entities. 

```rust
assert!(ct_eq!(one!(), scalar!(1u64)));
```

**WARNING!**: This is for the moment common equality check. It is
constant time only provided that equality provided by the wrapped
library (currently [arkworks-rs/curves/bls12_381](https://github.com/arkworks-rs/curves/tree/master/bls12_381))
is really constant-time.

### `ct_ne`

Constant-time inequality check between algebraic entities.

```rust
assert!(ct_eq!(zero!(), scalar!(1u64)));
```

**WARNING!**: This is for the moment common inequality check. It is
constant time only provided that equality provided by the wrapped
library (currently [arkworks-rs/curves/bls12_381](https://github.com/arkworks-rs/curves/tree/master/bls12_381))
is really constant-time.

### `inv`

Field element inversion.

```rust
let seven = scalar!(7u64);
let invseven = inv!(seven);
```

### `pow`

Field element raising to power

```rust
let base = scalar!(7u64);
let exp = 10_usize;
let result = pow!(base, exp);
```

### `add1`

G1 group operation (addition).

```rust
add1!(elem1, elemN);
```

**NOTE**: No need to use it for the moment, as addition operator
is overload for G1 elements in the currently wrapped library (arkworks):

```
assert!(add1!(G, G, G), G + G + G);
```

### `add2`

G2 group operation (addition).

```rust
add2!(elem1, elemN);
```

**NOTE**: No need to use it for the moment, as addition operator
is overload for G2 elements in the currently wrapped library (arkworks):

```
assert!(add2!(H, H, H), H + H + H);
```

### `smul1`

Action of scalars over G1 elements (multiplicative)

```rust
let factor = scalar!(7u64);
let G = genG1!();
let result = smul1!(factor, G);   // 7G
```

### `smul2`

Action of scalars over G2 elements (multiplicative)

```rust
let factor = scalar!(7u64);
let H = genG2!();
let result = smul1!(factor, H);   // 7H
```

### `pair`

The bilinear pairing.

```rust
let G = genG1!();
let H = genG2!();
let result = pair!(G, H);         // G * H
```

### `hashG1`

Hash byte sequence as G1 element.

```rust
use std::convert::TryInto;
use std::io::Cursor;
use sha2::Digest;
use ark_ff::FromBytes;

let bytes: Vec<u8> = (0..5).collect();
let digest = hashG1!(&bytes);
```
