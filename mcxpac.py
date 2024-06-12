import os
import re
import shutil
import subprocess
from glob import glob
from typing import Dict, List, Tuple

from jinja2 import Environment, FileSystemLoader
from svdsuite import (
    SVDCPU,
    SVDCluster,
    SVDDevice,
    SVDParser,
    SVDPeripheral,
    SVDRegister,
)

from mappings import *

_environment = Environment(loader=FileSystemLoader("templates/"))


class SVD:
    device: SVDDevice
    name: str
    irqs: List[Tuple[str, str, int]]
    peripherals: Dict[str, List[SVDPeripheral]]
    cpu: SVDCPU

    def __init__(self, device: SVDDevice, name: str):
        self.device = device
        self.name = name

        irqs = set()
        for p in device.peripherals:
            for i in p.interrupts:
                irq = (
                    i.name.upper(),
                    (
                        i.name.upper()
                        if i.description is None
                        else description_escape(i.description)
                    ),
                    i.value,
                )
                irqs.add(irq)
        irqs = list(irqs)
        irqs.sort(key=lambda i: i[2])
        self.irqs = irqs

        self.peripherals = dict()
        for p in device.peripherals:
            if p.group_name not in self.peripherals:
                self.peripherals[p.group_name] = []
            self.peripherals[p.group_name].append(p)
        for g in self.peripherals:
            contains = False
            for p in self.peripherals[g]:
                if len(p.registers_clusters) != 0:
                    if contains:
                        raise LookupError(
                            f"Group {g} has more than one definition, please check"
                        )
                    contains = True
        for k in self.peripherals:
            self.peripherals[k].sort(key=lambda x: x.name)
        self.cpu = device.cpu

    @classmethod
    def from_svd_file(cls, path: str) -> "SVD":
        device = SVDParser.for_xml_file(path).get_device()
        name = os.path.basename(path).split(".")[0]
        return cls(device, name)


def description_escape(desc: str) -> str:
    desc = desc.replace("\n", "\\n")
    desc = desc.split()
    desc = " ".join(desc)
    desc = (
        desc
        # .replace("[", "\\[")
        # .replace("]", "\\]")
        .replace('"', '\\"').replace("'", "\\'")
    )
    return desc


def sanitize_str(s: str) -> str:
    if s in KEYWORDS:
        return s + "_"
    return s


def width_to_mask(width: int) -> str:
    return "0b" + "1" * width


def render_register(r: SVDRegister, name_prefix="") -> str:
    if r.dim:
        name = r.name.replace("[%s]", "_").replace("%s", "_").removesuffix("_")
    else:
        name = r.name
    name = name_prefix + name
    return _environment.get_template("register.template").render(
        reg=r,
        width_to_mask=width_to_mask,
        name=name,
        description_escape=description_escape,
        sanitize_str=sanitize_str,
    )


def render_block(b: SVDCluster | SVDPeripheral, bit_size: int = 8) -> str:
    reserved_num = 0
    offset = 0

    out = f'#[doc = "{description_escape(b.description)}"]\n#[repr(C)]\n'
    out += "pub struct RegisterBlock {\n"

    b.registers_clusters.sort(key=lambda x: x.address_offset)
    for i in range(len(b.registers_clusters)):
        e = b.registers_clusters[i]
        if offset < e.address_offset:
            out += f"    _reserved{reserved_num}: [u8; {hex(e.address_offset - offset)}],\n"
            reserved_num += 1
            offset = e.address_offset
        name = e.name.replace("[%s]", "_").replace("%s", "_").removesuffix("_")
        name = sanitize_str(name)
        out += f'    #[doc = "{description_escape(e.description)}"]\n'
        if isinstance(e, SVDRegister):
            if e.dim:
                out += (
                    f"    pub {name}: [crate::RWRegister<u{e.size}>; {e.dim}usize],\n"
                )
                offset += e.dim * e.dim_increment
            else:
                out += f"    pub {name}: crate::RWRegister<u{e.size}>,\n"
                offset += int(e.size / bit_size)
        elif isinstance(e, SVDCluster):
            out += f"    pub {name}: [{name.lower()}::RegisterBlock; {e.dim}usize],\n"
            offset += e.dim * e.dim_increment
        else:
            raise TypeError()

    out += "}"
    return out


