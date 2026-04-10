use super::passthrough;
use crate::paths::Paths;
use anyhow::Result;

pub fn run(script: Option<String>, args: Vec<String>) -> Result<()> {
    let paths = Paths::new();
    let mut sapphire_args = vec!["run".to_string()];
    if let Some(s) = script {
        sapphire_args.push(s);
    }
    sapphire_args.extend(args);
    passthrough::exec(&sapphire_args, &paths)
}
