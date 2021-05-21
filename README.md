# snarky

Rust implementation of [Snarky Ceremonies](https://eprint.iacr.org/2021/219.pdf).

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

## Usage

See [`protocol`](./protocol)

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
cargo bench
```

### Documentation

```commandline
cargo doc --open
```
