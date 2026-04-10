use crate::cli::SapphireCommand;

pub fn run(subcommand: SapphireCommand) -> anyhow::Result<()> {
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
    }
    Ok(())
}
