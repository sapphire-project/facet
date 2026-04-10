use anyhow::Result;
use crate::paths::Paths;
use super::passthrough;

pub fn run() -> Result<()> {
    let paths = Paths::new();
    passthrough::exec(&["console".to_string()], &paths)
}
