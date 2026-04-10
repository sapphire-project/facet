use anyhow::Result;
use crate::paths::Paths;
use super::passthrough;

pub fn run(args: Vec<String>) -> Result<()> {
    let paths = Paths::new();
    let mut sapphire_args = vec!["lint".to_string()];
    sapphire_args.extend(args);
    passthrough::exec(&sapphire_args, &paths)
}
