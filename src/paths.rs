use std::path::PathBuf;

/// Resolved XDG-compliant base directories for facet.
///
/// Resolution order:
///   1. XDG / platform default via the `dirs` crate
///   2. Fallback: `~/.facet/{data,config,cache}`
pub struct Paths {
    data_dir: PathBuf,
    config_dir: PathBuf,
    cache_dir: PathBuf,
}

impl Paths {
    pub fn new() -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        Self {
            data_dir: dirs::data_dir()
                .unwrap_or_else(|| home.join(".facet/data"))
                .join("facet"),
            config_dir: dirs::config_dir()
                .unwrap_or_else(|| home.join(".facet/config"))
                .join("facet"),
            cache_dir: dirs::cache_dir()
                .unwrap_or_else(|| home.join(".facet/cache"))
                .join("facet"),
        }
    }

    /// Root directory that holds all installed toolchain versions.
    ///
    /// `<data_dir>/toolchains/`
    pub fn toolchains_dir(&self) -> PathBuf {
        self.data_dir.join("toolchains")
    }

    /// Directory for a specific installed toolchain version.
    ///
    /// `<data_dir>/toolchains/<version>/`
    pub fn toolchain_dir(&self, version: &str) -> PathBuf {
        self.toolchains_dir().join(version)
    }

    /// Path to the facet configuration file.
    ///
    /// `<config_dir>/facet.toml`
    pub fn config_file(&self) -> PathBuf {
        self.config_dir.join("facet.toml")
    }

    /// Root cache directory.
    ///
    /// `<cache_dir>/`
    pub fn cache_dir(&self) -> PathBuf {
        self.cache_dir.clone()
    }
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
    fn all_base_dirs_end_with_facet() {
        let paths = Paths::new();
        assert!(paths.data_dir.ends_with("facet"));
        assert!(paths.config_dir.ends_with("facet"));
        assert!(paths.cache_dir.ends_with("facet"));
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
