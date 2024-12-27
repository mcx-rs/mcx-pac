#!/usr/bin/env python3

import os
from pathlib import Path


devices = []
for p in Path("./devices").glob("*.yaml"):
    devices.append(p.name.removesuffix(".yaml").lower())
print(devices)

# generate device.rs
with open("./src/device.rs", "w") as f:
    for device in devices:
        f.write(
            f'#[cfg_attr(feature = "{device}", path = "devices/{device}/device.rs")]\n'
        )
    f.write('#[cfg(feature = "_device_selected")]\n')
    f.write("mod _device;\n")
    f.write('#[cfg(feature = "_device_selected")]\n')
    f.write("pub use _device::*;\n")

# generate Cargo.toml
with open("Cargo.toml", "w") as f:
    f.write(
        """# This manifest is generated automatically, edit this in script/generate.py
[package]
name = "mcx-pac"
description = "Peripheral Access Crate for NXP MCX Series MCUs"
version = "0.1.1"
edition = "2021"
license = "MIT"
keywords = ["no-std", "arm", "cortex-m", "nxp", "mcx"]
categories = ["embedded", "no-std"]
readme = "README.md"
authors = ["Logiase Song <logiase.song@gmail.com>"]
repository = "https://github.com/mcx-rs/mcx-pac"
include = ["/src", "build.rs", "/examples", "LICENSE-MIT", "LICENSE-BSD"]

[dependencies]
cortex-m = "0.7"
cortex-m-rt = { version = "0.7", optional = true }

[dev-dependencies]
panic-halt = "1"

[features]
rt = ["cortex-m-rt/device"]

_device_selected = []
"""
    )

    for device in devices:
        f.write(f'{device} = ["_device_selected"]\n')
