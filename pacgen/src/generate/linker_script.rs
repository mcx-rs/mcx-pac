use std::fmt::Write;

use anyhow::Result;
use chiptool::ir;

pub fn generate_linker_script(device: &ir::Device) -> Result<String> {
    let mut out = String::new();

    for p in &device.interrupts {
        writeln!(out, "PROVIDE({} = DefaultHandler);", p.name)?;
    }

    Ok(out)
}
