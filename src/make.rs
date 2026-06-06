use anyhow::{Context, Result};
use crate::config::XConfig;
use crate::metadata::{ScriptMetadata, Activity};
use crate::utils;
use inquire::{Text, Select, validator::Validation};
use std::fs;

fn validate_script_name(name: &str) -> Validation {
    if name.is_empty() {
        return Validation::Invalid("Script name cannot be empty".into());
    }
    
    // Validate: only letters, numbers, and dashes
    if !name.chars().all(|c| c.is_alphanumeric() || c == '-') {
        return Validation::Invalid(
            "Script name can only contain letters, numbers, and dashes".into()
        );
    }
    
    Validation::Valid
}

fn filename_stem(full_filename: &str) -> &str {
    match full_filename.rfind('.') {
        Some(i) if i > 0 => &full_filename[..i],
        _ => full_filename,
    }
}

fn remove_compile_cache_artifacts(config: &XConfig, full_filename: &str, program: &str) {
    if !crate::execute::needs_compilation(program) {
        return;
    }
    let script_path = config.get_script_path(full_filename);
    let Some(script_dir) = script_path.parent() else {
        return;
    };
    let cache_dir = script_dir.join(".cache");
    if !cache_dir.exists() {
        return;
    }
    let stem = filename_stem(full_filename);
    let _ = fs::remove_file(cache_dir.join(stem));
    let _ = fs::remove_file(cache_dir.join(format!("{stem}.class")));
    let _ = fs::remove_file(cache_dir.join(format!("{stem}.jar")));
}

fn apply_script_rename(
    config: &XConfig,
    old_full: &str,
    new_full: &str,
    program: &str,
) -> Result<()> {
    let old_path = config.get_script_path(old_full);
    let new_path = config.get_script_path(new_full);
    if new_path.exists() {
        anyhow::bail!("A script file '{}' already exists", new_full);
    }
    remove_compile_cache_artifacts(config, old_full, program);
    fs::rename(&old_path, &new_path)
        .with_context(|| format!("Failed to rename script to {}", new_full))?;
    if let Some(meta) = config.load_metadata(old_full)? {
        config.save_metadata(new_full, &meta)?;
        config.remove_metadata(old_full)?;
    }
    let mut activity = config.load_activity_metadata()?;
    if let Some(entry) = activity.scripts.remove(old_full) {
        activity.scripts.insert(new_full.to_string(), entry);
    }
    config.save_activity_metadata(&activity)?;
    Ok(())
}

/// When editing an existing script, offer to rename before opening the editor.
fn prompt_rename_if_existing(
    config: &XConfig,
    script_existed: bool,
    full_filename: String,
    name: String,
    program: &str,
) -> Result<(String, String)> {
    if !script_existed {
        return Ok((full_filename, name));
    }
    let new_name = Text::new("Script name:")
        .with_default(&name)
        .with_help_message("Change the command name before editing, or keep the current name.")
        .with_validator(|input: &str| {
            Ok::<Validation, Box<dyn std::error::Error + Send + Sync>>(validate_script_name(input))
        })
        .prompt()
        .context("Failed to get script name")?;
    let new_full = get_filename_with_extension(&new_name, program);
    if new_full == full_filename {
        return Ok((full_filename, name));
    }
    if let Some(existing) = config.find_script(&new_name)? {
        if existing != full_filename {
            anyhow::bail!(
                "A script named '{}' already exists (as '{}')",
                new_name,
                existing
            );
        }
    }
    apply_script_rename(config, &full_filename, &new_full, program)?;
    Ok((new_full, new_name))
}

fn get_filename_with_extension(name: &str, program: &str) -> String {
    // Map program types to file extensions
    let extension = match program {
        "c" => ".c",
        "cpp" => ".cpp",
        "java" => ".java",
        "go" => ".go",
        "rust" => ".rs",
        "python" => ".py",
        "python2" => ".py",
        "ruby" => ".rb",
        "perl" => ".pl",
        "php" => ".php",
        "lua" => ".lua",
        "swift" => ".swift",
        "r" => ".r",
        "scala" => ".scala",
        "kotlin" => ".kt",
        "haskell" => ".hs",
        "clj" => ".clj",
        "elixir" => ".exs",
        "powershell" => ".ps1",
        _ => "", // No extension for interpreted scripts (bash, sh, zsh, node, etc.)
    };
    
    // Only add extension if the filename doesn't already have it
    if extension.is_empty() || name.ends_with(extension) {
        name.to_string()
    } else {
        format!("{}{}", name, extension)
    }
}

