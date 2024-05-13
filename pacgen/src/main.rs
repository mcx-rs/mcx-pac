use std::fs;
use std::str::FromStr;

use anyhow::Result;
use glob::glob;
use proc_macro2::TokenStream;
use quote::quote;

use pacgen::generate::*;
use pacgen::{convert_svd_to_device, Peripherals};

fn main() -> Result<()> {
    let mapping = Peripherals::parse("./data/peripherals.yaml")?;

    // let items = generate_pac("mcxn947", &mapping)?;
    // fs::write("test.rs", items.to_string())?;

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
    println!("Found {} devices", names.len());

    for name in &names {
        println!("Generating PAC for {}", name);
        fs::create_dir_all(&format!("src/devices/{}", name))?;

        let svd = fs::read_to_string(&format!("data/svds/{}.svd.patched", name))?;
        let svd = svd_parser::parse(&svd)?;
        let device = convert_svd_to_device(svd)?;

        let items = generate_pac(&device, &name, &mapping)?;
        fs::write(&format!("src/devices/{}/pac.rs", name), items.to_string())?;

        let items = generate_linker_script(&device)?;
        fs::write(&format!("src/devices/{}/device.x", name), items)?;
    }

    let mut items = TokenStream::new();
    for name in &names {
        let path = format!("devices/{}/pac.rs", name);
        items.extend(quote! {
            #[cfg(feature = #name)]
            #[path = #path]
            pub mod pac;

        });
    }
    fs::write("src/device.rs", items.to_string())?;

    fs::write(
        "src/common.rs",
        TokenStream::from_str(std::str::from_utf8(chiptool::generate::COMMON_MODULE).unwrap())
            .unwrap()
            .to_string(),
    )?;

    let items = quote! {
        #![no_std]

        pub mod common;
        pub mod device;
    };
    fs::write("src/lib.rs", items.to_string())?;

    Ok(())
}
