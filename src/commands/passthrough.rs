use anyhow::{bail, Context, Result};
use std::path::PathBuf;
use std::process;

use crate::paths::Paths;
use crate::version;

/// Resolve the active Sapphire binary path, or bail with a clear error.
pub fn resolve_binary(paths: &Paths) -> Result<PathBuf> {
    let cwd = std::env::current_dir().context("failed to get current directory")?;

    let resolved = version::resolve(&cwd, paths)?.ok_or_else(|| {
        anyhow::anyhow!(
            "no active Sapphire version found\n\
             hint: run `facet sapphire install <version>` then `facet sapphire default <version>`"
        )
    })?;

    let bin = paths
        .toolchain_dir(&resolved.version)
        .join("bin")
        .join("sapphire");

    if !bin.exists() {
        bail!(
            "Sapphire {} is not installed (expected binary at {})\n\
             hint: run `facet sapphire install {}`",
            resolved.version,
            bin.display(),
            resolved.version,
        );
    }

    Ok(bin)
}

/// Exec the active Sapphire binary with the given arguments.
///
/// The process is replaced via `std::process::Command`; the caller's exit code
/// matches the child's. This function never returns on success.
pub fn exec(args: &[String], paths: &Paths) -> Result<()> {
    let bin = resolve_binary(paths)?;

    let status = process::Command::new(&bin)
        .args(args)
        .status()
        .with_context(|| format!("failed to execute {}", bin.display()))?;

    process::exit(status.code().unwrap_or(1));
}

/// Entry point for bare passthrough — unrecognised top-level tokens forwarded
/// verbatim (e.g. `facet build` where `build` is a Sapphire subcommand).
pub fn run(args: Vec<String>) -> Result<()> {
    let paths = Paths::new();
    exec(&args, &paths)
}
