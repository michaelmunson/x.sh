//! Implementation of `x --src <name>`: print the absolute path of a script
//! or app file without running it.
//!
//! Resolution order (most specific first):
//!
//! 1. `./<name>.x.yml` (current directory)
//! 2. `~/.x.sh/apps/<name>.x.yml`
//! 3. `~/.x.sh/scripts/<name>` (or with extension via `find_script`)
//!
//! Local `x.yml` keys are intentionally not matched — they are inline strings,
//! not file paths.

use anyhow::{anyhow, Context, Result};
use std::path::PathBuf;

use crate::config::XConfig;

pub fn print_src(config: &XConfig, name: &str) -> Result<()> {
    let path = resolve(config, name)?
        .ok_or_else(|| anyhow!("no script or app named `{}` was found", name))?;
    let abs = std::fs::canonicalize(&path)
        .with_context(|| format!("failed to canonicalize {}", path.display()))?;
    println!("{}", abs.display());
    Ok(())
}

fn resolve(config: &XConfig, name: &str) -> Result<Option<PathBuf>> {
    if let Some(path) = config.find_app(name)? {
        return Ok(Some(path));
    }
    if let Some(filename) = config.find_script(name)? {
        return Ok(Some(config.get_script_path(&filename)));
    }
    Ok(None)
}
