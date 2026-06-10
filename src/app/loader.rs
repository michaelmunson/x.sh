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
//! import:
//!   $:
//!     - ./handlers.yml
//!   env: ./.env
//!   sh:
//!     - ./helpers/example.sh
//! env:
//!   HELLO: global
//!   .env1:
//!     MY_NAME: env1
//! $:
//!   create.file: |
//!     ...
//! # OR legacy
//! $.import: /path/to/handlers.yml
//! ```

use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use serde::Deserialize;
use serde_yaml::Value;

use crate::app::spec::{App, AppEnv, ArgDef, Command, OptionDef, OptionGroupDef};
use crate::app::synopsis::{self, SynopsisEntry};

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum SynopsisField {
    Single(String),
    Many(Vec<String>),
}

impl SynopsisField {
    fn as_vec(&self) -> Vec<String> {
        match self {
            SynopsisField::Single(s) => vec![s.clone()],
            SynopsisField::Many(v) => v.clone(),
        }
    }

    fn into_vec(self) -> Vec<String> {
        match self {
            SynopsisField::Single(s) => vec![s],
            SynopsisField::Many(v) => v,
        }
    }
}

#[derive(Debug, Deserialize, Default)]
struct RawImport {
    #[serde(rename = "$", default)]
    dollar: Option<SynopsisField>,
    #[serde(default)]
    env: Option<SynopsisField>,
    #[serde(default)]
    sh: Option<SynopsisField>,
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
    #[serde(default)]
    import: Option<RawImport>,

    /// Inline handlers map (`$:`).
    #[serde(rename = "$", default)]
    dollar: Option<BTreeMap<String, String>>,

    /// External handler imports (`$.import`, legacy).
    #[serde(rename = "$.import", default)]
    dollar_import: Option<SynopsisField>,

    #[serde(default)]
    env: Option<Value>,
}

/// Load and parse an app config file.
pub fn load(path: &Path) -> Result<App> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("failed to read {}", path.display()))?;
    parse(&content, path)
}

/// Parse YAML content. `path` is used only for resolving import entries
/// relative to the file (and for error context).
pub fn parse(content: &str, path: &Path) -> Result<App> {
    let raw: RawApp = serde_yaml::from_str(content)
        .with_context(|| format!("failed to parse {}", path.display()))?;

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

    let handlers = resolve_handlers(path, &raw)?;
    let env = resolve_env(path, &raw)?;
    let sh_imports = resolve_sh_imports(path, &raw)?;

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

    Ok(App {
        name: app_name,
        version: raw.version,
        description: raw.description,
        root,
        handlers,
        env,
        sh_imports,
    })
}

fn resolve_handlers(path: &Path, raw: &RawApp) -> Result<BTreeMap<String, String>> {
    let import_paths = handler_import_paths(raw)?;
    let mut handlers = if import_paths.is_empty() {
        BTreeMap::new()
    } else {
        load_handler_imports(path, import_paths)?
    };
    if let Some(dollar) = &raw.dollar {
        for (k, v) in normalize_handlers(dollar.clone())? {
            handlers.insert(k, v);
        }
    }
    Ok(handlers)
}

fn handler_import_paths(raw: &RawApp) -> Result<Vec<String>> {
    let from_legacy = raw
        .dollar_import
        .as_ref()
        .map(SynopsisField::as_vec)
        .unwrap_or_default();
    let from_import = raw
        .import
        .as_ref()
        .and_then(|i| i.dollar.as_ref())
        .map(SynopsisField::as_vec)
        .unwrap_or_default();
    if !from_legacy.is_empty() && !from_import.is_empty() {
        bail!("cannot use both `$.import` and `import.$`");
    }
    Ok(if from_import.is_empty() {
        from_legacy
    } else {
        from_import
    })
}

fn env_import_paths(raw: &RawApp) -> Vec<String> {
    raw.import
        .as_ref()
        .and_then(|i| i.env.as_ref())
        .map(SynopsisField::as_vec)
        .unwrap_or_default()
}

fn sh_import_paths(raw: &RawApp) -> Vec<String> {
    raw.import
        .as_ref()
        .and_then(|i| i.sh.as_ref())
        .map(SynopsisField::as_vec)
        .unwrap_or_default()
}

fn resolve_sh_imports(path: &Path, raw: &RawApp) -> Result<Vec<PathBuf>> {
    let paths = sh_import_paths(raw);
    if paths.is_empty() {
        return Ok(Vec::new());
    }
    let app_dir = app_dir(path);
    let mut out = Vec::with_capacity(paths.len());
    for p in paths {
        let resolved = resolve_path(&app_dir, &p);
        if !resolved.is_file() {
            bail!("sh import {} is not a file", resolved.display());
        }
        out.push(resolved);
    }
    Ok(out)
}

