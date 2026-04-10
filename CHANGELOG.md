# Changelog

## [0.2.0] - 2026-04-10

### Added

- `facet sapphire uninstall <version>` — removes an installed toolchain; clears the global default if it pointed at the removed version
- `facet sapphire use <version>` — switches the global default to an already-installed version, with an error and install hint if the version is not present
- `facet sapphire current` — prints the active Sapphire version and how it was resolved (env var, project pin, or global default)
- `facet info` — shows the facet binary path and XDG data, config, and cache directory locations
- `facet version` — prints the facet version and the active Sapphire version
- Progress bars and spinners for downloads and GitHub API calls (using `indicatif`)
- Linux x86_64 support (`sapphire-linux-x86_64`)
- README with installation instructions, command reference, version resolution docs, and environment variable reference

### Removed

- `--version` flag (use `facet version` instead)
- `facet init` stub — `facet new` covers the common case
- `facet add`, `facet remove`, `facet install` stubs — no package registry exists yet
- `facet self update` and `facet self uninstall` stubs — no install mechanism exists yet
- `facet self` subcommand group — `facet self info` is now `facet info`

## [0.1.0] - 2026-04-09

Initial release.

- `facet sapphire list` / `facet sapphire list --remote`
- `facet sapphire install <version>`
- `facet sapphire default <version>`
- `facet sapphire pin <version>`
- `facet new <name>`
- `facet run`, `facet test`, `facet lint`, `facet console` (passthrough to active Sapphire binary)
- SHA-256 verification of downloaded binaries
- XDG Base Directory support
- GitHub Actions CI and release workflows
