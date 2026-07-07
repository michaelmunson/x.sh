//! Typed spec model for an `x` app, built from a `<name>.x.yml` file.
//!
//! The synopsis DSL strings in `options:` / `arguments:` are parsed in
//! [`crate::app::synopsis`] into `OptionDef` / `ArgDef` values. Handler bodies
//! from the `$:` block (or imported handler files) are stored on [`App::handlers`] keyed by
//! the dotted command path (root = `""`, nested = `"create.file"`).

use std::collections::BTreeMap;
use std::path::PathBuf;

/// What kind of value, if any, an option takes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueKind {
    /// Boolean flag — no value (e.g. `[-v | --version]`).
    None,
    /// One value required when the option is given. The string is the
    /// placeholder name the spec used (e.g. `<file>`), used for help output.
    Required(String),
    /// One value optional when the option is given. Same placeholder semantics.
    Optional(String),
}

/// A single option definition (a flag or a flag-with-value).
#[derive(Debug, Clone)]
pub struct OptionDef {
    pub short: Option<char>,
    pub long: Option<String>,
    pub takes_value: ValueKind,
    /// Default value (literal string, used when option is omitted).
    pub default: Option<String>,
    /// If set, the value must be one of these.
    pub choices: Option<Vec<String>>,
    /// `...` form — option may be repeated, values accumulate.
    pub repeats: bool,
    /// Long names of other options that must be provided alongside this one.
    /// Derived from nested-bracket syntax: `[--input=<a> [--output=<b>]]`
    /// records `requires: ["input"]` on `output`.
    pub requires: Vec<String>,
    /// True if the option must appear (top-level required form).
    pub required: bool,
    /// Short description (currently always empty; reserved for future use).
    pub description: Option<String>,
}

impl OptionDef {
    /// The canonical name for this option (long if present, else short).
    pub fn canonical_name(&self) -> String {
        if let Some(long) = &self.long {
            long.clone()
        } else if let Some(short) = self.short {
            short.to_string()
        } else {
            String::new()
        }
    }
}

/// Mutually exclusive option group — exactly one member must be provided when
/// `required` is true (e.g. `(--long | --short)`).
#[derive(Debug, Clone)]
pub struct OptionGroupDef {
    /// Canonical names of member options (long if present, else short char).
    pub members: Vec<String>,
    /// When true, exactly one member must appear on the command line.
    pub required: bool,
}

/// A single positional argument definition.
#[derive(Debug, Clone)]
pub struct ArgDef {
    pub name: String,
    pub required: bool,
    pub default: Option<String>,
    /// `...` form — argument may repeat.
    pub repeats: bool,
    /// If set, the value must be one of these.
    pub choices: Option<Vec<String>>,
}

/// A command (root or nested subcommand).
#[derive(Debug, Clone)]
pub struct Command {
    #[allow(dead_code)]
    pub name: String,
    pub description: Option<String>,
    pub options: Vec<OptionDef>,
    pub option_groups: Vec<OptionGroupDef>,
    pub arguments: Vec<ArgDef>,
    pub subcommands: BTreeMap<String, Command>,
    pub dir: Option<PathBuf>,
    pub env: AppEnv,
    pub alias: Option<PathBuf>,
}

impl Command {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            options: Vec::new(),
            option_groups: Vec::new(),
            arguments: Vec::new(),
            subcommands: BTreeMap::new(),
            dir: None,
            env: AppEnv::default(),
            alias: None,
        }
    }
}

/// Environment configuration for an app.
#[derive(Debug, Clone, Default)]
pub struct AppEnv {
    /// Applied to the shell before handler runs (import + inline, inline wins).
    pub globals: BTreeMap<String, String>,
    /// Named groups keyed without leading `.` (e.g. `"env1"`). Inline only.
    pub groups: BTreeMap<String, BTreeMap<String, String>>,
}

/// A fully loaded and parsed application spec.
#[derive(Debug, Clone)]
pub struct App {
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    /// Root command. The app name is treated as the root command's name.
    /// Carries the top-level `dir:` and `env:` values.
    pub root: Command,
    /// Map from dotted command path (e.g. `""`, `"create"`, `"create.file"`)
    /// to bash handler body.
    pub handlers: BTreeMap<String, String>,
    /// Bash/zsh scripts sourced before each handler runs (`import.sh`, in order).
    pub sh_imports: Vec<PathBuf>,
}
