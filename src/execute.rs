use anyhow::{Context, Result};
use crate::config::XConfig;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub fn needs_compilation(program: &str) -> bool {
    matches!(program, "c" | "cpp" | "rust" | "go" | "java" | "haskell" | "scala" | "kotlin")
}

/// Get the executable path for a script (compiled binary if needed, or script path for interpreted)
pub fn get_executable_path(
    config: &XConfig,
    actual_name: &str,
    program: &str,
) -> Result<PathBuf> {
    let script_path = config.get_script_path(actual_name);
    
    if needs_compilation(program) {
        compile_and_get_executable(program, &script_path, actual_name)
    } else {
        Ok(script_path)
    }
}

pub fn compile_and_get_executable(
    program: &str,
    script_path: &PathBuf,
    actual_name: &str,
) -> Result<PathBuf> {
    let script_dir = script_path.parent()
        .context("Script path has no parent directory")?;
    
    // Create a cache directory for compiled binaries
    let cache_dir = script_dir.join(".cache");
    fs::create_dir_all(&cache_dir)
        .context("Failed to create cache directory")?;
    
    // Get base name without extension for the executable
    let base_name = actual_name
        .rsplit('.')
        .skip(1)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join(".");
    let base_name = if base_name.is_empty() { actual_name } else { &base_name };
    
    match program {
        "c" => {
            let exe_path = cache_dir.join(base_name);
            let status = Command::new("gcc")
                .arg(script_path)
                .arg("-o")
                .arg(&exe_path)
                .status()
                .context("Failed to compile C program. Is gcc installed?")?;
            
            if !status.success() {
                anyhow::bail!("C compilation failed");
            }
            Ok(exe_path)
        }
        "cpp" => {
            let exe_path = cache_dir.join(base_name);
            let status = Command::new("g++")
                .arg(script_path)
                .arg("-o")
                .arg(&exe_path)
                .status()
                .context("Failed to compile C++ program. Is g++ installed?")?;
            
            if !status.success() {
                anyhow::bail!("C++ compilation failed");
            }
            Ok(exe_path)
        }
        "rust" => {
            let exe_path = cache_dir.join(base_name);
            let status = Command::new("rustc")
                .arg(script_path)
                .arg("-o")
                .arg(&exe_path)
                .status()
                .context("Failed to compile Rust program. Is rustc installed?")?;
            
            if !status.success() {
                anyhow::bail!("Rust compilation failed");
            }
            Ok(exe_path)
        }
        "go" => {
            // Go: compile to executable
            let exe_path = cache_dir.join(base_name);
            // Change to script directory for go build to work properly
            let script_dir = script_path.parent()
                .context("Script path has no parent")?;
            
            let status = Command::new("go")
                .arg("build")
                .arg("-o")
                .arg(&exe_path)
                .arg(script_path)
                .current_dir(script_dir)
                .status()
                .context("Failed to compile Go program. Is go installed?")?;
            
            if !status.success() {
                anyhow::bail!("Go compilation failed");
            }
            Ok(exe_path)
        }
        "java" => {
            // Java needs to compile to .class, then run with java
            // The class name should match the file name (without .java)
            let class_name = base_name;
            let class_path = cache_dir.join(format!("{}.class", class_name));
            
            // Compile to cache directory
            let status = Command::new("javac")
                .arg("-d")
                .arg(&cache_dir)
                .arg(script_path)
                .status()
                .context("Failed to compile Java program. Is javac installed?")?;
            
            if !status.success() {
                anyhow::bail!("Java compilation failed");
            }
            
            // Return the class path (java will be called with -cp and class name)
            Ok(class_path)
        }
        "haskell" => {
            let exe_path = cache_dir.join(base_name);
            let status = Command::new("ghc")
                .arg("-o")
                .arg(&exe_path)
                .arg(script_path)
                .status()
                .context("Failed to compile Haskell program. Is ghc installed?")?;
            
            if !status.success() {
                anyhow::bail!("Haskell compilation failed");
            }
            Ok(exe_path)
        }
        "scala" => {
            // Scala compiles to .class files
            let class_name = base_name;
            let class_path = cache_dir.join(format!("{}.class", class_name));
            
            let status = Command::new("scalac")
                .arg("-d")
                .arg(&cache_dir)
                .arg(script_path)
                .status()
                .context("Failed to compile Scala program. Is scalac installed?")?;
            
            if !status.success() {
                anyhow::bail!("Scala compilation failed");
            }
            Ok(class_path)
        }
        "kotlin" => {
            let exe_path = cache_dir.join(format!("{}.jar", base_name));
            let status = Command::new("kotlinc")
                .arg(script_path)
                .arg("-include-runtime")
                .arg("-d")
                .arg(&exe_path)
                .status()
                .context("Failed to compile Kotlin program. Is kotlinc installed?")?;
            
            if !status.success() {
                anyhow::bail!("Kotlin compilation failed");
            }
            Ok(exe_path)
        }
        _ => {
            anyhow::bail!("Unknown compiled language: {}", program);
        }
    }
}

