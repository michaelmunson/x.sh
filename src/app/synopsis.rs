//! Hand-rolled parser for the option/argument synopsis DSL.
//!
//! The grammar is small and closed; rather than pull in a parser-combinator
//! crate we tokenise into characters and recurse on bracketed groups.
//!
//! ## Positional arguments
//!
//! - `<name>` - required positional
//! - `[<name>]` - optional positional
//! - `[<name='val'>]` - optional positional with default
//! - `<name>...` / `[<name>...]` - repeating positional
//! - `<name={a|b|c}>` / `[<name={a|b|c}>]` - value from set
//! - `(a|b|c)` - required choice (becomes positional `choice`)
//!
//! ## Options
//!
//! All `[optional]` forms have bare `(required)` equivalents (drop the brackets).
//!
//! - `[-s | --long]` / `(-l | --long)` - bool flag (optional / required)
//! - `[-s | --long <arg>]` - flag with value
//! - `[-s | --long <arg='v'>]` - flag value with default
//! - `[-s | --long <arg> ...]` - repeating values for one flag use
//! - `[-s | --long <arg>]...` - repeating flag occurrences
//! - `[--long={a|b|c}]` / `[--long=<arg>]` / `[--long=<arg='v'>]`
//! - `[--input=<a> [--output=<b>]]` - nested bracket = dependency
//! - `(--long | --short)` - required mutually exclusive flags
//! - `(-l | --long | -s | --short)` - mutex with alias pairs
//! - `--long` - required bare flag
//!
//! [`OptionDef`]: crate::app::spec::OptionDef
//! [`ArgDef`]: crate::app::spec::ArgDef
//! [`Command`]: crate::app::spec::Command

use anyhow::{anyhow, bail, Context, Result};

use crate::app::spec::{ArgDef, OptionDef, OptionGroupDef, ValueKind};

/// One parsed entry from a synopsis string.
#[derive(Debug, Clone)]
pub enum SynopsisEntry {
    Option(OptionDef),
    /// One option plus follow-up dependent options (from nested brackets).
    /// e.g. `[--input=<a> [--output=<b>]]` -> primary=input, dependents=[output]
    Argument(ArgDef),
    /// Required choice as positional, e.g. `(render | build | clean)`.
    /// Modeled as an `ArgDef` with `choices` set and `required = true`.
    RequiredChoice(ArgDef),
    /// Mutually exclusive option group, e.g. `(--long | --short)`.
    OptionGroup(OptionGroupDef),
}

/// Parse a single synopsis fragment (one YAML list item or the value of a
/// scalar `arguments:` line). The fragment may itself contain multiple
/// space-separated tokens (e.g. `<path> [<content='empty'>]`).
pub fn parse_fragment(input: &str) -> Result<Vec<SynopsisEntry>> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }
    let tokens = tokenize_top(trimmed)
        .with_context(|| format!("failed to tokenise synopsis fragment: {:?}", input))?;
    let mut entries = Vec::new();
    for tok in tokens {
        let parsed = parse_token(&tok)
            .with_context(|| format!("failed to parse synopsis token: {:?}", tok))?;
        entries.extend(parsed);
    }
    Ok(entries)
}

/// Split an arbitrary synopsis string into top-level tokens, respecting nested
/// `[...]`, `(...)`, `{...}`, and `<...>` groups and quoted strings.
fn tokenize_top(input: &str) -> Result<Vec<String>> {
    let mut out = Vec::new();
    let mut current = String::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0usize;
    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            if !current.is_empty() {
                out.push(std::mem::take(&mut current));
            }
            i += 1;
            continue;
        }
        match c {
            '[' | '(' | '{' | '<' => {
                let close = matching_close(c);
                let end = find_matching(&chars, i, c, close)?;
                current.push_str(&chars[i..=end].iter().collect::<String>());
                i = end + 1;
            }
            '\'' | '"' => {
                let end = find_quote_end(&chars, i, c)?;
                current.push_str(&chars[i..=end].iter().collect::<String>());
                i = end + 1;
            }
            _ => {
                current.push(c);
                i += 1;
            }
        }
    }
    if !current.is_empty() {
        out.push(current);
    }
    Ok(out)
}

