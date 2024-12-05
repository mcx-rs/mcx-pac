use std::env;

fn main() {
    if env::var("CARGO_FEATURE_RT").is_ok() {
        let device = env::vars()
            .map(|(d, _)| d)
            .filter(|d| d.starts_with("CARGO_FEATURE_MCX"))
            .next();
        if let Some(device) = device {
            let device = device
                .strip_prefix("CARGO_FEATURE_")
                .unwrap()
                .to_lowercase();

            println!(
                "cargo:rustc-link-search={}/src/{}",
                env!("CARGO_MANIFEST_DIR"),
                device
            );
        }
    }
    println!("cargo:rerun-if-changed=build.rs");
}