fn get_program_template(program: &str, script_name: &str) -> String {
    // Convert script name to camelCase for Java class name
    let camel_case_name: String = script_name
        .split('-')
        .enumerate()
        .map(|(i, part)| {
            if i == 0 {
                part.to_string()
            } else {
                let mut chars = part.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            }
        })
        .collect();
    
    match program {
        "bash" => format!("#!/usr/bin/env bash\n# {}\n", script_name),
        "zsh" => format!("#!/usr/bin/env zsh\n# {}\n", script_name),
        "sh" => format!("#!/usr/bin/env sh\n# {}\n", script_name),
        "node" => format!("#!/usr/bin/env node\n\n(async () => {{\n  // {}\n}})()\n", script_name),
        "python" => format!("#!/usr/bin/env python3\n# {}\n", script_name),
        "ruby" => format!("#!/usr/bin/env ruby\n# {}\n", script_name),
        "perl" => format!("#!/usr/bin/env perl\n# {}\n", script_name),
        "go" => format!("// {}\npackage main\n\nimport \"fmt\"\n\nfunc main() {{\n    fmt.Println(\"Hello from {}\")\n}}\n", script_name, script_name),
        "rust" => format!("// {}\nfn main() {{\n    println!(\"Hello from {}\");\n}}\n", script_name, script_name),
        "python2" => format!("#!/usr/bin/env python2\n# {}\n", script_name),
        "php" => format!("#!/usr/bin/env php\n<?php\n// {}\n", script_name),
        "lua" => format!("#!/usr/bin/env lua\n-- {}\n", script_name),
        "deno" => format!("#!/usr/bin/env -S deno run\n// {}\n", script_name),
        "swift" => format!("#!/usr/bin/env swift\n// {}\nprint(\"Hello from {}\")\n", script_name, script_name),
        "c" => format!("// {}\n#include <stdio.h>\n\nint main() {{\n    printf(\"Hello from {}\\n\");\n    return 0;\n}}\n", script_name, script_name),
        "cpp" => format!("// {}\n#include <iostream>\n\nint main() {{\n    std::cout << \"Hello from {}\" << std::endl;\n    return 0;\n}}\n", script_name, script_name),
        "java" => format!("// {}\npublic class {} {{\n    public static void main(String[] args) {{\n        System.out.println(\"Hello from {}\");\n    }}\n}}\n", script_name, camel_case_name, script_name),
        "r" => format!("#!/usr/bin/env Rscript\n# {}\n", script_name),
        "awk" => format!("#!/usr/bin/env awk\n# {}\nBEGIN {{ print \"Hello from {}\" }}\n", script_name, script_name),
        "elixir" => format!("#!/usr/bin/env elixir\n# {}\nIO.puts(\"Hello from {}\")\n", script_name, script_name),
        "clj" => format!("#!/usr/bin/env clojure\n;; {}\n(println \"Hello from {}\")\n", script_name, script_name),
        "scala" => format!("#!/usr/bin/env scala\n// {}\nprintln(\"Hello from {}\")\n", script_name, script_name),
        "haskell" => format!("#!/usr/bin/env runhaskell\n-- {}\nmain = putStrLn \"Hello from {}\"\n", script_name, script_name),
        "powershell" => format!("#!/usr/bin/env pwsh\n# {}\nWrite-Output \"Hello from {}\"\n", script_name, script_name),
        "kotlin" => format!("#!/usr/bin/env kotlin\n// {}\nprintln(\"Hello from {}\")\n", script_name, script_name),
        _ => format!("#!/usr/bin/env bash\n# {}\n", script_name),
    }
}

