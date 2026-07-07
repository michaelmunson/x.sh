# x.sh - VSCode Extension

Syntax highlighting and linting for [x.sh](https://github.com/michaelmunson/x.sh) app definition files (`*.x.yml`) and project-local scripts (`x.yml`), which share the same v3 syntax.

## Features

### Syntax Highlighting

| Element                                  | Highlighted as           |
| ----------------------------------------- | ------------------------ |
| `name`, `version`, `description`/`help`  | Metadata keywords        |
| `.command-name:`                          | Command entities         |
| `options:`/`opts:`, `arguments:`/`args:` | Section keywords         |
| `dir:`, `env:`, `alias:`                  | Command property keywords |
| `$:`                                      | Inline script keyword    |
| `--flag`, `-f`                            | Option flags             |
| `<placeholder>`                           | Parameters                |
| `[...]` brackets                          | Optional grouping         |
| `{a\|b}`                                  | Enum choice                |
| `...`                                     | Variadic marker           |
| `x-opt`, `x-arg`, `x-usage`               | x.sh built-ins            |
| Bash/zsh/sh scripts (inline and block `\|`/`>`) | Embedded shell     |

### Linting

The linter runs on every open, change, and save. Checks mirror the x.sh runtime loader/validator (`src/app/loader.rs`, `src/app/validate.rs`):

| Severity | Check                                                        |
| -------- | ------------------------------------------------------------ |
| Error    | YAML parse errors                                             |
| Error    | `commands:` used (removed in v3 - use `.command-name:` keys)  |
| Error    | Top-level `$:` used as a handler map (removed in v3)          |
| Error    | `name` contains invalid characters                            |
| Error    | Both `$:` and `$.import:` in the same file                    |
| Error    | Both `help:`/`description:`, `options:`/`opts:`, or `arguments:`/`args:` on the same node |
| Error    | Unknown key on the document root or a command                 |
| Error    | Leaf command has no inline `$:` script (and no `alias:`)       |
| Error    | Alias command defines more than `alias:` and `help:`/`description:` |
| Error    | Duplicate option or argument on a command                     |
| Error    | Option `requires:` references unknown sibling option          |
| Error    | Unbalanced `[` / `]` in synopsis expressions                   |
| Warning  | `version` doesn't follow semver                                |
| Warning  | Command missing description *(optional, off by default)*       |

## File Association

Files matching `*.x.yml` are recognized as apps (**x.sh app** language). The project-local
`x.yml` file is recognized separately (**x.sh script** language) but shares the exact same
grammar and schema - the only difference is that apps need their name specified to run
them (`x <app> <command>` vs `x <command>`).

Legacy `*.cli.yaml` / `*.cli.yml` extensions are also supported.

Any YAML file with `# x.sh` as the first line is treated as an x.sh file too.

## Settings

```jsonc
{
  // Enable/disable all linting
  "xsh.lint.enabled": true,

  // Report when a leaf command has no inline $ script (and no alias)
  "xsh.lint.warnOnMissingHandlers": true,

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

## Command Definition Syntax

Commands are declared with a `.` prefix. A command is either an inline script string
(shorthand for `$:`) or a mapping:

```yaml
.build:
  help: build the project           # alternative: description
  args: <target>                    # alternative: arguments
  opts: |                           # multiline string, split on newlines
    [--release]
    [--target=<triple>]
  $: |
    cargo build $(x-opt release && echo --release)

  .docs: cargo doc --no-deps        # shorthand: string value is the inline script

.alias-cmd:
  alias: ./path/to/x.yml            # dispatch remaining args to another x file
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

- `x-opt <name>` - expand an option value
- `x-arg <name>` - expand an argument value
- `x-usage [command]` - print usage/help
