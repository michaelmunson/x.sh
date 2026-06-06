use anyhow::{Context, Result};
use std::env;
use std::fs;
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

pub fn edit_script(initial_content: &str) -> Result<String> {
    let editor = get_editor();
    let mut temp_file = tempfile::NamedTempFile::new()?;
    temp_file.write_all(initial_content.as_bytes())?;
    let temp_path = temp_file.path().to_path_buf();
    temp_file.flush()?;
    drop(temp_file);
    
    let status = Command::new(&editor)
        .arg(&temp_path)
        .status()
        .context(format!("Failed to open editor: {}", editor))?;
    
    if !status.success() {
        anyhow::bail!("Editor exited with non-zero status");
    }
    
    let content = fs::read_to_string(&temp_path)
        .context("Failed to read edited script")?;
    Ok(content)
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

