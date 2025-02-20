#!/usr/bin/env python3

import os
import subprocess

devices = []
for root, dir, files in os.walk("./devices"):
    for file in files:
        devices.append(file.removesuffix(".yaml").lower())
        
for device in devices:
    print(f"building device {device}")
    ret = subprocess.call(
        ["cargo", "build", "--features", f"rt,defmt,{device}"],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.STDOUT,
    )
    
    if ret != 0:
        print(f"build device {device} failed")
        print(f"run `cargo build --features rt,defmt,{device}` to see what happened")
        exit(ret)
