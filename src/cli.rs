use clap::Subcommand;

#[derive(Subcommand)]
pub enum SapphireCommand {
    /// List installed Sapphire versions
    List,
    /// Install a Sapphire version
    Install {
        /// Version to install (e.g. "1.0.0" or "latest")
        version: String,
    },
    /// Uninstall a Sapphire version
    Uninstall {
        /// Version to uninstall
        version: String,
    },
    /// Set the default Sapphire version
    Use {
        /// Version to use
        version: String,
    },
    /// Show the active Sapphire version
    Current,
}

#[derive(Subcommand)]
pub enum SelfCommand {
    /// Update facet to the latest version
    Update,
    /// Uninstall facet
    Uninstall,
    /// Show facet installation info
    Info,
}
