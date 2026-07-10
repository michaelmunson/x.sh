use anyhow::{Context, Result};
use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

pub fn prompt_input(prompt: &str) -> Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn get_editor() -> String {
    if let Ok(e) = env::var("EDITOR") {
        return e;
    }
    if let Ok(v) = env::var("VISUAL") {
        return v;
    }
    for candidate in &["nvim", "vi", "nano"] {
        if Command::new(candidate).arg("--version").output().is_ok() {
            return candidate.to_string();
        }
    }
    "vi".to_string()
}

pub fn edit_file(file_path: &Path) -> Result<()> {
    let editor = get_editor();
    
    let status = Command::new(&editor)
        .arg(file_path)
        .status()
        .context(format!("Failed to open editor: {}", editor))?;
    
    if !status.success() {
        anyhow::bail!("Editor exited with non-zero status");
    }
    
    Ok(())
}

