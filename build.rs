use std::env;

fn main() {
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
    }
    println!("cargo:rerun-if-changed=build.rs");
}
