# `x` Script Management CLI Tool

A useful cli tool that helps you manage your scripts

## Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Commands](#commands)
- [Project-local Scripts](#project-local-scripts)
- [App Framework](#app-framework)
- [Supported Languages](#supported-languages)
- [Examples](#examples)

## Installation

### Option 1: Automated Installation Script
```bash
curl -fsSL https://raw.githubusercontent.com/michaelmunson/x.sh/main/install.sh | bash
```

Or download and run manually:

```bash
wget https://raw.githubusercontent.com/michaelmunson/x.sh/main/install.sh
bash install.sh
```

**Requirements:**
- Rust/Cargo (the script will prompt if not installed)
- Git (for cloning the repository)
- Internet connection

The script will:
1. Check for Rust/Cargo installation
2. Clone the repository (or use current directory if already in repo)
3. Build the binary in release mode
4. Install to `~/.local/bin/`
5. Verify the installation

**Note:** Make sure `~/.local/bin` is in your PATH. Add this to your `~/.bashrc`, `~/.zshrc`, or `~/.profile`:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

### Option 2: Manual Build

If you prefer to build manually:

```bash
cd x
cargo build --release
```

The binary will be at `target/release/x`. You can:
- Copy it to a directory in your PATH (e.g., `~/.local/bin/`)
- Create an alias in your shell configuration
- Use it directly with the full path

### Shell completion

Tab completion follows the same command resolution order as runtime (`x.yml` → apps → global scripts), including nested subcommands.

**Bash:**

```bash
mkdir -p ~/.local/share/bash-completion/completions
x __complete bash > ~/.local/share/bash-completion/completions/x
```

**Zsh** (add to `~/.zshrc`):
```bash
source <(x __complete zsh)
# or (if that doesn't work)
source <(x __complete bash)
```

## Quick Start

1. **Create your first script:**
   ```bash
   x --init hello-world
   # or: x -i hello-world
   # Follow the interactive prompts to select language and add description
   # Your editor will open for you to write the script
   ```

2. **Run your script:**
   ```bash
   x hello-world
   ```

3. **List all scripts:**
   ```bash
   x --ls
   # or: x -l
   ```

4. **Add script to path**
  ```bash
  x --ln hello-world
  ```

## Commands

#### `x --init [name] [script]` or `x -i [name] [script]`

Create a new script or initialize editing of an existing one.

**Interactive mode (recommended):**
```bash
x --init
# or: x -i
# Prompts for:
# - Script name (validated: alphanumeric + dashes only)
# - Description (optional)
# - Programming language (from supported list)
# - Opens editor for script content
```

**With script name:**
```bash
x --init my-script
# or: x -i my-script
# Prompts for description and language, then opens editor
```

If `my-script` already exists, you are prompted for the script name first so you can rename it before the editor opens (metadata and usage history follow the new name).

**With script name and content:**
```bash
x --init my-script "echo 'Hello, World!'"
# or: x -i my-script "echo 'Hello, World!'"
# Creates script with provided content (uses default or existing program)
```

**Script Name Rules:**
- Must contain only letters, numbers, and dashes
- Cannot be empty

#### `x <script_name> [args...]`

Execute a script with optional arguments.

```bash
x curl-google
x curl-google /docs
x my-script arg1 arg2 arg3
```

The script will be executed using the program specified for that script (or the default program if not set). Arguments are passed directly to the script.

If `./x.yml` exists in the **current working directory** and defines a top-level entry whose name matches `<script_name>`, that local definition runs instead of a global script in `~/.x.sh/scripts`. Otherwise `x` falls back to your installed scripts as usual. See [Project-local scripts (`x.yml`)](#project-local-scripts).

#### `x --ls` or `x -l`

List all available scripts with detailed information in a formatted table.

```bash
x --ls
# or: x -l
```

#### `x --delete <name>` or `x -d <name>`

Remove a script and its associated metadata.

```bash
x --delete my-script
# or: x -d my-script
```

**Note:** This does not remove symlinks created with `x --ln`. Remove those separately if needed.

#### `x --ln <name> [link_name]`

Create a symbolic link to a script in `~/.local/bin/`, allowing you to run the script without the `x` prefix.

**Create link with same name:**
```bash
x --ln my-script
# Now you can run: my-script
```

**Create link with custom name:**
```bash
x --ln my-script my-alias
# Now you can run: my-alias
```

**Delete a link:**
```bash
x --delete --ln my-script
# or: x -d --ln my-script
```

**Note:** 
- The `~/.local/bin/` directory is created automatically if it doesn't exist
- Make sure `~/.local/bin` is in your PATH to use linked scripts directly
- Existing links are overwritten when creating a new link

#### `x --ai`
**Note:** Requires LLM provider configuration (see `x --config`).
Generate CLI commands via LLM. Prompts for instructions, generates a command, and optionally executes it.

```bash
x --ai
# instructions> aws command to list parameters
# 
# Generated Command
# aws ssm describe-parameters
# Run command? [y/N]
```

#### `x --init --ai [name]`
Generate a script via LLM 

```bash
x --init --ai my-script
```

#### `x --src <name>`

Print the absolute path of a script or app file without running it. Useful for
piping into editors or scripts.

```bash
x --src my-script
# /home/you/.x.sh/scripts/my-script

x --src my-app
# /home/you/.x.sh/apps/my-app.x.yml
```

Resolution order:

1. local app:     `./<name>.x.yml` (current directory)
2. global app:    `~/.x.sh/apps/<name>.x.yml`
3. global script: `~/.x.sh/scripts/<name>` (matched with or without extension)

Local `x.yml` keys are intentionally not matched — they are inline strings, not
file paths.

#### `x -i --app [--local | --global] [<name>]`

Create a new app config (`<name>.x.yml`). See the
[App framework](#app-framework) section for the file format.

```bash
x -i --app my-app             # prompts for scope
x -i --app --local my-app     # creates ./my-app.x.yml
x -i --app --global my-app    # creates ~/.x.sh/apps/my-app.x.yml
```

After the editor closes, `x` validates the file. On failure it prints all
errors and offers **edit** (re-open the editor) or **revert** (restore the
previous file or delete the new one).

#### `x --config`

Configure default settings for `x`.

```bash
x --config
# Configuration Options:
#   > Default Script Language
#   > LLM Provider
```

**Configuration Options:**

1. **Default Script Language**: Set the default programming language for new scripts
   - The selected default will be pre-selected when creating new scripts

2. **LLM Provider**: Configure the AI/LLM integration
   - Opens your editor with a template script
   - The script should accept a prompt as the first argument (`$1`)
   - Should output the completion to stdout
   - Saved to `~/.x.sh/config/llm.sh`
   - Example template:
     ```bash
      #!/bin/bash
      PROMPT="$1"
      ollama run llama2 "$PROMPT"
     ```

## Project-local Scripts

You can define **project-local** commands in a YAML file named `x.yml` in the **current working directory**. When you run `x <name> …`, `x` loads `./x.yml` (if present). If the file defines a top-level key matching `<name>`, that entry is resolved and executed. If there is no `x.yml`, or it does not define `<name>`, `x` looks up a **global** script under `~/.x.sh/scripts` as before.

### File format

Each top-level key is a command name. The value is either:

1. **A string** — inline script (often a block scalar with `|`). Extra CLI arguments after `<name>` are passed as positional parameters (`$1`, `$2`, …) when the script is run.

2. **A mapping** — subcommands and an optional default:
   - **`$`** — script for the parent command when you run `x <parent>` with **no** further arguments (after `<parent>` has been consumed).
   - **Any other key** — nested command; you can nest mappings arbitrarily deep.

Examples:

```yaml
build: |
  VERSION="$1"
  npm run build
  npm version "$VERSION"
  git push --tags

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

- `x build 1.2.3` runs `build` with `1.2.3` as `$1`.
- `x deploy` runs `deploy.$`.
- `x deploy prod` runs `deploy.prod`.
- `x deploy test unit` runs `deploy.test.unit`.

If a mapping has **no** `$` key and you invoke the parent with no subcommand arguments (e.g. `x deploy` when only nested keys exist), `x` reports an error asking you to add `$` or pass a subcommand.

### Execution

Local scripts are executed with your configured **default program** (`x --config`, or `default_program` in `~/.x.sh/config.json`), same as other inline shell-style usage—typically **`bash`**. The implementation runs `<program> -c '<script>' x <args…>`, so `$1` in the script refers to the first argument after the command name.

### Notes

- Local scripts do not use per-script metadata files; only the default program applies.
- If `x.yml` exists but is invalid YAML, parsing fails before falling back to a global script—fix or rename the file.
- Activity tracking in `x --ls` applies to global scripts; local `x.yml` runs are not recorded there.

## App Framework

Apps turn a single YAML file into a multi-command CLI. Unlike `x.yml` (which is
a flat map of inline scripts), an app file declares **options**, **arguments**,
**nested commands**, and a `$:` block of bash handlers. `x` parses the synopsis
strings and validates the user's input *before* any handler runs.

### Where apps live

- **Local**: `./<name>.x.yml` in the current directory.
- **Global**: `~/.x.sh/apps/<name>.x.yml`.

When you run `x <name> …`, resolution order is:

1. `./x.yml` entry (existing project-local feature)
2. `./<name>.x.yml` (local app)
3. `~/.x.sh/apps/<name>.x.yml` (global app)
4. `~/.x.sh/scripts/<name>` (installed script)

### Creating an app

Use [`x -i --app`](#x--i---app---local----global-name) to create a seed file and initialize the app

### File format

```yaml
# metadata
name: my-app
version: 0.0.0
description: an example app

# top-level options (apply to the root command)
options:
  - "[-v | --version]"

# nested commands; each may have its own options/arguments/commands
commands:
  build:
    description: build the project
    options:
      - "[--input=<file> [--output=<dir>]]"
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

# bash handlers, keyed by dotted command path
$:
  build: |
    echo "mode=$(x-opt mode), assets=$(x-arg assets)"
  create: x-usage create        # default: print help when called bare
  create.file: |
    echo "creating $(x-arg path) with $(x-arg content)"
  create.folder: |
    mkdir -p "$(x-arg path)"

# OR load handlers from one or more external YAML files
# "$.import": ./handlers.yml
# "$.import":
#   - ./create.yml
#   - ./build.yml
```

`$:` and `$.import:` are mutually exclusive. Imported files have the same shape
as the inline `$:` block (a flat map of `<dotted.path>: <bash body>`).

### Synopsis cheat-sheet

| Synopsis                              | Meaning                                                           |
|---------------------------------------|-------------------------------------------------------------------|
| `<name>`                              | required positional                                                |
| `[<name>]`                            | optional positional                                                |
| `[<name='val'>]`                      | optional positional with default literal                           |
| `<name>...` / `[<name>...]`           | repeating positional                                               |
| `[-s \| --long]`                      | optional bool flag (alias pair)                                    |
| `[-s \| --long <arg>]`                | flag with one required value                                       |
| `[-s \| --long <arg='v'>]`            | flag value with default                                            |
| `[-s \| --long <arg> ...]`            | flag value repeats                                                 |
| `[--long={a\|b\|c}]`                  | choice from set                                                    |
| `[--long=<arg='v'>]`                  | `=` form, value defaults                                           |
| `[--input=<a> [--output=<b>]]`        | nesting = dependency: `--output` requires `--input`                |
| `(a\|b\|c)`                           | required choice (top-level)                                        |
| `--long`                              | required option (bare, no brackets)                                |

`x` enforces all of the above before any bash runs: required args, defaults,
choice membership, `requires:` chains, repeats, and unknown options.

### Built-in helpers (in scope inside every handler)

`x` injects a small bash preamble before each handler so you can read the
parsed values without manual env-var wrangling.

| Builtin                         | Behavior                                                                          |
|---------------------------------|-----------------------------------------------------------------------------------|
| `x-opt <name>`                  | Print the value of an option. Bool flags print `true`. Repeats join with newline. |
| `x-arg <name>`                  | Print the value of a positional argument.                                         |
| `x-opts <assoc-name>`           | Populate a caller-named bash assoc array with all option values.                  |
| `x-args <assoc-name>`           | Populate a caller-named bash assoc array with all argument values.                |
| `x-run <cmd> [args...]`         | Run a command with the `x-*` helpers still in scope (functions are exported).     |
| `x-usage <cmd-path>`            | Print the auto-generated help for a command (e.g. `x-usage create.file`).         |
| `x-io-read …`                   | Prompt for a line (default prompt `?`). Optional `-v NAME`, `--var NAME`, or `--var=NAME` assigns the answer to a global scalar instead of stdout. |
| `x-io-confirm …`                | Yes/no (`--default yes` / `--default no`, default `no`). With `-v` / `--var`, assigns `true` or `false` to a global scalar instead of stdout. |
| `x-io-select …`                 | Menu of `id=label` options (`--multi`, optional `--search` no-op, `--no-search`). With `-v` / `--var`, assigns a global indexed array if `--multi`, else a single id string (same order as without `-v`). Otherwise prints one id per line when `--multi`. |

With `-v`, results are stored in the **caller's** global shell variable (not stdout): scalars for `x-io-read`, `x-io-confirm`, and single select; an indexed array for `x-io-select --multi`.

```bash
x-io-read "Name:" -v name
echo "$name"

x-io-select --multi -v colors "Choose a color" "red=Red" "green=Green" "blue=Blue"
echo "${colors[0]}"
```

The assoc-array helpers use bash namerefs (`declare -n`), so the call site is:

```bash
declare -A opts
x-opts opts
echo "${opts[mode]}"

declare -A args
x-args args
for k in "${!args[@]}"; do echo "$k=${args[$k]}"; done
```

For repeating options, `x-opt` returns one value per line. Idiomatic bash:

```bash
mapfile -t dirs < <(x-opt dir)
for d in "${dirs[@]}"; do echo "dir: $d"; done
```

### Help

Every command supports `-h` / `--help` automatically:

```bash
x my-app --help
x my-app build --help
x my-app create file --help
```

The same renderer powers `x-usage <cmd-path>` so handlers can print
context-appropriate help.

### Validation

When you save an app via `x -i --app`, `x` parses the YAML, parses every
synopsis string, and runs structural checks (missing handlers for leaf
commands, unknown handler keys, duplicate options, dangling `requires:`,
conflicting `$/$.import`). On failure you can **edit** the file again or
**revert** to the prior contents.

A lighter-weight validate also runs on every `x <app>` invocation so a stale
or hand-edited file fails fast with a readable error.


## Supported Languages
`x` supports creating scripts in the following languages:

| Language | Program Value | Description |
|----------|--------------|-------------|
| Bash | `bash` | Bash/Shell Script |
| Zsh | `zsh` | Zsh Script |
| POSIX Shell | `sh` | POSIX Shell Script |
| Node.js | `node` | JavaScript/TypeScript via Node.js |
| Python 3 | `python` | Python 3 Script |
| Python 2 | `python2` | Python 2 Script |
| Ruby | `ruby` | Ruby Script |
| Perl | `perl` | Perl Script |
| Go | `go` | Go Program |
| Rust | `rust` | Rust Program |
| PHP | `php` | PHP Script |
| Lua | `lua` | Lua Script |
| Deno | `deno` | Deno (JavaScript/TypeScript runtime) |
| Swift | `swift` | Swift Script |
| C | `c` | C Program |
| C++ | `cpp` | C++ Program |
| Java | `java` | Java Program |
| R | `r` | R Script |
| AWK | `awk` | AWK Script |
| Elixir | `elixir` | Elixir Script |
| Clojure | `clj` | Clojure Script |
| Scala | `scala` | Scala Script |
| Haskell | `haskell` | Haskell Program |
| PowerShell | `powershell` | PowerShell Script |
| Kotlin | `kotlin` | Kotlin Script/Program |


## Examples
- [example app](https://github.com/michaelmunson/x.sh/blob/main/docs/examples/app/exapp.x.yml)