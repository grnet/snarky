#!/bin/bash

set -e  # Exit if any subcommand fails
cargo test --release --verbose