pub fn execute_script(config: &XConfig, name: &str, args: Vec<String>) -> Result<()> {
    // Find the actual script file (handles extensions)
    let actual_name = config.find_script(name)?
        .ok_or_else(|| {
            eprintln!("❌ Script '{}' not found. Use 'x --ls' to see available scripts.", name);
            anyhow::anyhow!("Script '{}' not found. Use 'x --ls' to see available scripts.", name)
        })?;
    
    let script_path = config.get_script_path(&actual_name);
    
    // Load metadata to get the program to use (use actual_name for metadata lookup)
    let metadata = config.load_metadata(&actual_name)?;
    let program = metadata
        .and_then(|m| m.program)
        .or_else(|| config.load_default_program().ok().flatten())
        .unwrap_or_else(|| "bash".to_string());
    
    // Update last_executed timestamp (use actual_name for activity tracking)
    let now = chrono::Utc::now().to_rfc3339();
    let mut activity_metadata = config.load_activity_metadata()?;
    let activity = activity_metadata.scripts.entry(actual_name.clone()).or_insert_with(|| crate::metadata::Activity {
        created: now.clone(),
        updated: now.clone(),
        last_executed: None,
    });
    activity.last_executed = Some(now);
    config.save_activity_metadata(&activity_metadata)?;
    
    // Handle compiled languages
    if needs_compilation(&program) {
        let exe_path = compile_and_get_executable(&program, &script_path, &actual_name)?;
        
        match program.as_str() {
            "java" => {
                // Java needs special handling: java -cp <classpath> <classname>
                let class_name = exe_path.file_stem()
                    .and_then(|s| s.to_str())
                    .context("Invalid class name")?;
                let class_dir = exe_path.parent()
                    .context("Executable has no parent")?;
                
                let mut cmd = Command::new("java");
                cmd.arg("-cp")
                    .arg(class_dir)
                    .arg(class_name)
                    .args(args);
                cmd.stdin(Stdio::inherit());
                cmd.stdout(Stdio::inherit());
                cmd.stderr(Stdio::inherit());
                
                let status = cmd.status()
                    .context("Failed to execute Java program")?;
                std::process::exit(status.code().unwrap_or(1));
            }
            "scala" => {
                // Scala needs: scala -cp <classpath> <classname>
                let class_name = exe_path.file_stem()
                    .and_then(|s| s.to_str())
                    .context("Invalid class name")?;
                let class_dir = exe_path.parent()
                    .context("Executable has no parent")?;
                
                let mut cmd = Command::new("scala");
                cmd.arg("-cp")
                    .arg(class_dir)
                    .arg(class_name)
                    .args(args);
                cmd.stdin(Stdio::inherit());
                cmd.stdout(Stdio::inherit());
                cmd.stderr(Stdio::inherit());
                
                let status = cmd.status()
                    .context("Failed to execute Scala program")?;
                std::process::exit(status.code().unwrap_or(1));
            }
            "kotlin" => {
                // Kotlin JAR can be run with java -jar
                let mut cmd = Command::new("java");
                cmd.arg("-jar")
                    .arg(&exe_path)
                    .args(args);
                cmd.stdin(Stdio::inherit());
                cmd.stdout(Stdio::inherit());
                cmd.stderr(Stdio::inherit());
                
                let status = cmd.status()
                    .context("Failed to execute Kotlin program")?;
                std::process::exit(status.code().unwrap_or(1));
            }
            _ => {
                // For C, C++, Rust, Go, Haskell: execute the compiled binary directly
                let mut cmd = Command::new(&exe_path);
                cmd.args(args);
                cmd.stdin(Stdio::inherit());
                cmd.stdout(Stdio::inherit());
                cmd.stderr(Stdio::inherit());
                
                let status = cmd.status()
                    .context(format!("Failed to execute compiled program: {}", exe_path.display()))?;
                std::process::exit(status.code().unwrap_or(1));
            }
        }
    } else {
        // Interpreted languages: execute directly
        let mut cmd = Command::new(&program);
        cmd.arg(&script_path);
        cmd.args(args);
        cmd.stdin(Stdio::inherit());
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());
        
        let status = cmd.status()
            .context(format!("Failed to execute script with program: {}", program))?;
        
        std::process::exit(status.code().unwrap_or(1));
    }
}

