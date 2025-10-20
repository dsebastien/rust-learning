#!/bin/bash
cd $1
cargo build && ./target/debug/tests.exe
cd -
