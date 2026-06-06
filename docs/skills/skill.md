---
name: x-cli
description: >-
  Create, run, and maintain scripts and multi-command CLIs with the `x` tool
  (global scripts, project-local `x.yml`, and `*.x.yml` apps). Use when the user
  mentions `x`, `x.sh`, `x.yml`, `.x.yml` apps, `x -i`, `x --init`, app handlers,
  synopsis DSL, or wants to add project commands or personal shell utilities.
---

# `x` CLI Skill

`x` is a Rust CLI for managing personal scripts and YAML-defined command trees.
Global state lives under `~/.x.sh/`. Project-local commands live in `./x.yml` or
`./<name>.x.yml`.

## When to use what

| Need | Use | File / location |
|------|-----|-----------------|
| Reusable personal script (any language) | `x -i <name>` | `~/.x.sh/scripts/<name>` + `~/.x.sh/metadata/<name>.toml` |
| Simple project commands, no arg parsing | `./x.yml` | Inline bash (or default program) in repo root |
| Structured CLI with flags, help, validation | `x -i --app [--local\|--global] <name>` | `./<name>.x.yml` or `~/.x.sh/apps/<name>.x.yml` |
| Run without `x` prefix | `x --ln <name> [alias]` | Symlink in `~/.local/bin/` |

**Prefer `x.yml`** for a few repo-specific one-liners. **Prefer an app** when you
need options, positional args, nested subcommands, auto-generated `--help`, or
interactive prompts (`x-io-*`).

## Command resolution

When the user runs `x <name> [args…]`, `x` resolves in this order:

1. **`./x.yml`** top-level key `<name>` (current directory only)
2. **App** `<name>.x.yml` — walk from CWD up through parent directories, then `~/.x.sh/apps/`
3. **Global script** `~/.x.sh/scripts/<name>`

`x --src <name>` skips `x.yml` and returns the path to an app or global script.

Invalid `./x.yml` YAML fails immediately; it does not fall back to global scripts.

## Core commands

```bash
x <name> [args…]          # run script, local x.yml entry, or app
x -i [name] [content]     # create/edit global script (opens $EDITOR)
x -i --ai [name]          # generate script via configured LLM
x -l | --ls               # list global scripts (with metadata/activity)
x -d | --delete <name>    # remove global script + metadata
x --ln <name> [alias]     # symlink into ~/.local/bin/
x -d --ln <name>          # remove symlink only
x --src <name>            # print absolute path to app or script file
x --config                # default language + LLM provider
x --ai                    # LLM one-shot shell command (requires LLM config)
x -i --app [--local|--global] [name]   # create/edit app YAML
```

Script names: letters, numbers, dashes only. App names also allow underscores.

## Global layout

```
~/.x.sh/
├── scripts/          # script files (no extension required)
├── metadata/         # <name>.toml per script (description, program, groups)
├── apps/             # global <name>.x.yml apps
├── config.json       # default_program, etc.
├── config/llm.sh     # LLM hook: receives prompt as $1, prints to stdout
└── metadata.json     # activity timestamps for global scripts
```

Default interpreter for inline/`x.yml` scripts: `default_program` from
`x --config`, usually `bash`.

## Project-local `x.yml`

Only read from `./x.yml` in the **current working directory**.

Each top-level key is a command. Values:

- **String** — inline script; args after the command become `$1`, `$2`, …
- **Mapping** — nested subcommands; `$` is the default when no subcommand is given

```yaml
build: |
  VERSION="$1"
  npm run build
  npm version "$VERSION"

deploy:
  $: |
    echo "Deploying (default)"
    x deploy dev
  dev: npm run deploy:dev
  prod: npm run deploy:prod
  test:
    unit: npx jest unit
    integration: npx jest integration
```

- `x build 1.2.3` → `$1` is `1.2.3`
- `x deploy` → runs `deploy.$`
- `x deploy prod` → runs `deploy.prod`
- `x deploy test unit` → runs `deploy.test.unit`

Execution: `<default_program> -c '<script>' x <args…>`.

## App framework (`*.x.yml`)

