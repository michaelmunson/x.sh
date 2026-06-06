---
name: x-app
description: >-
  Design, author, and debug `x` apps — YAML-defined multi-command CLIs in
  `<name>.x.yml` files with synopsis DSL, bash handlers, and built-in validation.
  Use when creating or editing `.x.yml` apps, app handlers, `$.import`, synopsis
  strings, `x-opt`/`x-arg`/`x-io-*` builtins, or running `x -i --app`.
---

# `x` App Framework Skill

An **app** is a single `<name>.x.yml` file that becomes a multi-command CLI.
`x` parses options and arguments from synopsis strings, validates user input,
then runs a matching **bash handler** from the `$:` block (or imported files).

Invoke: `x <name> [subcommands…] [options] [args…]`

## When to build an app

Build an app (not `./x.yml` or a global script) when you need:

- Nested subcommands with auto-generated `-h` / `--help`
- Typed flags, positional args, defaults, choices, or `requires:` chains
- Pre-run validation (unknown flags, missing required args, bad choices)
- Interactive prompts (`x-io-read`, `x-io-confirm`, `x-io-select`)
- A reusable CLI shared across a repo or globally

Use `./x.yml` for a handful of inline shell one-liners without arg parsing.

## File locations

| Scope | Path | Create with |
|-------|------|-------------|
| Local (project) | `./<name>.x.yml` | `x -i --app --local <name>` |
| Global (personal) | `~/.x.sh/apps/<name>.x.yml` | `x -i --app --global <name>` |

**Resolution** when running `x <name>`: walk from CWD up through parent
directories for `<name>.x.yml`, then check `~/.x.sh/apps/`. Nearest file wins.

Note: `./x.yml` entries take priority over apps with the same top-level name.

Locate a file: `x --src <name>`

App names: letters, numbers, dashes, underscores. Saved as `<name>.x.yml`.

## Create and edit

```bash
x -i --app my-app              # prompts: local vs global
x -i --app --local my-app      # ./my-app.x.yml
x -i --app --global my-app     # ~/.x.sh/apps/my-app.x.yml
```

Flow: seed template → open `$EDITOR` → validate → on failure offer **Edit config**
or **Revert**. Every `x <app>` run also re-validates the file.

## File structure

```yaml
name: my-app
version: 0.0.0
description: short summary

# Root-level options/args apply when invoking `x my-app …` directly
options:
  - "[-v | --version]"
arguments:
  - "[<topic='overview'>]"

commands:
  build:
    description: build the project
    options:
      - "[--mode={fast|safe|deep}]"
    arguments:
      - "<assets>..."

  create:
    description: create things
    commands:
      file:
        description: create a file
        arguments: "<path> [<content='empty'>]"
      folder:
        arguments: "<path>"

# Handlers — bash bodies keyed by dotted command path
$:
  "": |
    echo "root handler — topic=$(x-arg topic)"
  build: |
    echo "mode=$(x-opt mode), assets=$(x-arg assets)"
  create: x-usage create
  create.file: |
    echo "creating $(x-arg path)"
  create.folder: |
    mkdir -p "$(x-arg path)"
```

### Top-level keys

| Key | Purpose |
|-----|---------|
| `name` | App name (falls back to filename without `.x.yml`) |
| `version`, `description` | Metadata shown in help |
| `options`, `arguments` | Root command synopsis (string or list of strings) |
| `commands` | Nested subcommand tree |
| `$:` | Inline handler map |
| `$.import` | List of external YAML files with handler maps |

**`$:` and `$.import` are mutually exclusive.**

### Handler keys

| Key | Runs when |
|-----|-----------|
| `""` (empty string) | User invokes `x my-app` with no subcommand |
| `build` | `x my-app build …` |
| `create.file` | `x my-app create file …` |

Handler values are either:

- **Multi-line bash** (`\|`) — most common
- **One-liner** — e.g. `create: x-usage create` prints help for that group

**All handlers are bash.** They call external tools (`cargo`, `npm`, etc.) as needed.

### Split handlers with `$.import`

```yaml
$.import:
  - ./handlers/build.yml
  - ./handlers/create.yml
```

Each import file is a flat YAML map of `dotted.path: <bash body>`. Paths resolve
relative to the app file. Duplicate keys across imports are an error.

Reference: `docs/examples/app/exapp.x.yml` + `exapp.handlers-a.yml` / `exapp.handlers-b.yml`.

## Synopsis DSL

Synopsis strings live in `options:` and `arguments:` (per command or at root).
Each entry is one fragment; use a list for multiple items or a single string for
a combined positional spec.

### Options

| Synopsis | Meaning |
|----------|---------|
| `[-s \| --long]` | optional bool flag |
| `[-s \| --long <arg>]` | flag with required value |
| `[-s \| --long <arg='v'>]` | flag value with default when flag is given |
| `[-s \| --long <arg> ...]` | repeating flag values |
| `[--long={a\|b\|c}]` | choice from set |
| `[--long=<arg='v'>]` | `=` form with default |
| `[--input=<a> [--output=<b>]]` | `--output` requires `--input` (`requires:` chain) |
| `--long` | **required** option (no brackets) |

