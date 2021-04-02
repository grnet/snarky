# snarky

Rust implementation of [Snarky Ceremonies](https://eprint.iacr.org/2021/219.pdf).

## Setup

```commandline
git clone https://gitlab.grnet.gr/priviledge/snarky.git --recurse-submodules
```

You need to have installed Rust (`cargo>=1.51.0`). Alternatively, assuming you
have Docker, you can work in the dev container:

```commandline
./run-container.sh
```

## Demo

Run the demo with:

```commandline
./demo.sh
```

To run the demo with compiler optimizations (and view measurements closer to
those of a release version), do

```commandline
./demo.sh --release
```

## Usage

## Development

```commandline
git submodule update --remote
```

```commandline
./run-container.sh [--help]
```

### Tests

```commandline
cargo test
```

### Benchmarks

```commandline
cargo bench
```

### Documentation

```commandline
cargo doc --open
```