def render_peripheral(p: SVDPeripheral, bit_size: int = 8) -> str:
    b = (
        "#![allow(\n"
        "    non_camel_case_types,\n"
        "    non_snake_case,\n"
        "    non_upper_case_globals,\n"
        "    dead_code\n"
        ")]\n"
    )
    b += render_block(p, bit_size)
    b += "\n"

    for e in p.registers_clusters:
        if isinstance(e, SVDRegister):
            b += render_register(e) + "\n"
        elif isinstance(e, SVDCluster):
            name = e.name.replace("[%s]", "%s").replace("%s", "_").removesuffix("_")
            name = sanitize_str(name)
            b += f"pub mod {name.lower()} {{"
            b += render_peripheral(e, bit_size)
            b += "}\n"
        else:
            raise TypeError()
    return b


def generate_vectors(irqs: List[Tuple[str, str, int]]) -> str:
    template = _environment.get_template("vectors.template")
    vectors = []
    vectors_index = 0
    for i in irqs:
        while vectors_index < i[2]:
            vectors.append("Vector { _reserved: 0 },")
            vectors_index += 1
        vectors.append(f"Vector {{ _handler: {i[0]} }},")
        vectors_index += 1

    return template.render(
        irqs=irqs,
        vectors=vectors,
        description_escape=description_escape,
        sanitize_str=sanitize_str,
    )


def generate_peripherals(svds: Dict[str, SVD]):
    shutil.rmtree("src/peripherals")
    os.makedirs("src/peripherals", exist_ok=True)

    for k in PERIPHERALS_MAPPING:
        from_ = PERIPHERALS_MAPPING[k]["from"]
        g = None
        for r in PERIPHERALS_MAPPING[k]["mapping"]:
            if re.match(r, from_):
                g = PERIPHERALS_MAPPING[k]["mapping"][r][0]
                break
        if g is None:
            raise ValueError(f"Peripheral {k} not found")

        with open(f"src/peripherals/{k.lower()}.rs", "w") as f:
            out = render_peripheral(svds[from_].peripherals[g][0])
            f.write(out)


def generate_device(svds: Dict[str, SVD]):
    shutil.rmtree("src/devices")
    out = ""
    instances = []
    template = _environment.get_template("block.template")
    device_template = _environment.get_template("device.template")

    for d in svds:
        os.makedirs(f"src/devices/{d}")
        for g in svds[d].peripherals:
            path = None
            for p in PERIPHERALS_MAPPING:
                for r in PERIPHERALS_MAPPING[p]["mapping"]:
                    if not re.match(r, d):
                        continue
                    if g in PERIPHERALS_MAPPING[p]["mapping"][r]:
                        path = p
                        break
                if path:
                    break
            out += (
                template.render(
                    name=g, path=path, peripherals=svds[d].peripherals[g], hex=hex
                )
                + "\n"
            )
            if svds[d].peripherals[g].__len__() == 1:
                pname = svds[d].peripherals[g][0].name.upper()
                instances.append({"name": pname, "type": f"{g.lower()}::{pname}"})
                # instances += f"    pub {pname}: {g.lower()}::{pname},\n"
            else:
                for i in range(svds[d].peripherals[g].__len__()):
                    pname = svds[d].peripherals[g][i].name.upper()
                    # instances += f"    pub {pname}: {g.lower()}::{pname},\n"
                    instances.append({"name": pname, "type": f"{g.lower()}::{pname}"})
                pass

        # instances += "}"
        # out += instances
        # out += (
        #     "impl Instances {\n"
        #     "    pub const unsafe fn instances() -> Self {\n"
        #     "        Self {\n"
        #     "            "
        # )

        with open(f"src/devices/{d}/mod.rs", "w") as f:
            f.write(device_template.render(blocks=out, instances=instances, nvic_prio_bits=svds[d].cpu.nvic_prio_bits))
        with open(f"src/devices/{d}/irq.rs", "w") as f:
            f.write(generate_vectors(svds[d].irqs))
        with open(f"src/devices/{d}/device.x", "w") as f:
            x = ""
            for i in svds[d].irqs:
                x += f"PROVIDE({i[0]} = DefaultHandler);\n"
            f.write(x)


if __name__ == "__main__":
    devices = [
        os.path.basename(f).removesuffix(".svd.patched")
        for f in glob("data/*.svd.patched")
    ]
    svds: Dict[str, SVD] = {}
    for d in devices:
        svds[d] = SVD.from_svd_file(f"data/{d}.svd.patched")
    generate_peripherals(svds)
    generate_device(svds)