### Positional arguments

| Synopsis | Meaning |
|----------|---------|
| `<name>` | required positional |
| `[<name>]` | optional positional |
| `[<name='val'>]` | optional with default |
| `<name>...` / `[<name>...]` | repeating (greedy; must be last positional) |
| `(north\|south\|east\|west)` | required choice; arg name is `choice` |

### YAML tips

- Quote synopsis strings that start with `[` or contain `{` to avoid YAML parsing issues.
- YAML anchors work for shared option lists (see `exapp.x.yml` `&demo_verbose`).
- `arguments` may be a single string: `arguments: "<path> [<content='empty'>]"`.

`x` enforces synopsis rules **before** any handler runs.

## Handler builtins

Injected via bash preamble before each handler body:

| Builtin | Purpose |
|---------|---------|
| `x-opt <name>` | Option value; bool flags → `true`; repeats → one value per line |
| `x-arg <name>` | Positional value; repeats → one per line |
| `x-opts <assoc>` | Fill caller-named bash assoc array with all options |
| `x-args <assoc>` | Fill caller-named bash assoc array with all args |
| `x-run <cmd> …` | Run a command with `x-*` helpers still exported |
| `x-usage <cmd.path>` | Print auto-generated help (e.g. `x-usage create.file`) |
| `x-io-read …` | Prompt for a line; `-v NAME` assigns global scalar |
| `x-io-confirm …` | Yes/no; `--default yes\|no` (default `no`) |
| `x-io-select …` | Menu of `id=label` pairs; `--multi` for indexed array |

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

## Help

Every command gets `-h` / `--help` automatically:

```bash
x my-app --help
x my-app build --help
x my-app create file --help
```

Non-leaf commands **without** a handler auto-print help when invoked bare.
For explicit help-only leaves, set handler to `x-usage <path>`.

## Validation rules

Checked on `x -i --app` save and every `x <app>` invocation:

| Rule | Error |
|------|-------|
| Every **leaf** command has a `$:` handler | `leaf command has no handler` |
| Every handler key matches a command path | `handler key … does not match` |
| No duplicate `--long` or `-s` on same command | `duplicate option` |
| No duplicate positional names | `duplicate argument` |
| `requires:` references a defined sibling option | `requires --foo, but --foo is not defined` |
| No duplicate keys in `$:` or across `$.import` | `duplicate handler key` |
| Both `$:` and `$.import` present | `cannot use both` |

Fix with `x -i --app [--local|--global] <name>` or edit the YAML directly, then
run `x <app> --help` to confirm.

## Agent workflow

When authoring or modifying an app:

1. **Pick scope** — local for repo tooling (`xpkg.x.yml`), global for personal utilities.
2. **Design the command tree** — sketch subcommands before writing handlers.
3. **Declare synopsis first** — options/arguments on the right command node (root vs leaf).
4. **Add handlers last** — one `$:` key per leaf; use `x-usage` for non-leaf groups.
5. **Keep handlers thin** — parse with `x-opt`/`x-arg`, delegate to `cargo`, `npm`, etc.
6. **Split large apps** — use `$.import` when handlers grow beyond ~100 lines.
7. **Validate** — `x <app> --help` for each new subcommand; run a happy-path invocation.

### Real-world pattern (this repo)

`xpkg.x.yml` — nested `build docs|bin` and `test`, uses `x-opt` for `--start`:

```yaml
commands:
  build:
    commands:
      docs:
        options: ['[--start]']
      bin: {}
  test: {}

$:
  build: |
    x xpkg build docs
    x xpkg build bin
  build.docs: |
    pushd docs/pages && npm install
    if [ "$(x-opt start)" = "true" ]; then npm run start; else npm run build; fi
    popd
  build.bin: cargo build --release
  test: cargo test --bin x
```

## Pitfalls

- **Handler key typos** — `create.file` must match the nested `commands:` path exactly.
- **Missing root handler** — bare `x my-app` needs `$:` key `""` if root has no subcommands-only design.
- **Greedy repeats** — `<rest>...` must be the last positional in the synopsis.
- **Requires chains** — `--dst` nested inside `--src` means `--dst` alone is rejected.
- **Not `x.yml`** — apps use `<name>.x.yml`; `x.yml` is a separate, simpler mechanism.
- **CWD matters for local apps** — a local app in a parent dir is found when running from a child dir, but edit the file at its actual path.

## References

- App framework docs: [README.md](../../README.md#app-framework)
- Full synopsis demo: [docs/examples/app/exapp.x.yml](../examples/app/exapp.x.yml)
- General `x` CLI (scripts, `x.yml`, install): [docs/skills/skill.md](skill.md)
