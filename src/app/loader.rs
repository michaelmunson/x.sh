//! Load an `x.yml` or `<name>.x.yml` file into an [`App`].

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use serde::Deserialize;
use serde_yaml::Value;

use crate::app::spec::{App, AppEnv, Command};
use crate::app::synopsis::{self, SynopsisEntry};

const ROOT_RESERVED: &[&str] = &[
    "name", "version", "description", "help", "dir", "env", "import",
    "options", "opts", "arguments", "args", "$",
];

const CMD_RESERVED: &[&str] = &[
    "description", "help", "options", "opts", "arguments", "args",
    "$", "dir", "env", "alias",
];

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

/// Load and parse an app config file.
pub fn load(path: &Path) -> Result<App> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("failed to read {}", path.display()))?;
    parse(&content, path)
}

/// Parse YAML content. `path` is used only for resolving paths relative to the file.
pub fn parse(content: &str, path: &Path) -> Result<App> {
    let value: Value = serde_yaml::from_str(content)
        .with_context(|| format!("failed to parse {}", path.display()))?;

    let mapping = value
        .as_mapping()
        .ok_or_else(|| anyhow!("app file must be a YAML mapping"))?;

    if mapping.contains_key("commands") {
        bail!(
            "`commands:` was removed in v3; define commands with `.command-name:` keys instead"
        );
    }

    if let Some(dollar) = mapping.get("$") {
        if dollar.is_mapping() {
            bail!(
                "top-level `$:` handler map was removed in v3; define scripts inline with `$:` under each command"
            );
        }
    }

    let app_dir = app_dir(path);
    let is_project = path.file_name().is_some_and(|n| n == "x.yml");

    let app_name = mapping
        .get("name")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .or_else(|| derive_name_from_path(path))
        .ok_or_else(|| anyhow!("app file is missing top-level `name:`"))?;

    let version = mapping
        .get("version")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let description = read_help(mapping)?;

    let import = mapping
        .get("import")
        .map(parse_import)
        .transpose()?;

    let mut handlers = resolve_handler_imports(path, import.as_ref())?;
    let sh_imports = resolve_sh_imports(path, import.as_ref())?;

    let mut root_env = resolve_env_imports(path, import.as_ref())?;
    if let Some(env_val) = mapping.get("env") {
        let inline = parse_inline_env(env_val)?;
        merge_env(&mut root_env, inline);
    }

    let mut root = Command::new(app_name.clone());
    root.description = description.clone();
    root.env = root_env;
    if let Some(dir) = mapping.get("dir").and_then(|v| v.as_str()) {
        root.dir = resolve_dir(path, Some(dir))?;
    }

    apply_synopsis_fields(mapping, &mut root, "root")?;

    if let Some(script) = mapping.get("$").and_then(|v| v.as_str()) {
        handlers.insert(String::new(), script.to_string());
    }

    for (key, val) in mapping {
        let key_str = key
            .as_str()
            .ok_or_else(|| anyhow!("YAML keys must be strings"))?;

        if is_reserved(key_str, ROOT_RESERVED) {
            continue;
        }

        if let Some(cmd_name) = key_str.strip_prefix('.') {
            if cmd_name.is_empty() {
                bail!("command name must not be empty after `.` prefix");
            }
            let cmd = parse_command_node(cmd_name, val, path, &app_dir, &mut handlers, cmd_name)?;
            root.subcommands.insert(cmd_name.to_string(), cmd);
        } else if is_project {
            bail!(
                "unknown key `{key_str}` in x.yml; v3 requires dot-prefixed command keys (e.g. `.{key_str}:`)"
            );
        } else {
            bail!(
                "unknown key `{key_str}`; use dot-prefixed command keys (e.g. `.{key_str}:`) or a reserved property"
            );
        }
    }

    Ok(App {
        name: app_name,
        version,
        description,
        root,
        handlers,
        sh_imports,
    })
}

fn derive_name_from_path(path: &Path) -> Option<String> {
    path.file_name()
        .and_then(|s| s.to_str())
        .and_then(|s| s.strip_suffix(".x.yml").or_else(|| s.strip_suffix(".yml")))
        .map(|s| s.to_string())
}