fn matching_close(open: char) -> char {
    match open {
        '[' => ']',
        '(' => ')',
        '{' => '}',
        '<' => '>',
        _ => open,
    }
}

fn find_matching(chars: &[char], start: usize, open: char, close: char) -> Result<usize> {
    let mut depth = 0i32;
    let mut i = start;
    while i < chars.len() {
        let c = chars[i];
        if c == '\'' || c == '"' {
            i = find_quote_end(chars, i, c)? + 1;
            continue;
        }
        if c == open {
            depth += 1;
        } else if c == close {
            depth -= 1;
            if depth == 0 {
                return Ok(i);
            }
        }
        i += 1;
    }
    bail!("unbalanced {} starting at position {}", open, start);
}

fn find_quote_end(chars: &[char], start: usize, quote: char) -> Result<usize> {
    let mut i = start + 1;
    while i < chars.len() {
        if chars[i] == '\\' && i + 1 < chars.len() {
            i += 2;
            continue;
        }
        if chars[i] == quote {
            return Ok(i);
        }
        i += 1;
    }
    bail!("unterminated quote starting at position {}", start)
}

fn parse_token(tok: &str) -> Result<Vec<SynopsisEntry>> {
    let tok = tok.trim();
    if tok.is_empty() {
        return Ok(Vec::new());
    }

    let (body, flag_repeats) = strip_repeat_suffix(tok);

    if let Some(inner) = strip_outer(body, '[', ']') {
        let mut entries = parse_optional_group(inner)?;
        if flag_repeats {
            for entry in &mut entries {
                if let SynopsisEntry::Option(o) = entry {
                    o.repeats = true;
                }
            }
        }
        return Ok(entries);
    }

    if let Some(inner) = strip_outer(body, '(', ')') {
        if is_option_paren_group(inner) {
            return parse_paren_option_group(inner);
        }
        return parse_required_choice(inner);
    }

    if body.starts_with('<') {
        let (inner, inner_repeats) = strip_repeat_suffix(body);
        let repeats = flag_repeats || inner_repeats;
        let inner = strip_outer(inner, '<', '>')
            .ok_or_else(|| anyhow!("expected `<name>` form, got {:?}", tok))?;
        let arg = parse_arg_inner(inner, true, repeats)?;
        Ok(vec![SynopsisEntry::Argument(arg)])
    } else if body.starts_with('-') {
        let entries = parse_option_chain(body, /*optional=*/ false)?;
        Ok(entries)
    } else {
        bail!("unrecognised synopsis token: {:?}", tok);
    }
}

fn strip_outer<'a>(s: &'a str, open: char, close: char) -> Option<&'a str> {
    let s = s.trim();
    let mut chars = s.chars();
    if chars.next()? != open {
        return None;
    }
    if !s.ends_with(close) {
        return None;
    }
    let bytes = s.as_bytes();
    if bytes.len() < 2 {
        return None;
    }
    Some(&s[1..s.len() - 1])
}

fn strip_repeat_suffix(s: &str) -> (&str, bool) {
    let trimmed = s.trim_end();
    if let Some(prefix) = trimmed.strip_suffix("...") {
        (prefix.trim_end(), true)
    } else {
        (trimmed, false)
    }
}

