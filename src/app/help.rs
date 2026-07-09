//! Auto-generated `--help` output for an [`App`] / command path.

use std::fmt::Write;

use anyhow::Result;

use crate::app::spec::{App, ArgDef, Command, OptionDef, OptionGroupDef, ValueKind};

/// Render the help text for the command at `path` (each element a subcommand
/// name, root if empty). Returns the formatted string, ready to print.
pub fn render(app: &App, path: &[String]) -> Result<String> {
    let cmd = resolve(app, path)?;
    Ok(render_command(app, path, cmd))
}

fn resolve<'a>(app: &'a App, path: &[String]) -> Result<&'a Command> {
    let mut cur = &app.root;
    for seg in path {
        let next = cur
            .subcommands
            .get(seg)
            .ok_or_else(|| anyhow::anyhow!("unknown subcommand `{}`", seg))?;
        cur = next;
    }
    Ok(cur)
}

fn render_command(app: &App, path: &[String], cmd: &Command) -> String {
    let mut out = String::new();
    let invocation = std::iter::once(app.name.as_str())
        .chain(path.iter().map(|s| s.as_str()))
        .collect::<Vec<_>>()
        .join(" ");

    if let Some(desc) = cmd.description.as_ref().or(app.description.as_ref()) {
        let _ = writeln!(out, "{}\n", desc);
    }

    let _ = writeln!(out, "Usage: {}{}{}{}",
        invocation,
        if cmd.options.is_empty() { "" } else { " [OPTIONS]" },
        if cmd.subcommands.is_empty() { "" } else { " <COMMAND>" },
        format_args_inline(cmd),
    );

    if !cmd.subcommands.is_empty() {
        let _ = writeln!(out, "\nCommands:");
        let width = cmd
            .subcommands
            .keys()
            .map(|k| k.len())
            .max()
            .unwrap_or(0);
        for (name, sub) in &cmd.subcommands {
            let desc = sub.description.as_deref().unwrap_or("");
            let _ = writeln!(out, "  {:<w$}  {}", name, desc, w = width);
        }
    }

    if !cmd.options.is_empty() || !cmd.option_groups.is_empty() {
        let _ = writeln!(out, "\nOptions:");
        for group in &cmd.option_groups {
            let _ = writeln!(out, "  {}", format_option_group(cmd, group));
        }
        let grouped: std::collections::BTreeSet<&str> = cmd
            .option_groups
            .iter()
            .flat_map(|g| g.members.iter().map(|s| s.as_str()))
            .collect();
        for opt in &cmd.options {
            if grouped.contains(opt.canonical_name().as_str()) {
                continue;
            }
            let _ = writeln!(out, "  {}", format_option(opt));
        }
    }

    if !cmd.arguments.is_empty() {
        let _ = writeln!(out, "\nArguments:");
        for arg in &cmd.arguments {
            let _ = writeln!(out, "  {}", format_arg(arg));
        }
    }

    if let Some(version) = app.version.as_ref() {
        let _ = writeln!(out, "\nVersion: {}", version);
    }

    out
}

fn format_args_inline(cmd: &Command) -> String {
    let mut out = String::new();
    for a in &cmd.arguments {
        out.push(' ');
        out.push_str(&format_arg_synopsis(a));
    }
    out
}

fn format_arg_synopsis(arg: &ArgDef) -> String {
    let core = if let Some(choices) = &arg.choices {
        if arg.name.is_empty() {
            format!("{{{}}}", choices.join("|"))
        } else {
            format!("<{}={{{}}}>", arg.name, choices.join("|"))
        }
    } else {
        format!("<{}>", arg.name)
    };
    let with_repeat = if arg.repeats {
        format!("{}...", core)
    } else {
        core
    };
    if arg.required {
        with_repeat
    } else {
        format!("[{}]", with_repeat)
    }
}

fn format_option_group(cmd: &Command, group: &OptionGroupDef) -> String {
    let mut parts = Vec::new();
    for member in &group.members {
        if let Some(opt) = cmd.options.iter().find(|o| o.canonical_name() == *member) {
            parts.push(format_option_synopsis(opt));
        } else {
            parts.push(format!("--{}", member));
        }
    }
    let inner = parts.join(" | ");
    let wrapped = if group.required {
        format!("({})", inner)
    } else {
        format!("[{}]", inner)
    };
    if group.required {
        format!("{}  (required; pick one)", wrapped)
    } else {
        format!("{}  (pick one)", wrapped)
    }
}

fn format_option_synopsis(opt: &OptionDef) -> String {
    let mut name = String::new();
    if let Some(s) = opt.short {
        name.push('-');
        name.push(s);
    }
    if let (Some(_), Some(l)) = (opt.short, &opt.long) {
        name.push_str(" | --");
        name.push_str(l);
    } else if let Some(l) = &opt.long {
        name.push_str("--");
        name.push_str(l);
    }
    let value_part = match &opt.takes_value {
        ValueKind::None => String::new(),
        ValueKind::Required(p) => {
            if let Some(choices) = &opt.choices {
                format!("={{{}}}", choices.join("|"))
            } else {
                format!(" <{}>", p)
            }
        }
        ValueKind::Optional(p) => {
            if let Some(choices) = &opt.choices {
                format!("={{{}}}", choices.join("|"))
            } else {
                format!(" <{}>", p)
            }
        }
    };
    let mut out = format!("{}{}", name, value_part);
    if opt.repeats || opt.value_repeats {
        out.push_str(" ...");
    }
    out
}

