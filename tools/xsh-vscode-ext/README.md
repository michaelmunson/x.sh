# x.sh — VSCode Extension

Syntax highlighting and linting for [x.sh](https://github.com/michaelmunson/x.sh) app definition files (`*.x.yml`).

## Features

### Syntax Highlighting

| Element | Highlighted as |
|---|---|
| `name`, `version`, `description` | Metadata keywords |
| `options:`, `arguments:`, `commands:` | Section keywords |
| `$:`, `$.import:` | Handler section keywords |
| Command names | Function entities |
| `--flag`, `-f` | Option flags |
| `<placeholder>` | Parameters |
| `[...]` brackets | Optional grouping |
| `{a\|b\|c}` enums | Enum values |
| `...` | Variadic marker |
| `x-opt`, `x-arg`, `x-usage` | x.sh built-ins |
| Bash/zsh/sh scripts (inline and block `\|` / `>`) | Full embedded shell highlighting |

### Linting

The linter runs on every open, change, and save. Checks mirror the x.sh runtime validator (`src/app/validate.rs`):

| Severity | Check |
|---|---|
| Error | YAML parse errors |
| Error | Missing required `name` field |
| Error | `name` contains invalid characters |
| Error | Both `$:` and `$.import:` in the same file |
| Error | Duplicate handler key in `$:` |
| Error | Leaf command has no `$:` handler |
| Error | Handler key does not match any command |
| Error | Duplicate option or argument on a command |
| Error | Option `requires:` references unknown sibling option |
| Error | Unbalanced `[` / `]` in synopsis expressions |
| Warning | `version` doesn't follow semver |
| Warning | Command missing description _(optional, off by default)_ |
| Hint | Handler value looks like a bare word (missing `x-` or `\|`) |

## File Association

Files matching `*.x.yml` are automatically recognized (primary format).

The root task file `x.yml` is recognized separately (**x.sh script** language): top-level
command keys map to inline scripts, and nested groups use `$` (not root `$:`) as the
default when invoked without a subcommand.

Legacy `*.cli.yaml` / `*.cli.yml` extensions are also supported.

Any YAML file with `# x.sh` as the first line is treated like `x.yml` (local
scripts / handler imports), not as an app definition:

```
# x.sh
build: |
  npm run build
```

## Settings

```jsonc
{
  // Enable/disable all linting
  "xsh.lint.enabled": true,

  // Report when a leaf command has no $ handler
  "xsh.lint.warnOnMissingHandlers": true,

  // Report when a $ key has no matching command
  "xsh.lint.warnOnUnreferencedHandlers": true,

  // Require description on all commands
  "xsh.lint.requireDescription": false
}
```

## Building & Installing

```bash
npm install
npm run compile

# Package as .vsix
npm run package

# Install locally
code --install-extension xsh-vscode-ext-0.0.1.vsix
```

## Option Expression Syntax

The extension understands the full option expression grammar:

```
[-v | --version]            # short/long flag alternative
[--output=<dir>]            # flag with placeholder value
[--mode={fast|safe|deep}]   # flag with enum value
[--input=<file> [--output=<dir>]]  # nested optional (requires chain)
--commit                    # required bare flag
```

## Handler Built-ins

The following x.sh built-ins receive special highlighting inside bash scripts:

- `x-opt <name>` — expand an option value
- `x-arg <name>` — expand an argument value
- `x-usage [command]` — print usage/help
