//! Shell tab completion for `x`.

use anyhow::{bail, Result};
use clap::CommandFactory;
use std::collections::HashSet;

use crate::app::loader;
use crate::app::spec::{Command, OptionDef, ValueKind};
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

    let before = if rest_cword <= rest.len() {
        &rest[..rest_cword]
    } else {
        rest
    };
    let after_cli = skip_leading_cli_flags(before);

    if after_cli.is_empty() {
        if cur.starts_with('-') {
            return Ok(filter_prefix(cli_flag_completions(), cur));
        }
        if let Some(prev) = before.last() {
            if cli_flag_takes_value(prev) {
                return Ok(filter_prefix(names_for_flag_value(config), cur));
            }
        }
        return Ok(filter_prefix(all_top_level_commands(config)?, cur));
    }

    let cmd_name = after_cli[0].as_str();
    let trail: Vec<&str> = after_cli[1..].iter().map(String::as_str).collect();

    let Some(cmd) = resolve_command(config, cmd_name, &trail)? else {
        return Ok(Vec::new());
    };

    if let Some(eq_completions) = complete_inline_option_value(&cmd, cur) {
        return Ok(eq_completions);
    }

    if rest_cword >= 1 {
        let prev = &rest[rest_cword - 1];
        if let Some(opt) = find_option_by_token(&cmd, prev) {
            if !matches!(opt.takes_value, ValueKind::None) {
                if let Some(choices) = &opt.choices {
                    return Ok(filter_prefix(choices.clone(), cur));
                }
                return Ok(Vec::new());
            }
        }
    }

    if cur.starts_with('-') {
        return Ok(filter_prefix(option_completions(&cmd), cur));
    }

    let mut names: Vec<String> = cmd.subcommands.keys().cloned().collect();
    names.sort();
    Ok(filter_prefix(names, cur))
}

/// Skip leading top-level `x` CLI flags (and their values) before the first command.
fn skip_leading_cli_flags(before: &[String]) -> Vec<String> {
    let mut i = 0;
    while i < before.len() {
        let w = &before[i];
        if !w.starts_with('-') {
            break;
        }
        if cli_flag_takes_value(w) {
            i += 2;
        } else {
            i += 1;
        }
    }
    before[i.min(before.len())..].to_vec()
}

fn cli_flag_takes_value(flag: &str) -> bool {
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

/// Resolve the command leaf for completion, following `alias:` redirects.
///
/// `cmd_name` is the first positional; `trail` is subsequent tokens before the
/// cursor. Subcommands are walked until a non-subcommand token (option/arg) or
/// end of trail. Alias nodes load the target app and continue against its root.
fn resolve_command(
    config: &XConfig,
    cmd_name: &str,
    trail: &[&str],
) -> Result<Option<Command>> {
    let mut walk: Vec<&str> = Vec::new();

    if let Ok(Some(path)) = XConfig::project_x_yml_path() {
        if let Ok(app) = loader::load(&path) {
            if app.root.subcommands.contains_key(cmd_name) {
                walk.push(cmd_name);
                walk.extend_from_slice(trail);
                return Ok(Some(walk_command_tree(app.root, &walk)?));
            }
        }
    }

    if let Some(path) = config.find_app(cmd_name)? {
        let app = loader::load(&path)?;
        walk.extend_from_slice(trail);
        return Ok(Some(walk_command_tree(app.root, &walk)?));
    }

    Ok(None)
}

fn walk_command_tree(root: Command, args: &[&str]) -> Result<Command> {
    let mut cmd = root;
    let mut i = 0;
    while i < args.len() {
        let head = args[i];
        let Some(sub) = cmd.subcommands.get(head).cloned() else {
            break;
        };
        if let Some(target) = sub.alias.clone() {
            let target_app = loader::load(&target)?;
            cmd = target_app.root;
            i += 1;
            continue;
        }
        cmd = sub;
        i += 1;
    }
    Ok(cmd)
}

fn option_completions(cmd: &Command) -> Vec<String> {
    let mut flags = Vec::new();
    for opt in &cmd.options {
        push_option_flags(&mut flags, opt);
    }
    flags.push("-h".into());
    flags.push("--help".into());
    flags.sort();
    flags.dedup();
    flags
}

fn push_option_flags(flags: &mut Vec<String>, opt: &OptionDef) {
    if let Some(long) = &opt.long {
        flags.push(format!("--{long}"));
    }
    if let Some(short) = opt.short {
        flags.push(format!("-{short}"));
    }
}

fn find_option_by_token<'a>(cmd: &'a Command, tok: &str) -> Option<&'a OptionDef> {
    if let Some(rest) = tok.strip_prefix("--") {
        let name = rest.split('=').next().unwrap_or(rest);
        return cmd.options.iter().find(|o| o.long.as_deref() == Some(name));
    }
    if let Some(rest) = tok.strip_prefix('-') {
        if rest.chars().count() == 1 {
            let c = rest.chars().next()?;
            return cmd.options.iter().find(|o| o.short == Some(c));
        }
    }
    None
}

