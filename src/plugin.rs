//! Plugin install and invocation (`x -i --plugin`, `x --plugin`).

use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::config::XConfig;

const REPO_URL: &str = "https://github.com/michaelmunson/x.sh.git";

/// Install a plugin binary into `~/.x.sh/plugins/<name>` (never on PATH).
pub fn install_plugin(config: &XConfig, name: &str) -> Result<()> {
    let name = name.trim();
    if name.is_empty() {
        bail!("Plugin name required");
    }
    if !is_valid_plugin_name(name) {
        bail!(
            "Invalid plugin name '{}': use letters, numbers, dashes, and underscores",
            name
        );
    }

    config.ensure_plugin_dirs()?;

    let temp_dir = tempfile::tempdir().context("Failed to create temp directory")?;
    let (build_dir, package_path) = resolve_plugin_source(name, temp_dir.path())?;

    println!("Building plugin '{}'…", name);
    let status = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("-p")
        .arg(plugin_package_name(name))
        .current_dir(&build_dir)
        .stdin(Stdio::null())
        .status()
        .context("Failed to run cargo build (is Rust/Cargo installed?)")?;

    if !status.success() {
        bail!("Failed to build plugin '{}'", name);
    }

    let binary_name = plugin_binary_name(name);
    let built = find_built_binary(&build_dir, &package_path, &binary_name)?;
    let dest = config.plugin_path(name);

    if dest.exists() {
        println!("Updating existing plugin at {}", dest.display());
    }

    fs::copy(&built, &dest)
        .with_context(|| format!("Failed to install plugin to {}", dest.display()))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&dest)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&dest, perms)?;
    }

    println!("Installed plugin '{}' → {}", name, dest.display());
    println!("Run with: x --plugin {} …", name);
    Ok(())
}

/// Run an installed plugin: `x --plugin <name> [args…]`.
pub fn run_plugin(config: &XConfig, name: &str, args: &[String]) -> Result<()> {
    let name = name.trim();
    if name.is_empty() {
        bail!("Plugin name required");
    }

    let path = config.plugin_path(name);
    if !path.is_file() {
        bail!(
            "Plugin '{}' is not installed. Install with: x -i --plugin {}",
            name,
            name
        );
    }

    let status = Command::new(&path)
        .args(args)
        .status()
        .with_context(|| format!("Failed to execute plugin '{}'", name))?;

    if !status.success() {
        let code = status.code().unwrap_or(1);
        std::process::exit(code);
    }
    Ok(())
}

fn is_valid_plugin_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

fn plugin_package_name(name: &str) -> String {
    format!("x-plugin-{}", name)
}

fn plugin_binary_name(name: &str) -> String {
    // Prefer a binary named after the plugin (`openapi`), falling back to the
    // cargo package name (`x-plugin-openapi`).
    name.to_string()
}

/// Locate plugin sources: prefer the current repo's `plugins/<name>`, else clone.
fn resolve_plugin_source(name: &str, temp_dir: &Path) -> Result<(PathBuf, PathBuf)> {
    if let Some(root) = find_repo_root() {
        let package_path = root.join("plugins").join(name);
        if package_path.join("Cargo.toml").is_file() {
            return Ok((root, package_path));
        }
    }

    println!("Cloning {}…", REPO_URL);
    let repo_dir = temp_dir.join("repo");
    let status = Command::new("git")
        .args(["clone", "--depth", "1", REPO_URL])
        .arg(&repo_dir)
        .stdin(Stdio::null())
        .status()
        .context("Failed to run git clone")?;

    if !status.success() {
        bail!("Failed to clone repository for plugin '{}'", name);
    }

    let package_path = repo_dir.join("plugins").join(name);
    if !package_path.join("Cargo.toml").is_file() {
        bail!(
            "Plugin '{}' not found in repository (expected plugins/{})",
            name,
            name
        );
    }

    Ok((repo_dir, package_path))
}

fn find_repo_root() -> Option<PathBuf> {
    // Walk from the running binary's directory and from CWD.
    let mut candidates = Vec::new();
    if let Ok(cwd) = std::env::current_dir() {
        candidates.push(cwd);
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            candidates.push(parent.to_path_buf());
        }
    }

    for start in candidates {
        for dir in start.ancestors() {
            let plugins = dir.join("plugins");
            let cargo = dir.join("Cargo.toml");
            if cargo.is_file() && plugins.is_dir() {
                return Some(dir.to_path_buf());
            }
        }
    }
    None
}

fn find_built_binary(build_dir: &Path, package_path: &Path, binary_name: &str) -> Result<PathBuf> {
    let release_dir = build_dir.join("target/release");
    let candidates = [
        release_dir.join(binary_name),
        release_dir.join(format!("x-plugin-{}", binary_name)),
        package_path
            .join("target/release")
            .join(binary_name),
    ];

    for path in &candidates {
        if path.is_file() {
            return Ok(path.clone());
        }
    }

    bail!(
        "Built plugin binary not found (looked for '{}' under {})",
        binary_name,
        release_dir.display()
    );
}
