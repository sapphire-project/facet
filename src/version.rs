use crate::{config::GlobalConfig, manifest::find_pinned_version, paths::Paths};
use anyhow::Result;
use std::path::Path;

/// The source from which a Sapphire version was resolved.
#[derive(Debug, PartialEq)]
pub enum VersionSource {
    /// `SAPPHIRE_VERSION` environment variable.
    EnvVar,
    /// `[toolchain]` section in a `facet.toml` found by walking up the tree.
    ProjectPin,
    /// `[toolchain] default` in the global facet config.
    GlobalDefault,
}

/// The resolved Sapphire version and where it came from.
#[derive(Debug)]
pub struct ResolvedVersion {
    pub version: String,
    pub source: VersionSource,
}

/// Resolve the active Sapphire version.
///
/// Resolution order:
///   1. `SAPPHIRE_VERSION` environment variable
///   2. Walk up from `cwd` looking for a `facet.toml` with `[toolchain]`
///   3. Global config default
pub fn resolve(cwd: &Path, paths: &Paths) -> Result<Option<ResolvedVersion>> {
    // 1. Env var
    if let Ok(v) = std::env::var("SAPPHIRE_VERSION")
        && !v.is_empty()
    {
        return Ok(Some(ResolvedVersion {
            version: v,
            source: VersionSource::EnvVar,
        }));
    }

    // 2. Project pin
    if let Some(v) = find_pinned_version(cwd) {
        return Ok(Some(ResolvedVersion {
            version: v,
            source: VersionSource::ProjectPin,
        }));
    }

    // 3. Global default
    let config = GlobalConfig::load(paths)?;
    if let Some(v) = config.toolchain.default {
        return Ok(Some(ResolvedVersion {
            version: v,
            source: VersionSource::GlobalDefault,
        }));
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paths::Paths;
    use std::fs;

    fn temp_paths() -> (tempfile::TempDir, Paths) {
        let dir = tempfile::tempdir().unwrap();
        let paths = Paths::with_base(dir.path());
        (dir, paths)
    }

    #[test]
    fn resolves_env_var_first() {
        let (_dir, paths) = temp_paths();
        let cwd = tempfile::tempdir().unwrap();
        // SAFETY: single-threaded test binary (each test is its own process here)
        unsafe { std::env::set_var("SAPPHIRE_VERSION", "9.9.9") };
        let r = resolve(cwd.path(), &paths).unwrap().unwrap();
        unsafe { std::env::remove_var("SAPPHIRE_VERSION") };
        assert_eq!(r.version, "9.9.9");
        assert_eq!(r.source, VersionSource::EnvVar);
    }

    #[test]
    fn resolves_project_pin_when_no_env() {
        let (_dir, paths) = temp_paths();
        let cwd = tempfile::tempdir().unwrap();
        unsafe { std::env::remove_var("SAPPHIRE_VERSION") };
        fs::write(
            cwd.path().join("facet.toml"),
            "[package]\nname=\"x\"\nversion=\"0.1.0\"\n[toolchain]\nversion=\"3.0.0\"\n",
        )
        .unwrap();
        let r = resolve(cwd.path(), &paths).unwrap().unwrap();
        assert_eq!(r.version, "3.0.0");
        assert_eq!(r.source, VersionSource::ProjectPin);
    }

    #[test]
    fn resolves_global_default_as_fallback() {
        let (_dir, paths) = temp_paths();
        let cwd = tempfile::tempdir().unwrap();
        unsafe { std::env::remove_var("SAPPHIRE_VERSION") };
        crate::config::set_default_version(&paths, "1.0.0").unwrap();
        let r = resolve(cwd.path(), &paths).unwrap().unwrap();
        assert_eq!(r.version, "1.0.0");
        assert_eq!(r.source, VersionSource::GlobalDefault);
    }

    #[test]
    fn returns_none_when_nothing_configured() {
        let (_dir, paths) = temp_paths();
        let cwd = tempfile::tempdir().unwrap();
        unsafe { std::env::remove_var("SAPPHIRE_VERSION") };
        let r = resolve(cwd.path(), &paths).unwrap();
        assert!(r.is_none());
    }
}
