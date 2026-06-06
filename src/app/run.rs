//! Resolve a parsed app invocation to a handler body and exec bash.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::{Command as ProcCommand, Stdio};

use anyhow::{anyhow, Context, Result};

use crate::app::help;
use crate::app::loader;
use crate::app::parse::{self, Parsed};
use crate::app::preamble::PREAMBLE;
use crate::app::spec::App;
use crate::app::validate;
use crate::config::XConfig;

/// Locate `<name>.x.yml`, validate it, parse argv against it, then exec bash
/// with the matching handler body. Exits the process on success.
pub fn run_app(config: &XConfig, name: &str, argv: &[String]) -> Result<()> {
    let path = config
        .find_app(name)?
        .ok_or_else(|| anyhow!("app `{}` not found (looked in CWD and {})", name, config.apps_dir.display()))?;

    let app = loader::load(&path).with_context(|| format!("loading app {}", path.display()))?;

    if let Err(errors) = validate::validate(&app) {
        eprintln!("{}", validate::format_errors(&errors));
        anyhow::bail!("app `{}` failed validation; fix it with `x -i --app {}`", name, name);
    }

    let parsed = parse::parse(&app, argv)?;

    if parsed.help {
        let text = help::render(&app, &parsed.command_path)?;
        print!("{}", text);
        std::process::exit(0);
    }

    let handler_key = parsed.command_path.join(".");
    let body = match app.handlers.get(&handler_key) {
        Some(b) => b.clone(),
        None => {
            // If we matched a non-leaf command without a handler, fall through
            // to auto-printing help instead of erroring.
            let cmd = resolve_command(&app, &parsed.command_path);
            if cmd.map(|c| !c.subcommands.is_empty()).unwrap_or(false) {
                let text = help::render(&app, &parsed.command_path)?;
                print!("{}", text);
                std::process::exit(0);
            }
            return Err(anyhow!(
                "no handler defined for `{}`; add `{}: ...` under `$:`",
                if handler_key.is_empty() { "$" } else { handler_key.as_str() },
                if handler_key.is_empty() { "$" } else { handler_key.as_str() }
            ));
        }
    };

    exec_handler(&app, &path, &parsed, &body)
}

fn resolve_command<'a>(app: &'a App, path: &[String]) -> Option<&'a crate::app::spec::Command> {
    let mut cur = &app.root;
    for seg in path {
        cur = cur.subcommands.get(seg)?;
    }
    Some(cur)
}

fn exec_handler(app: &App, app_path: &Path, parsed: &Parsed, body: &str) -> Result<()> {
    let bash_cmd = format!("{}\n{}", PREAMBLE, body);

    let opts_pairs = pairs_string(&parsed.options);
    let args_pairs = pairs_string(&parsed.arguments);

    let bin = std::env::current_exe()
        .unwrap_or_else(|_| PathBuf::from("x"));

    let mut cmd = ProcCommand::new("bash");
    cmd.arg("-c").arg(&bash_cmd).arg(app.name.clone());

    cmd.env("X_BIN", &bin);
    cmd.env("X_APP", &app.name);
    cmd.env("X_APP_FILE", app_path);
    cmd.env("X_OPTS_PAIRS", &opts_pairs);
    cmd.env("X_ARGS_PAIRS", &args_pairs);

    for (k, vs) in &parsed.options {
        cmd.env(format!("X_OPT_{}", env_key(k)), vs.join("\n"));
    }
    for (k, vs) in &parsed.arguments {
        cmd.env(format!("X_ARG_{}", env_key(k)), vs.join("\n"));
    }

    cmd.stdin(Stdio::inherit());
    cmd.stdout(Stdio::inherit());
    cmd.stderr(Stdio::inherit());

    let status = cmd.status().context("failed to spawn bash for app handler")?;
    std::process::exit(status.code().unwrap_or(1));
}

fn pairs_string(map: &BTreeMap<String, Vec<String>>) -> String {
    let mut out = String::new();
    for (k, vs) in map {
        for v in vs {
            out.push_str(k);
            out.push('=');
            out.push_str(v);
            out.push('\n');
        }
    }
    out
}

fn env_key(name: &str) -> String {
    name.replace('-', "_")
}

/// Hidden `x __usage <app-file> <cmd-path>` entry point used by the bash
/// `x-usage` builtin.
pub fn print_usage(app_file: &Path, cmd_path: &str) -> Result<()> {
    let app = loader::load(app_file)
        .with_context(|| format!("loading app {}", app_file.display()))?;
    let path: Vec<String> = if cmd_path.is_empty() {
        Vec::new()
    } else {
        cmd_path.split('.').map(|s| s.to_string()).collect()
    };
    let text = help::render(&app, &path)?;
    print!("{}", text);
    Ok(())
}
