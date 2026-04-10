use crate::cli::SelfCommand;

pub fn run(subcommand: SelfCommand) -> anyhow::Result<()> {
    match subcommand {
        SelfCommand::Update => {
            println!("not yet implemented: self update");
        }
        SelfCommand::Uninstall => {
            println!("not yet implemented: self uninstall");
        }
        SelfCommand::Info => {
            println!("not yet implemented: self info");
        }
    }
    Ok(())
}