fn is_reserved(key: &str, reserved: &[&str]) -> bool {
    reserved.contains(&key)
}

fn read_help(mapping: &serde_yaml::Mapping) -> Result<Option<String>> {
    let help = mapping.get("help").and_then(|v| v.as_str());
    let desc = mapping.get("description").and_then(|v| v.as_str());
    if help.is_some() && desc.is_some() {
        bail!("cannot use both `help:` and `description:`");
    }
    Ok(help.or(desc).map(|s| s.to_string()))
}

fn parse_import(value: &Value) -> Result<RawImport> {
    serde_yaml::from_value(value.clone()).context("failed to parse `import:` block")
}

fn parse_command_node(
    name: &str,
    value: &Value,
    app_path: &Path,
    app_dir: &Path,
    handlers: &mut BTreeMap<String, String>,
    handler_key: &str,
) -> Result<Command> {
    match value {
        Value::String(script) => {
            handlers.insert(handler_key.to_string(), script.clone());
            Ok(Command::new(name))
        }
        Value::Mapping(mapping) => {
            let mut cmd = Command::new(name);
            cmd.description = read_help(mapping)?;
            apply_synopsis_fields(mapping, &mut cmd, name)?;
            apply_dir_env_fields(mapping, app_path, &mut cmd)?;

            if let Some(alias) = mapping.get("alias") {
                let alias_str = alias
                    .as_str()
                    .ok_or_else(|| anyhow!("`alias` on `{name}` must be a string"))?;
                let resolved = resolve_path(app_dir, alias_str);
                if !resolved.is_file() {
                    bail!("alias target `{}` is not a file", resolved.display());
                }
                cmd.alias = Some(resolved);
            }

            if let Some(script) = mapping.get("$").and_then(|v| v.as_str()) {
                handlers.insert(handler_key.to_string(), script.to_string());
            }

            for (key, val) in mapping {
                let key_str = key
                    .as_str()
                    .ok_or_else(|| anyhow!("YAML keys must be strings"))?;

                if is_reserved(key_str, CMD_RESERVED) {
                    continue;
                }

                if let Some(sub_name) = key_str.strip_prefix('.') {
                    if sub_name.is_empty() {
                        bail!("command name must not be empty after `.` prefix");
                    }
                    let sub_key = format!("{handler_key}.{sub_name}");
                    let sub = parse_command_node(sub_name, val, app_path, app_dir, handlers, &sub_key)?;
                    cmd.subcommands.insert(sub_name.to_string(), sub);
                } else {
                    bail!(
                        "unknown key `{key_str}` on command `{name}`; use dot-prefixed subcommand keys (e.g. `.{key_str}:`)"
                    );
                }
            }

            Ok(cmd)
        }
        _ => bail!("command `{name}` must be a string or mapping"),
    }
}

fn apply_synopsis_fields(
    mapping: &serde_yaml::Mapping,
    cmd: &mut Command,
    context: &str,
) -> Result<()> {
    if mapping.contains_key("options") && mapping.contains_key("opts") {
        bail!("cannot use both `options:` and `opts:` on `{context}`");
    }
    if mapping.contains_key("arguments") && mapping.contains_key("args") {
        bail!("cannot use both `arguments:` and `args:` on `{context}`");
    }

    let opts_key = if mapping.contains_key("opts") {
        "opts"
    } else {
        "options"
    };
    if let Some(opts) = mapping.get(opts_key) {
        for frag in synopsis_strings(opts)? {
            let entries = synopsis::parse_fragment(&frag)
                .with_context(|| format!("in `{context}.{opts_key}`"))?;
            apply_entries_to_command(cmd, entries)?;
        }
    }

    let args_key = if mapping.contains_key("args") {
        "args"
    } else {
        "arguments"
    };
    if let Some(args) = mapping.get(args_key) {
        for frag in synopsis_strings(args)? {
            let entries = synopsis::parse_fragment(&frag)
                .with_context(|| format!("in `{context}.{args_key}`"))?;
            apply_entries_to_command(cmd, entries)?;
        }
    }

    Ok(())
}