fn format_option(opt: &OptionDef) -> String {
    let mut name = String::new();
    if let Some(s) = opt.short {
        name.push('-');
        name.push(s);
    }
    if let (Some(_), Some(l)) = (opt.short, &opt.long) {
        name.push_str(", --");
        name.push_str(l);
    } else if let Some(l) = &opt.long {
        name.push_str("--");
        name.push_str(l);
    }
    let value_part = match &opt.takes_value {
        ValueKind::None => String::new(),
        ValueKind::Required(p) => {
            if let Some(choices) = &opt.choices {
                format!(" {{{}}}", choices.join("|"))
            } else {
                format!(" <{}>", p)
            }
        }
        ValueKind::Optional(p) => {
            let inside = if let Some(choices) = &opt.choices {
                format!("{{{}}}", choices.join("|"))
            } else {
                format!("<{}>", p)
            };
            format!(" [{}]", inside)
        }
    };
    let mut suffix = String::new();
    if opt.repeats || opt.value_repeats {
        suffix.push_str(" ...");
    }
    if let Some(default) = &opt.default {
        suffix.push_str(&format!(" (default: {})", default));
    }
    if !opt.requires.is_empty() {
        suffix.push_str(&format!(" (requires: --{})", opt.requires.join(", --")));
    }
    if let Some(desc) = &opt.description {
        suffix.push_str(&format!("  {}", desc));
    }
    format!("{}{}{}", name, value_part, suffix)
}

fn format_arg(arg: &ArgDef) -> String {
    let synopsis = format_arg_synopsis(arg);
    if let Some(d) = &arg.default {
        format!("{}  (default: {})", synopsis, d)
    } else {
        synopsis
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::loader;
    use std::path::Path;

    fn render_yaml(yaml: &str, path: &[&str]) -> String {
        let app = loader::parse(yaml, Path::new("test.x.yml")).unwrap();
        let path: Vec<String> = path.iter().map(|s| s.to_string()).collect();
        render(&app, &path).unwrap()
    }

    #[test]
    fn root_help_contains_name_usage_and_version() {
        let text = render_yaml(
            r#"
name: mytool
version: 1.2.3
description: does things
options:
  - "[-v | --verbose]"
arguments:
  - "[<file>]"
$: echo
.build:
  description: compile
  $: echo
"#,
            &[],
        );
        assert!(text.contains("does things"));
        assert!(text.contains("Usage: mytool"));
        assert!(text.contains("[OPTIONS]"));
        assert!(text.contains("<COMMAND>"));
        assert!(text.contains("Commands:"));
        assert!(text.contains("build"));
        assert!(text.contains("compile"));
        assert!(text.contains("Options:"));
        assert!(text.contains("-v, --verbose"));
        assert!(text.contains("Arguments:"));
        assert!(text.contains("[<file>]"));
        assert!(text.contains("Version: 1.2.3"));
    }

    #[test]
    fn subcommand_help_lists_options_and_arguments() {
        let text = render_yaml(
            r#"
name: app
.demo:
  description: demo command
  options:
    - "[--mode={fast|safe}]"
  arguments:
    - "<name>"
  $: echo
"#,
            &["demo"],
        );
        assert!(text.contains("Usage: app demo"));
        assert!(text.contains("demo command"));
        assert!(text.contains("--mode {fast|safe}"));
        assert!(text.contains("<name>"));
        assert!(!text.contains("Commands:"));
    }

    #[test]
    fn option_help_shows_default_and_requires() {
        let text = render_yaml(
            r#"
name: app
options:
  - "[--src=<path> [--dst=<path>]]"
  - "[-c | --count <n='1'>]"
$: echo
"#,
            &[],
        );
        assert!(text.contains("(default: 1)"));
        assert!(text.contains("(requires: --src)"));
    }

    #[test]
    fn unknown_subcommand_errors() {
        let app = loader::parse("name: app\n", Path::new("test.x.yml")).unwrap();
        let err = render(&app, &["nope".into()]).unwrap_err();
        assert!(err.to_string().contains("unknown subcommand"));
    }

    #[test]
    fn mutex_option_group_renders_combined() {
        let text = render_yaml(
            r#"
name: app
options:
  - "(-l | --long | -s | --short)"
$: echo
"#,
            &[],
        );
        assert!(text.contains("(-l | --long | -s | --short)"));
        assert!(text.contains("required; pick one"));
    }

    #[test]
    fn positional_choice_shows_name_in_synopsis() {
        let text = render_yaml(
            r#"
name: app
arguments:
  - "<mode={read|write}>"
$: echo
"#,
            &[],
        );
        assert!(text.contains("<mode={read|write}>"));
    }
}
