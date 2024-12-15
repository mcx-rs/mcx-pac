#!/usr/bin/env bash

svdtools=svdtools

if ! type "$svdtools" >/dev/null 2>/dev/null; then
    cargo install --root .cargo-root svdtools
    svdtools=./.cargo-root/bin/svdtools
fi

for i in svds/MCX*.yaml; do
    echo "Patchingg $i"
    $svdtools patch $i
done

echo "Generating crate"
RUST_LOG=mcx_pac_generator=debug cargo run --release -- crate

echo "Formatting output crate"
cargo fmt --manifest-path output/Cargo.toml