fn synopsis_strings(value: &Value) -> Result<Vec<String>> {
    match value {
        Value::String(s) => Ok(s
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(|l| l.to_string())
            .collect()),
        Value::Sequence(seq) => {
            let mut out = Vec::new();
            for item in seq {
                let s = item
                    .as_str()
                    .ok_or_else(|| anyhow!("synopsis list items must be strings"))?;
                out.push(s.to_string());
            }
            Ok(out)
        }
        _ => bail!("synopsis field must be a string or list of strings"),
    }
}

fn apply_dir_env_fields(
    mapping: &serde_yaml::Mapping,
    app_path: &Path,
    cmd: &mut Command,
) -> Result<()> {
    if let Some(dir) = mapping.get("dir").and_then(|v| v.as_str()) {
        cmd.dir = resolve_dir(app_path, Some(dir))?;
    }
    if let Some(env_val) = mapping.get("env") {
        cmd.env = parse_inline_env(env_val)?;
    }
    Ok(())
}

fn apply_entries_to_command(cmd: &mut Command, entries: Vec<SynopsisEntry>) -> Result<()> {
    for entry in entries {
        match entry {
            SynopsisEntry::Option(o) => cmd.options.push(o),
            SynopsisEntry::Argument(a) => cmd.arguments.push(a),
            SynopsisEntry::RequiredChoice(a) => cmd.arguments.push(a),
            SynopsisEntry::OptionGroup(g) => cmd.option_groups.push(g),
        }
    }
    Ok(())
}

fn merge_env(target: &mut AppEnv, overlay: AppEnv) {
    for (k, v) in overlay.globals {
        target.globals.insert(k, v);
    }
    for (group, vars) in overlay.groups {
        target.groups.insert(group, vars);
    }
}

fn resolve_handler_imports(app_path: &Path, import: Option<&RawImport>) -> Result<BTreeMap<String, String>> {
    let paths = import
        .and_then(|i| i.dollar.as_ref())
        .map(SynopsisField::as_vec)
        .unwrap_or_default();
    if paths.is_empty() {
        return Ok(BTreeMap::new());
    }
    load_handler_imports(app_path, paths)
}

