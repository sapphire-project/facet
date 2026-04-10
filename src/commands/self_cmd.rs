use crate::cli::SelfCommand;
use crate::paths::Paths;

pub fn run(subcommand: SelfCommand) -> anyhow::Result<()> {
    match subcommand {
        SelfCommand::Update => {
            println!("not yet implemented: self update");
        }
        SelfCommand::Uninstall => {
            println!("not yet implemented: self uninstall");
        }
        SelfCommand::Info => {
            info()?;
        }
    }
    Ok(())
}

fn info() -> anyhow::Result<()> {
    let paths = Paths::new();

    let exe = std::env::current_exe()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "(unknown)".to_string());

    println!("facet {}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("  binary    {exe}");
    println!("  toolchains {}", paths.toolchains_dir().display());
    println!("  config    {}", paths.config_file().display());
    println!("  cache     {}", paths.cache_dir().display());

    Ok(())
}
