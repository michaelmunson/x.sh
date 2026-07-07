---
name: x-app
description: >-
  Design, author, and debug `x` files - YAML-defined multi-command CLIs using
  dot-prefixed `.command:` keys, inline `$:` scripts, synopsis DSL, and
  built-in validation. Covers both `<name>.x.yml` apps and project-local
  `x.yml` (same syntax). Use when creating or editing these files, working
  with `$.import`/`import.$`, `alias:`, synopsis strings, or `x-opt`/`x-arg`/
  `x-io-*` builtins, or running `x -i --app`.
---

# `x` File Syntax Skill

An **x file** (`<name>.x.yml`, or the project-local `x.yml`) becomes a
multi-command CLI. `x` parses options and arguments from synopsis strings,
validates user input, then runs a matching **inline bash script** (or an
imported handler, or an `alias:` redirect to another x file).

- App: `x <name> [subcommands…] [options] [args…]`
- Project-local: `x <command> [subcommands…] [options] [args…]` (no name prefix)

Both share **exactly the same syntax** described below. The differences are
purely about *where the file lives* and *how it's invoked* - see
[skill.md](skill.md) for resolution/scope details.

## When to build one

Reach for the full `.command:` syntax (in either an app or `x.yml`) when you need:

- Nested subcommands with auto-generated `-h` / `--help`
- Typed flags, positional args, defaults, choices, or `requires:` chains
- Pre-run validation (unknown flags, missing required args, bad choices)
- Interactive prompts (`x-io-read`, `x-io-confirm`, `x-io-select`)
- Per-command working directory (`dir:`) or environment (`env:`)
- A reusable CLI shared across a repo or globally

## File locations

| Scope | Path | Create with |
|-------|------|-------------|
| Project-local commands | `./x.yml` | edit directly, or `x -i --app --local` style is not needed - just create the file |
| Local app | `./<name>.x.yml` | `x -i --app --local <name>` |
| Global app | `~/.x.sh/apps/<name>.x.yml` | `x -i --app --global <name>` |

**Resolution** when running `x <name>`: `./x.yml` command named `<name>` wins
first, then walk from CWD up through parent directories for `<name>.x.yml`,
then check `~/.x.sh/apps/`. Nearest file wins.

Locate a file: `x --src <name>`

App names: letters, numbers, dashes, underscores. Saved as `<name>.x.yml`.
`name:` is optional - it defaults to the filename without the `.x.yml`/`.yml`
suffix if omitted.

## Create and edit

```bash
x -i --app my-app              # prompts: local vs global
x -i --app --local my-app      # ./my-app.x.yml
x -i --app --global my-app     # ~/.x.sh/apps/my-app.x.yml
```

Flow: seed template → open `$EDITOR` → validate → on failure offer **Edit config**
or **Revert**. Every `x <app>` run also re-validates the file.

## File structure

Commands are declared with **dot-prefixed keys** (`.name:`) - never
`commands:` (removed in v3). A command's value is either:

- a **string** - shorthand for `{ $: <string> }` (inline script)
- a **mapping** with any of: `description`/`help`, `options`/`opts`,
  `arguments`/`args`, `dir`, `env`, `alias`, `$`, and nested `.sub-command:` keys

```yaml
name: my-app
version: 0.0.0
description: short summary
import:
  $:
    - handlers/test.yml
  env: .env.example

env:
  .env1:
    - MY_NAME: "env1"
  HELLO: "global"

# Root-level options/args apply when invoking `x my-app …` directly
options:
  - "[-v | --version]"
arguments:
  - "[<topic='overview'>]"

$: |
  echo "root handler - topic=$(x-arg topic)"

.get-env:
  description: print the environment variable
  $: |
    echo "HELLO = $HELLO"
    x-env-load .env1
    echo "MY_NAME = $MY_NAME"

.run-test:
  description: run a test   # handler comes from the imported handlers/test.yml

.build:
  description: build the project
  options:
    - "[--mode={fast|safe|deep}]"
  arguments:
    - "<assets>..."
  $: |
    echo "mode=$(x-opt mode), assets=$(x-arg assets)"

.create:
  description: create things
  $: x-usage create        # non-leaf default: print help when called bare

  .file:
    description: create a file
    arguments: "<path> [<content='empty'>]"
    $: |
      echo "creating $(x-arg path)"

  .folder:
    arguments: "<path>"
    $: |
      mkdir -p "$(x-arg path)"
```

