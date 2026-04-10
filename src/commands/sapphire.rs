use crate::cli::SapphireCommand;
use crate::config::set_default_version;
use crate::manifest::pin_version;
use crate::paths::Paths;

pub fn run(subcommand: SapphireCommand) -> anyhow::Result<()> {
    let paths = Paths::new();

    match subcommand {
        SapphireCommand::List => {
            println!("not yet implemented: sapphire list");
        }
        SapphireCommand::Install { version } => {
            println!("not yet implemented: sapphire install {version}");
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