/// Parse the insides of a `[...]` optional group. May contain a positional
/// (e.g. `<name>`), an option (e.g. `-s | --long`), or an option with a nested
/// dependent option.
fn parse_optional_group(inner: &str) -> Result<Vec<SynopsisEntry>> {
    let inner = inner.trim();
    let mut tokens = tokenize_top(inner)?;
    if tokens.is_empty() {
        return Ok(Vec::new());
    }

    // A trailing standalone `...` marks repetition, e.g. `[<arg> ...]`
    // (equivalent to the spaceless `[<arg>...]`). Peel it off so the single
    // positional / option-chain detection below sees the underlying form.
    let mut trailing_repeat = false;
    if tokens.len() > 1 && tokens.last().map(|t| t == "...").unwrap_or(false) {
        trailing_repeat = true;
        tokens.pop();
    }

    // Positional inside brackets, e.g. `[<name>]` or `[<name='x'>]` or `[<name>...]`
    if tokens.iter().all(|t| {
        let (t, _) = strip_repeat_suffix(t);
        t.trim().starts_with('<')
    }) && tokens.len() == 1
    {
        let (token, repeats) = strip_repeat_suffix(&tokens[0]);
        let inside = strip_outer(token, '<', '>')
            .ok_or_else(|| anyhow!("expected `<name>` inside [...]"))?;
        let arg = parse_arg_inner(inside, false, repeats || trailing_repeat)?;
        return Ok(vec![SynopsisEntry::Argument(arg)]);
    }

    // Otherwise treat the whole inner as an option chain (with optional nesting).
    parse_option_chain(inner, /*optional=*/ true)
}

/// True when every `|`-separated segment in a paren group starts with `-`
/// (option tokens), distinguishing `(-l | --long)` from `(north | south)`.
fn is_option_paren_group(inner: &str) -> bool {
    inner
        .split('|')
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .all(|seg| seg.starts_with('-'))
}

/// Collect mutex alternatives from a tokenised option paren group.
/// Alias pairs (`-l | --long`) stay in one alternative; `--long | --short`
/// become separate alternatives.
fn collect_mutex_alternatives(tokens: &[String]) -> Vec<Vec<String>> {
    let mut alternatives: Vec<Vec<String>> = Vec::new();
    let mut i = 0usize;
    while i < tokens.len() {
        if tokens[i] == "|" {
            i += 1;
            continue;
        }
        let mut alt = vec![tokens[i].clone()];
        i += 1;
        loop {
            if i < tokens.len() && tokens[i] == "|" {
                let prev = alt.last().map(|s| s.as_str()).unwrap_or("");
                if prev.starts_with('-')
                    && prev.len() == 2
                    && i + 1 < tokens.len()
                    && tokens[i + 1].starts_with("--")
                {
                    i += 1;
                    alt.push(tokens[i].clone());
                    i += 1;
                    continue;
                }
                break;
            }
            if i >= tokens.len() || tokens[i] == "|" {
                break;
            }
            alt.push(tokens[i].clone());
            i += 1;
        }
        alternatives.push(alt);
    }
    alternatives
}

/// Parse `(-l | --long)` or `(--long | --short)` parenthesised option forms.
fn parse_paren_option_group(inner: &str) -> Result<Vec<SynopsisEntry>> {
    let tokens = tokenize_top(inner)?;
    let alternatives = collect_mutex_alternatives(&tokens);
    if alternatives.is_empty() {
        bail!("empty option group `(…)`");
    }

    if alternatives.len() == 1 {
        let opt = parse_single_option(&alternatives[0], /*optional=*/ false)?;
        return Ok(vec![SynopsisEntry::Option(opt)]);
    }

    let mut entries = Vec::new();
    let mut members = Vec::new();
    for alt_tokens in alternatives {
        let mut opt = parse_single_option(&alt_tokens, /*optional=*/ false)?;
        opt.required = false;
        let name = opt.canonical_name();
        if name.is_empty() {
            bail!("option in mutex group has no name");
        }
        members.push(name);
        entries.push(SynopsisEntry::Option(opt));
    }
    entries.push(SynopsisEntry::OptionGroup(OptionGroupDef {
        members,
        required: true,
    }));
    Ok(entries)
}

