//! AI shell command generator plugin.
//!
//! Usage:
//!   `x --plugin ai-cmd-gen --config`
//!   `x --plugin ai-cmd-gen [--shell bash|zsh] <instructions…>`
//!
//! Via the shell wrapper (after `source <(x --plugin ai-cmd-gen __wrapper)`):
//!   `x --ai --config` / `x -A --config`
//!   `x --ai <instructions…>` / `x -A <instructions…>`

use anyhow::{bail, Context, Result};
use clap::Parser;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
enum ShellKind {
    Bash,
    Zsh,
}

impl ShellKind {
    fn system_prompt(self) -> &'static str {
        match self {
            ShellKind::Bash => {
                "You are an expert Bash shell user. Given an instruction, respond ONLY with the appropriate Bash command to accomplish the task. Do not include explanations, comments, or any other text."
            }
            ShellKind::Zsh => {
                "You are an expert Zsh shell user. Given an instruction, respond ONLY with the appropriate Zsh command to accomplish the task. Do not include explanations, comments, or any other text."
            }
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            ShellKind::Bash => "bash",
            ShellKind::Zsh => "zsh",
        }
    }
}

#[derive(Parser, Debug)]
#[command(
    name = "ai-cmd-gen",
    about = "Generate a shell command from natural-language instructions"
)]
struct Cli {
    /// Configure the LLM provider script (~/.x.sh/config/llm.sh)
    #[arg(long = "config")]
    config: bool,

    /// Target shell dialect for the generated command
    #[arg(long = "shell", value_enum)]
    shell: Option<ShellKind>,

    /// Natural-language instructions describing the desired command
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    instructions: Vec<String>,
}

fn llm_script_path() -> PathBuf {
    let home = dirs::home_dir().expect("Could not find home directory");
    home.join(".x.sh").join("config").join("llm.sh")
}

fn llm_template() -> &'static str {
    r#"# write a shell script that outputs your model's completion
PROMPT="$1"
echo "example completion"
"#
}

fn get_editor() -> String {
    if let Ok(e) = env::var("EDITOR") {
        return e;
    }
    if let Ok(v) = env::var("VISUAL") {
        return v;
    }
    for candidate in &["nvim", "vi", "nano"] {
        if Command::new(candidate).arg("--version").output().is_ok() {
            return candidate.to_string();
        }
    }
    "vi".to_string()
}

fn edit_file(file_path: &Path) -> Result<()> {
    let editor = get_editor();
    let status = Command::new(&editor)
        .arg(file_path)
        .status()
        .with_context(|| format!("Failed to open editor: {editor}"))?;
    if !status.success() {
        bail!("Editor exited with non-zero status");
    }
    Ok(())
}

fn configure_llm_provider() -> Result<()> {
    let llm_script_path = llm_script_path();

    if let Some(parent) = llm_script_path.parent() {
        fs::create_dir_all(parent).context("Failed to create config directory")?;
    }

    if !llm_script_path.exists() {
        fs::write(&llm_script_path, llm_template()).context("Failed to write LLM script template")?;
    }

    edit_file(&llm_script_path).context("Failed to edit LLM script")?;

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

fn detect_shell() -> ShellKind {
    let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/bash".into());
    let base = shell.rsplit('/').next().unwrap_or(&shell);
    if base.contains("zsh") {
        ShellKind::Zsh
    } else {
        ShellKind::Bash
    }
}

fn build_prompt(shell: ShellKind, instructions: &str) -> String {
    format!(
        "{}\n\nInstruction: {}",
        shell.system_prompt(),
        instructions.trim()
    )
}

fn call_llm(prompt: &str) -> Result<String> {
    let llm_script_path = llm_script_path();

    if !llm_script_path.exists() {
        bail!(
            "LLM provider is not configured.\nRun `x --ai --config` to set it up."
        );
    }

    let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());

    let output = Command::new(&shell)
        .arg(&llm_script_path)
        .arg(prompt)
        .output()
        .context("Failed to execute LLM script")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("LLM script failed: {}", stderr);
    }

    let completion = String::from_utf8(output.stdout).context("LLM script returned invalid UTF-8")?;

    Ok(completion.trim().to_string())
}

const WRAPPER_SCRIPT: &str = r#"# x shell wrapper for ai-cmd-gen (-A / --ai)
_x_ai_plugin() {
    if [[ -x "$HOME/.x.sh/plugins/ai-cmd-gen" ]]; then
        echo "$HOME/.x.sh/plugins/ai-cmd-gen"
    fi
}

x() {
    if [[ $# -ge 1 && ( "$1" == "-A" || "$1" == "--ai" ) ]]; then
        local plugin
        plugin=$(_x_ai_plugin)
        shell=$(basename $SHELL)
        if [[ -n "$plugin" ]]; then
            if [[ "$shell" == "bash" ]]; then
                shift
                if [[ $# -ge 1 && "$1" == "--config" ]]; then
                    "$plugin" --config
                    return $?
                fi
                local instructions="$*"
                if [[ -z "$instructions" ]]; then
                    read -er -p $'\e[2m'"instructions> "$'\e[0m' instructions || return $?
                fi
                local cmd
                cmd=$("$plugin" --shell bash "$instructions") || return $?
                if [[ -z "$cmd" ]]; then
                    echo "x: LLM returned empty completion" >&2
                    return 1
                fi
                read -e -i "$cmd" -r -p $'\e[2m'"x.sh> "$'\e[0m' cmd || return $?
                history -s "$cmd"
                eval "$cmd"
                return $?
            elif [[ "$shell" == "zsh" ]]; then
                shift
                if (( $# >= 1 )) && [[ "$1" == "--config" ]]; then
                    "$plugin" --config
                    return $?
                fi
                local instructions="$*"
                if [[ -z "$instructions" ]]; then
                    read -r "instructions?instructions> " || return $?
                fi
                local cmd
                cmd=$("$plugin" --shell zsh "$instructions") || return $?
                if [[ -z "$cmd" ]]; then
                    print -u2 "x: LLM returned empty completion"
                    return 1
                fi
                vared -p $'%{\e[2m%}x.sh> %{\e[0m%}' cmd || return $?
                print -s -- "$cmd"
                eval "$cmd"
                return $?
            fi
        fi
    fi
    command x "$@"
}
"#;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 && args[1] == "__wrapper" {
        print!("{WRAPPER_SCRIPT}");
        return Ok(());
    }

    let cli = Cli::parse();

    if cli.config {
        return configure_llm_provider();
    }

    let shell = cli.shell.unwrap_or_else(detect_shell);

    let instructions = cli.instructions.join(" ");
    if instructions.trim().is_empty() {
        bail!(
            "Instructions required: x --plugin ai-cmd-gen [--shell {}] <instructions…>\nOr configure the LLM with: x --ai --config",
            shell.as_str()
        );
    }

    let prompt = build_prompt(shell, &instructions);
    let completion = call_llm(&prompt)?;

    if completion.is_empty() {
        bail!("LLM returned empty completion");
    }

    print!("{completion}");
    Ok(())
}
