#!/bin/bash
cd $1

# Build & run
#cargo build && ./target/debug/tests.exe
cargo run

cd -