### Top-level keys

| Key | Purpose |
|-----|---------|
| `name` | App name (falls back to filename without `.x.yml`/`.yml`) |
| `version`, `description`/`help` | Metadata shown in help (`help`/`description` are aliases - pick one) |
| `options`/`opts`, `arguments`/`args` | Root command synopsis - string, multiline string, or list of strings (aliases - pick one of each pair) |
| `dir` | Working directory for the root handler (and all commands, unless overridden) |
| `env` | Global vars and `.group` named sets |
| `import` | Unified imports: `$` (handler YAML files), `env` (`.env` files), `sh` (scripts to source) |
| `$` | Root command's inline script (runs on bare `x my-app` invocation) |
| `$.import` | Legacy handler imports (use `import.$` instead) |
| `.command-name` | A subcommand - string shorthand or a command mapping (see below) |

**Removed in v3** (clean break, no fallback): `commands:` mapping, and a
top-level `$:` used as a handler map (`$:` is now only ever the root's own
inline script string).

### Command mapping keys

| Key | Purpose |
|-----|---------|
| `description`/`help` | One-line description shown in help (aliases - pick one) |
| `options`/`opts`, `arguments`/`args` | This command's synopsis (aliases - pick one of each pair) |
| `dir` | Working directory for this command's handler and its subcommands (relative to the file, or absolute) |
| `env` | Environment for this command's handler and its subcommands (merges over parent's) |
| `alias` | Path to another x file; remaining args are dispatched there (mutually exclusive with everything except `help`/`description`) |
| `$` | This command's inline bash script |
| `.sub-command` | A nested subcommand (same shape, recursively) |

**`$:` and `import.$` / `$.import` may be combined** - inline `$:` overrides
an imported handler with the same dotted-path key.

### Multiline `opts`/`args` shorthand

`opts`/`options` and `args`/`arguments` accept a single multiline string as an
alternative to a YAML list - each non-empty line becomes one synopsis fragment:

```yaml
opts: |
  [-n | --dry-run]
  [-o | --out <path>]
  --commit
```

is equivalent to:

```yaml
options:
  - "[-n | --dry-run]"
  - "[-o | --out <path>]"
  - "--commit"
```

### Environment

```yaml
import:
  env: ./.env

env:
  HELLO: global
  .env1:
    - MY_NAME: env1
```

Global vars from `.env` imports and inline `env:` are exported before the
handler runs (inline wins on duplicate keys). Named groups load on demand:

```bash
echo "$HELLO"
x-env-load .env1
echo "$MY_NAME"
```

Per-command `dir:`/`env:` merge downward: a subcommand inherits its parent's
`dir`/`env` unless it defines its own (its own values win, but parent
`env` vars not overridden still apply).

Reference: `docs/examples/app/simple.x.yml`.

### Alias commands - dispatch to another x file

```yaml
.legacy:
  alias: ./tools/legacy.x.yml
```

`x my-app legacy <rest…>` runs `<rest…>` against `legacy.x.yml` as if you'd
run it directly (as its own root command). An alias command may define
**only** `alias` and `help`/`description` - no `options`, `arguments`, `dir`,
`env`, `$`, or nested `.sub-command` keys.

### Script shorthand

```yaml
.docs: cargo doc --no-deps      # shorthand for { $: cargo doc --no-deps }
```

`x` treats any string value on a `.command:` key as its inline script - no
need to write `$: cargo doc --no-deps` for one-liners.

### Split scripts with `import` or `$.import`

```yaml
import:
  $:
    - ./handlers/build.yml
    - ./handlers/create.yml
```

Legacy (still supported, mutually exclusive with `import.$`):

```yaml
$.import:
  - ./handlers/build.yml
  - ./handlers/create.yml
```

