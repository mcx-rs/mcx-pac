pub mod generate;

use std::collections::HashMap;
use std::fs;

use anyhow::Result;
use chiptool::ir;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Peripherals(HashMap<String, HashMap<String, Vec<String>>>);

impl Peripherals {
    pub fn parse(path: &str) -> Result<Self> {
        let peripherals = fs::read_to_string(path)?;
        Ok(serde_yaml::from_str(&peripherals)?)
    }

    pub fn get_peripheral_name(&self, device: &str, block: &str) -> Option<String> {
        for (name, dp) in &self.0 {
            for (d, p) in dp {
                if d != device {
                    continue;
                }
                if p.iter().any(|x| x == block) {
                    return Some(name.clone());
                }
            }
        }

        None
    }
}

pub fn convert_svd_to_device(svd: svd_parser::svd::Device) -> Result<ir::Device> {
    let mut device = ir::Device {
        nvic_priority_bits: svd.cpu.as_ref().map(|cpu| cpu.nvic_priority_bits as u8),
        peripherals: vec![],
        interrupts: vec![],
    };

    for p in &svd.peripherals {
        let block_name = p.derived_from.as_ref().unwrap_or(&p.name).to_string();
        let peripheral_name = p.name.to_ascii_uppercase();

        let peripheral = ir::Peripheral {
            name: peripheral_name.clone(),
            description: p.description.clone(),
            base_address: p.base_address,
            block: Some(block_name),
            array: None,
            interrupts: HashMap::new(),
        };

        let mut irqs: Vec<&svd_parser::svd::Interrupt> = vec![];
        for i in &p.interrupt {
            if !irqs.iter().any(|&j| j.name == i.name) {
                irqs.push(i);
            }
        }
        for &i in irqs.iter() {
            let irq_name = i.name.to_ascii_uppercase();
            if device.interrupts.iter().any(|j| j.name == irq_name) {
                continue;
            }
            device.interrupts.push(ir::Interrupt {
                name: irq_name.clone(),
                description: i.description.clone(),
                value: i.value,
            });
        }
        device.peripherals.push(peripheral);
    }

    Ok(device)
}
