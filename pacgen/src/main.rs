use std::{
    fs,
    path::PathBuf,
    process::{Command, Stdio},
    str::FromStr,
};

use anyhow::Result;
use clap::Parser;
use glob::glob;
use log::{debug, info, trace, warn};
use pacgen::{
    convert_svd_to_device,
    generate::{generate_linker_script, generate_pac, generate_peripheral},
    PeripheralMappings,
};

#[derive(Debug, Parser)]
struct Opts {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    All,

    Patch,
    Extract,
    Transform,
    Pacs,
    Peripherals,
}

fn main() -> Result<()> {
    env_logger::init();
    let opts = Opts::parse();
    debug!("opts: {:?}", opts);

    let devices = glob(&format!("./data/svds/*.svd"))?
        .into_iter()
        .map(|f| {
            f.unwrap()
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string()
        })
        .collect::<Vec<_>>();
    info!("Found devices {:?}", devices);

    let mappings = PeripheralMappings::parse("./data/peripherals.yaml")?;

    match opts.subcommand {
        SubCommand::Patch => patch(&devices),
        SubCommand::Extract => extract(&devices),
        SubCommand::Transform => transform(&mappings),
        SubCommand::Peripherals => peripherals(),
        SubCommand::Pacs => pacs(&devices, &mappings),
        SubCommand::All => {
            patch(&devices)?;
            extract(&devices)?;
            transform(&mappings)?;
            peripherals()?;
            pacs(&devices, &mappings)?;
            Ok(())
        }
    }
}

fn patch(devices: &Vec<String>) -> Result<()> {
    info!("Start patch");
    for device in devices {
        info!("Patching device {}", device);
        debug!(
            "Applying ./data/svds/{}.yaml output to ./data/svds/{}.svd.patched",
            device, device
        );
        Command::new("svdtools")
            .args([
                "patch",
                &format!("./data/svds/{}.yaml", device),
                &format!("./data/svds/{}.svd.patched", device),
            ])
            .env("RUST_LOG", "info")
            .stdout(Stdio::null())
            .spawn()?
            .wait()?;
    }
    info!("Patch finished, {} devices patched", devices.len());
    Ok(())
}

fn extract(devices: &Vec<String>) -> Result<()> {
    info!("Start extract");

    for device in devices {
        info!("Extract Peripherals from {}", device);
        Command::new("chiptool")
            .args([
                "extract-all",
                "--svd",
                &format!("./data/svds/{}.svd.patched", device),
                "--output",
                &format!("./data/temp/{}", device),
            ])
            .env("RUST_LOG", "info")
            .stdout(Stdio::null())
            .spawn()?
            .wait()?;
    }

    info!("Extract {} devices' peripherals", devices.len());
    Ok(())
}

fn transform(mappings: &PeripheralMappings) -> Result<()> {
    info!("Start transform");

    debug!("Create directory `./data/temp/peripherals`");
    fs::create_dir_all("./data/temp/peripherals")?;

    for (pname, mapping) in mappings {
        let device = mapping.get_from().to_string();
        let op = if let Some(opnames) = mapping.contain_device(&device) {
            Some(opnames[0].to_string())
        } else {
            None
        };
        if let Some(op) = op {
            let src = format!("./data/temp/{}/{}.yaml", device, op);
            let dst = &format!("./data/temp/peripherals/{}.yaml", pname);
            debug!("Copy {} to {}", src, dst);
            fs::copy(src, dst)?;
        } else {
            warn!("Peripheral {} not found in temp", pname);
        }
    }

    let periph_names = glob("./data/temp/peripherals/*.yaml")?
        .map(|f| {
            f.unwrap()
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string()
        })
        .collect::<Vec<_>>();
    trace!("Found peripherals waiting transform: {:?}", periph_names);
    info!("Found {} peripherals waiting transform", periph_names.len());
    debug!("Create directory `./data/peripherals`");
    fs::create_dir_all("./data/peripherals")?;

    let mut num = 0;
    for name in periph_names {
        let trans = format!("./data/transforms/{}.yaml", name);
        if !PathBuf::from_str(&trans)?.exists() {
            warn!("Transform file for {} does NOT exists", name);
            continue;
        }
        info!("Transforming {}", name);
        Command::new("chiptool")
            .args([
                "transform",
                "--input",
                &format!("./data/temp/peripherals/{}.yaml", name),
                "--output",
                &format!("./data/peripherals/{}.yaml", name),
                "--transform",
                &trans,
            ])
            .env("RUST_LOG", "info")
            .stdout(Stdio::null())
            .spawn()?
            .wait()?;
        num += 1;
    }

    info!("Transformed {} peripherals", num);
    Ok(())
}

fn peripherals() -> Result<()> {
    info!("Start generate peripherals source");
    debug!("Create ./src/peripherals");
    fs::create_dir_all("./src/peripherals")?;

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
    trace!("Found peripherals: {:?}", names);
    info!("Found {} peripherals", names.len());

    for name in &names {
        info!("Generating peripheral source {}", name);

        let content = fs::read_to_string(&format!("data/peripherals/{}.yaml", name))?;
        let mut ir: chiptool::ir::IR = serde_yaml::from_str(&content)?;
        let items = generate_peripheral(&mut ir)?;

        fs::write(&format!("src/peripherals/{}.rs", name), items.to_string())?;
    }

    info!("Generated {} peripherals source", names.len());
    Ok(())
}

fn pacs(devices: &Vec<String>, mappings: &PeripheralMappings) -> Result<()> {
    for device_name in devices {
        info!("Generating PAC for {}", device_name);
        fs::create_dir_all(&format!("src/devices/{}", device_name))?;

        let svd = fs::read_to_string(&format!("data/svds/{}.svd.patched", device_name))?;
        let svd = svd_parser::parse(&svd)?;
        let device = convert_svd_to_device(svd)?;

        let items = generate_pac(&device, &device_name, &mappings)?;
        fs::write(
            &format!("src/devices/{}/pac.rs", device_name),
            items.to_string(),
        )?;
        fs::write(
            &format!("src/devices/{}/device.x", device_name),
            generate_linker_script(&device)?,
        )?;
    }

    Ok(())
}
