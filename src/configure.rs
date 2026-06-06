use anyhow::{Context, Result};
use crate::config::XConfig;
use crate::ai;
use inquire::Select;

pub fn configure(config: &XConfig) -> Result<()> {
    let options = vec![
        "Default Script Language",
        "LLM Provider",
    ];
    
    let selected = Select::new("Configuration Options:", options.clone())
        .with_help_message("Select a configuration option to modify")
        .prompt()
        .context("Failed to get configuration selection")?;
    
    if selected == "Default Script Language" {
        configure_default_program(config)?;
    } else if selected == "LLM Provider" {
        configure_llm_provider()?;
    } else {
        anyhow::bail!("Invalid selection");
    }
    
    Ok(())
}

fn configure_default_program(config: &XConfig) -> Result<()> {
    let program_options: Vec<(&str, &str)> = vec![
        ("bash", "Bash/Shell Script"),
        ("zsh", "Zsh Script"),
        ("sh", "POSIX Shell Script"),
        ("node", "JavaScript/TypeScript via Node.js"),
        ("python", "Python 3 Script"),
        ("ruby", "Ruby Script"),
        ("perl", "Perl Script"),
        ("go", "Go Program"),
        ("rust", "Rust Program"),
        ("python2", "Python 2 Script"),
        ("php", "PHP Script"),
        ("lua", "Lua Script"),
        ("deno", "Deno (JavaScript/TypeScript runtime)"),
        ("swift", "Swift Script"),
        ("c", "C Program"),
        ("cpp", "C++ Program"),
        ("java", "Java Program"),
        ("r", "R Script"),
        ("awk", "AWK Script"),
        ("elixir", "Elixir Script"),
        ("clj", "Clojure Script"),
        ("scala", "Scala Script"),
        ("haskell", "Haskell Program"),
        ("powershell", "PowerShell Script"),
        ("kotlin", "Kotlin Script/Program"),
    ];
    
    let display_options: Vec<String> = program_options.iter()
        .map(|(_, display)| format!("{}", display))
        .collect();
    
    // Load current default program if it exists
    let current_default = config.load_default_program()
        .context("Failed to load current default program")?;
    
    let starting_cursor = current_default.as_ref()
        .and_then(|default| {
            program_options.iter()
                .position(|(val, _)| *val == default.as_str())
        })
        .unwrap_or(0);
    
    let selected_display = Select::new("Select default program:", display_options.clone())
        .with_help_message("This will be used as the default program for new scripts")
        .with_starting_cursor(starting_cursor)
        .prompt()
        .context("Failed to get program selection")?;
    
    // Find the index of the selected display option and get the corresponding program value
    let selected_index = display_options.iter()
        .position(|opt| opt == &selected_display)
        .context("Failed to find selected program")?;
    let program = program_options[selected_index].0;
    
    config.save_default_program(program)
        .context("Failed to save default program")?;
    
    println!("✓ Default program set to: {}", program);
    Ok(())
}

fn configure_llm_provider() -> Result<()> {
    use crate::utils;
    use std::fs;
    
    let llm_script_path = ai::get_llm_script_path();
    
    // Ensure the config directory exists
    if let Some(parent) = llm_script_path.parent() {
        fs::create_dir_all(parent)
            .context("Failed to create config directory")?;
    }
    
    // Write template if file doesn't exist, so editor can open it with content
    if !llm_script_path.exists() {
        let template = ai::get_llm_template();
        fs::write(&llm_script_path, template)
            .context("Failed to write LLM script template")?;
    }
    
    // Edit the script (now it exists with template or existing content)
    utils::edit_file(&llm_script_path)
        .context("Failed to edit LLM script")?;
    
    // Make it executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&llm_script_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&llm_script_path, perms)?;
    }
    
    println!("✓ LLM provider script saved to: {}", llm_script_path.display());
    Ok(())
}

