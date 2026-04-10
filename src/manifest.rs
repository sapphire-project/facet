use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

pub const MANIFEST_FILE: &str = "facet.toml";

/// Project manifest — `facet.toml` in the project root.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Manifest {
    #[serde(default)]
    pub package: PackageMetadata,

    /// Optional toolchain pin for this project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolchain: Option<ToolchainPin>,

    /// Direct dependencies.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub dependencies: BTreeMap<String, String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PackageMetadata {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolchainPin {
    /// Pinned Sapphire version for this project.
    pub version: String,
}

impl Manifest {
    /// Load `facet.toml` from the given directory.
    pub fn load(dir: &Path) -> Result<Self> {
        let path = dir.join(MANIFEST_FILE);
        let raw = std::fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        toml::from_str(&raw)
            .with_context(|| format!("failed to parse {}", path.display()))
    }

    /// Persist `facet.toml` into the given directory.
    pub fn save(&self, dir: &Path) -> Result<()> {
        let path = dir.join(MANIFEST_FILE);
        let contents = toml::to_string_pretty(self)
            .context("failed to serialise manifest")?;
        std::fs::write(&path, contents)
            .with_context(|| format!("failed to write {}", path.display()))
    }

    /// Return the pinned toolchain version, if present.
    pub fn pinned_version(&self) -> Option<&str> {
        self.toolchain.as_ref().map(|t| t.version.as_str())
    }
}

/// Pin the toolchain version in the `facet.toml` found in `dir`.
/// The file must already exist (i.e. inside a project).
pub fn pin_version(dir: &Path, version: &str) -> Result<()> {
    let path = dir.join(MANIFEST_FILE);
    if !path.exists() {
        bail!(
            "no {} found in {}. Run `facet init` first.",
            MANIFEST_FILE,
            dir.display()
        );
    }
    let mut manifest = Manifest::load(dir)?;
    manifest.toolchain = Some(ToolchainPin {
        version: version.to_string(),
    });
    manifest.save(dir)
}

/// Walk up from `start` until a `facet.toml` containing a `[toolchain]`
/// section is found. Returns the pinned version string if found.
pub fn find_pinned_version(start: &Path) -> Option<String> {
    let mut dir = start;
    loop {
        let candidate = dir.join(MANIFEST_FILE);
        if candidate.exists() {
            if let Ok(manifest) = Manifest::load(dir) {
                if let Some(v) = manifest.pinned_version() {
                    return Some(v.to_string());
                }
            }
        }
        match dir.parent() {
            Some(parent) => dir = parent,
            None => return None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn write(dir: &Path, contents: &str) {
        fs::write(dir.join(MANIFEST_FILE), contents).unwrap();
    }

    #[test]
    fn load_minimal_manifest() {
        let dir = tempfile::tempdir().unwrap();
        write(
            dir.path(),
            r#"
[package]
name = "hello"
version = "0.1.0"
"#,
        );
        let m = Manifest::load(dir.path()).unwrap();
        assert_eq!(m.package.name, "hello");
        assert!(m.toolchain.is_none());
    }

    #[test]
    fn load_manifest_with_toolchain_pin() {
        let dir = tempfile::tempdir().unwrap();
        write(
            dir.path(),
            r#"
[package]
name = "hello"
version = "0.1.0"

[toolchain]
version = "1.2.3"
"#,
        );
        let m = Manifest::load(dir.path()).unwrap();
        assert_eq!(m.pinned_version(), Some("1.2.3"));
    }

    #[test]
    fn pin_version_round_trips() {
        let dir = tempfile::tempdir().unwrap();
        write(
            dir.path(),
            "[package]\nname = \"x\"\nversion = \"0.1.0\"\n",
        );
        pin_version(dir.path(), "2.0.0").unwrap();
        let m = Manifest::load(dir.path()).unwrap();
        assert_eq!(m.pinned_version(), Some("2.0.0"));
    }

    #[test]
    fn pin_version_overwrites_existing_pin() {
        let dir = tempfile::tempdir().unwrap();
        write(
            dir.path(),
            "[package]\nname = \"x\"\nversion = \"0.1.0\"\n[toolchain]\nversion = \"1.0.0\"\n",
        );
        pin_version(dir.path(), "3.0.0").unwrap();
        let m = Manifest::load(dir.path()).unwrap();
        assert_eq!(m.pinned_version(), Some("3.0.0"));
    }

    #[test]
    fn pin_version_fails_without_manifest() {
        let dir = tempfile::tempdir().unwrap();
        let err = pin_version(dir.path(), "1.0.0").unwrap_err();
        assert!(err.to_string().contains("no facet.toml"));
    }

    #[test]
    fn find_pinned_version_in_same_dir() {
        let dir = tempfile::tempdir().unwrap();
        write(
            dir.path(),
            "[package]\nname = \"x\"\nversion = \"0.1.0\"\n[toolchain]\nversion = \"1.1.0\"\n",
        );
        assert_eq!(
            find_pinned_version(dir.path()),
            Some("1.1.0".to_string())
        );
    }

    #[test]
    fn find_pinned_version_walks_up() {
        let root = tempfile::tempdir().unwrap();
        write(
            root.path(),
            "[package]\nname = \"x\"\nversion = \"0.1.0\"\n[toolchain]\nversion = \"4.0.0\"\n",
        );
        let sub = root.path().join("src/deep");
        fs::create_dir_all(&sub).unwrap();
        assert_eq!(
            find_pinned_version(&sub),
            Some("4.0.0".to_string())
        );
    }

    #[test]
    fn find_pinned_version_returns_none_when_not_found() {
        let dir = tempfile::tempdir().unwrap();
        // manifest exists but has no [toolchain] section
        write(
            dir.path(),
            "[package]\nname = \"x\"\nversion = \"0.1.0\"\n",
        );
        assert_eq!(find_pinned_version(dir.path()), None);
    }
}
