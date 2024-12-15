use std::path::PathBuf;

use anyhow::Result;
use chiptool::ir::IR;
use chiptool::transform::Transform;
use clap::{Parser, Subcommand};
use glob::glob;
use log::{debug, info};
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
        #[arg(required = true)]
        svds: std::path::PathBuf,

        #[arg(required = true)]
        transforms: std::path::PathBuf,

        #[arg(required = true)]
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
        } => {
            std::fs::create_dir_all(output.join("src"))?;
            for path in glob(svds.join("*.svd").to_str().unwrap())
                .unwrap()
                .filter_map(Result::ok)
            {
                let device_name = path.file_stem().unwrap().to_str().unwrap();
                info!("generating device {}", device_name);
                let svd = path.clone();
                let transform = transforms.join(format!("{}.yaml", device_name));
                let output = output.join("src").join(format!("{}", device_name));
                generate(&svd, &transform, &output)?;
            }
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
