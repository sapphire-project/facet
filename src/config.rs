use crate::paths::Paths;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Global facet configuration stored at `<config_dir>/facet/facet.toml`.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GlobalConfig {
    #[serde(default)]
    pub toolchain: ToolchainConfig,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ToolchainConfig {
    /// The default Sapphire version used when no project pin is present.
    pub default: Option<String>,
}

impl GlobalConfig {
    /// Load from disk, returning a default config if the file doesn't exist.
    pub fn load(paths: &Paths) -> Result<Self> {
        let path = paths.config_file();
        if !path.exists() {
            return Ok(Self::default());
        }
        let raw = std::fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        toml::from_str(&raw).with_context(|| format!("failed to parse {}", path.display()))
    }

    /// Persist to disk, creating parent directories as needed.
    pub fn save(&self, paths: &Paths) -> Result<()> {
        let path = paths.config_file();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {}", parent.display()))?;
        }
        let contents = toml::to_string_pretty(self).context("failed to serialise global config")?;
        std::fs::write(&path, contents)
            .with_context(|| format!("failed to write {}", path.display()))
    }
}

/// Set `default` in the global config and save.
pub fn set_default_version(paths: &Paths, version: &str) -> Result<()> {
    let mut config = GlobalConfig::load(paths)?;
    config.toolchain.default = Some(version.to_string());
    config.save(paths)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paths::Paths;

    fn temp_paths() -> (tempfile::TempDir, Paths) {
        let dir = tempfile::tempdir().unwrap();
        let paths = Paths::with_base(dir.path());
        (dir, paths)
    }

    #[test]
    fn default_when_file_missing() {
        let (_dir, paths) = temp_paths();
        let config = GlobalConfig::load(&paths).unwrap();
        assert!(config.toolchain.default.is_none());
    }

    #[test]
    fn round_trips_default_version() {
        let (_dir, paths) = temp_paths();
        set_default_version(&paths, "1.2.3").unwrap();
        let config = GlobalConfig::load(&paths).unwrap();
        assert_eq!(config.toolchain.default.as_deref(), Some("1.2.3"));
    }

    #[test]
    fn overwrites_existing_default() {
        let (_dir, paths) = temp_paths();
        set_default_version(&paths, "1.0.0").unwrap();
        set_default_version(&paths, "2.0.0").unwrap();
        let config = GlobalConfig::load(&paths).unwrap();
        assert_eq!(config.toolchain.default.as_deref(), Some("2.0.0"));
    }

    #[test]
    fn creates_parent_dirs() {
        let (_dir, paths) = temp_paths();
        // config file lives inside a freshly created temp dir — parents don't exist yet
        set_default_version(&paths, "1.0.0").unwrap();
        assert!(paths.config_file().exists());
    }
}
