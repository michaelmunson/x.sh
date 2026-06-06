//! Load a `<name>.x.yml` file into an [`App`].
//!
//! The on-disk YAML shape:
//!
//! ```yaml
//! name: my-app
//! version: 0.0.0
//! description: ...
//! options:
//!   - "[-v | --version]"
//! commands:
//!   create:
//!     description: ...
//!     options: [...]
//!     arguments: [...]
//!     commands: { ... }
//! $:
//!   create.file: |
//!     ...
//! # OR
//! $.import: /path/to/handlers.yml
//! ```

use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use serde::Deserialize;

use crate::app::spec::{App, ArgDef, Command, OptionDef};
use crate::app::synopsis::{self, SynopsisEntry};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum SynopsisField {
    Single(String),
    Many(Vec<String>),
}

impl SynopsisField {
    fn into_vec(self) -> Vec<String> {
        match self {
            SynopsisField::Single(s) => vec![s],
            SynopsisField::Many(v) => v,
        }
    }
}

#[derive(Debug, Deserialize, Default)]
struct RawCommand {
    description: Option<String>,
    #[serde(default)]
    options: Option<SynopsisField>,
    #[serde(default)]
    arguments: Option<SynopsisField>,
    #[serde(default)]
    commands: Option<BTreeMap<String, RawCommand>>,
}

#[derive(Debug, Deserialize)]
struct RawApp {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    version: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    options: Option<SynopsisField>,
    #[serde(default)]
    arguments: Option<SynopsisField>,
    #[serde(default)]
    commands: Option<BTreeMap<String, RawCommand>>,

    /// Inline handlers map (`$:`).
    #[serde(rename = "$", default)]
    dollar: Option<BTreeMap<String, String>>,

    /// External handler imports (`$.import`).
    #[serde(rename = "$.import", default)]
    dollar_import: Option<SynopsisField>,
}

/// Load and parse an app config file.
pub fn load(path: &Path) -> Result<App> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("failed to read {}", path.display()))?;
    parse(&content, path)
}

/// Parse YAML content. `path` is used only for resolving `$.import` entries
/// relative to the file (and for error context).
pub fn parse(content: &str, path: &Path) -> Result<App> {
    let raw: RawApp = serde_yaml::from_str(content)
        .with_context(|| format!("failed to parse {}", path.display()))?;

    if raw.dollar.is_some() && raw.dollar_import.is_some() {
        bail!("cannot use both `$:` and `$.import:` in the same app file");
    }

    let app_name = raw
        .name
        .clone()
        .or_else(|| {
            path.file_name()
                .and_then(|s| s.to_str())
                .and_then(|s| s.strip_suffix(".x.yml"))
                .map(|s| s.to_string())
        })
        .ok_or_else(|| anyhow!("app file is missing top-level `name:`"))?;

    let mut root = Command::new(app_name.clone());
    root.description = raw.description.clone();

    if let Some(opts) = raw.options {
        for frag in opts.into_vec() {
            let entries = synopsis::parse_fragment(&frag)
                .with_context(|| "in root `options`")?;
            apply_entries_to_command(&mut root, entries)?;
        }
    }
    if let Some(args) = raw.arguments {
        for frag in args.into_vec() {
            let entries = synopsis::parse_fragment(&frag)
                .with_context(|| "in root `arguments`")?;
            apply_entries_to_command(&mut root, entries)?;
        }
    }
    if let Some(commands) = raw.commands {
        for (name, raw_sub) in commands {
            let sub = build_command(&name, raw_sub)?;
            root.subcommands.insert(name, sub);
        }
    }

    let handlers = if let Some(dollar) = raw.dollar {
        normalize_handlers(dollar)?
    } else if let Some(import) = raw.dollar_import {
        load_imports(path, import.into_vec())?
    } else {
        BTreeMap::new()
    };

    Ok(App {
        name: app_name,
        version: raw.version,
        description: raw.description,
        root,
        handlers,
    })
}

