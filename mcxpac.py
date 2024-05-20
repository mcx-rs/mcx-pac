import os
import time
from typing import Dict, List, Tuple

from jinja2 import Environment, FileSystemLoader, Template
from svdsuite import (
    SVDCPU,
    SVDCluster,
    SVDDevice,
    SVDParser,
    SVDPeripheral,
    SVDRegister,
)

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

    def generate_peripherals(self) -> dict[str, str]:
        # TODO: generate all peripheral groups
        ret = {}
        pb = None
        for g in self.peripherals:
            for p in self.peripherals[g]:
                if len(p.registers_clusters) != 0:
                    pb = p
                    break

            regs = []
            offset = 0
            reserved_num = 0
            pb.registers_clusters.sort(key=lambda r: r.address_offset)
            for i in range(len(pb.registers_clusters)):
                r = pb.registers_clusters[i]
                if offset < r.address_offset:
                    regs.append(
                        f"_reserved{reserved_num}: [u8; {hex(r.address_offset - offset)}],"
                    )
                    reserved_num += 1
                    offset += r.address_offset
                name = r.name.replace("[%s]", "_").removesuffix("_")

                if isinstance(r, SVDRegister):
                    if r.dim is None:
                        regs.append(f"{name.lower()}: {name},")
                        offset += int(r.size / 8)
                    else:
                        regs.append(f"{name.lower()}: [{name}; {r.dim}],")
                        offset += r.size * r.dim
                elif isinstance(r, SVDCluster):
                    regs.append(f"{name.lower()}: [{name}; {r.dim}]")
                    offset += (
                        r.registers_clusters[-1].address_offset
                        + int(r.registers_clusters[-1].size / 8)
                    ) * r.dim
                else:
                    raise TypeError("Unreachable")

            template = _environment.get_template("peripheral.rs.template")
            ret[g] = template.render(regs=regs)
        return ret

    @classmethod
    def from_svd_file(cls, path: str) -> "SVD":
        device = SVDParser.for_xml_file(path).get_device()
        name = os.path.basename(path).split(".")[0]
        return cls(device, name)


if __name__ == "__main__":
    svd = SVD.from_svd_file("./data/mcxn947.svd.patched")
    out = svd.generate_peripherals()
