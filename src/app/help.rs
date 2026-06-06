//! Auto-generated `--help` output for an [`App`] / command path.

use std::fmt::Write;

use anyhow::Result;

use crate::app::spec::{App, ArgDef, Command, OptionDef, ValueKind};

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

    if !cmd.options.is_empty() {
        let _ = writeln!(out, "\nOptions:");
        for opt in &cmd.options {
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
        let core = if let Some(choices) = &a.choices {
            format!("{{{}}}", choices.join("|"))
        } else {
            format!("<{}>", a.name)
        };
        let with_repeat = if a.repeats {
            format!("{}...", core)
        } else {
            core
        };
        if a.required {
            out.push_str(&with_repeat);
        } else {
            out.push_str(&format!("[{}]", with_repeat));
        }
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
    if opt.repeats {
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
    let core = if let Some(choices) = &arg.choices {
        format!("{{{}}}", choices.join("|"))
    } else if arg.required {
        format!("<{}>", arg.name)
    } else {
        format!("[<{}>]", arg.name)
    };
    let with_repeat = if arg.repeats {
        format!("{}...", core)
    } else {
        core
    };
    if let Some(d) = &arg.default {
        format!("{}  (default: {})", with_repeat, d)
    } else {
        with_repeat
    }
}
