//! Shell tab completion for `x`.

use anyhow::{bail, Result};
use clap::CommandFactory;
use std::collections::HashSet;

use crate::app::loader;
use crate::app::spec::Command;
use crate::config::XConfig;
use crate::Cli;

/// Handle `x __complete …` (hidden).
pub fn run_complete(args: &[String]) -> Result<()> {
    let Some(shell) = args.first() else {
        bail!("usage: x __complete <bash|zsh> [words…] <cword>");
    };

    if args.len() == 1 {
        print_completion_script(shell)?;
        return Ok(());
    }

    let cword: usize = args
        .last()
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| anyhow::anyhow!("invalid cword"))?;
    let words = &args[1..args.len() - 1];

    let config = XConfig::new()?;
    let matches = candidates(&config, words, cword)?;
    for m in matches {
        println!("{m}");
    }
    Ok(())
}

pub fn candidates(config: &XConfig, words: &[String], cword: usize) -> Result<Vec<String>> {
    let rest_start = if words.first().is_some_and(|w| w == "x") {
        1
    } else {
        0
    };
    let rest = if rest_start < words.len() {
        &words[rest_start..]
    } else {
        &[][..]
    };
    let rest_cword = cword.saturating_sub(rest_start);
    let cur = rest.get(rest_cword).map(String::as_str).unwrap_or("");

    let structure = structure_positionals(rest, rest_cword);

    if cur.starts_with('-') && structure.is_empty() {
        return Ok(filter_prefix(cli_flag_completions(), cur));
    }

    if rest_cword >= 1 {
        let prev = &rest[rest_cword - 1];
        if flag_takes_value(prev) {
            return Ok(filter_prefix(names_for_flag_value(config), cur));
        }
    }

    if structure.is_empty() {
        return Ok(filter_prefix(all_top_level_commands(config)?, cur));
    }

    let cmd_name = structure[0];
    let sub_args: Vec<&str> = structure[1..].iter().copied().collect();
    Ok(filter_prefix(
        subcommand_candidates(config, cmd_name, &sub_args)?,
        cur,
    ))
}

/// Non-flag positional tokens before the word being completed.
fn structure_positionals(rest: &[String], rest_cword: usize) -> Vec<&str> {
    let mut positionals = Vec::new();
    let mut i = 0;
    while i < rest_cword && i < rest.len() {
        let w = &rest[i];
        if w.starts_with('-') {
            if flag_takes_value(w) {
                i += 2;
            } else {
                i += 1;
            }
            continue;
        }
        positionals.push(w.as_str());
        i += 1;
    }
    positionals
}

fn flag_takes_value(flag: &str) -> bool {
    matches!(
        flag,
        "--src" | "--ln" | "-d" | "--delete" | "-i" | "--init"
    )
}

fn cli_flag_completions() -> Vec<String> {
    let cmd = Cli::command();
    let mut flags = Vec::new();
    for arg in cmd.get_arguments() {
        if let Some(long) = arg.get_long() {
            flags.push(format!("--{long}"));
        }
        if let Some(short) = arg.get_short() {
            flags.push(format!("-{short}"));
        }
    }
    flags.sort();
    flags.dedup();
    flags
}

fn names_for_flag_value(config: &XConfig) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut names = Vec::new();
    for name in config.list_app_names().unwrap_or_default() {
        if seen.insert(name.clone()) {
            names.push(name);
        }
    }
    for name in config.list_script_names().unwrap_or_default() {
        if seen.insert(name.clone()) {
            names.push(name);
        }
    }
    names
}

fn all_top_level_commands(config: &XConfig) -> Result<Vec<String>> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();

    for name in loader::list_project_commands().unwrap_or_default() {
        if seen.insert(name.clone()) {
            out.push(name);
        }
    }
    for name in config.list_app_names()? {
        if seen.insert(name.clone()) {
            out.push(name);
        }
    }
    for name in config.list_script_names()? {
        if seen.insert(name.clone()) {
            out.push(name);
        }
    }
    Ok(out)
}

fn subcommand_candidates(
    config: &XConfig,
    cmd_name: &str,
    sub_args: &[&str],
) -> Result<Vec<String>> {
    if let Ok(Some(path)) = XConfig::project_x_yml_path() {
        if let Ok(app) = loader::load(&path) {
            if app.root.subcommands.contains_key(cmd_name) {
                let mut all_args = vec![cmd_name];
                all_args.extend_from_slice(sub_args);
                return Ok(app_subcommands(&app.root, &all_args));
            }
        }
    }

    if let Some(path) = config.find_app(cmd_name)? {
        let app = loader::load(&path)?;
        return Ok(app_subcommands(&app.root, sub_args));
    }

    Ok(Vec::new())
}

fn app_subcommands(root: &Command, sub_args: &[&str]) -> Vec<String> {
    let mut cmd = root;
    for arg in sub_args {
        match cmd.subcommands.get(*arg) {
            Some(sub) => cmd = sub,
            None => return Vec::new(),
        }
    }
    let mut names: Vec<String> = cmd.subcommands.keys().cloned().collect();
    names.sort();
    names
}

fn filter_prefix(candidates: Vec<String>, prefix: &str) -> Vec<String> {
    candidates
        .into_iter()
        .filter(|c| c.starts_with(prefix))
        .collect()
}

fn print_completion_script(shell: &str) -> Result<()> {
    match shell {
        "bash" => print!("{BASH_COMPLETION}"),
        "zsh" => print!("{ZSH_COMPLETION}"),
        other => bail!("unsupported shell: {other} (expected bash or zsh)"),
    }
    Ok(())
}

