# snarky

Rust implementation of [Snarky Ceremonies](https://eprint.iacr.org/2021/219.pdf).

## Setup

```commandline
git clone https://gitlab.grnet.gr/priviledge/snarky.git
```

You need to have installed Rust. Alternatively, run the dev container:

```commandline
./run-container.sh [--help]
```

## Demo

Run the demo with:

```commandline
./demo.sh
```

To run the demo with compiler optimizations (and view measurements 
closer to those of a release version), do

```commandline
./demo.sh --release
```

This creates a QAP with shape `(m, n, l) = (50, 40, 30)`. You can control these
parameters by directly passing them to the command, that is,

To further take advantage of your CPU's architecture, run 

```commandline
RUSTFLAGS="-C target-cpu=native" ./demo.sh --release ...
```

```commandline
./demo.sh [--release] 500 400 300
```

## Usage

See [`protocol`](./protocol)

## Development

### Tests

```commandline
cargo test [--release]
```

### Benchmarks

```commandline
cargo bench
```

### Documentation

```commandline
cargo doc --open
```
