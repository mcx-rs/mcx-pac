use std::{env, path::PathBuf};

fn main() {
    let device_name = env::vars()
        .map(|(a, _)| a)
        .filter(|x| x.starts_with("CARGO_FEATURE_MCX"))
        .collect::<Vec<_>>();
    let device_name = device_name
        .get(0)
        .unwrap()
        .strip_prefix("CARGO_FEATURE_MCX")
        .unwrap()
        .to_lowercase();

    // #[cfg(feature = "rt")]
    {
        println!(
            "cargo:rustc-link-search={}/src/devices/{}",
            PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).display(),
            device_name
        );
    }

    println!("cargo:rerun-if-changed=build.rs");
}