pub fn add_script(
    config: &XConfig,
    name: Option<String>,
    script: Option<String>,
) -> Result<()> {
    config.ensure_directories()?;
    
    let name = if let Some(n) = name {
        // Validate provided name
        match validate_script_name(&n) {
            Validation::Valid => n,
            Validation::Invalid(msg) => {
                let msg_str = format!("{:?}", msg);
                eprintln!("❌ {}", msg_str);
                anyhow::bail!("Invalid script name: {}", msg_str);
            }
        }
    } else {
        Text::new("Script Name:")
            .with_help_message("This will be the command you run, e.g., 'my-script'.")
            .with_validator(|input: &str| Ok::<Validation, Box<dyn std::error::Error + Send + Sync>>(validate_script_name(input)))
            .prompt()
            .context("Failed to get script name")?
    };
    
    // First, try to find existing script (handles extensions)
    let existing_script = config.find_script(&name)?;
    let (full_filename, program, description, script_existed) = if let Some(existing_name) = existing_script {
        // Script exists - use existing filename and metadata
        let existing_metadata = config.load_metadata(&existing_name)?;
        let program = existing_metadata.as_ref()
            .and_then(|m| m.program.clone())
            .or_else(|| config.load_default_program().ok().flatten())
            .unwrap_or_else(|| "bash".to_string());
        let description = existing_metadata.as_ref()
            .and_then(|m| m.description.clone());
        (existing_name, program, description, true)
    } else {
        // Script doesn't exist - need to get program type first to determine filename
        let mut program = config.load_default_program()
            .context("Failed to load default program")?
            .unwrap_or_else(|| "bash".to_string());
        let mut description = None;
        
        // Prompt for description (optional)
        let description_input = Text::new("Enter a short description for your script")
            .with_help_message("This is an optional, concise description, e.g., 'Downloads page and prints summary'.")
            .with_default(description.as_deref().unwrap_or("'n/a'"))
            .prompt_skippable()
            .context("Failed to get description")?;
        
        description = description_input.filter(|s| !s.trim().is_empty());
        
        // Prompt for program selection
        let mut program_options: Vec<(&str, &str)> = vec![
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
        
        // Reorder list to put default program first
        let default_program = config.load_default_program()
            .context("Failed to load default program")?
            .unwrap_or_else(|| "bash".to_string());
        
        if let Some(default_idx) = program_options.iter()
            .position(|(val, _)| *val == default_program.as_str()) {
            let default_option = program_options.remove(default_idx);
            program_options.insert(0, default_option);
        }
        
        let display_options: Vec<String> = program_options.iter()
            .map(|(_, display)| format!("{}", display))
            .collect();
        
        let selected_display = Select::new("Which language do you want to use for your script?", 
            display_options.clone())
            .with_help_message("Select the programming language for your script")
            .with_starting_cursor(
                program_options.iter()
                    .position(|(val, _)| *val == format!("(default) {}", program.as_str()))
                    .unwrap_or(0)
            )
            .prompt()
            .context("Failed to get program selection")?;
        
        // Find the index of the selected display option and get the corresponding program value
        let selected_index = display_options.iter()
            .position(|opt| opt == &selected_display)
            .context("Failed to find selected program")?;
        program = program_options[selected_index].0.to_string();
        
        // Determine the full filename with extension
        let full_filename = get_filename_with_extension(&name, &program);
        (full_filename, program, description, false)
    };
    
    let (full_filename, name) =
        prompt_rename_if_existing(config, script_existed, full_filename, name, &program)?;
    
    let script_path = config.get_script_path(&full_filename);
    let should_edit_metadata = !script_existed;
    
    // Now handle script content
    let script_content = if let Some(s) = script {
        s
    } else {
        if !script_path.exists() {
            // Prepopulate new script with template based on selected program
            let template = get_program_template(&program, &name);
            fs::write(&script_path, &template)?;
        }
        utils::edit_file(&script_path)?;
        fs::read_to_string(&script_path)
            .with_context(|| format!("Failed to read script file: {}", script_path.display()))?
    };
    
    if script_content.trim().is_empty() {
        anyhow::bail!("Script content cannot be empty");
    }
    
    // Don't modify shebang if it already exists - the template should have the correct one
    fs::write(&script_path, script_content)
        .with_context(|| format!("Failed to write script file: {}", script_path.display()))?;
    
   
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&script_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms)?;
    }
    
    let now = chrono::Utc::now().to_rfc3339();
    
    // Update activity timestamps in JSON metadata (use full_filename for tracking)
    let mut activity_metadata = config.load_activity_metadata()?;
    let activity = activity_metadata.scripts.entry(full_filename.clone()).or_insert_with(|| Activity {
        created: now.clone(),
        updated: now.clone(),
        last_executed: None,
    });
    activity.updated = now.clone();
    if !script_existed {
        activity.created = now.clone();
    }
    config.save_activity_metadata(&activity_metadata)?;
    
    // Save metadata if we collected it (use full_filename for metadata)
    if should_edit_metadata {
        let existing_metadata = config.load_metadata(&full_filename)?;
        let program_for_metadata = program.clone();
        let metadata = ScriptMetadata {
            description,
            groups: existing_metadata.as_ref()
                .map(|m| m.groups.clone())
                .unwrap_or_default(),
            program: Some(program_for_metadata),
        };
        
        config.save_metadata(&full_filename, &metadata)?;
    }
    
    // If script was updated and is linked, recompile if needed
    if script_existed {
        use crate::link;
        use crate::execute;
        if link::is_script_linked(config, &full_filename, &program)
            .unwrap_or(false) {
            // Script is linked, recompile if it's a compiled language
            if execute::needs_compilation(&program) {
                let script_path = config.get_script_path(&full_filename);
                let _ = execute::compile_and_get_executable(&program, &script_path, &full_filename);
                // Don't fail if recompilation fails - the link will still work with old binary
            }
        }
    }
    
    println!("✓ Script '{}' saved successfully", full_filename);
    Ok(())
}

