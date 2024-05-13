#!/bin/bash

make -C data transform

RUST_LOG=debug cargo run -p pacgen -- all

echo "Formatting..."
cargo fmt
for f in src/peripherals/*; do
    rustfmt $f
done

for f in src/devices/*/pac.rs; do
    rustfmt $f
done