Each import file is a flat YAML map of `dotted.path: <bash body>` - same
dotted-path keys as before v3 (these files are **not** rewritten to
dot-prefixed keys; they're just plain script lookups). Paths resolve relative
to the x file. Duplicate keys across imports are an error; inline `$:`
entries override imported scripts with the same key.

Reference: `docs/examples/app/exapp.x.yml` + `exapp.handlers-a.yml` / `exapp.handlers-b.yml`.

## Synopsis DSL

Synopsis strings live in `options`/`opts` and `arguments`/`args` (per command
or at root). Use a list, or a multiline string (one fragment per line), or -
for a single fragment - an inline string.

All `[optional]` forms have bare `(required)` equivalents - drop the brackets
(or use parenthesised option groups where shown).

### Arguments

| Synopsis | Meaning | Example |
|----------|---------|---------|
| `<name>` | Required positional | `x app file.txt` |
| `[<name>]` | Optional positional | `x app` or `x app file.txt` |
| `[<name='val'>]` | Optional with default | `x app` or `x app myfile.txt` |
| `<name>...` | Required repeating (greedy; last) | `x app f1.txt f2.txt` |
| `[<name>...]` | Optional repeating | `x app` or `x app f1.txt f2.txt` |
| `<name={a\|b\|c}>` | Required, value from set | `x app write` |
| `[<name={a\|b\|c}>]` | Optional, value from set | `x app` or `x app fast` |
| `(north\|south\|…)` | Required choice; arg name is `choice` | `x app north` |

### Options

| Synopsis | Meaning | Example |
|----------|---------|---------|
| `--long` | Required bool flag | `x app --long` |
| `(-l \| --long)` | Required bool, alias pair | `x app -l` |
| `(--long \| --short)` | Required mutually exclusive | `x app --long` or `x app --short` |
| `(-l \| --long \| -s \| --short)` | Required mutex with aliases | `x app -l` or `x app -s` |
| `[-s \| --long]` | Optional bool flag | `x app -l` |
| `[-s \| --long <arg>]` | Flag with required value | `x app -l ./dist` |
| `[-s \| --long <arg='v'>]` | Flag value with default | `x app` or `x app --long ./dist` |
| `[-s \| --long <arg> ...]` | Repeating values for one flag | `x app --long "a" "b"` |
| `[-s \| --long <arg>]...` | Flag repeats | `x app --long abc --long xyz` |
| `[--long=<arg>]` | Value via `=` | `x app --long=./dist` |
| `[--long=<arg='v'>]` | `=` form with default | `x app` or `x app --long=./dist` |
| `[--long={a\|b\|c}]` | Value from set | `x app --long=a` |
| `[--input=<a> [--output=<b>]]` | `--output` requires `--input` | `x app --input=foo --output=bar` |

### YAML tips

- Quote synopsis strings that start with `[` or contain `{` to avoid YAML parsing issues.
- YAML anchors work for shared option lists (see `exapp.x.yml` `&demo_verbose`).
- `args`/`arguments` may be a single string: `args: "<path> [<content='empty'>]"`.

`x` enforces synopsis rules **before** any handler runs.

## Handler builtins

Injected via bash preamble before each script body:

| Builtin | Purpose |
|---------|---------|
| `x-opt <name>` | Option value; bool flags → `true`; repeats → one value per line |
| `x-arg <name>` | Positional value; repeats → one per line |
| `x-opts <assoc>` | Fill caller-named bash assoc array with all options |
| `x-args <assoc>` | Fill caller-named bash assoc array with all args |
| `x-usage <cmd.path>` | Print auto-generated help (e.g. `x-usage create.file`) |
| `x-io-read …` | Prompt for a line; `-v NAME` assigns global scalar |
| `x-io-confirm …` | Yes/no; `--default yes\|no` (default `no`) |
| `x-io-select …` | Menu of `id=label` pairs; `--multi` for indexed array |
| `x-prt …` | Styled print; `(-s\|--style) <style>` with comma-separated names (`red`, `bold`, `bg-white`, …) |
| `x-tui …` | Terminal control (`--init`, `--exit`, `--clear`, cursor moves, …) |
| `x-path-root` | Print the x file's containing directory |

### Reading values

```bash
mode=$(x-opt mode)
path=$(x-arg path)

# repeating option
mapfile -t dirs < <(x-opt dir)

# repeating positional
mapfile -t items < <(x-arg items)

# all options at once
declare -A opts
x-opts opts
echo "${opts[mode]}"
```

Bool flags: compare `[[ $(x-opt verbose) == true ]]`.

Env vars are also set: `X_OPT_<name>`, `X_ARG_<name>` (hyphens → underscores),
`X_OPTS_PAIRS`, `X_ARGS_PAIRS`, `X_BIN`, `X_APP`, `X_APP_FILE`.

### Interactive I/O

```bash
x-io-read "Name:" -v name
echo "$name"

x-io-confirm "Delete?" --default no -v ok
[[ "$ok" == true ]] && rm -f "$file"

x-io-select "Color" "red=Red" "green=Green" -v color
x-io-select --multi -v colors "Pick colors" "red=Red" "blue=Blue"
echo "${colors[0]}"
```

### Styling output

```bash
x-prt --style red,underline,bg-white "Hello" --style blue,bg-yellow " World" --newline
x-prt --style green,bold "app prt demo"
x-tui --init --clear
x-tui --home
x-tui --exit
```

## Help

Every command gets `-h` / `--help` automatically:

```bash
x my-app --help
x my-app build --help
x my-app create file --help
```

Non-leaf commands **without** their own `$:` auto-print help when invoked
bare. For explicit help-only leaves, set the script to `x-usage <path>`.

## Validation rules

Checked on `x -i --app` save and every `x <app>`/project `x.yml` invocation:

| Rule | Error |
|------|-------|
| `commands:` used anywhere | `` `commands:` was removed in v3 `` |
| Top-level `$:` used as a mapping | `` top-level `$:` handler map was removed in v3 `` |
| Every **leaf** command has a `$:` script or `alias:` | `leaf command has no script` |
| Every script key (inline or imported) matches a command path | `script key … does not match any defined command` |
| Both `help:`/`description:`, `options:`/`opts:`, or `arguments:`/`args:` on one node | `cannot use both …` |
| Alias command defines anything besides `alias`/`help`/`description` | (alias commands may only define those) |
| No duplicate `--long` or `-s` on same command | `duplicate option` |
| No duplicate positional names | `duplicate argument` |
| `requires:` references a defined sibling option | `requires --foo, but --foo is not defined` |
| No duplicate keys across imports | `duplicate handler key` |
| Both `$.import` and `import.$` present | `cannot use both` |

Fix with `x -i --app [--local|--global] <name>` or edit the YAML directly, then
run `x <app> --help` to confirm.

## Example
- this is an example from the official x.sh repository
```yml
name: xpkg
version: 0.0.0
description: x package meta app
env:
  HELLO: "world"

.build:
  help: build the binary/docs
  $: |
    x xpkg build bin
    x xpkg build tool

  .bin:
    description: build the binary
    $: |
      cargo build --release

  .tool:
    description: build the tools
    options:
      - '[--cursor]'
      - '[--vscode]'
      - '[--compile <test>]'
    $: |
      
      if ! ls Cargo.toml >/dev/null 2>&1; then
        echo "Error: Wrong Directory. Please navigate to the root of the project."
        exit 1
      fi
      x-prt --style yellow,bold "Removing existing extension" && echo
      rm tools/xsh-vscode-ext/xsh-vscode-ext-0.0.1.vsix
      x-prt --style green,bold "Building extension" && echo
      pushd tools/xsh-vscode-ext
      if [ "$(x-opt "compile")" = "true" ]; then
        npm run compile
      else
        npm install
        npm run compile
        npm run package
        if [ "$(x-opt "cursor")" = "true" ]; then
          npm run install-cursor-extension
        elif [ "$(x-opt "vscode")" = "true" ]; then
          npm run install-vscode-extension
        else
          echo "No extension selected, skipping installation"
        fi
      fi
      popd

      

.test:
  description: run the tests
  arguments: >
    [<test>]
  options: |
    [--complete]
    [--integration]
    [--test <test>]
  $: |
    if [ -n "$(x-arg test)" ]; then
      cargo test $(x-arg test)
    elif [ "$(x-opt "test")" = "true" ]; then
      cargo test --test $TEST
    elif [ "$(x-opt "complete")" = "true" ]; then
      cargo test complete:: # this is a test for the complete module
    elif [ "$(x-opt "integration")" = "true" ]; then
      cargo test cli_integration::complete # this is a test for the integration tests
    else
      cargo test --bin x # this is a test for the x binary
    fi
```
