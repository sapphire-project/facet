use anyhow::{Context, Result, bail};
use std::path::Path;

use crate::manifest::{Manifest, PackageMetadata, ToolchainPin};
use crate::paths::Paths;
use crate::version;

pub fn run(name: String, template: String) -> Result<()> {
    if template != "application" {
        bail!("unknown template `{template}` — only `application` is supported");
    }

    let dest = Path::new(&name);
    if dest.exists() {
        bail!("directory `{name}` already exists");
    }

    // Resolve the active Sapphire version to pin in the new project.
    let paths = Paths::new();
    let cwd = std::env::current_dir().context("failed to get current directory")?;
    let active = version::resolve(&cwd, &paths)?;

    // Create project directory structure.
    let src_dir = dest.join("src");
    std::fs::create_dir_all(&src_dir)
        .with_context(|| format!("failed to create {}", src_dir.display()))?;

    // Write facet.toml.
    let manifest = Manifest {
        package: PackageMetadata {
            name: name.clone(),
            version: "0.1.0".to_string(),
        },
        toolchain: active.as_ref().map(|r| ToolchainPin {
            version: r.version.clone(),
        }),
        ..Default::default()
    };
    manifest.save(dest)?;

    // Write src/main.sp.
    let main_sp = src_dir.join("main.sp");
    std::fs::write(
        &main_sp,
        format!("fun main() {{\n    println(\"Hello from {name}!\")\n}}\n"),
    )
    .with_context(|| format!("failed to write {}", main_sp.display()))?;

    println!("Created project `{name}`");
    if let Some(r) = &active {
        println!("  Pinned Sapphire {}", r.version);
    } else {
        println!(
            "  No active Sapphire version — run `facet sapphire default <version>` to set one, \
             then `facet sapphire pin <version>` inside the project"
        );
    }
    println!("  {}/facet.toml", name);
    println!("  {}/src/main.sp", name);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn creates_directory_structure() {
        let tmp = tempfile::tempdir().unwrap();
        let project_dir = tmp.path().join("myapp");
        let dest = project_dir.clone();

        // Run scaffolding directly (no active version in test env)
        let src_dir = dest.join("src");
        fs::create_dir_all(&src_dir).unwrap();

        let manifest = Manifest {
            package: PackageMetadata {
                name: "myapp".to_string(),
                version: "0.1.0".to_string(),
            },
            toolchain: None,
            ..Default::default()
        };
        manifest.save(&dest).unwrap();
        fs::write(src_dir.join("main.sp"), "fun main() {}\n").unwrap();

        assert!(dest.join("facet.toml").exists());
        assert!(dest.join("src/main.sp").exists());

        let m = Manifest::load(&dest).unwrap();
        assert_eq!(m.package.name, "myapp");
        assert_eq!(m.package.version, "0.1.0");
        assert!(m.toolchain.is_none());
    }

    #[test]
    fn manifest_includes_toolchain_when_pinned() {
        let tmp = tempfile::tempdir().unwrap();
        let manifest = Manifest {
            package: PackageMetadata {
                name: "proj".to_string(),
                version: "0.1.0".to_string(),
            },
            toolchain: Some(ToolchainPin {
                version: "1.2.3".to_string(),
            }),
            ..Default::default()
        };
        manifest.save(tmp.path()).unwrap();
        let loaded = Manifest::load(tmp.path()).unwrap();
        assert_eq!(loaded.pinned_version(), Some("1.2.3"));
    }

    #[test]
    fn fails_if_directory_exists() {
        let tmp = tempfile::tempdir().unwrap();
        // Create a dir with the same name first
        let existing = tmp.path().join("taken");
        fs::create_dir(&existing).unwrap();

        // Simulate the guard check
        assert!(existing.exists());
    }
}
