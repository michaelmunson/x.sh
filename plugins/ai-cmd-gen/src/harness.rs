//! Prompt harness for creating an x.sh script or app from instructions.
//!
//! The instructions file opened in `$EDITOR` starts as:
//!
//! ```markdown
//! # <name> Instructions
//! ```

use anyhow::{bail, Context, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Clone, Copy, Debug, PartialEq, Eq, clap::ValueEnum)]
pub enum CreateKind {
    Script,
    App,
}

impl CreateKind {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateKind::Script => "script",
            CreateKind::App => "app",
        }
    }
}

pub struct CreateRequest<'a> {
    pub kind: CreateKind,
    pub name: &'a str,
    pub program: &'a str,
    pub instructions: &'a str,
    pub context: Option<&'a str>,
}

pub fn instructions_template(name: &str) -> String {
    format!("# {name} Instructions\n")
}

pub fn instructions_heading(name: &str) -> String {
    format!("# {name} Instructions")
}

/// Body of an instructions file. The heading alone is not enough.
pub fn extract_instructions(name: &str, raw: &str) -> Result<String> {
    let normalized = raw.replace("\r\n", "\n");
    let trimmed = normalized.trim();
    let heading = instructions_heading(name);
    if trimmed.is_empty() || trimmed == heading {
        bail!("Instructions cannot be empty. Add them under the heading in the instructions file.");
    }
    let body = trimmed
        .strip_prefix(&heading)
        .map(str::trim)
        .filter(|body| !body.is_empty())
        .unwrap_or(trimmed);
    if body.is_empty() {
        bail!("Instructions cannot be empty. Add them under the heading in the instructions file.");
    }
    Ok(body.to_string())
}

/// Drop a single surrounding Markdown fence so the file can be saved directly.
pub fn strip_fences(text: &str) -> String {
    let trimmed = text.trim();
    let extracted = extract_fenced(trimmed).unwrap_or(trimmed);
    let mut out = extracted.trim().to_string();
    if !out.is_empty() && !out.ends_with('\n') {
        out.push('\n');
    }
    out
}

fn extract_fenced(text: &str) -> Option<&str> {
    let start = text.find("```")?;
    let after_open = &text[start + 3..];
    let newline = after_open.find('\n')?;
    let rest = &after_open[newline + 1..];
    let end = rest.find("```")?;
    Some(rest[..end].trim())
}

pub fn build_create_prompt(req: &CreateRequest<'_>) -> String {
    let system = match req.kind {
        CreateKind::Script => script_system_prompt(req.name, req.program),
        CreateKind::App => app_system_prompt(req.name),
    };
    let mut prompt = system;
    prompt.push_str("\n\nInstructions:\n");
    prompt.push_str(req.instructions.trim());
    prompt.push('\n');
    if let Some(context) = req.context {
        let context = context.trim();
        if !context.is_empty() {
            prompt.push_str("\nCurrent file contents (revise them to satisfy the instructions; keep behavior the instructions do not mention):\n");
            prompt.push_str(context);
            if !context.ends_with('\n') {
                prompt.push('\n');
            }
        }
    }
    prompt.push_str("\nRespond with only the file contents.\n");
    prompt
}

fn script_system_prompt(name: &str, program: &str) -> String {
    format!(
        r#"You are an expert programmer writing one executable script for the x.sh script manager.
The command name is `{name}`. x runs it as `{program} <script-file> [args…]`, so arguments are the process arguments after the script path.
Write a {program} script and nothing else.
Start interpreted scripts with a `#!/usr/bin/env …` shebang for `{program}`.
Do not wrap the script in Markdown fences. Do not add commentary before or after the script."#
    )
}

