use anyhow::{Context, Result, bail};
use sha2::{Digest, Sha256};
use std::path::Path;

pub(crate) const GITHUB_REPO: &str = "sapphire-project/sapphire";
pub(crate) const GITHUB_API_BASE: &str = "https://api.github.com";

// ── GitHub API types ─────────────────────────────────────────────────────────

#[derive(serde::Deserialize)]
struct GithubRelease {
    tag_name: String,
    assets: Vec<GithubAsset>,
}

#[derive(serde::Deserialize)]
struct GithubAsset {
    name: String,
    browser_download_url: String,
    /// SHA-256 digest embedded in the API response, e.g. `"sha256:<hex>"`.
    digest: Option<String>,
}

impl GithubAsset {
    /// Extract the hex portion of the `digest` field.
    fn sha256_hex(&self) -> Option<&str> {
        self.digest
            .as_deref()
            .and_then(|d| d.strip_prefix("sha256:"))
    }
}

// ── Platform ─────────────────────────────────────────────────────────────────

fn platform_asset_name() -> &'static str {
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        "sapphire-macos-aarch64"
    }
    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    {
        "sapphire-macos-x86_64"
    }
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
        "sapphire-linux-x86_64"
    }
}

// ── HTTP client ──────────────────────────────────────────────────────────────

fn http_client() -> Result<reqwest::blocking::Client> {
    let mut builder = reqwest::blocking::Client::builder()
        .user_agent(concat!("facet/", env!("CARGO_PKG_VERSION")));

    if let Ok(token) = std::env::var("GITHUB_TOKEN")
        && !token.is_empty()
    {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!("Bearer {token}"))
                .context("invalid GITHUB_TOKEN value")?,
        );
        builder = builder.default_headers(headers);
    }

    builder.build().context("failed to build HTTP client")
}

// ── Release fetching ─────────────────────────────────────────────────────────

fn fetch_release(client: &reqwest::blocking::Client, version: &str) -> Result<GithubRelease> {
    let url = if version == "latest" {
        format!("{GITHUB_API_BASE}/repos/{GITHUB_REPO}/releases/latest")
    } else {
        let tag = if version.starts_with('v') {
            version.to_string()
        } else {
            format!("v{version}")
        };
        format!("{GITHUB_API_BASE}/repos/{GITHUB_REPO}/releases/tags/{tag}")
    };

    let resp = client
        .get(&url)
        .send()
        .context("network error reaching GitHub API")?;

    let status = resp.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        bail!("Sapphire version `{version}` not found on GitHub");
    }
    if !status.is_success() {
        bail!("GitHub API error {status} for {url}");
    }

    resp.json::<GithubRelease>()
        .context("failed to parse GitHub release response")
}

// ── Download + verify ─────────────────────────────────────────────────────────

fn download_and_verify(client: &reqwest::blocking::Client, asset: &GithubAsset) -> Result<Vec<u8>> {
    eprint!("  Downloading {}... ", asset.name);

    let resp = client
        .get(&asset.browser_download_url)
        .send()
        .with_context(|| format!("network error downloading {}", asset.name))?;

    if !resp.status().is_success() {
        bail!(
            "download of {} failed with HTTP {}",
            asset.name,
            resp.status()
        );
    }

    let bytes = resp
        .bytes()
        .with_context(|| format!("failed to read response body for {}", asset.name))?
        .to_vec();

    eprintln!("{:.1} MiB", bytes.len() as f64 / 1_048_576.0);

    if let Some(expected) = asset.sha256_hex() {
        eprint!("  Verifying checksum... ");
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        let actual: String = hasher
            .finalize()
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        if actual != expected {
            bail!(
                "checksum mismatch for {}\n  expected: {expected}\n  actual:   {actual}",
                asset.name
            );
        }
        eprintln!("OK.");
    }

    Ok(bytes)
}

// ── Install ───────────────────────────────────────────────────────────────────

