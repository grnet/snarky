#!/bin/bash

usage_string="usage: ./$(basename "$0") [ARGS] [OPTIONS]

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
 ./$(basename "$0") --shape 50 40 30 --phases 12 10 --release
 ./$(basename "$0") --phases 50 50 --naive
"

usage() { echo -n "$usage_string" 1>&2; }

MDIM=50
NDIM=40
LDIM=30
PHASE1=4
PHASE2=3
NAIVE=false

cargo_opts=()

while [[ $# -gt 0 ]]
do
	arg="$1"
	case $arg in
		--shape)
      MDIM="$2"
      NDIM="$3"
      LDIM="$4"
			shift
			shift
			shift
			shift
			;;
		--phases)
      PHASE1="$2"
      PHASE2="$3"
			shift
			shift
			shift
			;;
		-r|--release)
      cargo_opts+=($arg)
			shift
			;;
		# --naive)
		# 	NAIVE=true
		# 	shift
		# 	;;
		-h|--help)
			usage
			exit 0
			;;
		*)
			echo "[-] Invalid argument: $arg"
			echo
			usage
			exit 1
			;;
	esac
done
 
cargo run --example flow $cargo_opts \
   $MDIM \
   $NDIM \
   $LDIM \
   $PHASE1 \
   $PHASE2 
   # $NAIVE
