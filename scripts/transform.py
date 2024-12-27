#!/usr/bin/env python3

import os
from pathlib import Path

transforms = []
for p in Path("transforms").glob("*.yaml"):
    transforms.append(p.name)

# transform peripherals if transform file present
for root, dir, files in os.walk("peripherals"):
    for file in files:
        if file not in transforms:
            continue

        peripheral_file_path = f"{root}/{file}"
        transform_file_path = f"transforms/{file}"
        os.system(
            f"chiptool transform -i {peripheral_file_path} -o {peripheral_file_path} -t {transform_file_path}"
        )
