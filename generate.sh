#!/bin/bash

make -C data transform
for f in data/peripherals/*.yaml; do
    name=$(basename $f .yaml)
    echo "Generating $name.rs"
    chiptool gen-block -i $f -o src/peripherals/$name.rs
    rustfmt src/peripherals/$name.rs
done
