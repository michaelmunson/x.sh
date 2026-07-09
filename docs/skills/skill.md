---
name: x-cli
description: >-
  Create, run, and maintain scripts and multi-command CLIs with the `x` tool
  (global scripts, project-local `x.yml`, and `*.x.yml` apps - both share the
  same v3 syntax). Use when the user mentions `x`, `x.sh`, `x.yml`, `.x.yml`
  apps, `x -i`, `x --init`, dot-prefixed commands, `$` handlers, synopsis DSL,
  or wants to add project commands or personal shell utilities.
---

# `x` CLI Skill

`x` is a Rust CLI for managing personal scripts and YAML-defined command trees.
Global state lives under `~/.x.sh/`. Project-local commands live in `./x.yml` or
`./<name>.x.yml` - both use the **same file format** (see
[app.skill.md](app.skill.md) for the full syntax reference).

## When to use what

| Need | Use | File / location |
|------|-----|-----------------|
| Reusable personal script (any language) | `x -i <name>` | `~/.x.sh/scripts/<name>` + `~/.x.sh/metadata/<name>.toml` |
| A few project commands, run as `x <cmd>` | `./x.yml` | `.command:` keys in repo root |
| Structured CLI with flags, help, validation, run as `x <app> <cmd>` | `x -i --app [--local\|--global] <name>` | `./<name>.x.yml` or `~/.x.sh/apps/<name>.x.yml` |
| Convert OpenAPI → x app | `x -i --plugin openapi` then `x --plugin openapi <spec>` | writes `./<name>.x.yml` |
| Run without `x` prefix | `x --ln <name> [alias]` | Symlink in `~/.local/bin/` |

**Prefer `x.yml`** for a few repo-specific commands. **Prefer an app**
(`<name>.x.yml`) when you want a namespaced CLI (`x myapp <cmd>`), or need
nested subcommands, typed flags, positional args, auto-generated `--help`,
or interactive prompts (`x-io-*`) - `x.yml` supports all the same syntax too,
it's just always invoked without a name prefix.

## Command resolution

When the user runs `x <name> [args…]`, `x` resolves in this order:

1. **`./x.yml`** top-level command `<name>` (current directory only)
2. **App** `<name>.x.yml` - walk from CWD up through parent directories, then `~/.x.sh/apps/`
3. **Global script** `~/.x.sh/scripts/<name>`

`x --src <name>` skips `x.yml` and returns the path to an app or global script.

Invalid `./x.yml` YAML fails immediately; it does not fall back to global scripts.

## Core commands

```bash
x <name> [args…]          # run script, local x.yml command, or app
x -i [name] [content]     # create/edit global script (opens $EDITOR)
x -i --ai [name]          # generate script via configured LLM
x -l | --ls               # list global scripts (with metadata/activity)
x -d | --delete <name>    # remove global script + metadata
x --ln <name> [alias]     # symlink into ~/.local/bin/
x -d --ln <name>          # remove symlink only
x --src <name>            # print absolute path to app or script file
x --config                # default language + LLM provider
x -i --app [--local|--global] [name]   # create/edit app YAML
x -i --plugin <name>                   # install plugin into ~/.x.sh/plugins/
x --plugin <name> [args…]              # run an installed plugin
```

Script names: letters, numbers, dashes only. App names also allow underscores.

## Plugins

Plugins are separate Rust packages under `plugins/` in the x.sh repo. They install
into `~/.x.sh/plugins/<name>` (never on `PATH`) and are always invoked via `x`:

```bash
x -i --plugin openapi
x --plugin openapi ./openapi.yaml          # → ./<name>.x.yml
x --plugin openapi ./spec.json -o api.x.yml
x --plugin openapi ./spec.yaml --stdout
```

See [plugins/README.md](../../plugins/README.md).

## Global layout