/// Parse `(a|b|c)` form as a required positional choice argument.
fn parse_required_choice(inner: &str) -> Result<Vec<SynopsisEntry>> {
    let parts: Vec<String> = inner
        .split('|')
        .map(|p| p.trim().to_string())
        .filter(|p| !p.is_empty())
        .collect();
    if parts.len() < 2 {
        bail!("required choice `(a|b|c)` must have at least two alternatives");
    }
    Ok(vec![SynopsisEntry::RequiredChoice(ArgDef {
        name: "choice".to_string(),
        required: true,
        default: None,
        repeats: false,
        choices: Some(parts),
    })])
}

/// Parse the inside of a `<...>` form: `name` or `name='default'` or
/// `name={a|b|c}` or `name={a|b|c}='default'`.
fn parse_arg_inner(inside: &str, required: bool, repeats: bool) -> Result<ArgDef> {
    let inside = inside.trim();
    let (name_part, default, choices) = split_arg_inner(inside)?;
    let name = name_part.trim().to_string();
    if name.is_empty() {
        bail!("empty argument name in `<>`");
    }
    Ok(ArgDef {
        name,
        required: required && default.is_none(),
        default,
        repeats,
        choices,
    })
}

fn split_arg_inner(inside: &str) -> Result<(String, Option<String>, Option<Vec<String>>)> {
    // Forms:
    //   name
    //   name='default'
    //   name={a|b|c}
    //   name={a|b|c}='default'
    //   {a|b|c}                    <- value-only (used for option values)
    //   {a|b|c}='default'
    let chars: Vec<char> = inside.chars().collect();
    let mut i = 0;
    let mut name = String::new();
    let mut choices: Option<Vec<String>> = None;
    let mut default: Option<String> = None;

    if chars.first() == Some(&'{') {
        let end = find_matching(&chars, 0, '{', '}')?;
        choices = Some(parse_choice_set(&inside[1..end])?);
        i = end + 1;
    } else {
        while i < chars.len() {
            let c = chars[i];
            if c == '=' || c == '{' {
                break;
            }
            name.push(c);
            i += 1;
        }
    }

    if i < chars.len() && chars[i] == '=' {
        i += 1;
        if i < chars.len() && chars[i] == '{' {
            let end = find_matching(&chars, i, '{', '}')?;
            choices = Some(parse_choice_set(&inside[i + 1..end])?);
            i = end + 1;
        } else {
            default = Some(parse_default_literal(&inside[i..])?);
            return Ok((name, default, choices));
        }
    } else if i < chars.len() && chars[i] == '{' {
        let end = find_matching(&chars, i, '{', '}')?;
        choices = Some(parse_choice_set(&inside[i + 1..end])?);
        i = end + 1;
    }

    if i < chars.len() && chars[i] == '=' {
        i += 1;
        default = Some(parse_default_literal(&inside[i..])?);
    }

    Ok((name, default, choices))
}

fn parse_choice_set(inside: &str) -> Result<Vec<String>> {
    let parts: Vec<String> = inside
        .split('|')
        .map(|p| p.trim().to_string())
        .filter(|p| !p.is_empty())
        .collect();
    if parts.is_empty() {
        bail!("empty choice set `{{}}`");
    }
    Ok(parts)
}

fn parse_default_literal(rest: &str) -> Result<String> {
    let rest = rest.trim();
    if rest.is_empty() {
        bail!("default value missing after `=`");
    }
    let first = rest.chars().next().unwrap();
    if first == '\'' || first == '"' {
        let chars: Vec<char> = rest.chars().collect();
        let end = find_quote_end(&chars, 0, first)?;
        Ok(rest[1..end].to_string())
    } else {
        Ok(rest.to_string())
    }
}

