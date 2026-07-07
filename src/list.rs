use anyhow::Result;
use crate::config::XConfig;
use comfy_table::Table;
use comfy_table::presets::UTF8_FULL;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use std::fs;

fn format_timestamp(ts: &str) -> String {
    // Parse RFC3339 timestamp and format as a more readable date
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(ts) {
        dt.format("%Y-%m-%d %H:%M").to_string()
    } else {
        ts.to_string()
    }
}

pub fn list_scripts(config: &XConfig) -> Result<()> {
    if !config.scripts_dir.exists() {
        println!("No scripts found");
        return Ok(());
    }
    
    let mut scripts: Vec<_> = fs::read_dir(&config.scripts_dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() {
                path.file_name()?.to_str().map(|s| s.to_string())
            } else {
                None
            }
        })
        .collect();
    
    scripts.sort();
    
    if scripts.is_empty() {
        println!("No scripts found");
        return Ok(());
    }
    
    let activity_metadata = config.load_activity_metadata()?;
    
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["Name", "Type", "Summary", "Created", "Updated", "Last Executed"]);
    
    for script in scripts {
        let metadata = config.load_metadata(&script).ok().flatten();
        let description = metadata
            .as_ref()
            .and_then(|m| m.description.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("-");
        let script_type = metadata
            .as_ref()
            .and_then(|m| m.program.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("bash");
        
        // Remove file extension from script name for display
        let display_name = if let Some(dot_pos) = script.rfind('.') {
            &script[..dot_pos]
        } else {
            &script
        };
        
        let activity = activity_metadata.scripts.get(&script);
        let created = activity
            .map(|a| format_timestamp(&a.created))
            .unwrap_or_else(|| "-".to_string());
        let updated = activity
            .map(|a| format_timestamp(&a.updated))
            .unwrap_or_else(|| "-".to_string());
        let last_executed = activity
            .and_then(|a| a.last_executed.as_ref())
            .map(|ts| format_timestamp(ts))
            .unwrap_or_else(|| "-".to_string());
        
        table.add_row(vec![
            display_name.to_string(),
            script_type.to_string(),
            description.to_string(),
            created,
            updated,
            last_executed,
        ]);
    }
    
    println!("{}", table);
    
    Ok(())
}

