use clap::Subcommand;

#[derive(Subcommand)]
pub enum SapphireCommand {
    /// List installed Sapphire versions
    List {
        /// List versions available on GitHub instead of locally installed ones
        #[arg(long)]
        remote: bool,
    },
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
    /// Set the global default Sapphire version
    Default {
        /// Version to set as default (e.g. "1.2.0")
        version: String,
    },
    /// Pin the Sapphire version for the current project
    Pin {
        /// Version to pin in facet.toml
        version: String,
    },
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
