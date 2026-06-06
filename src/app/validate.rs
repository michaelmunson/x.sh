//! Structural validation of a parsed [`App`].
//!
//! Catches mistakes the loader can't (the loader is purely syntactic):
//!
//! - leaf commands that have no `$:` handler
//! - handler keys in `$:` that don't match any command path
//! - options whose `requires:` references an unknown sibling option
//! - duplicate option names within a single command
//! - illegal defaults / inconsistent option specs

use std::collections::BTreeSet;

use crate::app::spec::{App, Command};

/// One validation error. We collect a list rather than bailing on the first
/// so the user can see all problems at once.
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub path: String,
    pub message: String,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.path.is_empty() {
            write!(f, "{}", self.message)
        } else {
            write!(f, "[{}] {}", self.path, self.message)
        }
    }
}

/// Run all checks. Returns `Ok(())` only if there are no errors.
pub fn validate(app: &App) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    let mut all_paths: BTreeSet<String> = BTreeSet::new();
    collect_paths(&app.root, "", &mut all_paths);
    let mut leaf_paths: BTreeSet<String> = BTreeSet::new();
    collect_leaf_paths(&app.root, "", &mut leaf_paths);

    // Every leaf command should have a handler in $: (unless it has no body
    // because it only exists as a help wrapper). We surface a warning-style
    // error to keep things explicit; users can add `: x-usage <path>` if they
    // want a help-only leaf.
    for leaf in &leaf_paths {
        if !app.handlers.contains_key(leaf) {
            errors.push(ValidationError {
                path: leaf.clone(),
                message: format!(
                    "leaf command has no handler — add `{}: ...` under `$:`",
                    if leaf.is_empty() { "$" } else { leaf.as_str() }
                ),
            });
        }
    }

    // Every key in $: must correspond to a known command path.
    for key in app.handlers.keys() {
        if !all_paths.contains(key) {
            errors.push(ValidationError {
                path: key.clone(),
                message: format!(
                    "handler key `{}` does not match any command in `commands:`",
                    if key.is_empty() { "$" } else { key.as_str() }
                ),
            });
        }
    }

    validate_command(&app.root, "", &mut errors);

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn collect_paths(cmd: &Command, prefix: &str, out: &mut BTreeSet<String>) {
    out.insert(prefix.to_string());
    for (name, sub) in &cmd.subcommands {
        let next = if prefix.is_empty() {
            name.clone()
        } else {
            format!("{}.{}", prefix, name)
        };
        collect_paths(sub, &next, out);
    }
}

fn collect_leaf_paths(cmd: &Command, prefix: &str, out: &mut BTreeSet<String>) {
    if cmd.subcommands.is_empty() {
        out.insert(prefix.to_string());
        return;
    }
    for (name, sub) in &cmd.subcommands {
        let next = if prefix.is_empty() {
            name.clone()
        } else {
            format!("{}.{}", prefix, name)
        };
        collect_leaf_paths(sub, &next, out);
    }
}

fn validate_command(cmd: &Command, path: &str, errors: &mut Vec<ValidationError>) {
    let mut seen_long: BTreeSet<&str> = BTreeSet::new();
    let mut seen_short: BTreeSet<char> = BTreeSet::new();
    for opt in &cmd.options {
        if let Some(long) = &opt.long {
            if !seen_long.insert(long.as_str()) {
                errors.push(ValidationError {
                    path: path.to_string(),
                    message: format!("duplicate option `--{}`", long),
                });
            }
        }
        if let Some(short) = opt.short {
            if !seen_short.insert(short) {
                errors.push(ValidationError {
                    path: path.to_string(),
                    message: format!("duplicate option `-{}`", short),
                });
            }
        }
        for req in &opt.requires {
            let known = cmd.options.iter().any(|o| o.long.as_deref() == Some(req.as_str()));
            if !known {
                errors.push(ValidationError {
                    path: path.to_string(),
                    message: format!(
                        "option `{}` requires `--{}`, but `--{}` is not defined on this command",
                        opt.canonical_name(),
                        req,
                        req
                    ),
                });
            }
        }
    }

    let mut seen_arg: BTreeSet<&str> = BTreeSet::new();
    for arg in &cmd.arguments {
        if !seen_arg.insert(arg.name.as_str()) {
            errors.push(ValidationError {
                path: path.to_string(),
                message: format!("duplicate argument `<{}>`", arg.name),
            });
        }
    }

    // Recurse into subcommands.
    for (name, sub) in &cmd.subcommands {
        let next = if path.is_empty() {
            name.clone()
        } else {
            format!("{}.{}", path, name)
        };
        validate_command(sub, &next, errors);
    }
}

/// Pretty-print a list of errors for terminal output.
pub fn format_errors(errors: &[ValidationError]) -> String {
    use std::fmt::Write;
    let mut out = String::new();
    let _ = writeln!(out, "Validation failed with {} error(s):", errors.len());
    for e in errors {
        let _ = writeln!(out, "  - {}", e);
    }
    out
}
