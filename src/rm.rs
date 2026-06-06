use anyhow::{Context, Result};
use crate::config::XConfig;
use inquire::Confirm;
use std::fs;

pub fn remove_script(config: &XConfig, name: &str) -> Result<()> {
    // Find the actual script file (handles extensions)
    let actual_name = config.find_script(name)?
        .ok_or_else(|| anyhow::anyhow!("Script '{}' not found", name))?;
    
    let script_path = config.get_script_path(&actual_name);
    
    // Ask for confirmation before removal
    let confirmed = Confirm::new(&format!("Are you sure you want to remove script '{}'?", name))
        .with_default(false)
        .with_help_message("This action cannot be undone")
        .prompt()
        .context("Failed to get confirmation")?;
    
    if !confirmed {
        println!("Removal cancelled");
        return Ok(());
    }
    
    // Remove script file (this is critical, so fail if it doesn't work)
    fs::remove_file(&script_path)
        .with_context(|| format!("Failed to remove script file: {}", script_path.display()))?;
    
    // Remove TOML metadata file if it exists (non-critical, but try anyway)
    let _ = config.remove_metadata(&actual_name);
    
    // Remove from JSON activity metadata (non-critical, but try anyway)
    let _ = (|| -> Result<()> {
        let mut activity_metadata = config.load_activity_metadata()?;
        activity_metadata.scripts.remove(&actual_name);
        config.save_activity_metadata(&activity_metadata)?;
        Ok(())
    })();
    
    println!("✓ {} removed successfully", actual_name);
    Ok(())
}

