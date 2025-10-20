#!/bin/bash
cd $1

# Check for errors
cargo check

# Build & run
#cargo build && ./target/debug/tests.exe
cargo build --release

cd -
