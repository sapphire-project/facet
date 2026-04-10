use std::path::PathBuf;

/// Resolved XDG Base Directory paths for facet.
///
/// Follows the XDG Base Directory specification on all platforms:
///   - config: `$XDG_CONFIG_HOME/facet`  (default: `~/.config/facet`)
///   - data:   `$XDG_DATA_HOME/facet`    (default: `~/.local/share/facet`)
///   - cache:  `$XDG_CACHE_HOME/facet`   (default: `~/.cache/facet`)
pub struct Paths {
    data_dir: PathBuf,
    config_dir: PathBuf,
    cache_dir: PathBuf,
}

impl Paths {
    pub fn new() -> Self {
        let home = std::env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."));
        Self {
            data_dir: xdg_dir("XDG_DATA_HOME", &home, ".local/share"),
            config_dir: xdg_dir("XDG_CONFIG_HOME", &home, ".config"),
            cache_dir: xdg_dir("XDG_CACHE_HOME", &home, ".cache"),
        }
    }

    /// Construct paths rooted at a single base directory — used in tests.
    #[cfg(test)]
    pub fn with_base(base: &std::path::Path) -> Self {
        Self {
            data_dir: base.join("data"),
            config_dir: base.join("config"),
            cache_dir: base.join("cache"),
        }
    }

    /// Root directory that holds all installed toolchain versions.
    ///
    /// `$XDG_DATA_HOME/facet/toolchains/`
    pub fn toolchains_dir(&self) -> PathBuf {
        self.data_dir.join("toolchains")
    }

    /// Directory for a specific installed toolchain version.
    ///
    /// `$XDG_DATA_HOME/facet/toolchains/<version>/`
    pub fn toolchain_dir(&self, version: &str) -> PathBuf {
        self.toolchains_dir().join(version)
    }

    /// Path to the facet configuration file.
    ///
    /// `$XDG_CONFIG_HOME/facet/facet.toml`
    pub fn config_file(&self) -> PathBuf {
        self.config_dir.join("facet.toml")
    }

    /// Root cache directory.
    ///
    /// `$XDG_CACHE_HOME/facet/`
    pub fn cache_dir(&self) -> PathBuf {
        self.cache_dir.clone()
    }
}

/// Resolve an XDG base directory.
///
/// Uses the env var if set and non-empty, otherwise falls back to `~/default_suffix`.
/// Appends `facet` as the application subdirectory.
fn xdg_dir(env_var: &str, home: &PathBuf, default_suffix: &str) -> PathBuf {
    let base = match std::env::var(env_var) {
        Ok(val) if !val.is_empty() => PathBuf::from(val),
        _ => home.join(default_suffix),
    };
    base.join("facet")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_base_dirs_are_absolute() {
        let paths = Paths::new();
        assert!(paths.data_dir.is_absolute());
        assert!(paths.config_dir.is_absolute());
        assert!(paths.cache_dir.is_absolute());
    }

    #[test]
    fn defaults_follow_xdg_spec() {
        let home = PathBuf::from(std::env::var("HOME").unwrap());
        let paths = Paths::new();
        // Only check the defaults if the XDG env vars are not set
        if std::env::var("XDG_CONFIG_HOME").is_err() {
            assert_eq!(paths.config_dir, home.join(".config/facet"));
        }
        if std::env::var("XDG_DATA_HOME").is_err() {
            assert_eq!(paths.data_dir, home.join(".local/share/facet"));
        }
        if std::env::var("XDG_CACHE_HOME").is_err() {
            assert_eq!(paths.cache_dir, home.join(".cache/facet"));
        }
    }

    #[test]
    fn toolchains_dir_is_under_data_dir() {
        let paths = Paths::new();
        assert_eq!(paths.toolchains_dir(), paths.data_dir.join("toolchains"));
    }

    #[test]
    fn toolchain_dir_appends_version() {
        let paths = Paths::new();
        assert_eq!(
            paths.toolchain_dir("1.2.3"),
            paths.data_dir.join("toolchains/1.2.3")
        );
    }

    #[test]
    fn config_file_is_toml_under_config_dir() {
        let paths = Paths::new();
        assert_eq!(paths.config_file(), paths.config_dir.join("facet.toml"));
    }

    #[test]
    fn cache_dir_helper_matches_base() {
        let paths = Paths::new();
        assert_eq!(paths.cache_dir(), paths.cache_dir);
    }
}
