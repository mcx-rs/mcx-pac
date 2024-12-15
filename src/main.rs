use std::path::PathBuf;
use std::{fmt::Write, str::FromStr};

use anyhow::Result;
use chiptool::ir::IR;
use chiptool::transform::Transform;
use clap::{Parser, Subcommand};
use glob::glob;
use log::{debug, info};
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug, serde::Deserialize)]
struct Config {
    includes: Option<Vec<String>>,
    transforms: Option<Vec<Transform>>,
}

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Generate {
        #[arg(required = true)]
        svd: std::path::PathBuf,

        #[arg(required = true)]
        transform: std::path::PathBuf,

        #[arg(required = true)]
        output: std::path::PathBuf,
    },

    GenerateAll {
        #[arg(long, short, default_value = "svds/*.svd.patched")]
        svds: std::path::PathBuf,

        #[arg(long, short, default_value = "transforms")]
        transforms: std::path::PathBuf,

        #[arg(long, short, default_value = "output")]
        output: std::path::PathBuf,
    },

    Crate {
        #[arg(long, short, default_value = "svds/*.svd.patched")]
        svds: std::path::PathBuf,

        #[arg(long, short, default_value = "transforms")]
        transforms: std::path::PathBuf,

        #[arg(long, short, default_value = "output")]
        output: std::path::PathBuf,
    },
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Cli::parse();
    debug!("{:?}", args);

    match args.command {
        Commands::Generate {
            svd,
            transform,
            output,
        } => generate(&svd, &transform, &output)?,
        Commands::GenerateAll {
            svds,
            transforms,
            output,
        } => generate_all(&svds, &transforms, &output)?,
        Commands::Crate {
            svds,
            transforms,
            output,
        } => {
            std::fs::create_dir_all(output.join("src"))?;
            std::fs::write(
                output.join("src").join("common.rs"),
                chiptool::generate::COMMON_MODULE,
            )?;

            let devices: Vec<String> = glob(svds.to_str().unwrap())
                .unwrap()
                .filter_map(Result::ok)
                .map(|p| {
                    p.file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .split(".")
                        .collect::<Vec<_>>()[0]
                        .to_string()
                })
                .collect();
            let items = render_lib(&devices)?;
            std::fs::write(output.join("src").join("lib.rs"), items.to_string())?;
            std::fs::write(output.join("src").join("device.rs"), "")?;
            generate_all(&svds, &transforms, &output)?;

            let items = render_cargo(&devices)?;
            std::fs::write(output.join("Cargo.toml"), items)?;

            let items = render_build()?;
            std::fs::write(output.join("build.rs"), items)?;

            let mut ignores = String::new();
            writeln!(&mut ignores, "/target\n/Cargo.lock")?;
            std::fs::write(output.join(".gitignore"), ignores)?;

            std::process::Command::new("cargo")
                .args([
                    "fmt",
                    "--manifest-path",
                    &format!("{}/Cargo.toml", output.display()),
                ])
                .output()?;
        }
    }

    Ok(())
}

fn apply<P: AsRef<std::path::Path>>(ir: &mut IR, p: P) -> Result<()> {
    debug!("loading transform {}", p.as_ref().display());
    let content = std::fs::read(&p)?;
    let config: Config = serde_yaml::from_slice(&content)?;

    if let Some(includes) = &config.includes {
        for include in includes {
            let subp = &p.as_ref().parent().unwrap().join(include);
            apply(ir, subp)?;
        }
    }

    if let Some(transforms) = &config.transforms {
        for transform in transforms {
            transform.run(ir)?;
        }
    }

    Ok(())
}

fn generate(svd: &PathBuf, transform: &PathBuf, output: &PathBuf) -> Result<()> {
    let svd_parser_config = svd_parser::Config::default();
    let render_opts = chiptool::generate::Options {
        common_module: chiptool::generate::CommonModule::External(quote!(crate::common)),
    };

    debug!(
        "svd: {}, transform: {}, output: {}",
        svd.display(),
        transform.display(),
        output.display()
    );

    let content = std::fs::read_to_string(svd)?;
    let device = svd_parser::parse_with_config(&content, &svd_parser_config)?;
    let mut ir = chiptool::svd2ir::convert_svd(&device)?;
    apply(&mut ir, transform)?;

    std::fs::create_dir_all(&output)?;
    let items = chiptool::generate::render(&mut ir, &render_opts)?;
    let path = output.join("device.rs");
    std::fs::write(path, items.to_string())?;
    let items = chiptool::generate::render_device_x(&ir, &ir.devices.values().next().unwrap())?;
    let path = output.join("device.x");
    std::fs::write(path, items.to_string())?;

    Ok(())
}

fn generate_all(svds: &PathBuf, transforms: &PathBuf, output: &PathBuf) -> Result<()> {
    std::fs::create_dir_all(output.join("src"))?;
    for path in glob(svds.to_str().unwrap()).unwrap().filter_map(Result::ok) {
        let device_name = path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .split(".")
            .collect::<Vec<_>>()[0]
            .to_string();
        info!("generating device {}", device_name);
        let svd = path.clone();
        let transform = transforms.join(format!("{}.yaml", device_name));
        let output = output
            .join("src")
            .join(format!("{}", device_name.to_lowercase()));
        generate(&svd, &transform, &output)?;
    }
    Ok(())
}

fn render_lib(devices: &Vec<String>) -> Result<TokenStream> {
    let mut items = TokenStream::new();

    items.extend(quote! {
        #![no_std]
        #![allow(dead_code)]
        #![allow(unused_attributes)]
        #![allow(non_camel_case_types)]

        mod common;
    });

    for device in devices {
        let device = device.to_lowercase();
        let path = format!("{}/device.rs", device);
        items.extend(quote! {
            #[cfg_attr(feature = #device, path = #path)]
        });
    }
    items.extend(quote! {pub mod device;});
    items.extend(quote! {
        #[cfg(feature = "_device_selected")]
        pub use device::*;
    });

    Ok(items)
}

fn render_cargo(devices: &Vec<String>) -> Result<String> {
    let mut cargo = include_str!("Cargo.toml").to_string();

    let output = std::process::Command::new("git").arg("describe").output()?;
    let version = if output.status.success() {
        String::from_utf8(output.stdout)?
            .trim()
            .to_string()
            .strip_prefix("v")
            .unwrap()
            .to_string()
    } else {
        String::from_str("0.0.0-nightly")?
    };

    cargo = cargo.replace("__VERSION__", &version);

    for device in devices {
        writeln!(
            &mut cargo,
            "{} = [\"_device_selected\"]",
            device.to_lowercase()
        )?;
    }

    Ok(cargo)
}

fn render_build() -> Result<String> {
    let items = include_str!("build.rs").to_string();
    Ok(items)
}
