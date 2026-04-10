use anyhow::Context;
use crate::cli::SapphireCommand;
use crate::config::{set_default_version, GlobalConfig};
use crate::download;
use crate::manifest::pin_version;
use crate::paths::Paths;

pub fn run(subcommand: SapphireCommand) -> anyhow::Result<()> {
    let paths = Paths::new();

    match subcommand {
        SapphireCommand::List { remote } => {
            if remote {
                list_remote()?;
            } else {
                list_local(&paths)?;
            }
        }
        SapphireCommand::Install { version } => {
            let resolved = download::install(&version, &paths.toolchains_dir())?;
            if resolved != version && version != "latest" {
                println!("Installed Sapphire {resolved}");
            }
        }
        SapphireCommand::Uninstall { version } => {
            println!("not yet implemented: sapphire uninstall {version}");
        }
        SapphireCommand::Use { version } => {
            println!("not yet implemented: sapphire use {version}");
        }
        SapphireCommand::Current => {
            println!("not yet implemented: sapphire current");
        }
        SapphireCommand::Default { version } => {
            set_default_version(&paths, &version)?;
            println!("Set default Sapphire version to {version}");
        }
        SapphireCommand::Pin { version } => {
            let cwd = std::env::current_dir()?;
            pin_version(&cwd, &version)?;
            println!("Pinned Sapphire {version} in facet.toml");
        }
    }
    Ok(())
}

fn list_local(paths: &Paths) -> anyhow::Result<()> {
    let toolchains_dir = paths.toolchains_dir();
    let default_version = GlobalConfig::load(paths)?.toolchain.default;

    if !toolchains_dir.exists() {
        println!("No Sapphire versions installed.");
        return Ok(());
    }

    let mut versions: Vec<String> = std::fs::read_dir(&toolchains_dir)
        .with_context(|| format!("failed to read {}", toolchains_dir.display()))?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().ok()?.is_dir() {
                entry.file_name().into_string().ok()
            } else {
                None
            }
        })
        .collect();

    if versions.is_empty() {
        println!("No Sapphire versions installed.");
        return Ok(());
    }

    versions.sort_by(|a, b| {
        // sort descending by semver-ish comparison; fall back to string order
        b.cmp(a)
    });

    for v in &versions {
        if default_version.as_deref() == Some(v.as_str()) {
            println!("* {v}");
        } else {
            println!("  {v}");
        }
    }

    Ok(())
}

fn list_remote() -> anyhow::Result<()> {
    eprintln!("Fetching available Sapphire versions...");
    let versions = download::fetch_versions()?;

    if versions.is_empty() {
        println!("No releases found.");
    } else {
        for v in &versions {
            println!("{v}");
        }
    }

    Ok(())
}
