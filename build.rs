use std::{env, path::PathBuf};

fn main() {
    let device_name = env::vars()
        .map(|(a, _)| a)
        .filter(|x| x.starts_with("CARGO_FEATURE_MCX"))
        .collect::<Vec<_>>();
    let device_name = device_name.get(0);
    if device_name.is_none() {
        return;
    }

    let device_name = device_name
        .unwrap()
        .strip_prefix("CARGO_FEATURE_")
        .unwrap()
        .to_lowercase();

    // #[cfg(feature = "rt")]
    {
        let p = match device_name.as_str() {
            "mcxn947" | "mcxn946" => "mcxn947",
            "mcxn547" | "mcxn546" => "mcxn546",
            _ => unreachable!(),
        };
        println!(
            "cargo:rustc-link-search={}/src/devices/{}",
            PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).display(),
            p
        );
    }

    println!("cargo:rerun-if-changed=build.rs");
}
