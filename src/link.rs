use anyhow::{Context, Result};
use crate::config::XConfig;
use crate::execute;
use inquire::Confirm;
use std::fs;
use std::process::Command;

/// Check if a script has been linked (by checking if any link in ~/.local/bin points to it)
pub fn is_script_linked(config: &XConfig, actual_name: &str, program: &str) -> Result<bool> {
    use crate::execute;
    
    let target_path = execute::get_executable_path(config, actual_name, program)?;
    
    let home = dirs::home_dir().context("Could not find home directory")?;
    let local_bin = home.join(".local/bin");
    
    if !local_bin.exists() {
        return Ok(false);
    }
    
    // Check all symlinks in ~/.local/bin to see if any point to our target
    let entries = std::fs::read_dir(&local_bin)
        .context("Failed to read ~/.local/bin directory")?;
    
    for entry in entries {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();
        
        if path.is_symlink() {
            if let Ok(target) = std::fs::read_link(&path) {
                if target == target_path {
                    return Ok(true);
                }
            }
        }
    }
    
    Ok(false)
}

pub fn remove_link(_config: &XConfig, link_name: &str) -> Result<()> {
    let home = dirs::home_dir().context("Could not find home directory")?;
    let local_bin = home.join(".local/bin");
    let link_path = local_bin.join(link_name);
    
    if !link_path.exists() && !link_path.is_symlink() {
        eprintln!("❌ Link '{}' not found at {}", link_name, link_path.display());
        anyhow::bail!("Link '{}' not found at {}", link_name, link_path.display());
    }
    
    // Ask for confirmation before removal
    let confirmed = Confirm::new(&format!("Are you sure you want to remove link '{}'?", link_name))
        .with_default(false)
        .with_help_message("This action cannot be undone")
        .prompt()
        .context("Failed to get confirmation")?;
    
    if !confirmed {
        println!("Removal cancelled");
        return Ok(());
    }
    
    fs::remove_file(&link_path)
        .with_context(|| format!("Failed to remove link: {}", link_path.display()))?;
    
    println!("✓ Link '{}' removed successfully", link_name);
    Ok(())
}

pub fn link_script(config: &XConfig, name: &str, link_name: Option<String>) -> Result<()> {
    // Find the actual script file (handles extensions)
    let actual_name = config.find_script(name)?
        .ok_or_else(|| anyhow::anyhow!("Script '{}' not found", name))?;
    
    let link_name = link_name.unwrap_or_else(|| name.to_string());
    
    // Load metadata to get the program type
    let metadata = config.load_metadata(&actual_name)?;
    let program = metadata
        .and_then(|m| m.program)
        .or_else(|| config.load_default_program().ok().flatten())
        .unwrap_or_else(|| "bash".to_string());
    
    // Get the executable path (compiled binary if needed, or script path for interpreted)
    let target_path = execute::get_executable_path(config, &actual_name, &program)?;
    
    let home = dirs::home_dir().context("Could not find home directory")?;
    let local_bin = home.join(".local/bin");
    let link_path = local_bin.join(&link_name);
    
    fs::create_dir_all(&local_bin)
        .context("Failed to create ~/.local/bin directory")?;
    
    if link_path.exists() || link_path.is_symlink() {
        fs::remove_file(&link_path)?;
    }
    
    let target_path_str = target_path.to_str()
        .context("Target path contains invalid UTF-8")?;
    let link_path_str = link_path.to_str()
        .context("Link path contains invalid UTF-8")?;
    
    let status = Command::new("ln")
        .arg("-s")
        .arg(target_path_str)
        .arg(link_path_str)
        .status()
        .context("Failed to execute ln command")?;
    
    if !status.success() {
        anyhow::bail!("ln command failed with exit code: {:?}", status.code());
    }
    
    println!("✓ Link '{}' created successfully at {}", link_name, link_path.display());
    Ok(())
}