/// Parse a (possibly nested) option chain inside `[...]` or as a bare option.
///
/// Examples handled here:
///   `-v | --version`
///   `-a | --add <name>`
///   `-d | --dir <path> ...`
///   `--input=<file> [--output=<dir>]`
///   `--long={a|b|c}`
///   `--long`
fn parse_option_chain(inner: &str, optional: bool) -> Result<Vec<SynopsisEntry>> {
    let inner = inner.trim();
    let tokens = tokenize_top(inner)?;
    if tokens.is_empty() {
        bail!("empty option group");
    }

    // Find the dependent nested optional (anything wrapped in `[...]` after the
    // primary). Everything before it forms the primary option.
    let mut split = tokens.len();
    for (idx, t) in tokens.iter().enumerate() {
        if t.starts_with('[') && t.ends_with(']') {
            split = idx;
            break;
        }
    }

    let primary_tokens = &tokens[..split];
    let dependent_tokens = &tokens[split..];

    let primary = parse_single_option(primary_tokens, optional)?;

    let mut entries = Vec::new();
    for dep_tok in dependent_tokens {
        let inner_dep = strip_outer(dep_tok, '[', ']')
            .ok_or_else(|| anyhow!("expected nested optional `[...]`, got {:?}", dep_tok))?;
        let dep_entries = parse_option_chain(inner_dep, /*optional=*/ true)?;
        for ent in dep_entries {
            if let SynopsisEntry::Option(mut o) = ent {
                let primary_name = primary.canonical_name();
                if !primary_name.is_empty() {
                    o.requires.push(primary_name);
                }
                entries.push(SynopsisEntry::Option(o));
            } else {
                entries.push(ent);
            }
        }
    }

    let mut all = vec![SynopsisEntry::Option(primary)];
    all.extend(entries);
    Ok(all)
}

