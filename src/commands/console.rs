use super::passthrough;
use crate::paths::Paths;
use anyhow::Result;

pub fn run() -> Result<()> {
    let paths = Paths::new();
    passthrough::exec(&["console".to_string()], &paths)
}