fn write_binary(data: &[u8], dest: &Path) -> Result<()> {
    use std::io::Write;
    use std::os::unix::fs::PermissionsExt;

    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }

    let mut file = std::fs::File::create(dest)
        .with_context(|| format!("failed to create {}", dest.display()))?;
    file.write_all(data)
        .with_context(|| format!("failed to write {}", dest.display()))?;

    std::fs::set_permissions(dest, std::fs::Permissions::from_mode(0o755))
        .with_context(|| format!("failed to set permissions on {}", dest.display()))?;

    Ok(())
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Fetch the list of available Sapphire versions from GitHub Releases.
///
/// Returns versions in descending order (newest first), with the `v` prefix stripped.
pub fn fetch_versions() -> Result<Vec<String>> {
    let client = http_client()?;
    let url = format!("{GITHUB_API_BASE}/repos/{GITHUB_REPO}/releases?per_page=100");

    let resp = client
        .get(&url)
        .send()
        .context("network error reaching GitHub API")?;

    if !resp.status().is_success() {
        bail!("GitHub API error {} for {url}", resp.status());
    }

    #[derive(serde::Deserialize)]
    struct Release {
        tag_name: String,
        prerelease: bool,
    }

    let releases: Vec<Release> = resp
        .json()
        .context("failed to parse GitHub releases response")?;

    Ok(releases
        .into_iter()
        .filter(|r| !r.prerelease)
        .map(|r| r.tag_name.trim_start_matches('v').to_string())
        .collect())
}

/// Download and install a Sapphire version into `toolchains_dir`.
///
/// `version` is either a semver string (`"0.1.0"`) or `"latest"`.
/// The binary is installed to `<toolchains_dir>/<version>/bin/sapphire`.
/// Returns the resolved version string (useful when `"latest"` is passed).
pub fn install(version: &str, toolchains_dir: &Path) -> Result<String> {
    let client = http_client()?;

    eprintln!("Fetching Sapphire release info ({version})...");
    let release = fetch_release(&client, version)?;

    let resolved = release.tag_name.trim_start_matches('v').to_string();

    let bin_path = toolchains_dir.join(&resolved).join("bin").join("sapphire");
    if bin_path.exists() {
        eprintln!("Sapphire {resolved} is already installed.");
        return Ok(resolved);
    }

    let asset_name = platform_asset_name();
    let asset = release
        .assets
        .iter()
        .find(|a| a.name == asset_name)
        .with_context(|| {
            format!(
                "no asset `{asset_name}` in release {}. Available: {}",
                release.tag_name,
                release
                    .assets
                    .iter()
                    .map(|a| a.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })?;

    let data = download_and_verify(&client, asset)?;

    eprint!("  Installing to {}... ", bin_path.display());
    write_binary(&data, &bin_path)?;
    eprintln!("done.");

    eprintln!("Installed Sapphire {resolved}.");
    Ok(resolved)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_hex_strips_prefix() {
        let asset = GithubAsset {
            name: "sapphire-macos-aarch64".into(),
            browser_download_url: String::new(),
            digest: Some("sha256:abc123".into()),
        };
        assert_eq!(asset.sha256_hex(), Some("abc123"));
    }

    #[test]
    fn sha256_hex_none_when_no_digest() {
        let asset = GithubAsset {
            name: "sapphire-macos-aarch64".into(),
            browser_download_url: String::new(),
            digest: None,
        };
        assert_eq!(asset.sha256_hex(), None);
    }

    #[test]
    fn platform_asset_name_is_known() {
        let name = platform_asset_name();
        assert!(
            name.starts_with("sapphire-macos-") || name.starts_with("sapphire-linux-"),
            "unexpected asset name: {name}"
        );
    }

    #[test]
    fn write_binary_creates_executable() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let dest = dir.path().join("bin/sapphire");
        write_binary(b"#!/bin/sh\necho hi", &dest).unwrap();
        assert!(dest.exists());
        let mode = std::fs::metadata(&dest).unwrap().permissions().mode();
        assert!(mode & 0o111 != 0, "expected executable bit to be set");
    }
}