fn build_command(name: &str, raw: RawCommand) -> Result<Command> {
    let mut cmd = Command::new(name);
    cmd.description = raw.description;
    if let Some(opts) = raw.options {
        for frag in opts.into_vec() {
            let entries = synopsis::parse_fragment(&frag)
                .with_context(|| format!("in `{}.options`", name))?;
            apply_entries_to_command(&mut cmd, entries)?;
        }
    }
    if let Some(args) = raw.arguments {
        for frag in args.into_vec() {
            let entries = synopsis::parse_fragment(&frag)
                .with_context(|| format!("in `{}.arguments`", name))?;
            apply_entries_to_command(&mut cmd, entries)?;
        }
    }
    if let Some(commands) = raw.commands {
        for (sub_name, raw_sub) in commands {
            let sub = build_command(&sub_name, raw_sub)?;
            cmd.subcommands.insert(sub_name, sub);
        }
    }
    Ok(cmd)
}

fn apply_entries_to_command(cmd: &mut Command, entries: Vec<SynopsisEntry>) -> Result<()> {
    for entry in entries {
        match entry {
            SynopsisEntry::Option(o) => push_option(cmd, o)?,
            SynopsisEntry::Argument(a) => push_arg(cmd, a)?,
            SynopsisEntry::RequiredChoice(a) => push_arg(cmd, a)?,
        }
    }
    Ok(())
}

fn push_option(cmd: &mut Command, opt: OptionDef) -> Result<()> {
    cmd.options.push(opt);
    Ok(())
}

fn push_arg(cmd: &mut Command, arg: ArgDef) -> Result<()> {
    cmd.arguments.push(arg);
    Ok(())
}

fn normalize_handlers(raw: BTreeMap<String, String>) -> Result<BTreeMap<String, String>> {
    let mut out = BTreeMap::new();
    let mut seen = HashSet::new();
    for (key, body) in raw {
        if !seen.insert(key.clone()) {
            bail!("duplicate handler key in `$:` block: {:?}", key);
        }
        out.insert(key, body);
    }
    Ok(out)
}

fn load_imports(app_path: &Path, paths: Vec<String>) -> Result<BTreeMap<String, String>> {
    let app_dir = app_path
        .parent()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    let mut out: BTreeMap<String, String> = BTreeMap::new();
    for p in paths {
        let resolved = if Path::new(&p).is_absolute() {
            PathBuf::from(&p)
        } else {
            app_dir.join(&p)
        };
        let content = fs::read_to_string(&resolved)
            .with_context(|| format!("failed to read import {}", resolved.display()))?;
        let parsed: BTreeMap<String, String> = serde_yaml::from_str(&content)
            .with_context(|| format!("failed to parse import {}", resolved.display()))?;
        for (k, v) in parsed {
            if out.contains_key(&k) {
                bail!(
                    "duplicate handler key {:?} when merging import {}",
                    k,
                    resolved.display()
                );
            }
            out.insert(k, v);
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(yaml: &str) -> App {
        parse(yaml, Path::new("test.x.yml")).unwrap()
    }

    #[test]
    fn loads_minimal() {
        let app = p("name: foo\n");
        assert_eq!(app.name, "foo");
        assert!(app.root.subcommands.is_empty());
    }

    #[test]
    fn loads_options_and_commands() {
        let app = p(r#"
name: my-app
version: 0.0.0
options:
  - "[-v | --version]"
commands:
  build:
    description: build it
    options:
      - "[--mode={fast|safe}]"
    arguments:
      - "<assets>..."
"$":
  build: "echo hi"
"#);
        assert_eq!(app.name, "my-app");
        assert_eq!(app.root.options.len(), 1);
        assert_eq!(app.root.subcommands.len(), 1);
        let build = &app.root.subcommands["build"];
        assert_eq!(build.options.len(), 1);
        assert_eq!(build.arguments.len(), 1);
        assert_eq!(app.handlers.get("build").map(String::as_str), Some("echo hi"));
    }

    #[test]
    fn rejects_dollar_and_import_together() {
        let err = parse(
            r#"
name: x
"$":
  a: echo
"$.import": "./foo.yml"
"#,
            Path::new("t.x.yml"),
        )
        .unwrap_err();
        assert!(err.to_string().contains("cannot use both"));
    }
}