```
~/.x.sh/
├── scripts/          # script files (no extension required)
├── metadata/         # <name>.toml per script (description, program, groups)
├── apps/             # global <name>.x.yml apps
├── plugins/          # installed plugin binaries (not on PATH)
├── config.json       # default_program, etc.
├── config/llm.sh     # LLM hook: receives prompt as $1, prints to stdout
└── metadata.json     # activity timestamps for global scripts
```

Default interpreter for global scripts: `default_program` from `x --config`,
usually `bash`. Commands defined in `x.yml`/`*.x.yml` always run as bash.

## Project-local `x.yml`

Only read from `./x.yml` in the **current working directory**. Commands are
`.name:` keys - a string is shorthand for an inline `$:` script, or use a
mapping for options/arguments/subcommands:

```yaml
.build:
  args: <version>
  $: |
    npm run build
    npm version "$(x-arg version)"

.deploy:
  $: |
    echo "Deploying (default)"
    x deploy dev
  .dev: npm run deploy:dev
  .prod: npm run deploy:prod
  .test:
    .unit: npx jest unit
    .integration: npx jest integration
```

- `x build 1.2.3` → `x-arg version` is `1.2.3`
- `x deploy` → runs `deploy`'s own `$`
- `x deploy prod` → runs `deploy.prod`
- `x deploy test unit` → runs `deploy.test.unit`

This is exactly the app framework's syntax (see [app.skill.md](app.skill.md))
- just invoked without an app name prefix, and `name:` is optional (defaults
to the containing directory name conceptually, though it's rarely shown).

## App framework (`*.x.yml`)

Apps use the identical `.command:` syntax as `x.yml`, plus top-level
`name`/`version`/`description` metadata. See [app.skill.md](app.skill.md) for
the full reference (synopsis DSL, handler builtins, `import`, `env`, `alias`,
validation rules).

Quick example:

```yaml
name: my-app
version: 0.0.0
description: an example app

options:
  - "[-v | --version]"

.build:
  description: build the project
  options:
    - "[--mode={fast|safe|deep}]"
  arguments:
    - "<assets>..."
  $: |
    echo "mode=$(x-opt mode), assets=$(x-arg assets)"
```

Run: `x my-app build --mode=fast src/`. Help: `x my-app --help`,
`x my-app build --help`.

## Supported script languages (global scripts only)

Set per-script in metadata or default via `x --config`: `bash`, `zsh`, `sh`,
`node`, `python`, `python2`, `ruby`, `perl`, `go`, `rust`, `php`, `lua`, `deno`,
`swift`, `c`, `cpp`, `java`, `r`, `awk`, `elixir`, `clj`, `scala`, `haskell`,
`powershell`, `kotlin`. (Commands in `x.yml`/`*.x.yml` are always bash.)

## Agent workflow

When helping a user with `x`:

1. **Clarify scope** - personal script, repo `x.yml`, or a namespaced app?
2. **Check resolution** - will the command name collide with an existing global
   script or `./x.yml` command?
3. **Match complexity** - a single inline command doesn't need typed
   flags/subcommands, but don't hesitate to use the full syntax in `x.yml` if
   it helps (it's the same engine as apps).
4. **Edit the right file** - use `x --src <name>` to locate app/script paths;
   edit `./x.yml` or `./<name>.x.yml` directly for project files.
5. **Validate** - run `x <app-or-name> --help` and exercise the changed
   subcommand after edits.
6. **Handlers are bash** - even when metadata mentions other languages, `$:`
   bodies are bash calling `x-opt` / `x-arg` / external tools.

## Pitfalls

- `./x.yml` is CWD-only; apps in parent dirs are found when **running**
  `x <app>` but not when resolving `x.yml` commands.
- `x --ls` tracks global scripts only, not local `x.yml` runs.
- `x -d` does not remove `x --ln` symlinks; use `x -d --ln`.
- `commands:` and a top-level `$:` handler map are **removed in v3** - use
  `.command-name:` keys and per-command inline `$:` instead.
- A command with subcommands but no `$:` of its own auto-prints help when
  invoked bare (no error).
