pub mod generate;

use std::collections::HashMap;
use std::fs;

use anyhow::Result;
use chiptool::ir;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PeripheralMapping {
    from: String,
    mapping: HashMap<String, Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeripheralMappings(HashMap<String, PeripheralMapping>);

impl PeripheralMapping {
    pub fn get_from(&self) -> &str {
        &self.from
    }

    pub fn contain_device(&self, device: &str) -> Option<&Vec<String>> {
        for (r, opnames) in &self.mapping {
            let r = fancy_regex::Regex::new(&format!("^{}$", r)).unwrap();
            if r.is_match(device).unwrap() {
                return Some(opnames);
            }
        }
        None
    }
}

impl PeripheralMappings {
    pub fn parse(path: &str) -> Result<Self> {
        let peripherals = fs::read_to_string(path)?;
        Ok(serde_yaml::from_str(&peripherals)?)
    }

    pub fn get_mapped_peripheral_name(&self, device_name: &str, block: &str) -> Option<&str> {
        for (pname, mapping) in self {
            if let Some(opnames) = mapping.contain_device(device_name) {
                if opnames.into_iter().any(|x| x == block) {
                    return Some(&pname);
                }
            }
        }
        None
    }
}

impl<'a> IntoIterator for &'a PeripheralMappings {
    type Item = (&'a String, &'a PeripheralMapping);
    type IntoIter = std::collections::hash_map::Iter<'a, String, PeripheralMapping>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Peripherals(HashMap<String, HashMap<String, Vec<String>>>);

impl Peripherals {
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