Apps declare **metadata**, **options**, **arguments**, **commands**, and a **`$:`**
handler block (or **`$.import`** list of external handler YAML files — mutually
exclusive with inline `$:`).

### Minimal app

```yaml
name: my-app
version: 0.0.0
description: an example app

options:
  - "[-v | --version]"

commands:
  build:
    description: build the project
    options:
      - "[--mode={fast|safe|deep}]"
    arguments:
      - "<assets>..."

$:
  build: |
    echo "mode=$(x-opt mode), assets=$(x-arg assets)"
```

Run: `x my-app build --mode=fast src/`. Help: `x my-app --help`,
`x my-app build --help`.

### Synopsis cheat-sheet

| Synopsis | Meaning |
|----------|---------|
| `<name>` | required positional |
| `[<name>]` | optional positional |
| `[<name='val'>]` | optional with default |
| `<name>...` / `[<name>...]` | repeating (greedy; last positional) |
| `[-s \| --long]` | optional bool flag |
| `[-s \| --long <arg>]` | flag with value |
| `[-s \| --long <arg='v'>]` | flag value with default |
| `[--long={a\|b\|c}]` | choice |
| `[--input=<a> [--output=<b>]]` | nested = `--output` requires `--input` |
| `(a\|b\|c)` | required choice positional |
| `--long` | required option (no brackets) |

`x` validates all of this **before** any handler runs.

### Handler builtins

Available in every `$:` handler (bash preamble injected by `x`):

| Builtin | Purpose |
|---------|---------|
| `x-opt <name>` | option value; bool → `true`; repeats → one per line |
| `x-arg <name>` | positional value |
| `x-opts <assoc>` | fill bash assoc array with all options |
| `x-args <assoc>` | fill bash assoc array with all args |
| `x-run <cmd> …` | run command with helpers in scope |
| `x-usage <cmd.path>` | print generated help (e.g. `x-usage create.file`) |
| `x-io-read …` | prompt; `-v NAME` assigns global scalar |
| `x-io-confirm …` | yes/no; `--default yes\|no` |
| `x-io-select …` | menu `id=label`; `--multi` for arrays |

Repeating options: `mapfile -t dirs < <(x-opt dir)`.

Non-leaf commands without a handler should use `x-usage <path>` as the handler
body so bare invocation prints help.

### Validation

- `x -i --app` validates after edit; on failure: **Edit config** or **Revert**
- Every `x <app>` invocation re-validates the app file

Leaf commands need a handler key matching their dotted path (e.g. `create.file`).

## Supported script languages

Set per-script in metadata or default via `x --config`: `bash`, `zsh`, `sh`,
`node`, `python`, `python2`, `ruby`, `perl`, `go`, `rust`, `php`, `lua`, `deno`,
`swift`, `c`, `cpp`, `java`, `r`, `awk`, `elixir`, `clj`, `scala`, `haskell`,
`powershell`, `kotlin`.

## Agent workflow

When helping a user with `x`:

1. **Clarify scope** — personal script, repo `x.yml`, or full app?
2. **Check resolution** — will the command name collide with an existing global
   script or `./x.yml` key?
3. **Match complexity** — do not build an app for a single inline command; do not
   put complex flag parsing in `x.yml`.
4. **Edit the right file** — use `x --src <name>` to locate app/script paths;
   edit `./x.yml` or `./<name>.x.yml` directly for project files.
5. **Validate** — after app edits, run `x <app> --help` and exercise the changed
   subcommand. For `x -i --app`, rely on built-in validation loop.
6. **Handlers are bash** — even when metadata mentions other languages, app `$:`
   bodies are bash calling `x-opt` / `x-arg` / external tools.

## Pitfalls

- `./x.yml` is CWD-only; apps in parent dirs are found when **running** `x <app>`
  but not when resolving `x.yml` entries.
- `x --ls` tracks global scripts only, not local `x.yml` runs.
- `x -d` does not remove `x --ln` symlinks; use `x -d --ln`.
- Mapping in `x.yml` without `$` errors when invoked with no subcommand.
- `$:` and `$.import` cannot both appear in the same app file.
- App handler keys must match declared command paths exactly.