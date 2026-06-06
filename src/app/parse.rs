//! Parse the user's argv against a resolved [`App`] / [`Command`].
//!
//! Supports the four standard option forms — `--long val`, `--long=val`,
//! `-s val`, `-sval` — descends through nested subcommands, applies defaults,
//! enforces `requires:` chains, and detects `-h` / `--help`.

use std::collections::BTreeMap;

use anyhow::{anyhow, bail, Result};

use crate::app::spec::{App, Command, OptionDef, ValueKind};

/// Result of parsing user argv.
#[derive(Debug, Clone)]
pub struct Parsed {
    /// Dotted path of the matched command (root = empty vec).
    pub command_path: Vec<String>,
    /// Option name (long if present, else single-char short) → values. Boolean
    /// flags are recorded as `["true"]`.
    pub options: BTreeMap<String, Vec<String>>,
    /// Argument name → values.
    pub arguments: BTreeMap<String, Vec<String>>,
    /// True if `-h` or `--help` was seen at any level.
    pub help: bool,
}

pub fn parse(app: &App, argv: &[String]) -> Result<Parsed> {
    // Walk subcommands.
    let mut path: Vec<String> = Vec::new();
    let mut cmd: &Command = &app.root;
    let mut idx = 0usize;
    while idx < argv.len() {
        let head = &argv[idx];
        if head == "-h" || head == "--help" {
            return Ok(Parsed {
                command_path: path,
                options: BTreeMap::new(),
                arguments: BTreeMap::new(),
                help: true,
            });
        }
        if let Some(sub) = cmd.subcommands.get(head) {
            path.push(head.clone());
            cmd = sub;
            idx += 1;
        } else {
            break;
        }
    }

    // Now parse options + positionals against `cmd`.
    let rest = &argv[idx..];
    let (options, positionals, help) = parse_command_args(cmd, rest)?;
    if help {
        return Ok(Parsed {
            command_path: path,
            options: BTreeMap::new(),
            arguments: BTreeMap::new(),
            help: true,
        });
    }

    let mut options = options;
    apply_option_defaults(cmd, &mut options);
    enforce_required_options(cmd, &options)?;
    enforce_requires(cmd, &options)?;

    let arguments = bind_arguments(cmd, positionals)?;

    Ok(Parsed {
        command_path: path,
        options,
        arguments,
        help: false,
    })
}

fn parse_command_args(
    cmd: &Command,
    argv: &[String],
) -> Result<(BTreeMap<String, Vec<String>>, Vec<String>, bool)> {
    let mut options: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut positionals: Vec<String> = Vec::new();

    let mut i = 0usize;
    while i < argv.len() {
        let tok = &argv[i];
        if tok == "--" {
            positionals.extend(argv[i + 1..].iter().cloned());
            break;
        }
        if tok == "-h" || tok == "--help" {
            return Ok((options, positionals, true));
        }
        if let Some(rest) = tok.strip_prefix("--") {
            let (name, inline) = match rest.find('=') {
                Some(idx) => (&rest[..idx], Some(rest[idx + 1..].to_string())),
                None => (rest, None),
            };
            let opt = find_option_long(cmd, name)
                .ok_or_else(|| anyhow!("unknown option `--{}`", name))?;
            let canonical = canonical_name(opt);
            let value = match &opt.takes_value {
                ValueKind::None => {
                    if inline.is_some() {
                        bail!("`--{}` does not take a value", name);
                    }
                    "true".to_string()
                }
                ValueKind::Required(_) | ValueKind::Optional(_) => match inline {
                    Some(v) => v,
                    None => {
                        i += 1;
                        if i >= argv.len() {
                            bail!("`--{}` expects a value", name);
                        }
                        argv[i].clone()
                    }
                },
            };
            validate_choice(opt, &value)?;
            push_value(&mut options, canonical, value, opt.repeats);
            i += 1;
            continue;
        }
        if let Some(rest) = tok.strip_prefix('-') {
            // Could be `-s`, `-s VAL`, `-sVAL`. Single-char short option.
            let mut chars = rest.chars();
            let s = chars
                .next()
                .ok_or_else(|| anyhow!("invalid short option `{}`", tok))?;
            let inline_after: String = chars.collect();
            let opt = find_option_short(cmd, s)
                .ok_or_else(|| anyhow!("unknown option `-{}`", s))?;
            let canonical = canonical_name(opt);
            let value = match &opt.takes_value {
                ValueKind::None => {
                    if !inline_after.is_empty() {
                        bail!("`-{}` does not take a value", s);
                    }
                    "true".to_string()
                }
                ValueKind::Required(_) | ValueKind::Optional(_) => {
                    if !inline_after.is_empty() {
                        inline_after
                    } else {
                        i += 1;
                        if i >= argv.len() {
                            bail!("`-{}` expects a value", s);
                        }
                        argv[i].clone()
                    }
                }
            };
            validate_choice(opt, &value)?;
            push_value(&mut options, canonical, value, opt.repeats);
            i += 1;
            continue;
        }
        positionals.push(tok.clone());
        i += 1;
    }

    Ok((options, positionals, false))
}

