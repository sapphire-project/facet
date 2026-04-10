# facet

The official toolchain manager and project CLI for the [Sapphire](https://github.com/sapphire-project/sapphire) programming language.

## Installation

Download the latest release for your platform from the [releases page](https://github.com/sapphire-project/facet/releases) and place the binary somewhere on your `PATH`.

## Quick start

```sh
# Install a Sapphire toolchain
facet sapphire install latest

# Create a new project
facet new myproject
cd myproject

# Run it
facet run
```

## Commands

### Project

| Command | Description |
|---------|-------------|
| `facet new <name>` | Create a new Sapphire project |
| `facet run [script] [args...]` | Run a project script or binary |
| `facet test [args...]` | Run the project test suite |
| `facet lint [args...]` | Run the linter |
| `facet console` | Start an interactive Sapphire console (REPL) |
| `facet version` | Print facet and active Sapphire versions |

### Toolchain management

| Command | Description |
|---------|-------------|
| `facet sapphire list` | List installed Sapphire versions |
| `facet sapphire list --remote` | List available versions on GitHub |
| `facet sapphire install <version>` | Install a Sapphire version |
| `facet sapphire uninstall <version>` | Uninstall a Sapphire version |
| `facet sapphire use <version>` | Switch the global default to an installed version |
| `facet sapphire default <version>` | Set the global default version (without install check) |
| `facet sapphire current` | Show the active version and how it was resolved |
| `facet sapphire pin <version>` | Pin a version in the current project's `facet.toml` |

### facet itself

| Command | Description |
|---------|-------------|
| `facet self info` | Show facet binary path and data directory locations |

## Version resolution

When running a Sapphire command, facet resolves the active version in this order:

1. `SAPPHIRE_VERSION` environment variable
2. `[toolchain]` pin in the nearest `facet.toml` (walks up from the current directory)
3. Global default set via `facet sapphire use` or `facet sapphire default`

## Environment variables

| Variable | Description |
|----------|-------------|
| `SAPPHIRE_VERSION` | Override the active Sapphire version for the current shell |
| `GITHUB_TOKEN` | GitHub personal access token — increases API rate limits for `sapphire list --remote` and `sapphire install` |

## Data directories

facet follows the [XDG Base Directory](https://specifications.freedesktop.org/basedir-spec/latest/) specification:

| Path | Purpose |
|------|---------|
| `$XDG_DATA_HOME/facet/toolchains/` | Installed Sapphire toolchains |
| `$XDG_CONFIG_HOME/facet/facet.toml` | Global facet configuration |
| `$XDG_CACHE_HOME/facet/` | Cache |

Run `facet self info` to see the resolved paths on your system.

## Building from source

Requires Rust 1.85 or later (edition 2024).

```sh
git clone https://github.com/sapphire-project/facet
cd facet
cargo build --release
```
