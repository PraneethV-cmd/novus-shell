#!/bin/bash

set -e # Exit if any command fails

cargo build --release

# Use exec to directly run the compiled binary
exec ./target/release/$(basename "$(pwd)") "$@"