fn find_option_long<'a>(cmd: &'a Command, name: &str) -> Option<&'a OptionDef> {
    cmd.options.iter().find(|o| o.long.as_deref() == Some(name))
}

fn find_option_short<'a>(cmd: &'a Command, name: char) -> Option<&'a OptionDef> {
    cmd.options.iter().find(|o| o.short == Some(name))
}

fn canonical_name(opt: &OptionDef) -> String {
    opt.canonical_name()
}

fn push_value(
    options: &mut BTreeMap<String, Vec<String>>,
    name: String,
    value: String,
    repeats: bool,
) {
    if repeats {
        options.entry(name).or_default().push(value);
    } else {
        options.insert(name, vec![value]);
    }
}

fn validate_choice(opt: &OptionDef, value: &str) -> Result<()> {
    if let Some(choices) = &opt.choices {
        if !choices.iter().any(|c| c == value) {
            bail!(
                "option `--{}` must be one of {{{}}}; got `{}`",
                opt.long.clone().unwrap_or_else(|| opt.canonical_name()),
                choices.join("|"),
                value
            );
        }
    }
    Ok(())
}

fn apply_option_defaults(cmd: &Command, options: &mut BTreeMap<String, Vec<String>>) {
    for opt in &cmd.options {
        let canonical = opt.canonical_name();
        if options.contains_key(&canonical) {
            continue;
        }
        if let Some(default) = &opt.default {
            options.insert(canonical, vec![default.clone()]);
        }
    }
}

fn enforce_required_options(
    cmd: &Command,
    options: &BTreeMap<String, Vec<String>>,
) -> Result<()> {
    for opt in &cmd.options {
        if opt.required && !options.contains_key(&opt.canonical_name()) {
            bail!("required option `--{}` was not provided", opt.canonical_name());
        }
    }
    Ok(())
}

fn enforce_requires(cmd: &Command, options: &BTreeMap<String, Vec<String>>) -> Result<()> {
    for opt in &cmd.options {
        if !options.contains_key(&opt.canonical_name()) {
            continue;
        }
        for req in &opt.requires {
            if !options.contains_key(req) {
                bail!(
                    "option `--{}` requires `--{}`",
                    opt.canonical_name(),
                    req
                );
            }
        }
    }
    Ok(())
}

fn bind_arguments(
    cmd: &Command,
    positionals: Vec<String>,
) -> Result<BTreeMap<String, Vec<String>>> {
    let mut out: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let arg_specs = &cmd.arguments;
    let mut pi = 0usize;

    for (ai, spec) in arg_specs.iter().enumerate() {
        let is_last = ai == arg_specs.len() - 1;
        if spec.repeats {
            // Greedy: take everything left, but leave room for any later
            // required args (we don't currently support those after a repeating
            // arg; per the DSL the repeating arg is last by convention).
            let take = positionals.len().saturating_sub(pi);
            let values: Vec<String> = positionals[pi..pi + take].iter().cloned().collect();
            pi += take;
            if values.is_empty() {
                if let Some(d) = &spec.default {
                    out.insert(spec.name.clone(), vec![d.clone()]);
                } else if spec.required {
                    bail!("missing required argument `<{}>`", spec.name);
                }
            } else {
                if let Some(choices) = &spec.choices {
                    for v in &values {
                        if !choices.iter().any(|c| c == v) {
                            bail!(
                                "argument `<{}>` must be one of {{{}}}; got `{}`",
                                spec.name,
                                choices.join("|"),
                                v
                            );
                        }
                    }
                }
                out.insert(spec.name.clone(), values);
            }
            continue;
        }

        if pi < positionals.len() {
            let v = positionals[pi].clone();
            pi += 1;
            if let Some(choices) = &spec.choices {
                if !choices.iter().any(|c| c == &v) {
                    bail!(
                        "argument `<{}>` must be one of {{{}}}; got `{}`",
                        spec.name,
                        choices.join("|"),
                        v
                    );
                }
            }
            out.insert(spec.name.clone(), vec![v]);
        } else if let Some(d) = &spec.default {
            out.insert(spec.name.clone(), vec![d.clone()]);
        } else if spec.required {
            bail!("missing required argument `<{}>`", spec.name);
        }
        if is_last && pi < positionals.len() {
            bail!(
                "unexpected extra arguments: {}",
                positionals[pi..].join(" ")
            );
        }
    }

    if arg_specs.is_empty() && pi < positionals.len() {
        bail!(
            "unexpected positional arguments: {}",
            positionals[pi..].join(" ")
        );
    }

    Ok(out)
}
