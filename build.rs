use std::{env, path::PathBuf};

fn main() {
    let device: Vec<String> = env::vars()
        .map(|(d, _)| d)
        .filter(|d| d.starts_with("CARGO_FEATURE_MCX"))
        .collect();
    let device = match device.len() {
        1 => device
            .first()
            .unwrap()
            .strip_prefix("CARGO_FEATURE_")
            .unwrap()
            .to_lowercase(),
        0 => panic!("No device feature selected"),
        _ => panic!("Multiple device features selected"),
    };

    let crate_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());

    // if `rt` feature is selected, add linker script.
    #[cfg(feature = "rt")]
    {
        println!(
            "cargo:rustc-link-search={}",
            crate_dir
                .join("src")
                .join("devices")
                .join(&device)
                .display()
        );
    }

    println!(
        "cargo:rustc-env=MCXPAC_METADATA_PATH=devices/{}/metadata.rs",
        &device
    );

    println!("cargo:rerun-if-changed=build.rs");
}
