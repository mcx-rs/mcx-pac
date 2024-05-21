import os
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
    )


def render_block(b: SVDCluster | SVDPeripheral, bit_size: int = 8) -> str:
    reserved_num = 0
    offset = 0

    out = f'#[doc = r"{b.description}"]\n#[repr(C)]\n'
    out += "pub struct RegisterBlock {\n"

    for i in range(len(b.registers_clusters)):
        e = b.registers_clusters[i]
        if offset < e.address_offset:
            out += f"    _reserved{reserved_num}: [u8; {hex(e.address_offset - offset)}],\n"
            reserved_num += 1
            offset += e.address_offset
        name = e.name.replace("[%s]", "_").replace("%s", "_").removesuffix("_")
        out += f'    #[doc = r"{e.description}"]\n'
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
                    i.name.upper() if i.description is None else i.description,
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

        self.cpu = device.cpu

    def generate_vectors(self) -> str:
        template = _environment.get_template("vectors.rs.template")
        vectors = []
        vectors_index = 0
        for i in self.irqs:
            while vectors_index < i[2]:
                vectors.append("Vector { _reserved: 0 },")
                vectors_index += 1
            vectors.append(f"Vector {{ _handler: {i[0]} }},")
            vectors_index += 1

        return template.render(irqs=self.irqs, vectors=vectors)

    @classmethod
    def from_svd_file(cls, path: str) -> "SVD":
        device = SVDParser.for_xml_file(path).get_device()
        name = os.path.basename(path).split(".")[0]
        return cls(device, name)


if __name__ == "__main__":
    svd = SVD.from_svd_file("./data/mcxn947.svd.patched")

    out = ""

    # r = svd.peripherals["SYSCON"][0].registers_clusters[9]
    # out = render_register(r)

    # r = svd.peripherals["SYSCON"][0].registers_clusters[0]
    # out = render_register(r)

    out = render_block(svd.peripherals["CAN"][0])

    with open("test.rs", "w") as f:
        f.write(out)

    pass
