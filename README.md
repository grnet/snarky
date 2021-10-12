# snarky

Rust implementation of the [Snarky Ceremonies](https://eprint.iacr.org/2021/219.pdf) protocol
over the BLS12-381 curve.

## Overview

Snarky fully implements the roles of prover and verifier of the protocol along with 
its constraint-system, SRS and proof structures. This includes the SRS
generation (ceremony) and update procedures. Verfication procedures have been
optimized by the technique of batching for handling a large number of bilinear
operations. It intends to serve as the first experimental version of a production-grade 
library for real-life applications.

### Usage

See [`protocol`](./protocol)

### Optimizations

The protocol layer builds on top of polynomial and group-theoretic structures which use 
[arkworks](https://github.com/arkworks-rs/algebra) as backend. Further boost to performance 
is given by application of data-parallellism ([rayon](https://github.com/rayon-rs/rayon)), 
which dynamically adapts the workload of iterators under account of runtime.

### Security

Unsafe Rust is nowhere used for the moment. Effort has been spent to mitigate timing 
attacks by applying constant-time operations, which included elimination of short-circuit 
evaluations and early returns. *Security review is desired*.

## Demo

```
$ ./demo.sh --help

usage: ./demo.sh [ARGS] [OPTIONS]

Simulates execution of the Snarky Ceremonies protocol

Arguments:
  --shape <m> <n> <l>   m, n, l dimensions of constraint system 
                        (default: 50 40 30)
  --phases <nr1> <n2>   Number of updates (default: 4 3)

Options:
  -r, --release     Compile in release mode (optimized)
  --naive           Run non-batched verification (non-optimized)
  -h, --help        Display help message and exit

Examples:
 ./demo.sh --shape 50 40 30 --phases 12 10 --release
 ./demo.sh --phases 50 50 --naive

```

## Development

You need to have installed Rust. Alternatively, run the dev container:

```commandline
./run-container.sh [--help]
```

### Tests

```commandline
cargo test [--release]
```

### Benchmarks

```commandline
cargo bench [--help]
```

### Documentation

```commandline
cargo doc --open
```