fn resolve_env(path: &Path, raw: &RawApp) -> Result<AppEnv> {
    let import_paths = env_import_paths(raw);
    let mut globals = if import_paths.is_empty() {
        BTreeMap::new()
    } else {
        load_env_imports(path, import_paths)?
    };

    let mut groups = BTreeMap::new();
    if let Some(value) = &raw.env {
        let inline = parse_inline_env(value)?;
        for (k, v) in inline.globals {
            globals.insert(k, v);
        }
        groups = inline.groups;
    }

    Ok(AppEnv { globals, groups })
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
            SynopsisEntry::OptionGroup(g) => push_option_group(cmd, g)?,
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

fn push_option_group(cmd: &mut Command, group: OptionGroupDef) -> Result<()> {
    cmd.option_groups.push(group);
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

fn load_handler_imports(app_path: &Path, paths: Vec<String>) -> Result<BTreeMap<String, String>> {
    let app_dir = app_dir(app_path);
    let mut out: BTreeMap<String, String> = BTreeMap::new();
    for p in paths {
        let resolved = resolve_path(&app_dir, &p);
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

fn load_env_imports(app_path: &Path, paths: Vec<String>) -> Result<BTreeMap<String, String>> {
    let app_dir = app_dir(app_path);
    let mut out: BTreeMap<String, String> = BTreeMap::new();
    for p in paths {
        let resolved = resolve_path(&app_dir, &p);
        let content = fs::read_to_string(&resolved)
            .with_context(|| format!("failed to read env import {}", resolved.display()))?;
        let parsed = parse_dotenv(&content)
            .with_context(|| format!("failed to parse env import {}", resolved.display()))?;
        for (k, v) in parsed {
            if out.contains_key(&k) {
                bail!(
                    "duplicate env key {:?} when merging import {}",
                    k,
                    resolved.display()
                );
            }
            validate_env_var_name(&k)?;
            out.insert(k, v);
        }
    }
    Ok(out)
}

fn parse_inline_env(value: &Value) -> Result<AppEnv> {
    let mapping = value.as_mapping().ok_or_else(|| anyhow!("`env:` must be a mapping"))?;
    let mut globals = BTreeMap::new();
    let mut groups = BTreeMap::new();

    for (key, val) in mapping {
        let key_str = key
            .as_str()
            .ok_or_else(|| anyhow!("`env:` keys must be strings"))?;

        if key_str.starts_with('.') {
            let group_name = &key_str[1..];
            validate_env_group_name(group_name)?;
            if groups.contains_key(group_name) {
                bail!("duplicate env group `.{}` in `env:` block", group_name);
            }
            let vars = parse_env_group_value(val)?;
            for var_name in vars.keys() {
                validate_env_var_name(var_name)?;
            }
            groups.insert(group_name.to_string(), vars);
        } else {
            validate_env_var_name(key_str)?;
            if globals.contains_key(key_str) {
                bail!("duplicate env key {:?} in `env:` block", key_str);
            }
            let scalar = val
                .as_str()
                .ok_or_else(|| anyhow!("env global `{}` must be a string", key_str))?;
            globals.insert(key_str.to_string(), scalar.to_string());
        }
    }

    Ok(AppEnv { globals, groups })
}

fn parse_env_group_value(value: &Value) -> Result<BTreeMap<String, String>> {
    let mut out = BTreeMap::new();
    match value {
        Value::Mapping(map) => {
            for (k, v) in map {
                let key = k
                    .as_str()
                    .ok_or_else(|| anyhow!("env group keys must be strings"))?;
                if out.contains_key(key) {
                    bail!("duplicate env key {:?} in env group", key);
                }
                let scalar = v
                    .as_str()
                    .ok_or_else(|| anyhow!("env group entry `{}` must be a string", key))?;
                out.insert(key.to_string(), scalar.to_string());
            }
        }
        Value::Sequence(seq) => {
            for item in seq {
                let map = item.as_mapping().ok_or_else(|| {
                    anyhow!("env group list entries must be single-key mappings")
                })?;
                if map.len() != 1 {
                    bail!("env group list entries must be single-key mappings");
                }
                for (k, v) in map {
                    let key = k
                        .as_str()
                        .ok_or_else(|| anyhow!("env group keys must be strings"))?;
                    if out.contains_key(key) {
                        bail!("duplicate env key {:?} in env group", key);
                    }
                    let scalar = v
                        .as_str()
                        .ok_or_else(|| anyhow!("env group entry `{}` must be a string", key))?;
                    out.insert(key.to_string(), scalar.to_string());
                }
            }
        }
        _ => bail!("env group value must be a mapping or list of single-key mappings"),
    }
    Ok(out)
}

fn parse_dotenv(content: &str) -> Result<BTreeMap<String, String>> {
    let mut out = BTreeMap::new();
    for (line_no, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            bail!("invalid .env syntax at line {}: expected KEY=VALUE", line_no + 1);
        };
        let key = key.trim();
        if key.is_empty() {
            bail!("invalid .env syntax at line {}: empty key", line_no + 1);
        }
        let value = strip_dotenv_quotes(value.trim());
        if out.contains_key(key) {
            bail!("duplicate env key {:?} in .env file", key);
        }
        out.insert(key.to_string(), value);
    }
    Ok(out)
}

fn strip_dotenv_quotes(value: &str) -> String {
    if (value.starts_with('"') && value.ends_with('"') && value.len() >= 2)
        || (value.starts_with('\'') && value.ends_with('\'') && value.len() >= 2)
    {
        value[1..value.len() - 1].to_string()
    } else {
        value.to_string()
    }
}

fn validate_env_var_name(name: &str) -> Result<()> {
    if is_valid_shell_identifier(name) {
        Ok(())
    } else {
        bail!(
            "invalid env variable name {:?}: must match [A-Za-z_][A-Za-z0-9_]*",
            name
        )
    }
}

fn validate_env_group_name(name: &str) -> Result<()> {
    if name.is_empty() {
        bail!("env group name must not be empty");
    }
    if is_valid_shell_identifier(name) {
        Ok(())
    } else {
        bail!(
            "invalid env group name {:?}: must match [A-Za-z_][A-Za-z0-9_]*",
            name
        )
    }
}

fn is_valid_shell_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn app_dir(app_path: &Path) -> PathBuf {
    app_path
        .parent()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
}

fn resolve_path(app_dir: &Path, p: &str) -> PathBuf {
    if Path::new(p).is_absolute() {
        PathBuf::from(p)
    } else {
        app_dir.join(p)
    }
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
        assert!(app.env.globals.is_empty());
        assert!(app.env.groups.is_empty());
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
    fn inline_handlers_override_imports() {
        let dir = tempfile::tempdir().unwrap();
        let handlers = dir.path().join("handlers.yml");
        std::fs::write(&handlers, "run: echo imported\nother: echo other\n").unwrap();
        let app_file = dir.path().join("app.x.yml");
        std::fs::write(
            &app_file,
            r#"
name: merged
import:
  $: ./handlers.yml
"$":
  run: echo inline
"#,
        )
        .unwrap();
        let app = load(&app_file).unwrap();
        assert_eq!(
            app.handlers.get("run").map(String::as_str),
            Some("echo inline")
        );
        assert_eq!(
            app.handlers.get("other").map(String::as_str),
            Some("echo other")
        );
    }

    #[test]
    fn rejects_both_dollar_import_and_import_dollar() {
        let err = parse(
            r#"
name: x
import:
  $: ./foo.yml
"$.import": "./bar.yml"
"#,
            Path::new("t.x.yml"),
        )
        .unwrap_err();
        assert!(err.to_string().contains("cannot use both"));
    }

    #[test]
    fn derives_name_from_filename() {
        let app = parse("version: 0.1.0\n", Path::new("widget.x.yml")).unwrap();
        assert_eq!(app.name, "widget");
    }

    #[test]
    fn loads_dollar_import_relative_to_app_file() {
        let dir = tempfile::tempdir().unwrap();
        let handlers = dir.path().join("handlers.yml");
        std::fs::write(&handlers, "run: echo imported\n").unwrap();
        let app_file = dir.path().join("app.x.yml");
        std::fs::write(
            &app_file,
            r#"
name: imported-app
"$.import": "./handlers.yml"
"#,
        )
        .unwrap();
        let app = load(&app_file).unwrap();
        assert_eq!(
            app.handlers.get("run").map(String::as_str),
            Some("echo imported")
        );
    }

    #[test]
    fn rejects_duplicate_handler_key_in_import_merge() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("a.yml"), "run: echo a\n").unwrap();
        std::fs::write(dir.path().join("b.yml"), "run: echo b\n").unwrap();
        let app_file = dir.path().join("app.x.yml");
        std::fs::write(
            &app_file,
            r#"
name: dup
"$.import":
  - "./a.yml"
  - "./b.yml"
"#,
        )
        .unwrap();
        let err = load(&app_file).unwrap_err();
        assert!(err.to_string().contains("duplicate handler key"));
    }

    #[test]
    fn loads_inline_env_globals_and_groups() {
        let app = p(
            r#"
name: env-app
env:
  HELLO: global
  .env1:
    - MY_NAME: env1
  .env2:
    MY_NAME: env2
    OTHER: val
commands:
  run:
    description: run
"$":
  run: echo
"#,
        );
        assert_eq!(app.env.globals.get("HELLO").map(String::as_str), Some("global"));
        assert_eq!(
            app.env.groups.get("env1").and_then(|g| g.get("MY_NAME")).map(String::as_str),
            Some("env1")
        );
        assert_eq!(
            app.env.groups.get("env2").and_then(|g| g.get("MY_NAME")).map(String::as_str),
            Some("env2")
        );
        assert_eq!(
            app.env.groups.get("env2").and_then(|g| g.get("OTHER")).map(String::as_str),
            Some("val")
        );
    }

    #[test]
    fn loads_env_import_from_dotenv() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join(".env"), "HELLO=imported\nFOO=bar\n").unwrap();
        let app_file = dir.path().join("app.x.yml");
        std::fs::write(
            &app_file,
            r#"
name: env-import
import:
  env: ./.env
commands:
  run:
    description: run
"$":
  run: echo
"#,
        )
        .unwrap();
        let app = load(&app_file).unwrap();
        assert_eq!(
            app.env.globals.get("HELLO").map(String::as_str),
            Some("imported")
        );
        assert_eq!(app.env.globals.get("FOO").map(String::as_str), Some("bar"));
    }

    #[test]
    fn inline_env_overrides_import() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join(".env"), "HELLO=imported\n").unwrap();
        let app_file = dir.path().join("app.x.yml");
        std::fs::write(
            &app_file,
            r#"
name: env-overlay
import:
  env: ./.env
env:
  HELLO: inline
commands:
  run:
    description: run
"$":
  run: echo
"#,
        )
        .unwrap();
        let app = load(&app_file).unwrap();
        assert_eq!(
            app.env.globals.get("HELLO").map(String::as_str),
            Some("inline")
        );
    }

    #[test]
    fn rejects_duplicate_env_key_in_import_merge() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("a.env"), "HELLO=a\n").unwrap();
        std::fs::write(dir.path().join("b.env"), "HELLO=b\n").unwrap();
        let app_file = dir.path().join("app.x.yml");
        std::fs::write(
            &app_file,
            r#"
name: dup-env
import:
  env:
    - ./a.env
    - ./b.env
commands:
  run:
    description: run
"$":
  run: echo
"#,
        )
        .unwrap();
        let err = load(&app_file).unwrap_err();
        assert!(err.to_string().contains("duplicate env key"));
    }

    #[test]
    fn loads_sh_import_paths() {
        let dir = tempfile::tempdir().unwrap();
        let helpers = dir.path().join("helpers");
        std::fs::create_dir_all(&helpers).unwrap();
        std::fs::write(helpers.join("example.sh"), "example_fn() { echo ok; }\n").unwrap();
        let app_file = dir.path().join("app.x.yml");
        std::fs::write(
            &app_file,
            r#"
name: sh-import
import:
  sh:
    - ./helpers/example.sh
commands:
  run:
    description: run
"$":
  run: example_fn
"#,
        )
        .unwrap();
        let app = load(&app_file).unwrap();
        assert_eq!(app.sh_imports.len(), 1);
        assert_eq!(
            app.sh_imports[0],
            helpers.join("example.sh")
        );
    }

    #[test]
    fn rejects_missing_sh_import_file() {
        let dir = tempfile::tempdir().unwrap();
        let app_file = dir.path().join("app.x.yml");
        std::fs::write(
            &app_file,
            r#"
name: missing-sh
import:
  sh: ./nope.sh
"#,
        )
        .unwrap();
        let err = load(&app_file).unwrap_err();
        assert!(err.to_string().contains("sh import"));
    }

    #[test]
    fn loads_exapp_example_from_repo() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("docs/examples/app/exapp.x.yml");
        let app = load(&path).unwrap();
        assert_eq!(app.name, "exapp");
        assert!(app.handlers.contains_key("demo.opts"));
        assert!(app.root.subcommands.contains_key("demo"));
    }
}