/// Complete `--opt=partial` inline values when the option has choices.
fn complete_inline_option_value(cmd: &Command, cur: &str) -> Option<Vec<String>> {
    let rest = cur.strip_prefix("--")?;
    let (name, partial) = rest.split_once('=')?;
    let opt = cmd.options.iter().find(|o| o.long.as_deref() == Some(name))?;
    let choices = opt.choices.as_ref()?;
    let _ = partial;
    Some(
        choices
            .iter()
            .map(|c| format!("--{name}={c}"))
            .filter(|c| c.starts_with(cur))
            .collect(),
    )
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

    fn empty_config(tmp: &TempDir) -> XConfig {
        XConfig {
            base_dir: tmp.path().join(".x.sh"),
            scripts_dir: tmp.path().join("scripts"),
            apps_dir: tmp.path().join("apps"),
            metadata_dir: tmp.path().join("metadata"),
            activity_metadata_path: tmp.path().join("metadata.json"),
            config_path: tmp.path().join("config.json"),
        }
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

        let config = empty_config(&tmp);

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

        let config = empty_config(&tmp);

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

    #[test]
    fn completes_command_options() {
        let tmp = TempDir::new().unwrap();
        write_x_yml(
            tmp.path(),
            r"
.test:
  options: |
    [--complete]
    [--integration]
    [--test <test>]
  $: echo
",
        );

        let config = empty_config(&tmp);

        with_temp_cwd(&tmp, || {
            let opts = candidates(
                &config,
                &["x".into(), "test".into(), "--".into()],
                2,
            )
            .unwrap();
            assert!(opts.contains(&"--complete".into()));
            assert!(opts.contains(&"--integration".into()));
            assert!(opts.contains(&"--test".into()));
            assert!(opts.contains(&"--help".into()));

            let filtered = candidates(
                &config,
                &["x".into(), "test".into(), "--c".into()],
                2,
            )
            .unwrap();
            assert_eq!(filtered, vec!["--complete".to_string()]);
        });
    }

    #[test]
    fn completes_app_nested_options() {
        let tmp = TempDir::new().unwrap();
        write_app(
            tmp.path(),
            "exapp",
            r"
name: exapp
.demo:
  .opts:
    opts: |
      [-n | --dry-run]
      [--kind={alpha|beta|gamma}]
    $: echo
",
        );

        let config = empty_config(&tmp);

        with_temp_cwd(&tmp, || {
            let all = candidates(
                &config,
                &[
                    "x".into(),
                    "exapp".into(),
                    "demo".into(),
                    "opts".into(),
                    "".into(),
                ],
                4,
            )
            .unwrap();
            // Empty prefix after a leaf: no subcommands, so empty (options need `-`/`--`).
            assert!(all.is_empty());

            let opts = candidates(
                &config,
                &[
                    "x".into(),
                    "exapp".into(),
                    "demo".into(),
                    "opts".into(),
                    "-".into(),
                ],
                4,
            )
            .unwrap();
            assert!(opts.contains(&"--dry-run".into()));
            assert!(opts.contains(&"-n".into()));
            assert!(opts.contains(&"--kind".into()));

            let long_only = candidates(
                &config,
                &[
                    "x".into(),
                    "exapp".into(),
                    "demo".into(),
                    "opts".into(),
                    "--".into(),
                ],
                4,
            )
            .unwrap();
            assert!(long_only.contains(&"--dry-run".into()));
            assert!(!long_only.iter().any(|o| o == "-n"));

            let choices = candidates(
                &config,
                &[
                    "x".into(),
                    "exapp".into(),
                    "demo".into(),
                    "opts".into(),
                    "--kind".into(),
                    "".into(),
                ],
                5,
            )
            .unwrap();
            assert_eq!(choices, vec!["alpha", "beta", "gamma"]);
        });
    }

    #[test]
    fn completes_through_alias_command() {
        let tmp = TempDir::new().unwrap();
        fs::write(
            tmp.path().join("other.x.yml"),
            r"
name: other
.hello:
  $: echo hi
.world:
  $: echo
",
        )
        .unwrap();
        write_x_yml(
            tmp.path(),
            r"
.sub:
  alias: ./other.x.yml
",
        );

        let config = empty_config(&tmp);

        with_temp_cwd(&tmp, || {
            let top = candidates(
                &config,
                &["x".into(), "sub".into(), "".into()],
                2,
            )
            .unwrap();
            assert_eq!(top, vec!["hello", "world"]);

            let partial = candidates(
                &config,
                &["x".into(), "sub".into(), "he".into()],
                2,
            )
            .unwrap();
            assert_eq!(partial, vec!["hello".to_string()]);
        });
    }

    #[test]
    fn completes_options_on_aliased_target() {
        let tmp = TempDir::new().unwrap();
        fs::write(
            tmp.path().join("other.x.yml"),
            r"
name: other
.run:
  options: |
    [--verbose]
    [--out <path>]
  $: echo
",
        )
        .unwrap();
        write_x_yml(
            tmp.path(),
            r"
.sub:
  alias: ./other.x.yml
",
        );

        let config = empty_config(&tmp);

        with_temp_cwd(&tmp, || {
            let opts = candidates(
                &config,
                &["x".into(), "sub".into(), "run".into(), "--".into()],
                3,
            )
            .unwrap();
            assert!(opts.contains(&"--verbose".into()));
            assert!(opts.contains(&"--out".into()));
        });
    }

    #[test]
    fn completes_nested_alias_inside_app() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("handlers")).unwrap();
        fs::write(
            tmp.path().join("handlers/nested.x.yml"),
            r"
name: nested
.list-dir:
  $: echo
",
        )
        .unwrap();
        write_app(
            tmp.path(),
            "exapp",
            r"
name: exapp
.demo:
  $: echo
.nested:
  alias: ./handlers/nested.x.yml
",
        );

        let config = empty_config(&tmp);

        with_temp_cwd(&tmp, || {
            let nested = candidates(
                &config,
                &["x".into(), "exapp".into(), "nested".into(), "".into()],
                3,
            )
            .unwrap();
            assert_eq!(nested, vec!["list-dir".to_string()]);
        });
    }
}
