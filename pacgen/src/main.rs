// use std::fs;
// use std::str::FromStr;

// use anyhow::Result;
// use glob::glob;
// use proc_macro2::TokenStream;
// use quote::quote;

// use pacgen::generate::*;
// use pacgen::{convert_svd_to_device, Peripherals};

// fn main() -> Result<()> {
//     let mapping = Peripherals::parse("./data/peripherals.yaml")?;

//     // let items = generate_pac("mcxn947", &mapping)?;
//     // fs::write("test.rs", items.to_string())?;

//     let names = glob("data/svds/*.svd.patched")?
//         .into_iter()
//         .map(|f| {
//             f.unwrap()
//                 .file_stem()
//                 .unwrap()
//                 .to_string_lossy()
//                 .strip_suffix(".svd")
//                 .unwrap()
//                 .to_string()
//         })
//         .collect::<Vec<_>>();
//     println!("Found {} devices", names.len());

//     for name in &names {
//         println!("Generating PAC for {}", name);
//         fs::create_dir_all(&format!("src/devices/{}", name))?;

//         let svd = fs::read_to_string(&format!("data/svds/{}.svd.patched", name))?;
//         let svd = svd_parser::parse(&svd)?;
//         let device = convert_svd_to_device(svd)?;

//         let items = generate_pac(&device, &name, &mapping)?;
//         fs::write(&format!("src/devices/{}/pac.rs", name), items.to_string())?;

//         let items = generate_linker_script(&device)?;
//         fs::write(&format!("src/devices/{}/device.x", name), items)?;
//     }

//     let mut items = TokenStream::new();
//     for name in &names {
//         let path = format!("devices/{}/pac.rs", name);
//         items.extend(quote! {
//             #[cfg(feature = #name)]
//             #[path = #path]
//             pub mod pac;

//         });
//     }
//     fs::write("src/device.rs", items.to_string())?;

//     fs::write(
//         "src/common.rs",
//         TokenStream::from_str(std::str::from_utf8(chiptool::generate::COMMON_MODULE).unwrap())
//             .unwrap()
//             .to_string(),
//     )?;

//     let items = quote! {
//         #![no_std]

//         pub mod common;
//         pub mod device;
//     };
//     fs::write("src/lib.rs", items.to_string())?;

//     Ok(())
// }

use std::{fs, str::FromStr};

use anyhow::Result;
use clap::Parser;
use glob::glob;
use log::{debug, info, trace};
use pacgen::{
    convert_svd_to_device,
    generate::{generate_linker_script, generate_pac, generate_peripheral},
    Peripherals,
};
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug, Parser)]
struct Opts {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    All,
    Devices,
    Peripherals,
}

fn main() -> Result<()> {
    env_logger::init();
    let opts = Opts::parse();
    debug!("opts: {:?}", opts);

    match opts.subcommand {
        SubCommand::All => {
            peripherals()?;
            devices()?;
            pcrate()?;
            Ok(())
        }
        SubCommand::Devices => devices(),
        SubCommand::Peripherals => peripherals(),
    }
}

fn peripherals() -> Result<()> {
    fs::create_dir_all("src/peripherals")?;

    let names = glob("./data/peripherals/*.yaml")?
        .into_iter()
        .map(|f| {
            f.unwrap()
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string()
        })
        .collect::<Vec<_>>();
    trace!("all peripherals: {:?}", names);
    for name in names {
        info!("Generating peripheral {}", name);

        let content = fs::read_to_string(&format!("data/peripherals/{}.yaml", name))?;
        let mut ir: chiptool::ir::IR = serde_yaml::from_str(&content)?;
        let items = generate_peripheral(&mut ir)?;

        fs::write(&format!("src/peripherals/{}.rs", name), items.to_string())?;
    }

    Ok(())
}

fn devices() -> Result<()> {
    let mapping = Peripherals::parse("./data/peripherals.yaml")?;

    let names = glob("data/svds/*.svd.patched")?
        .into_iter()
        .map(|f| {
            f.unwrap()
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .strip_suffix(".svd")
                .unwrap()
                .to_string()
        })
        .collect::<Vec<_>>();
    trace!("all devices: {:?}", names);
    for name in names {
        info!("Generating PAC for {}", name);
        fs::create_dir_all(&format!("src/devices/{}", name))?;

        let svd = fs::read_to_string(&format!("data/svds/{}.svd.patched", name))?;
        let svd = svd_parser::parse(&svd)?;
        let device = convert_svd_to_device(svd)?;

        let items = generate_pac(&device, &name, &mapping)?;
        fs::write(&format!("src/devices/{}/pac.rs", name), items.to_string())?;
        fs::write(
            &format!("src/devices/{}/device.x", name),
            generate_linker_script(&device)?,
        )?;
    }

    Ok(())
}

fn pcrate() -> Result<()> {
    let names = glob("data/svds/*.svd.patched")?
        .into_iter()
        .map(|f| {
            f.unwrap()
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .strip_suffix(".svd")
                .unwrap()
                .to_string()
        })
        .collect::<Vec<_>>();

    let mut items = TokenStream::new();
    items.extend(quote! {
        #![no_std]
        #![allow(non_camel_case_types)]

        pub mod common;

        macro_rules! mod_pac {
            ($($device:literal),*) => {
                paste::paste! {
                    $(
                        #[cfg(feature = $device)]
                        #[path = "devices/" $device "/pac.rs"]
                        pub mod pac;
                    )*
                }
            }
        }

        #[cfg(not(feature = "device-selected"))]
        compile_error!("No device selected");

        #(
            mod_pac!(#names);
        )*
    });
    fs::write("src/lib.rs", items.to_string())?;

    fs::write(
        "src/common.rs",
        TokenStream::from_str(std::str::from_utf8(chiptool::generate::COMMON_MODULE).unwrap())
            .unwrap()
            .to_string(),
    )?;

    Ok(())
}
