use anyhow::{Context, Result};
use std::env;
use std::path::PathBuf;
use std::process::Command;

/// Get the path to the LLM script
pub fn get_llm_script_path() -> PathBuf {
    let home = dirs::home_dir().expect("Could not find home directory");
    home.join(".x.sh").join("config").join("llm.sh")
}

/// Check if the LLM script exists
pub fn llm_script_exists() -> bool {
    get_llm_script_path().exists()
}

/// Call the LLM with a prompt and return the completion
pub fn call_llm(prompt: &str) -> Result<String> {
    let llm_script_path = get_llm_script_path();
    
    if !llm_script_path.exists() {
        anyhow::bail!(
            "LLM script not found at {}.\nPlease configure it with `x --config` and select 'LLM Provider'.",
            llm_script_path.display()
        );
    }
    
    // Get the default shell
    let shell = env::var("SHELL")
        .unwrap_or_else(|_| "/bin/sh".to_string());
    
    // Run the LLM script with the prompt as argument
    let output = Command::new(&shell)
        .arg(&llm_script_path)
        .arg(prompt)
        .output()
        .context("Failed to execute LLM script")?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("LLM script failed: {}", stderr);
    }
    
    let completion = String::from_utf8(output.stdout)
        .context("LLM script returned invalid UTF-8")?;
    
    Ok(completion.trim().to_string())
}

/// Get the LLM provider template
pub fn get_llm_template() -> String {
    r#"# write a shell script that outputs your model's completion
PROMPT="$1"
echo "example completion"
"#.to_string()
}

/// Handle the --ai command
pub fn ai_command() -> Result<()> {
    use inquire::{Text, Confirm};
    
    let prompt_text = Text::new("instructions>")
        .with_help_message("Enter a description of the CLI command you want to generate")
        .prompt()
        .context("Failed to get AI prompt")?;
    
    if prompt_text.trim().is_empty() {
        anyhow::bail!("Prompt cannot be empty");
    }
    
    let completion = call_llm(&prompt_text)?;
    
    if completion.trim().is_empty() {
        anyhow::bail!("LLM returned empty completion");
    }
    
    // Output the command with highlighting to make it stand out
    // Using ANSI escape codes for bold text
    println!("\n{}", "─".repeat(80));
    println!("\x1b[1m{}\x1b[0m", format!(" {:^78} ", "Generated Command"));
    println!();
    println!("\x1b[1;36m{}\x1b[0m ", completion);
    println!();
    println!("{}", "─".repeat(80));
    
    // Ask for confirmation to run the command
    let should_run = Confirm::new("Run command?")
        .with_default(false)
        .prompt()
        .context("Failed to get confirmation")?;
    
    if should_run {
        // Execute the command using the user's shell to support pipes, redirects, etc.
        let shell = env::var("SHELL")
            .unwrap_or_else(|_| "/bin/sh".to_string());
        
        // Use shell -c to execute the command
        let status = Command::new(&shell)
            .arg("-c")
            .arg(&completion)
            .status()
            .context(format!("Failed to execute command: {}", completion))?;
        
        std::process::exit(status.code().unwrap_or(1));
    }
    
    Ok(())
}