pub fn add_script_with_ai(
    config: &XConfig,
    name: Option<String>,
    _script: Option<String>,
) -> Result<()> {
    use crate::ai;
    
    config.ensure_directories()?;
    
    let name = if let Some(n) = name {
        // Validate provided name
        match validate_script_name(&n) {
            Validation::Valid => n,
            Validation::Invalid(msg) => {
                let msg_str = format!("{:?}", msg);
                eprintln!("❌ {}", msg_str);
                anyhow::bail!("Invalid script name: {}", msg_str);
            }
        }
    } else {
        Text::new("Script Name:")
            .with_help_message("This will be the command you run, e.g., 'my-script'.")
            .with_validator(|input: &str| Ok::<Validation, Box<dyn std::error::Error + Send + Sync>>(validate_script_name(input)))
            .prompt()
            .context("Failed to get script name")?
    };
    
    // First, try to find existing script (handles extensions)
    let existing_script = config.find_script(&name)?;
    let (full_filename, program, description, script_existed) = if let Some(existing_name) = existing_script {
        // Script exists - use existing filename and metadata
        let existing_metadata = config.load_metadata(&existing_name)?;
        let program = existing_metadata.as_ref()
            .and_then(|m| m.program.clone())
            .or_else(|| config.load_default_program().ok().flatten())
            .unwrap_or_else(|| "bash".to_string());
        let description = existing_metadata.as_ref()
            .and_then(|m| m.description.clone());
        (existing_name, program, description, true)
    } else {
        // Script doesn't exist - need to get program type first to determine filename
        let mut program = config.load_default_program()
            .context("Failed to load default program")?
            .unwrap_or_else(|| "bash".to_string());
        let mut description = None;
        
        // Prompt for description (optional) - Step 2
        let description_input = Text::new("Enter a short description for your script")
            .with_help_message("This is an optional, concise description, e.g., 'Downloads page and prints summary'.")
            .with_default(description.as_deref().unwrap_or("'n/a'"))
            .prompt_skippable()
            .context("Failed to get description")?;
        
        description = description_input.filter(|s| !s.trim().is_empty());
        
        // Prompt for program selection - Step 3
        let mut program_options: Vec<(&str, &str)> = vec![
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
        
        // Reorder list to put default program first
        let default_program = config.load_default_program()
            .context("Failed to load default program")?
            .unwrap_or_else(|| "bash".to_string());
        
        if let Some(default_idx) = program_options.iter()
            .position(|(val, _)| *val == default_program.as_str()) {
            let default_option = program_options.remove(default_idx);
            program_options.insert(0, default_option);
        }
        
        let display_options: Vec<String> = program_options.iter()
            .map(|(_, display)| format!("{}", display))
            .collect();
        
        let selected_display = Select::new("Which language do you want to use for your script?", 
            display_options.clone())
            .with_help_message("Select the programming language for your script")
            .with_starting_cursor(
                program_options.iter()
                    .position(|(val, _)| *val == format!("(default) {}", program.as_str()))
                    .unwrap_or(0)
            )
            .prompt()
            .context("Failed to get program selection")?;
        
        // Find the index of the selected display option and get the corresponding program value
        let selected_index = display_options.iter()
            .position(|opt| opt == &selected_display)
            .context("Failed to find selected program")?;
        program = program_options[selected_index].0.to_string();
        
        // Determine the full filename with extension
        let full_filename = get_filename_with_extension(&name, &program);
        (full_filename, program, description, false)
    };
    
    let (full_filename, name) =
        prompt_rename_if_existing(config, script_existed, full_filename, name, &program)?;
    
    let script_path = config.get_script_path(&full_filename);
    let should_edit_metadata = !script_existed;
    
    // Step 4: Open editor for AI instructions
    println!("> Preferred Editor (Instructions)");
    let instructions_template = format!(
        "# Enter instructions for generating your {} script\n# Example: Create a script that lists all files in the current directory\n\n",
        program
    );
    let instructions = utils::edit_script(&instructions_template)
        .context("Failed to get AI instructions")?;
    
    let instructions_trimmed = instructions.trim();
    if instructions_trimmed.is_empty() || instructions_trimmed.starts_with('#') && instructions_trimmed.lines().all(|l| l.trim().is_empty() || l.trim().starts_with('#')) {
        anyhow::bail!("Instructions cannot be empty");
    }
    
    // Step 5: Generate script using AI
    println!("Generating script...");
    let ai_prompt = format!(
        "Generate a {} script that: {}\n\nProvide only the script code, no explanations or markdown formatting.",
        program,
        instructions_trimmed
    );
    
    let generated_script = ai::call_llm(&ai_prompt)
        .context("Failed to generate script with AI")?;
    
    if generated_script.trim().is_empty() {
        anyhow::bail!("AI generated empty script");
    }
    
    // Ensure the script has a proper shebang if it doesn't already
    let script_with_shebang = if !generated_script.trim_start().starts_with("#!") {
        let template = get_program_template(&program, &name);
        let shebang = template.lines().next().unwrap_or("");
        format!("{}\n{}", shebang, generated_script)
    } else {
        generated_script
    };
    
    // Write the generated script to a temp file first
    fs::write(&script_path, &script_with_shebang)
        .with_context(|| format!("Failed to write generated script: {}", script_path.display()))?;
    
    // Step 6: Open editor for the generated script
    println!("> Preferred Editor (Script)");
    utils::edit_file(&script_path)?;
    let script_content = fs::read_to_string(&script_path)
        .with_context(|| format!("Failed to read edited script: {}", script_path.display()))?;
    
    if script_content.trim().is_empty() {
        anyhow::bail!("Script content cannot be empty");
    }
    
    // Write the final script
    fs::write(&script_path, script_content)
        .with_context(|| format!("Failed to write script file: {}", script_path.display()))?;
    
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&script_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms)?;
    }
    
    let now = chrono::Utc::now().to_rfc3339();
    
    // Update activity timestamps in JSON metadata
    let mut activity_metadata = config.load_activity_metadata()?;
    let activity = activity_metadata.scripts.entry(full_filename.clone()).or_insert_with(|| Activity {
        created: now.clone(),
        updated: now.clone(),
        last_executed: None,
    });
    activity.updated = now.clone();
    if !script_existed {
        activity.created = now.clone();
    }
    config.save_activity_metadata(&activity_metadata)?;
    
    // Save metadata if we collected it
    if should_edit_metadata {
        let existing_metadata = config.load_metadata(&full_filename)?;
        let program_for_metadata = program.clone();
        let metadata = ScriptMetadata {
            description,
            groups: existing_metadata.as_ref()
                .map(|m| m.groups.clone())
                .unwrap_or_default(),
            program: Some(program_for_metadata),
        };
        
        config.save_metadata(&full_filename, &metadata)?;
    }
    
    println!("✓ Script '{}' saved successfully", full_filename);
    Ok(())
}

