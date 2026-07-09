//! Parse the user's argv against a resolved [`App`] / [`Command`].
//!
//! Supports the four standard option forms - `--long val`, `--long=val`,
//! `-s val`, `-sval` - descends through nested subcommands, applies defaults,
//! enforces `requires:` chains, and detects `-h` / `--help`.

use std::collections::BTreeMap;

use anyhow::{anyhow, bail, Result};

use crate::app::spec::{App, Command, OptionDef, ValueKind};

/// Result of parsing user argv.
#[derive(Debug, Clone)]
pub struct Parsed {
    pub command_path: Vec<String>,
    pub options: BTreeMap<String, Vec<String>>,
    pub arguments: BTreeMap<String, Vec<String>>,
    pub help: bool,
    /// When set, dispatch remaining argv to this x file instead of running a handler.
    pub alias_redirect: Option<(std::path::PathBuf, Vec<String>)>,
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
                alias_redirect: None,
            });
        }
        if let Some(sub) = cmd.subcommands.get(head) {
            path.push(head.clone());
            if let Some(target) = &sub.alias {
                let remaining = argv[idx + 1..].to_vec();
                return Ok(Parsed {
                    command_path: path,
                    options: BTreeMap::new(),
                    arguments: BTreeMap::new(),
                    help: false,
                    alias_redirect: Some((target.clone(), remaining)),
                });
            }
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
            alias_redirect: None,
        });
    }

    let mut options = options;
    apply_option_defaults(cmd, &mut options);
    enforce_required_options(cmd, &options)?;
    enforce_option_groups(cmd, &options)?;
    enforce_requires(cmd, &options)?;

    let arguments = bind_arguments(cmd, positionals)?;

    Ok(Parsed {
        command_path: path,
        options,
        arguments,
        help: false,
        alias_redirect: None,
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
            push_value(&mut options, &canonical, value, accumulates_values(opt));
            consume_value_repeats(argv, &mut i, opt, &mut options, &canonical)?;
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
            push_value(&mut options, &canonical, value, accumulates_values(opt));
            consume_value_repeats(argv, &mut i, opt, &mut options, &canonical)?;
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

fn looks_like_option_token(tok: &str) -> bool {
    tok.starts_with('-')
}

fn accumulates_values(opt: &OptionDef) -> bool {
    opt.repeats || opt.value_repeats
}

fn consume_value_repeats(
    argv: &[String],
    i: &mut usize,
    opt: &OptionDef,
    options: &mut BTreeMap<String, Vec<String>>,
    canonical: &str,
) -> Result<()> {
    if !opt.value_repeats {
        return Ok(());
    }
    while *i + 1 < argv.len() {
        let next = &argv[*i + 1];
        if next == "--" || looks_like_option_token(next) {
            break;
        }
        *i += 1;
        let value = argv[*i].clone();
        validate_choice(opt, &value)?;
        push_value(options, canonical, value, true);
    }
    Ok(())
}

fn push_value(
    options: &mut BTreeMap<String, Vec<String>>,
    name: &str,
    value: String,
    repeats: bool,
) {
    if repeats {
        options.entry(name.to_string()).or_default().push(value);
    } else {
        options.insert(name.to_string(), vec![value]);
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

fn enforce_option_groups(cmd: &Command, options: &BTreeMap<String, Vec<String>>) -> Result<()> {
    for group in &cmd.option_groups {
        let present: Vec<&str> = group
            .members
            .iter()
            .filter(|m| options.contains_key(m.as_str()))
            .map(|m| m.as_str())
            .collect();

        if group.required {
            if present.is_empty() {
                let names: Vec<String> = group
                    .members
                    .iter()
                    .map(|m| format!("`--{}`", m))
                    .collect();
                bail!(
                    "required mutually exclusive options: pick one of {}",
                    names.join(", ")
                );
            }
            if present.len() > 1 {
                let names: Vec<String> = present.iter().map(|m| format!("`--{}`", m)).collect();
                bail!(
                    "mutually exclusive options: pick one of {}; got {}",
                    group.members.iter().map(|m| format!("`--{}`", m)).collect::<Vec<_>>().join(", "),
                    names.join(" and ")
                );
            }
        } else if present.len() > 1 {
            let names: Vec<String> = present.iter().map(|m| format!("`--{}`", m)).collect();
            bail!(
                "mutually exclusive options: pick one of {}; got {}",
                group.members.iter().map(|m| format!("`--{}`", m)).collect::<Vec<_>>().join(", "),
                names.join(" and ")
            );
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::loader;

    fn app(yaml: &str) -> App {
        loader::parse(yaml, std::path::Path::new("test.x.yml")).unwrap()
    }

    fn argv(args: &[&str]) -> Vec<String> {
        args.iter().map(|s| s.to_string()).collect()
    }

    fn parse_argv(yaml: &str, args: &[&str]) -> Parsed {
        parse(&app(yaml), &argv(args)).unwrap()
    }

    fn parse_err(yaml: &str, args: &[&str]) -> String {
        parse(&app(yaml), &argv(args))
            .unwrap_err()
            .to_string()
    }

    const FULL_SPEC: &str = r#"
name: cli
options:
  - "[-v | --verbose]"
  - "[-o | --out <path>]"
  - "[-c | --count <n='1'>]"
  - "[-D | --define <kv> ...]"
  - "[--kind={alpha|beta|gamma}]"
  - "[--label=<text='demo'>]"
  - "[--src=<path> [--dst=<path>]]"
  - "--commit"
arguments:
  - "<one> [<two>] [<three='3'>] [<rest>...]"
$: echo root
.pick:
  arguments:
    - "(north|south|east|west)"
  $: echo pick
.group:
  .alpha:
    options:
      - "[-V | --verbose]"
    arguments:
      - "<msg>"
    $: echo alpha
"#;

    #[test]
    fn walks_nested_subcommands() {
        let p = parse_argv(FULL_SPEC, &["group", "alpha", "hi"]);
        assert_eq!(p.command_path, vec!["group", "alpha"]);
        assert_eq!(p.arguments.get("msg").map(|v| v[0].as_str()), Some("hi"));
    }

    #[test]
    fn help_at_root_before_subcommand() {
        let p = parse_argv(FULL_SPEC, &["--help"]);
        assert!(p.help);
        assert!(p.command_path.is_empty());
    }

    #[test]
    fn help_after_entering_subcommand() {
        let p = parse_argv(FULL_SPEC, &["group", "-h"]);
        assert!(p.help);
        assert_eq!(p.command_path, vec!["group"]);
    }

    #[test]
    fn long_bool_flag() {
        let p = parse_argv(FULL_SPEC, &["one", "--commit", "--verbose"]);
        assert_eq!(p.options.get("verbose").map(|v| v[0].as_str()), Some("true"));
    }

    #[test]
    fn long_flag_with_separate_value() {
        let p = parse_argv(FULL_SPEC, &["one", "--commit", "--out", "out.txt"]);
        assert_eq!(p.options.get("out").map(|v| v[0].as_str()), Some("out.txt"));
    }

    #[test]
    fn long_flag_eq_form() {
        let p = parse_argv(FULL_SPEC, &["one", "--commit", "--label=custom"]);
        assert_eq!(p.options.get("label").map(|v| v[0].as_str()), Some("custom"));
    }

    #[test]
    fn short_bool_flag() {
        let p = parse_argv(FULL_SPEC, &["one", "--commit", "-v"]);
        assert_eq!(p.options.get("verbose").map(|v| v[0].as_str()), Some("true"));
    }

    #[test]
    fn short_flag_with_separate_value() {
        let p = parse_argv(FULL_SPEC, &["one", "--commit", "-o", "a.txt"]);
        assert_eq!(p.options.get("out").map(|v| v[0].as_str()), Some("a.txt"));
    }

    #[test]
    fn short_flag_attached_value() {
        let p = parse_argv(FULL_SPEC, &["one", "--commit", "-oout.txt"]);
        assert_eq!(p.options.get("out").map(|v| v[0].as_str()), Some("out.txt"));
    }

    #[test]
    fn option_default_applied_when_omitted() {
        let p = parse_argv(FULL_SPEC, &["one", "--commit"]);
        assert_eq!(p.options.get("count").map(|v| v[0].as_str()), Some("1"));
        assert_eq!(p.options.get("label").map(|v| v[0].as_str()), Some("demo"));
    }

    #[test]
    fn repeating_option_accumulates() {
        let p = parse_argv(FULL_SPEC, &["one", "--commit", "-D", "a=1", "-D", "b=2"]);
        assert_eq!(
            p.options.get("define").map(|v| v.as_slice()),
            Some(&["a=1".to_string(), "b=2".to_string()][..])
        );
    }

    #[test]
    fn repeating_option_value_form() {
        let p = parse_argv(FULL_SPEC, &["one", "--commit", "-D", "a=1", "b=2"]);
        assert_eq!(
            p.options.get("define").map(|v| v.as_slice()),
            Some(&["a=1".to_string(), "b=2".to_string()][..])
        );
    }

    #[test]
    fn required_bare_option_must_be_present() {
        let err = parse_err(FULL_SPEC, &["one"]);
        assert!(err.contains("required option `--commit`"));
    }

    #[test]
    fn requires_chain_satisfied() {
        let p = parse_argv(FULL_SPEC, &["one", "--commit", "--src", "a", "--dst", "b"]);
        assert_eq!(p.options.get("src").map(|v| v[0].as_str()), Some("a"));
        assert_eq!(p.options.get("dst").map(|v| v[0].as_str()), Some("b"));
    }

    #[test]
    fn requires_chain_missing_dependency() {
        let err = parse_err(FULL_SPEC, &["one", "--commit", "--dst", "b"]);
        assert!(err.contains("requires `--src`"));
    }

    #[test]
    fn invalid_option_choice() {
        let err = parse_err(FULL_SPEC, &["one", "--commit", "--kind", "delta"]);
        assert!(err.contains("must be one of"));
    }

    #[test]
    fn positional_required_and_optional_default() {
        let p = parse_argv(FULL_SPEC, &["first", "second", "--commit"]);
        assert_eq!(p.arguments.get("one").map(|v| v[0].as_str()), Some("first"));
        assert_eq!(p.arguments.get("two").map(|v| v[0].as_str()), Some("second"));
        assert_eq!(p.arguments.get("three").map(|v| v[0].as_str()), Some("3"));
    }

    #[test]
    fn repeating_positional_greedy() {
        let p = parse_argv(FULL_SPEC, &["a", "b", "c", "d", "e", "--commit"]);
        assert_eq!(p.arguments.get("one").map(|v| v[0].as_str()), Some("a"));
        assert_eq!(p.arguments.get("two").map(|v| v[0].as_str()), Some("b"));
        assert_eq!(p.arguments.get("three").map(|v| v[0].as_str()), Some("c"));
        assert_eq!(
            p.arguments.get("rest").map(|v| v.as_slice()),
            Some(&["d".to_string(), "e".to_string()][..])
        );
    }

    #[test]
    fn required_choice_positional() {
        let p = parse_argv(FULL_SPEC, &["pick", "north"]);
        assert_eq!(p.command_path, vec!["pick"]);
        assert_eq!(p.arguments.get("choice").map(|v| v[0].as_str()), Some("north"));
    }

    #[test]
    fn invalid_required_choice() {
        let err = parse_err(FULL_SPEC, &["pick", "up"]);
        assert!(err.contains("must be one of"));
    }

    #[test]
    fn double_dash_stops_option_parsing() {
        let p = parse_argv(
            r#"
name: t
arguments:
  - "<one> [<two>]"
$: echo
"#,
            &["--", "--not-an-option", "tail"],
        );
        assert!(p.options.is_empty());
        assert_eq!(
            p.arguments.get("one").map(|v| v[0].as_str()),
            Some("--not-an-option")
        );
        assert_eq!(p.arguments.get("two").map(|v| v[0].as_str()), Some("tail"));
    }

    #[test]
    fn unknown_long_option_errors() {
        let err = parse_err(FULL_SPEC, &["one", "--commit", "--nope"]);
        assert!(err.contains("unknown option"));
    }

    #[test]
    fn bool_flag_rejects_inline_value() {
        let err = parse_err(FULL_SPEC, &["one", "--commit", "--verbose=yes"]);
        assert!(err.contains("does not take a value"));
    }

    #[test]
    fn extra_positional_errors() {
        let err = parse_err(
            r#"
name: t
arguments:
  - "<one>"
$: echo
"#,
            &["a", "b"],
        );
        assert!(err.contains("unexpected extra arguments"));
    }

    #[test]
    fn missing_required_positional_errors() {
        let err = parse_err(
            r#"
name: t
arguments:
  - "<file>"
$: echo
"#,
            &[],
        );
        assert!(err.contains("missing required argument"));
    }

    #[test]
    fn required_mutex_option_group_exactly_one() {
        let yaml = r#"
name: t
options:
  - "(--long | --short)"
$: echo
"#;
        let p = parse_argv(yaml, &["--long"]);
        assert_eq!(p.options.get("long").map(|v| v[0].as_str()), Some("true"));
        let p = parse_argv(yaml, &["--short"]);
        assert_eq!(p.options.get("short").map(|v| v[0].as_str()), Some("true"));
    }

    #[test]
    fn required_mutex_missing_errors() {
        let err = parse_err(
            r#"
name: t
options:
  - "(--long | --short)"
$: echo
"#,
            &[],
        );
        assert!(err.contains("mutually exclusive"));
    }

    #[test]
    fn required_mutex_both_present_errors() {
        let err = parse_err(
            r#"
name: t
options:
  - "(--long | --short)"
$: echo
"#,
            &["--long", "--short"],
        );
        assert!(err.contains("mutually exclusive"));
    }

    #[test]
    fn flag_level_repeat_accumulates() {
        let p = parse_argv(
            r#"
name: t
options:
  - "[--tag <label>]..."
$: echo
"#,
            &["--tag", "a", "--tag", "b"],
        );
        assert_eq!(
            p.options.get("tag").map(|v| v.as_slice()),
            Some(&["a".to_string(), "b".to_string()][..])
        );
    }

    #[test]
    fn positional_choice_in_angle_brackets() {
        let p = parse_argv(
            r#"
name: t
arguments:
  - "<mode={read|write}>"
$: echo
"#,
            &["write"],
        );
        assert_eq!(p.arguments.get("mode").map(|v| v[0].as_str()), Some("write"));
    }
}