fn app_system_prompt(name: &str) -> String {
    format!(
        r#"You are an expert author of x.sh apps. Write one YAML app file and nothing else.
The file is saved as `{name}.x.yml` and invoked as `x {name} <command>`.
Do not wrap the YAML in Markdown fences. Do not add commentary.

Rules:
- Set `name: {name}`.
- Also set `version` and `description`.
- Commands are dot-prefixed keys such as `.build:` and nested `.build:` / `.bin:`. Never use a `commands:` key.
- A command value is either a string (shorthand for an inline `$:` script) or a mapping.
- Mapping fields: `description` or `help` (not both), `options` or `opts` (not both), `arguments` or `args` (not both), `dir`, `env`, `alias`, `$`, and nested `.sub:` commands.
- Every leaf command needs a `$:` bash script or an `alias:`.
- Handlers are bash. Read flag values with `$(x-opt name)` and positionals with `$(x-arg name)`. Boolean flags are the string `true`.
- Quote synopsis strings that contain `[`, `]`, `{{`, or `}}`.
- Examples: `"[--name=<name>]"`, `"[-v | --verbose]"`, `"[--mode={{fast|safe}}]"`, `"<path>"`, `'[<who="world">]'`, `"<files>..."`.
- Do not use a top-level `$:` mapping. Put `$:` under each command.

Example shape (do not copy it unless the instructions ask for a greeting):
name: {name}
version: 0.0.0
description: short summary

.hello:
  help: print a greeting
  args: '[<who="world">]'
  $: |
    who=$(x-arg who)
    echo "hello, $who""#
    )
}

pub fn get_editor() -> String {
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

pub fn edit_file(file_path: &Path) -> Result<()> {
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

fn temp_instructions_path(name: &str) -> PathBuf {
    let safe: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect();
    env::temp_dir().join(format!(
        "x-ai-{safe}-{}-instructions.md",
        std::process::id()
    ))
}

/// Open `$EDITOR` on a temporary Markdown file and return the instructions body.
pub fn edit_instructions(name: &str) -> Result<String> {
    let path = temp_instructions_path(name);
    let result = (|| {
        fs::write(&path, instructions_template(name))
            .with_context(|| format!("Failed to write {}", path.display()))?;
        eprintln!("Opening editor for {name} instructions…");
        edit_file(&path)?;
        let raw = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read {}", path.display()))?;
        extract_instructions(name, &raw)
    })();
    let _ = fs::remove_file(&path);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template_is_a_name_heading() {
        assert_eq!(
            instructions_template("my-script"),
            "# my-script Instructions\n"
        );
        assert_eq!(instructions_template("my_app"), "# my_app Instructions\n");
    }

    #[test]
    fn extract_rejects_heading_only() {
        let raw = instructions_template("demo");
        let err = extract_instructions("demo", &raw).unwrap_err().to_string();
        assert!(err.contains("cannot be empty"));
        assert!(extract_instructions("demo", "   \n").is_err());
    }

    #[test]
    fn extract_returns_body_under_heading() {
        let raw = "# demo Instructions\n\nPrint today's date.\n";
        assert_eq!(
            extract_instructions("demo", raw).unwrap(),
            "Print today's date."
        );
    }

    #[test]
    fn extract_keeps_text_when_heading_was_removed() {
        let raw = "List files in the current directory.\n";
        assert_eq!(
            extract_instructions("demo", raw).unwrap(),
            "List files in the current directory."
        );
    }

    #[test]
    fn strip_fences_unwraps_bash_and_yaml() {
        let bash = "```bash\n#!/usr/bin/env bash\necho hi\n```";
        assert_eq!(strip_fences(bash), "#!/usr/bin/env bash\necho hi\n");
        let yaml = "Here you go:\n```yaml\nname: demo\n```\n";
        assert_eq!(strip_fences(yaml), "name: demo\n");
        assert_eq!(strip_fences("echo hi"), "echo hi\n");
    }

    #[test]
    fn script_prompt_names_the_command_and_program() {
        let prompt = build_create_prompt(&CreateRequest {
            kind: CreateKind::Script,
            name: "hello",
            program: "python",
            instructions: "print hello",
            context: None,
        });
        assert!(prompt.contains("hello"));
        assert!(prompt.contains("python"));
        assert!(prompt.contains("print hello"));
        assert!(prompt.contains("Respond with only the file contents."));
    }

    #[test]
    fn app_prompt_includes_v3_rules_and_context() {
        let prompt = build_create_prompt(&CreateRequest {
            kind: CreateKind::App,
            name: "demo-app",
            program: "bash",
            instructions: "add a greet command",
            context: Some("name: demo-app\n"),
        });
        assert!(prompt.contains("name: demo-app"));
        assert!(prompt.contains("Never use a `commands:` key"));
        assert!(prompt.contains("x-opt"));
        assert!(prompt.contains("x-arg"));
        assert!(prompt.contains("add a greet command"));
        assert!(prompt.contains("Current file contents"));
    }
}
