//! Interactive `x -i --app` flow.
//!
//! 1. Resolve `<name>` and scope (local CWD vs global apps dir).
//! 2. Seed a minimal `<name>.x.yml` template if it doesn't already exist.
//! 3. Open the editor.
//! 4. Validate the saved file. On error, prompt edit / revert and loop.

use std::fs;
use std::path::Path;

use anyhow::{bail, Context, Result};
use inquire::{validator::Validation, Select, Text};

use crate::app::{loader, validate};
use crate::config::XConfig;
use crate::utils;

/// Where the new app lives.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    Local,
    Global,
}

pub fn create_app(
    config: &XConfig,
    name: Option<String>,
    scope: Option<Scope>,
) -> Result<()> {
    let name = match name {
        Some(n) => {
            validate_app_name(&n)?;
            n
        }
        None => prompt_app_name()?,
    };

    let scope = match scope {
        Some(s) => s,
        None => prompt_scope()?,
    };

    let path = match scope {
        Scope::Local => XConfig::local_app_path(&name)?,
        Scope::Global => {
            config.ensure_app_dirs()?;
            config.global_app_path(&name)
        }
    };

    let pre_existed = path.exists();
    let backup: Option<String> = if pre_existed {
        Some(fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?)
    } else {
        None
    };

    if !pre_existed {
        let template = seed_template(&name);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("failed to create parent of {}", path.display())
            })?;
        }
        fs::write(&path, template)
            .with_context(|| format!("failed to write {}", path.display()))?;
    }

    edit_validate_loop(&path, pre_existed, backup.as_deref())?;

    println!("✓ App '{}' saved to {}", name, path.display());
    Ok(())
}

fn validate_app_name(name: &str) -> Result<()> {
    if name.is_empty() {
        bail!("app name cannot be empty");
    }
    if !name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        bail!("app name can only contain letters, numbers, dashes, and underscores");
    }
    Ok(())
}

fn prompt_app_name() -> Result<String> {
    Text::new("App name:")
        .with_help_message("Will be saved as `<name>.x.yml`.")
        .with_validator(|input: &str| {
            Ok::<Validation, Box<dyn std::error::Error + Send + Sync>>(
                if input.is_empty() {
                    Validation::Invalid("name cannot be empty".into())
                } else if !input
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
                {
                    Validation::Invalid(
                        "letters, numbers, dashes, and underscores only".into(),
                    )
                } else {
                    Validation::Valid
                },
            )
        })
        .prompt()
        .context("failed to read app name")
}

fn prompt_scope() -> Result<Scope> {
    let choice = Select::new(
        "Where should this app live?",
        vec!["local (current directory)", "global (~/.x.sh/apps)"],
    )
    .prompt()
    .context("failed to read scope selection")?;
    Ok(if choice.starts_with("local") {
        Scope::Local
    } else {
        Scope::Global
    })
}

fn seed_template(name: &str) -> String {
    format!(
        r#"
# documentation: https://github.com/michaelmunson/x.sh/blob/main/README.md#app-framework
# examples:      https://github.com/michaelmunson/x.sh/blob/main/docs/examples/app/exapp.x.yml

name: {name}
version: 0.0.0
description: an x app

options: &options
  - '[--name=<name>]'

commands:
  hello:
    description: print a greeting
    arguments: '[<who="world">]'
    options: *options

$:
  hello: |
    name=$(x-opt "name")
    who=$(x-arg "who")
    echo "hello, $who : $name"
"#,
        name = name
    )
}

fn edit_validate_loop(
    path: &Path,
    pre_existed: bool,
    backup: Option<&str>,
) -> Result<()> {
    loop {
        utils::edit_file(path)
            .with_context(|| format!("failed to edit {}", path.display()))?;

        match validate_file(path) {
            Ok(()) => return Ok(()),
            Err(message) => {
                eprintln!("\n{}\n", message);
                let next = Select::new(
                    "How would you like to proceed?",
                    vec!["Edit config", "Revert"],
                )
                .prompt()
                .context("failed to read validation choice")?;
                if next == "Revert" {
                    return revert(path, pre_existed, backup);
                }
                // else: loop and re-edit.
            }
        }
    }
}

fn validate_file(path: &Path) -> std::result::Result<(), String> {
    let content = fs::read_to_string(path).map_err(|e| format!("read {}: {}", path.display(), e))?;
    let app = match loader::parse(&content, path) {
        Ok(a) => a,
        Err(e) => return Err(format!("Parse error: {:#}", e)),
    };
    if let Err(errors) = validate::validate(&app) {
        return Err(validate::format_errors(&errors));
    }
    Ok(())
}

fn revert(path: &Path, pre_existed: bool, backup: Option<&str>) -> Result<()> {
    if pre_existed {
        if let Some(prev) = backup {
            fs::write(path, prev)
                .with_context(|| format!("failed to revert {}", path.display()))?;
            println!("Reverted {} to its previous contents.", path.display());
        }
    } else {
        let _ = fs::remove_file(path);
        println!("Discarded new app file {}.", path.display());
    }
    Ok(())
}

