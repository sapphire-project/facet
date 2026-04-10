use crate::cli::SapphireCommand;
use crate::config::{GlobalConfig, set_default_version};
use crate::download;
use crate::manifest::pin_version;
use crate::paths::Paths;
use anyhow::Context;

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
            uninstall(&paths, &version)?;
        }
        SapphireCommand::Use { version } => {
            use_version(&paths, &version)?;
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

fn use_version(paths: &Paths, version: &str) -> anyhow::Result<()> {
    let version = version.trim_start_matches('v');
    let bin = paths
        .toolchain_dir(version)
        .join("bin")
        .join("sapphire");

    if !bin.exists() {
        anyhow::bail!(
            "Sapphire {version} is not installed\n\
             hint: run `facet sapphire install {version}` first"
        );
    }

    set_default_version(paths, version)?;
    println!("Now using Sapphire {version}");
    Ok(())
}

fn uninstall(paths: &Paths, version: &str) -> anyhow::Result<()> {
    let version = version.trim_start_matches('v');
    let dir = paths.toolchain_dir(version);

    if !dir.exists() {
        anyhow::bail!("Sapphire {version} is not installed");
    }

    std::fs::remove_dir_all(&dir)
        .with_context(|| format!("failed to remove {}", dir.display()))?;

    // If this was the global default, clear it so we don't point at a ghost.
    let mut config = GlobalConfig::load(paths)?;
    if config.toolchain.default.as_deref() == Some(version) {
        config.toolchain.default = None;
        config.save(paths)?;
        println!("Uninstalled Sapphire {version} (was global default; default cleared)");
    } else {
        println!("Uninstalled Sapphire {version}");
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