/// Parse a single option (no nested deps). `tokens` is the result of splitting
/// the primary part on whitespace, e.g. `["-d", "|", "--dir", "<path>", "..."]`.
fn parse_single_option(tokens: &[String], optional: bool) -> Result<OptionDef> {
    if tokens.is_empty() {
        bail!("empty option");
    }

    let mut short: Option<char> = None;
    let mut long: Option<String> = None;
    let mut value_placeholder: Option<String> = None;
    let mut value_default: Option<String> = None;
    let mut value_choices: Option<Vec<String>> = None;
    let mut value_required = false;
    let mut repeats = false;

    let mut i = 0;
    while i < tokens.len() {
        let t = &tokens[i];

        if t == "|" {
            i += 1;
            continue;
        }

        if t == "..." {
            repeats = true;
            i += 1;
            continue;
        }

        if t.starts_with('<') {
            let (stripped, t_repeats) = strip_repeat_suffix(t);
            let inside = strip_outer(stripped, '<', '>')
                .ok_or_else(|| anyhow!("expected `<value>`, got {:?}", t))?;
            let (name, default, choices) = split_arg_inner(inside)?;
            value_placeholder = Some(name);
            value_default = default;
            value_choices = choices;
            value_required = true;
            if t_repeats {
                repeats = true;
            }
            i += 1;
            continue;
        }

        if t.starts_with("--") {
            // Could be `--long`, `--long=<...>`, `--long={a|b|c}`,
            // `--long=<arg='default'>`, `--config-env=<key>=<envvar>`
            let body = &t[2..];
            if let Some(eq_idx) = body.find('=') {
                let (name, rest) = body.split_at(eq_idx);
                let after_eq = &rest[1..];
                long = Some(name.to_string());

                if after_eq.starts_with('<') {
                    let chars: Vec<char> = after_eq.chars().collect();
                    let end = find_matching(&chars, 0, '<', '>')?;
                    let inside = &after_eq[1..end];
                    let (vname, vdefault, vchoices) = split_arg_inner(inside)?;
                    value_placeholder = Some(if vname.is_empty() {
                        "value".into()
                    } else {
                        vname
                    });
                    value_default = vdefault;
                    value_choices = vchoices;
                    value_required = true;
                    // Anything after the closing `>` (e.g. literal `=<envvar>`)
                    // is treated as a literal continuation of the placeholder
                    // text - we keep it as-is in the placeholder for help.
                    let trailing = &after_eq[end + 1..];
                    if !trailing.is_empty() {
                        let cur = value_placeholder.take().unwrap_or_default();
                        value_placeholder = Some(format!("{}{}", cur, trailing));
                    }
                } else if after_eq.starts_with('{') {
                    let chars: Vec<char> = after_eq.chars().collect();
                    let end = find_matching(&chars, 0, '{', '}')?;
                    value_choices = Some(parse_choice_set(&after_eq[1..end])?);
                    let trailing = &after_eq[end + 1..];
                    if let Some(stripped) = trailing.strip_prefix('=') {
                        value_default = Some(parse_default_literal(stripped)?);
                    }
                    value_required = true;
                    value_placeholder = Some("value".to_string());
                } else {
                    // Literal default after `=`, e.g. `--long=value`.
                    value_default = Some(parse_default_literal(after_eq)?);
                    value_required = true;
                    value_placeholder = Some("value".to_string());
                }
            } else {
                long = Some(body.to_string());
            }
            i += 1;
            continue;
        }

        if t.starts_with('-') && t.len() >= 2 {
            let body = &t[1..];
            // First char is the short option; anything else is illegal here
            // because we tokenise with whitespace boundaries.
            let c = body.chars().next().unwrap();
            short = Some(c);
            if body.len() > 1 {
                bail!(
                    "invalid short option `{}`: expected `-x`, got `{}`",
                    t,
                    t
                );
            }
            i += 1;
            continue;
        }

        bail!("unexpected token in option spec: {:?}", t);
    }

    if short.is_none() && long.is_none() {
        bail!("option has no short or long name");
    }

    let takes_value = if value_required {
        if value_default.is_some() {
            ValueKind::Optional(value_placeholder.unwrap_or_else(|| "value".into()))
        } else {
            ValueKind::Required(value_placeholder.unwrap_or_else(|| "value".into()))
        }
    } else {
        ValueKind::None
    };

    Ok(OptionDef {
        short,
        long,
        takes_value,
        default: value_default,
        choices: value_choices,
        repeats,
        requires: Vec::new(),
        required: !optional,
        description: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn opt(entries: &[SynopsisEntry]) -> Vec<OptionDef> {
        entries
            .iter()
            .filter_map(|e| match e {
                SynopsisEntry::Option(o) => Some(o.clone()),
                _ => None,
            })
            .collect()
    }

    fn args(entries: &[SynopsisEntry]) -> Vec<ArgDef> {
        entries
            .iter()
            .filter_map(|e| match e {
                SynopsisEntry::Argument(a) => Some(a.clone()),
                SynopsisEntry::RequiredChoice(a) => Some(a.clone()),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn required_positional() {
        let e = parse_fragment("<path>").unwrap();
        let a = args(&e);
        assert_eq!(a.len(), 1);
        assert_eq!(a[0].name, "path");
        assert!(a[0].required);
        assert!(a[0].default.is_none());
        assert!(!a[0].repeats);
    }

    #[test]
    fn optional_positional_with_default() {
        let e = parse_fragment("[<content='empty'>]").unwrap();
        let a = args(&e);
        assert_eq!(a.len(), 1);
        assert_eq!(a[0].name, "content");
        assert!(!a[0].required);
        assert_eq!(a[0].default.as_deref(), Some("empty"));
    }

    #[test]
    fn two_positionals() {
        let e = parse_fragment("<path> [<content='empty'>]").unwrap();
        let a = args(&e);
        assert_eq!(a.len(), 2);
        assert_eq!(a[0].name, "path");
        assert!(a[0].required);
        assert_eq!(a[1].name, "content");
        assert!(!a[1].required);
    }

    #[test]
    fn repeating_optional_default_star() {
        let e = parse_fragment("[<tests='*'>...]").unwrap();
        let a = args(&e);
        assert_eq!(a.len(), 1);
        assert_eq!(a[0].name, "tests");
        assert!(a[0].repeats);
        assert_eq!(a[0].default.as_deref(), Some("*"));
        assert!(!a[0].required);
    }

    #[test]
    fn repeating_optional_positional_spaced_ellipsis() {
        let e = parse_fragment("[<arg> ...]").unwrap();
        let a = args(&e);
        assert_eq!(a.len(), 1);
        assert_eq!(a[0].name, "arg");
        assert!(a[0].repeats);
        assert!(!a[0].required);
        assert!(a[0].default.is_none());
    }

    #[test]
    fn repeating_required_positional() {
        let e = parse_fragment("<assets>...").unwrap();
        let a = args(&e);
        assert_eq!(a.len(), 1);
        assert_eq!(a[0].name, "assets");
        assert!(a[0].required);
        assert!(a[0].repeats);
    }

    #[test]
    fn optional_bool_flag_alias_pair() {
        let e = parse_fragment("[-v | --version]").unwrap();
        let o = opt(&e);
        assert_eq!(o.len(), 1);
        assert_eq!(o[0].short, Some('v'));
        assert_eq!(o[0].long.as_deref(), Some("version"));
        assert!(matches!(o[0].takes_value, ValueKind::None));
        assert!(!o[0].required);
    }

    #[test]
    fn flag_with_required_value() {
        let e = parse_fragment("[-a | --add <name>]").unwrap();
        let o = opt(&e);
        assert_eq!(o.len(), 1);
        match &o[0].takes_value {
            ValueKind::Required(p) => assert_eq!(p, "name"),
            _ => panic!("expected required value"),
        }
    }

    #[test]
    fn flag_with_repeating_value() {
        let e = parse_fragment("[-d | --dir <path> ...]").unwrap();
        let o = opt(&e);
        assert_eq!(o.len(), 1);
        assert!(o[0].repeats);
        match &o[0].takes_value {
            ValueKind::Required(p) => assert_eq!(p, "path"),
            _ => panic!("expected required value"),
        }
    }

    #[test]
    fn eq_form_with_choice() {
        let e = parse_fragment("[--mode={fast|safe|deep}]").unwrap();
        let o = opt(&e);
        assert_eq!(o.len(), 1);
        assert_eq!(o[0].long.as_deref(), Some("mode"));
        assert_eq!(
            o[0].choices.as_deref(),
            Some(&["fast".into(), "safe".into(), "deep".into()][..])
        );
    }

    #[test]
    fn eq_form_choice_with_default() {
        let e = parse_fragment("[ -m | --mode <{consecutive|parallel}='consecutive'>]").unwrap();
        let o = opt(&e);
        assert_eq!(o.len(), 1);
        assert_eq!(o[0].short, Some('m'));
        assert_eq!(o[0].long.as_deref(), Some("mode"));
        assert_eq!(o[0].default.as_deref(), Some("consecutive"));
        assert_eq!(
            o[0].choices.as_deref(),
            Some(&["consecutive".into(), "parallel".into()][..])
        );
    }

    #[test]
    fn nested_dependency() {
        let e = parse_fragment("[--input=<file> [--output=<dir>]]").unwrap();
        let o = opt(&e);
        assert_eq!(o.len(), 2);
        assert_eq!(o[0].long.as_deref(), Some("input"));
        assert_eq!(o[1].long.as_deref(), Some("output"));
        assert_eq!(o[1].requires, vec!["input".to_string()]);
    }

    #[test]
    fn required_choice_top_level() {
        let e = parse_fragment("(render | build | clean)").unwrap();
        let a = args(&e);
        assert_eq!(a.len(), 1);
        assert!(a[0].required);
        assert_eq!(
            a[0].choices.as_deref(),
            Some(&["render".into(), "build".into(), "clean".into()][..])
        );
    }

    #[test]
    fn bare_required_flag() {
        let e = parse_fragment("--commit").unwrap();
        let o = opt(&e);
        assert_eq!(o.len(), 1);
        assert_eq!(o[0].long.as_deref(), Some("commit"));
        assert!(o[0].required);
    }

    #[test]
    fn quoted_default_literal() {
        let e = parse_fragment("[<content='hello world'>]").unwrap();
        let a = args(&e);
        assert_eq!(a[0].default.as_deref(), Some("hello world"));
    }

    #[test]
    fn multiple_top_level_tokens() {
        let e = parse_fragment("<path> [<content='empty'>]").unwrap();
        assert_eq!(e.len(), 2);
    }

    fn full_error(err: impl std::fmt::Display) -> String {
        format!("{err:#}")
    }

    #[test]
    fn unbalanced_bracket_errors() {
        let err = parse_fragment("[--foo").unwrap_err();
        assert!(full_error(&err).contains("unbalanced"));
    }

    #[test]
    fn unrecognized_token_errors() {
        let err = parse_fragment("not-a-spec").unwrap_err();
        assert!(full_error(&err).contains("unrecognised"));
    }

    #[test]
    fn required_choice_needs_two_alternatives() {
        let err = parse_fragment("(only)").unwrap_err();
        assert!(full_error(&err).contains("at least two alternatives"));
    }

    #[test]
    fn empty_fragment_returns_nothing() {
        assert!(parse_fragment("   ").unwrap().is_empty());
    }

    #[test]
    fn positional_choice_in_angle_brackets() {
        let e = parse_fragment("<mode={write|read}>").unwrap();
        let a = args(&e);
        assert_eq!(a.len(), 1);
        assert_eq!(a[0].name, "mode");
        assert!(a[0].required);
        assert_eq!(
            a[0].choices.as_deref(),
            Some(&["write".into(), "read".into()][..])
        );
    }

    #[test]
    fn optional_positional_choice_with_default() {
        let e = parse_fragment("[<mode={write|read}='write'>]").unwrap();
        let a = args(&e);
        assert_eq!(a.len(), 1);
        assert_eq!(a[0].name, "mode");
        assert!(!a[0].required);
        assert_eq!(a[0].default.as_deref(), Some("write"));
    }

    #[test]
    fn required_paren_alias_bool() {
        let e = parse_fragment("(-l | --long)").unwrap();
        let o = opt(&e);
        assert_eq!(o.len(), 1);
        assert_eq!(o[0].short, Some('l'));
        assert_eq!(o[0].long.as_deref(), Some("long"));
        assert!(o[0].required);
    }

    #[test]
    fn required_paren_alias_with_value() {
        let e = parse_fragment("(-l | --long <path>)").unwrap();
        let o = opt(&e);
        assert_eq!(o.len(), 1);
        assert!(o[0].required);
        match &o[0].takes_value {
            ValueKind::Required(p) => assert_eq!(p, "path"),
            _ => panic!("expected required value"),
        }
    }

    #[test]
    fn required_mutex_option_group() {
        let e = parse_fragment("(--long | --short)").unwrap();
        let o = opt(&e);
        assert_eq!(o.len(), 2);
        assert!(!o[0].required);
        assert!(!o[1].required);
        let groups: Vec<_> = e
            .iter()
            .filter_map(|ent| match ent {
                SynopsisEntry::OptionGroup(g) => Some(g.clone()),
                _ => None,
            })
            .collect();
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].members, vec!["long", "short"]);
        assert!(groups[0].required);
    }

    #[test]
    fn required_mutex_with_alias_pairs() {
        let e = parse_fragment("(-l | --long | -s | --short)").unwrap();
        let o = opt(&e);
        assert_eq!(o.len(), 2);
        assert_eq!(o[0].short, Some('l'));
        assert_eq!(o[0].long.as_deref(), Some("long"));
        assert_eq!(o[1].short, Some('s'));
        assert_eq!(o[1].long.as_deref(), Some("short"));
    }

    #[test]
    fn flag_level_repeat() {
        let e = parse_fragment("[-l | --long <arg>]...").unwrap();
        let o = opt(&e);
        assert_eq!(o.len(), 1);
        assert!(o[0].repeats);
    }

    #[test]
    fn positional_choice_still_works() {
        let e = parse_fragment("(north|south)").unwrap();
        let a = args(&e);
        assert_eq!(a.len(), 1);
        assert_eq!(a[0].name, "choice");
    }
}