fn resolve_sh_imports(app_path: &Path, import: Option<&RawImport>) -> Result<Vec<PathBuf>> {
    let paths = import
        .and_then(|i| i.sh.as_ref())
        .map(SynopsisField::as_vec)
        .unwrap_or_default();
    if paths.is_empty() {
        return Ok(Vec::new());
    }
    let app_dir = app_dir(app_path);
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

fn resolve_env_imports(app_path: &Path, import: Option<&RawImport>) -> Result<AppEnv> {
    let paths = import
        .and_then(|i| i.env.as_ref())
        .map(SynopsisField::as_vec)
        .unwrap_or_default();
    if paths.is_empty() {
        return Ok(AppEnv::default());
    }
    let globals = load_env_imports(app_path, paths)?;
    Ok(AppEnv {
        globals,
        groups: BTreeMap::new(),
    })
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

fn resolve_dir(app_path: &Path, dir: Option<&str>) -> Result<Option<PathBuf>> {
    let Some(dir) = dir else {
        return Ok(None);
    };
    let resolved = resolve_path(&app_dir(app_path), dir);
    if !resolved.is_dir() {
        bail!("`dir` {} is not a directory", resolved.display());
    }
    Ok(Some(resolved))
}

fn resolve_path(app_dir: &Path, p: &str) -> PathBuf {
    if Path::new(p).is_absolute() {
        PathBuf::from(p)
    } else {
        app_dir.join(p)
    }
}

/// Top-level command names defined in `./x.yml` in the current directory.
pub fn list_project_commands() -> Result<Vec<String>> {
    let cwd = std::env::current_dir().context("Could not get current directory")?;
    let path = cwd.join("x.yml");
    if !path.is_file() {
        return Ok(Vec::new());
    }
    let app = load(&path)?;
    let mut names: Vec<String> = app.root.subcommands.keys().cloned().collect();
    names.sort();
    Ok(names)
}

/// Returns true if `./x.yml` in CWD defines the given top-level command.
pub fn project_has_command(name: &str) -> Result<bool> {
    let cwd = std::env::current_dir().context("Could not get current directory")?;
    let path = cwd.join("x.yml");
    if !path.is_file() {
        return Ok(false);
    }
    let app = load(&path)?;
    Ok(app.root.subcommands.contains_key(name))
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
    fn loads_dot_commands_with_inline_scripts() {
        let app = p(
            r#"
name: my-app
version: 0.0.0
opts:
  - "[-v | --version]"
.cmd1:
  help: command 1
  args: <arg1> <arg2>
  opts: |
    --bool
    [--str <arg>]
  $: |
    echo "cmd1"
  .subcmd:
    help: subcommand 1
    opts:
      - '--bool'
    $: |
      echo "subcmd1"
"#,
        );
        assert_eq!(app.name, "my-app");
        assert_eq!(app.root.options.len(), 1);
        assert_eq!(app.root.subcommands.len(), 1);
        let cmd1 = &app.root.subcommands["cmd1"];
        assert_eq!(cmd1.description.as_deref(), Some("command 1"));
        assert_eq!(cmd1.arguments.len(), 2);
        assert_eq!(cmd1.options.len(), 2);
        assert_eq!(cmd1.subcommands.len(), 1);
        assert_eq!(
            app.handlers.get("cmd1").map(String::as_str),
            Some("echo \"cmd1\"\n")
        );
        assert_eq!(
            app.handlers.get("cmd1.subcmd").map(String::as_str),
            Some("echo \"subcmd1\"\n")
        );
    }

    #[test]
    fn string_shorthand_for_command() {
        let app = p(
            r#"
name: t
.build: cargo build
"#,
        );
        assert_eq!(
            app.handlers.get("build").map(String::as_str),
            Some("cargo build")
        );
    }

    #[test]
    fn rejects_commands_key() {
        let err = parse("name: x\ncommands:\n  run: {}\n", Path::new("t.x.yml")).unwrap_err();
        assert!(err.to_string().contains("commands:"));
    }

    #[test]
    fn rejects_top_level_dollar_map() {
        let err = parse(
            r#"
name: x
$:
  run: echo
"#,
            Path::new("t.x.yml"),
        )
        .unwrap_err();
        assert!(err.to_string().contains("handler map"));
    }

    #[test]
    fn rejects_plain_key_in_x_yml() {
        let err = parse("build: echo hi\n", Path::new("x.yml")).unwrap_err();
        assert!(err.to_string().contains("dot-prefixed"));
    }

    #[test]
    fn derives_name_from_filename() {
        let app = parse("version: 0.1.0\n", Path::new("widget.x.yml")).unwrap();
        assert_eq!(app.name, "widget");
    }

    #[test]
    fn loads_x_yml_without_name() {
        let app = parse(".hello: echo hi\n", Path::new("x.yml")).unwrap();
        assert_eq!(app.name, "x");
        assert!(app.root.subcommands.contains_key("hello"));
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
.run:
  $: echo inline
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
    fn loads_per_command_dir_and_env() {
        let dir = tempfile::tempdir().unwrap();
        let sub = dir.path().join("sub");
        std::fs::create_dir(&sub).unwrap();
        let app_file = dir.path().join("app.x.yml");
        std::fs::write(
            &app_file,
            r#"
name: envdir
.run:
  dir: ./sub
  env:
    FOO: bar
  $: echo
"#,
        )
        .unwrap();
        let app = load(&app_file).unwrap();
        let run = &app.root.subcommands["run"];
        assert_eq!(run.dir.as_deref(), Some(sub.as_path()));
        assert_eq!(run.env.globals.get("FOO").map(String::as_str), Some("bar"));
    }

    #[test]
    fn loads_alias_command() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("other.x.yml");
        std::fs::write(&target, "name: other\n.run: echo\n").unwrap();
        let app_file = dir.path().join("app.x.yml");
        std::fs::write(
            &app_file,
            r#"
name: app
.alias-cmd:
  alias: ./other.x.yml
"#,
        )
        .unwrap();
        let app = load(&app_file).unwrap();
        let alias_cmd = &app.root.subcommands["alias-cmd"];
        assert_eq!(alias_cmd.alias.as_deref(), Some(target.as_path()));
    }

    #[test]
    fn loads_exapp_example_from_repo() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("docs/examples/app/exapp.x.yml");
        let app = load(&path).unwrap();
        assert_eq!(app.name, "exapp");
        assert!(app.handlers.contains_key("demo.opts"));
    }
}