const BASH_COMPLETION: &str = r#"# bash completion for x
_x() {
    local cur="${COMP_WORDS[COMP_CWORD]}"
    local candidates
    candidates=$("x" __complete bash "${COMP_WORDS[@]}" "$COMP_CWORD")
    if [[ -n "$candidates" ]]; then
        COMPREPLY=( $(compgen -W "$(echo "$candidates" | tr '\n' ' ')" -- "$cur") )
    fi
}
complete -F _x x
"#;

const ZSH_COMPLETION: &str = r#"#compdef x

_x() {
    local -a completions
    completions=("${(@f)$(command x __complete zsh "${words[@]}" $CURRENT 2>/dev/null)}")
    if (( ${#completions} )); then
        compadd -a completions
    fi
}

if (( $+functions[compdef] )); then
    compdef _x x
fi
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use std::sync::Mutex;
    use tempfile::TempDir;

    static CHDIR_LOCK: Mutex<()> = Mutex::new(());

    fn with_temp_cwd<F, R>(tmp: &TempDir, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let _guard = CHDIR_LOCK.lock().unwrap();
        let prev = std::env::current_dir().unwrap();
        std::env::set_current_dir(tmp.path()).unwrap();
        let result = f();
        std::env::set_current_dir(prev).unwrap();
        result
    }

    fn write_x_yml(dir: &Path, content: &str) {
        fs::write(dir.join("x.yml"), content).unwrap();
    }

    fn write_app(dir: &Path, name: &str, content: &str) {
        fs::write(dir.join(format!("{name}.x.yml")), content).unwrap();
    }

    #[test]
    fn top_level_resolution_order_dedupes() {
        let tmp = TempDir::new().unwrap();
        write_x_yml(tmp.path(), ".build: echo build\n");
        write_app(tmp.path(), "build", "name: build\n$: echo\n");
        write_app(tmp.path(), "other", "name: other\n$: echo\n");

        let scripts_dir = tmp.path().join("scripts");
        fs::create_dir_all(&scripts_dir).unwrap();
        fs::write(scripts_dir.join("build"), "#!/bin/bash\necho").unwrap();
        fs::write(scripts_dir.join("backup"), "#!/bin/bash\necho").unwrap();

        let config = XConfig {
            base_dir: tmp.path().join(".x.sh"),
            scripts_dir: scripts_dir.clone(),
            apps_dir: tmp.path().join("apps"),
            metadata_dir: tmp.path().join("metadata"),
            activity_metadata_path: tmp.path().join("metadata.json"),
            config_path: tmp.path().join("config.json"),
        };

        with_temp_cwd(&tmp, || {
            let all = all_top_level_commands(&config).unwrap();
            assert_eq!(all, vec!["build", "other", "backup"]);
        });
    }

    #[test]
    fn prefix_filter_matches_start() {
        let all = vec!["build".into(), "test".into(), "backup".into()];
        assert_eq!(
            filter_prefix(all, "b"),
            vec!["build".to_string(), "backup".to_string()]
        );
    }

    #[test]
    fn local_nested_subcommands() {
        let tmp = TempDir::new().unwrap();
        write_x_yml(
            tmp.path(),
            r"
.deploy:
  $: echo root
  .dev: echo dev
  .prod: echo prod
  .test:
    .unit: npx jest unit
    .integration: npx jest integration
",
        );

        let config = XConfig {
            base_dir: tmp.path().join(".x.sh"),
            scripts_dir: tmp.path().join("scripts"),
            apps_dir: tmp.path().join("apps"),
            metadata_dir: tmp.path().join("metadata"),
            activity_metadata_path: tmp.path().join("metadata.json"),
            config_path: tmp.path().join("config.json"),
        };

        with_temp_cwd(&tmp, || {
            let deploy = candidates(
                &config,
                &["x".into(), "deploy".into(), "".into()],
                2,
            )
            .unwrap();
            assert_eq!(deploy, vec!["dev", "prod", "test"]);

            let test_sub = candidates(
                &config,
                &["x".into(), "deploy".into(), "test".into(), "".into()],
                3,
            )
            .unwrap();
            assert_eq!(test_sub, vec!["integration", "unit"]);
        });
    }

    #[test]
    fn zsh_completion_script_does_not_invoke_on_load() {
        let script = ZSH_COMPLETION;
        assert!(!script.contains(r#"_x "$@""#));
        assert!(script.contains("compdef _x x"));
    }

    #[test]
    fn app_nested_subcommands() {
        let tmp = TempDir::new().unwrap();
        write_app(
            tmp.path(),
            "xpkg",
            r"
name: xpkg
.build:
  $: echo
  .docs:
    $: echo
  .bin:
    $: echo
.test:
  $: echo
",
        );

        let config = XConfig {
            base_dir: tmp.path().join(".x.sh"),
            scripts_dir: tmp.path().join("scripts"),
            apps_dir: tmp.path().join("apps"),
            metadata_dir: tmp.path().join("metadata"),
            activity_metadata_path: tmp.path().join("metadata.json"),
            config_path: tmp.path().join("config.json"),
        };

        with_temp_cwd(&tmp, || {
            let top = candidates(
                &config,
                &["x".into(), "xpkg".into(), "".into()],
                2,
            )
            .unwrap();
            assert_eq!(top, vec!["build", "test"]);

            let build = candidates(
                &config,
                &["x".into(), "xpkg".into(), "build".into(), "".into()],
                3,
            )
            .unwrap();
            assert_eq!(build, vec!["bin", "docs"]);
        });
    }
}
