//! Local project scripts from `./x.yml` (see README / user docs).

use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::process::{Command, Stdio};

use crate::config::XConfig;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum LocalEntry {
    Script(String),
    Map(HashMap<String, LocalEntry>),
}

/// If `x.yml` exists in the current directory and defines [name], resolve and run it and exit.
/// Returns `Ok(false)` when there is no local definition (caller should fall back to global scripts).
/// On success (script ran), this process exits and does not return.
pub fn try_run_local(config: &XConfig, name: &str, args: &[String]) -> Result<bool> {
    let cwd = std::env::current_dir().context("Could not get current directory")?;
    let yml_path = cwd.join("x.yml");
    if !yml_path.exists() {
        return Ok(false);
    }

    let content = fs::read_to_string(&yml_path)
        .with_context(|| format!("Failed to read {}", yml_path.display()))?;
    let root: HashMap<String, LocalEntry> = serde_yaml::from_str(&content)
        .with_context(|| format!("Failed to parse {}", yml_path.display()))?;

    let Some(entry) = root.get(name) else {
        return Ok(false);
    };

    let (script, invoke_args) =
        resolve_entry(entry, args).with_context(|| format!("x.yml command '{}'", name))?;

    run_inline_script(config, &script, invoke_args)?;
    unreachable!("inline script exits the process")
}

fn resolve_entry(entry: &LocalEntry, args: &[String]) -> Result<(String, Vec<String>)> {
    match entry {
        LocalEntry::Script(s) => Ok((s.clone(), args.to_vec())),
        LocalEntry::Map(m) => {
            if args.is_empty() {
                let default_entry = m.get("$").ok_or_else(|| {
                    anyhow::anyhow!(
                        "missing default script: add '$' or specify a subcommand"
                    )
                })?;
                resolve_entry(default_entry, &[])
            } else {
                let head = &args[0];
                let tail = &args[1..];
                let sub = m.get(head).ok_or_else(|| {
                    anyhow::anyhow!("unknown subcommand {:?}", head)
                })?;
                resolve_entry(sub, tail)
            }
        }
    }
}

fn run_inline_script(config: &XConfig, script: &str, args: Vec<String>) -> Result<()> {
    let program = config
        .load_default_program()
        .ok()
        .flatten()
        .unwrap_or_else(|| "bash".to_string());

    let mut cmd = Command::new(&program);
    cmd.arg("-c").arg(script).arg("x").args(args);
    cmd.stdin(Stdio::inherit());
    cmd.stdout(Stdio::inherit());
    cmd.stderr(Stdio::inherit());

    let status = cmd
        .status()
        .with_context(|| format!("Failed to run inline script with {}", program))?;

    std::process::exit(status.code().unwrap_or(1));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(yaml: &str) -> HashMap<String, LocalEntry> {
        serde_yaml::from_str(yaml).unwrap()
    }

    #[test]
    fn leaf_script_passes_args() {
        let root = parse(
            r"
build: |
  echo $1
",
        );
        let (s, a) = resolve_entry(&root["build"], &["1.0.0".into()]).unwrap();
        assert!(s.contains("echo"));
        assert_eq!(a, vec!["1.0.0"]);
    }

    #[test]
    fn nested_default_and_subcommand() {
        let root = parse(
            r"
deploy:
  $: echo root
  prod: echo prod
",
        );
        let (s, a) = resolve_entry(&root["deploy"], &[]).unwrap();
        assert_eq!(s.trim(), "echo root");
        assert!(a.is_empty());

        let (s2, a2) = resolve_entry(&root["deploy"], &["prod".into()]).unwrap();
        assert_eq!(s2.trim(), "echo prod");
        assert!(a2.is_empty());
    }

    #[test]
    fn deep_nested() {
        let root = parse(
            r"
deploy:
  test:
    unit: npx jest unit
",
        );
        let (s, a) = resolve_entry(&root["deploy"], &["test".into(), "unit".into()]).unwrap();
        assert_eq!(s.trim(), "npx jest unit");
        assert!(a.is_empty());
    }

    #[test]
    fn missing_top_level_not_checked_here() {
        let root = parse("other: echo hi\n");
        assert!(root.get("build").is_none());
    }

    #[test]
    fn missing_default_subcommand_errors() {
        let root = parse(
            r"
deploy:
  prod: echo prod
",
        );
        let err = resolve_entry(&root["deploy"], &[]).unwrap_err();
        assert!(err.to_string().contains("missing default script"));
    }

    #[test]
    fn unknown_subcommand_errors() {
        let root = parse(
            r"
deploy:
  prod: echo prod
",
        );
        let err = resolve_entry(&root["deploy"], &["staging".into()]).unwrap_err();
        assert!(err.to_string().contains("unknown subcommand"));
    }
}
