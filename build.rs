use proc_macro2::TokenStream;
use quote::quote;
use serde::Deserialize;
use std::{
    env, fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Deserialize)]
#[allow(dead_code)]
struct Config {
    nvic_priority_bits: u8,
    peripherals: Vec<Peripheral>,
    interrupts: Vec<Interrupt>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Peripheral {
    name: String,
    base_address: u32,
    block: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Interrupt {
    name: String,
    description: String,
    value: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if env::var("CARGO_FEATURE_RT").is_ok() {
        let device: Vec<String> = env::vars()
            .map(|(d, _)| d)
            .filter(|d| d.starts_with("CARGO_FEATURE_MCX"))
            .collect();
        if device.len() > 1 {
            panic!("Should select only 1 device at the same time");
        }

        let device = device[0]
            .strip_prefix("CARGO_FEATURE_")
            .unwrap()
            .to_string()
            .to_lowercase();

        println!(
            "cargo:rustc-link-search={}/src/devices/{}",
            env!("CARGO_MANIFEST_DIR"),
            device
        );

        if env::var("CARGO_FEATURE_METADATA").is_ok() {
            let mut ts = TokenStream::new();
            let device_yaml_path = format!(
                "{}/devices/{}/{}.yaml",
                env!("CARGO_MANIFEST_DIR"),
                &device[..4],
                device
            );
            let device_yaml = fs::read_to_string(device_yaml_path)?;
            let config: Config = serde_yaml::from_str(&device_yaml)?;
            let nvic_priority_bits = config.nvic_priority_bits;

            let peripheral_quotes = config.peripherals.iter().map(|p| {
                let peripheral_name = p.name.as_str();
                let peripheral_base_address = p.base_address;
                let peripheral_block = p.block.as_str();

                quote! {
                    crate::_metadata::Peripheral {
                        name: #peripheral_name,
                        base_address: #peripheral_base_address,
                        block: #peripheral_block,
                    },
                }
            });

            let interrupt_quotes = config.interrupts.iter().map(|i| {
                let interrut_name = i.name.as_str();
                let interrupt_description = i.description.as_str();
                let interrupt_value = i.value;

                quote! {
                    crate::_metadata::Interrupt {
                        name: #interrut_name,
                        description: #interrupt_description,
                        value: #interrupt_value,
                    },
                }
            });

            ts.extend(quote! {
                use crate::_metadata::Metadata;
                pub static METADATA: Metadata = Metadata {
                    nvic_priority_bits: #nvic_priority_bits,
                    peripherals: &[#(#peripheral_quotes)*],
                    interrupts: &[#(#interrupt_quotes)*],
                };
            });

            // Write generated_metadata.rs file
            let out_dir = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
            let generated_file = out_dir
                .join("_generated_metadata.rs")
                .to_string_lossy()
                .to_string();
            fs::write(&generated_file, ts.to_string()).unwrap();
            format_file(&generated_file);
        }
    }

    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}

fn format_file(path: impl AsRef<Path>) {
    let path = path.as_ref();

    match Command::new("rustfmt").args([path]).output() {
        Err(e) => eprintln!("Failed to execute rustfmt {:?}: {:?}", path, e),

        Ok(out) => {
            if !out.status.success() {
                eprintln!("rustfmt {:?} failed:", path);
                eprintln!("=== STDOUT:");
                std::io::stderr().write_all(&out.stdout).unwrap();
                eprintln!("=== STDERR:");
                std::io::stderr().write_all(&out.stderr).unwrap();
            }
        }
    }
}
