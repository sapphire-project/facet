use std::process;

pub fn run(args: Vec<String>) -> anyhow::Result<()> {
    // Delegate to the active `sapphire` binary on PATH.
    // Phase 1: just report the passthrough. Phase 2+ will resolve the binary.
    eprintln!(
        "facet: unknown command `{}` — passthrough not yet implemented",
        args[0]
    );
    process::exit(1);
}
