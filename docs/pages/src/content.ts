import type { ContentBlock } from "./components";
import { EXAPP_X, EXAPP_HANDLERS_A, EXAPP_HANDLERS_B } from "./examples";

export type NavItem = {
    id: string;
    label: string;
    group?: string;
};

export const NAV_ITEMS: NavItem[] = [
    { id: "overview", label: "Overview", group: "Getting Started" },
    { id: "installation", label: "Installation", group: "Getting Started" },
    { id: "quickstart", label: "Quick Start", group: "Getting Started" },
    { id: "architecture", label: "Architecture", group: "Getting Started" },
    { id: "commands", label: "Commands", group: "Reference" },
    { id: "global-scripts", label: "Global Scripts", group: "Reference" },
    { id: "project-local", label: "Local Scripts", group: "Reference" },
    { id: "app-framework", label: "App Framework", group: "Apps" },
    { id: "app-format", label: "App File Format", group: "Apps" },
    { id: "synopsis", label: "Synopsis DSL", group: "Apps" },
    { id: "builtins", label: "Built-in Helpers", group: "Apps" },
    { id: "validation-help", label: "Validation & Help", group: "Apps" },
    { id: "examples", label: "Examples", group: "Apps" },
    { id: "configuration", label: "Configuration", group: "Advanced" },
    { id: "ai", label: "AI Integration", group: "Advanced" },
    { id: "languages", label: "Supported Languages", group: "Advanced" },
    { id: "faq", label: "FAQ", group: "Advanced" },
];

export type DocSection = {
    id: string;
    title: string;
    subtitle?: string;
    blocks: ContentBlock[];
};

export const SECTIONS: DocSection[] = [
    {
        id: "overview",
        title: "Overview",
        subtitle: "A personal script manager and lightweight CLI app framework for your shell.",
        blocks: [
            { type: "p", text: "**x** is a Rust-powered CLI that helps you create, organize, and run scripts across projects. Think of it as a script registry with metadata, editor integration, and optional symlinks into your PATH — plus two YAML-based layers for project commands and full multi-command apps." },
            { type: "h3", text: "What makes x different" },
            { type: "ul", items: [
                "**Global scripts** live in `~/.x.sh/scripts/` with per-script metadata (description, language, groups) and usage tracking.",
                "**Project-local commands** in `./x.yml` let you version-control task runners alongside your repo — no install step required for teammates.",
                "**Apps** (`&lt;name&gt;.x.yml`) turn YAML into validated CLIs with options, arguments, nested subcommands, and bash handlers — before any bash runs, x parses and validates the user's input.",
                "**AI helpers** (`x --ai`, `x -i --ai`) generate shell commands and scripts when you configure an LLM provider.",
            ]},
            { type: "h3", text: "Three ways to run a command" },
            { type: "code", title: "Resolution order when you run `x &lt;name&gt; …`", code: `1. ./x.yml          → project-local inline script (CWD only)
2. ./&lt;name&gt;.x.yml    → local app (CWD, then parent dirs)
3. ~/.x.sh/apps/     → global app
4. ~/.x.sh/scripts/  → installed global script` },
            { type: "callout", variant: "tip", text: "Use `x --src &lt;name&gt;` to print the absolute path of a script or app file without executing it — handy for piping into editors or CI." },
        ],
    },
    {
        id: "installation",
        title: "Installation",
        subtitle: "Get the `x` binary on your system in minutes.",
        blocks: [
            { type: "h3", text: "Automated install (recommended)" },
            { type: "p", text: "The install script clones the repo, builds in release mode, and places the binary in `~/.local/bin/`." },
            { type: "code", title: "One-liner", code: `curl -fsSL https://raw.githubusercontent.com/michaelmunson/x.sh/main/install.sh | bash` },
            { type: "p", text: "Or download and run manually:" },
            { type: "code", code: `wget https://raw.githubusercontent.com/michaelmunson/x.sh/main/install.sh
bash install.sh` },
            { type: "h4", text: "Requirements" },
            { type: "ul", items: [
                "**Rust / Cargo** — the script prompts you to install via rustup if missing.",
                "**Git** — used to clone the repository (skipped if you run the script from inside the repo).",
                "An internet connection for the initial clone and build.",
            ]},
            { type: "h4", text: "What the script does" },
            { type: "ol", items: [
                "Checks for Rust/Cargo.",
                "Clones `github.com/michaelmunson/x.sh` (or uses the current directory if already in the repo).",
                "Runs `cargo build --release`.",
                "Copies `target/release/x` to `~/.local/bin/x`.",
                "Verifies the installation by running `x --help`.",
            ]},
            { type: "callout", variant: "warn", title: "PATH", text: "Ensure `~/.local/bin` is on your PATH. Add `export PATH=\"$HOME/.local/bin:$PATH\"` to `~/.bashrc`, `~/.zshrc`, or `~/.profile`, then restart your shell." },
            { type: "h3", text: "Manual build" },
            { type: "code", code: `git clone https://github.com/michaelmunson/x.sh.git
cd x.sh
cargo build --release
# Binary at target/release/x — copy or symlink into your PATH` },
            { type: "h3", text: "First-run directory layout" },
            { type: "p", text: "On first use, x creates `~/.x.sh/` with the following structure:" },
            { type: "code", title: "~/.x.sh/", code: `~/.x.sh/
├── scripts/          # your global script files
├── apps/             # global app YAML files (&lt;name&gt;.x.yml)
├── metadata/         # per-script TOML metadata
│   └── &lt;name&gt;.toml
├── config.json       # default_program and other settings
├── metadata.json     # usage / activity tracking
└── config/
    └── llm.sh        # LLM provider script (after x --config)` },
        ],
    },
    {
        id: "quickstart",
        title: "Quick Start",
        subtitle: "From zero to a runnable script in under a minute.",
        blocks: [
            { type: "h3", text: "1. Create your first script" },
            { type: "code", code: `x --init hello-world
# or: x -i hello-world` },
            { type: "p", text: "Interactive prompts ask for the script name (alphanumeric + dashes), an optional description, and a programming language. Your `$EDITOR` opens so you can write the script body." },
            { type: "h3", text: "2. Run it" },
            { type: "code", code: `x hello-world
x hello-world arg1 arg2   # args pass through to the script` },
            { type: "h3", text: "3. List installed scripts" },
            { type: "code", code: `x --ls
# or: x -l` },
            { type: "p", text: "Shows a formatted table with name, description, language, groups, and last-run timestamps." },
            { type: "h3", text: "4. Add to PATH (optional)" },
            { type: "code", code: `x --ln hello-world
# Now run directly: hello-world` },
            { type: "h3", text: "5. Project-local commands (optional)" },
            { type: "p", text: "Drop an `x.yml` in your project root to define repo-specific tasks:" },
            { type: "code", title: "x.yml", code: `test: npm test
build: |
  npm run build
  echo "Built $1"   # $1 = first arg after 'build'` },
            { type: "code", code: `x test
x build v1.0.0` },
        ],
    },
    {
        id: "architecture",
        title: "Architecture",
        subtitle: "How x resolves, stores, and executes commands.",
        blocks: [
            { type: "h3", text: "Command resolution flow" },
            { type: "p", text: "When you invoke `x &lt;name&gt; [args…]`, x walks a fixed resolution chain. The first match wins and the process exits after execution." },
            { type: "h3", text: "Local app discovery" },
            { type: "p", text: "Unlike `x.yml` (current directory only), **local apps** are discovered by walking from the current working directory up through every parent directory. The nearest `&lt;name&gt;.x.yml` wins. This lets monorepos keep a shared app at the repo root while you work in subdirectories." },
            { type: "h3", text: "Global script lookup" },
            { type: "p", text: "Scripts in `~/.x.sh/scripts/` are matched by exact filename first, then by basename without extension — so `x c-test` finds `c-test.c` automatically." },
            { type: "h3", text: "Metadata model" },
            { type: "p", text: "Each global script can have a companion TOML file at `~/.x.sh/metadata/&lt;name&gt;.toml`:" },
            { type: "code", title: "Example metadata", code: `description = "Fetch a URL with curl"
groups = ["network", "utils"]
program = "bash"` },
            { type: "p", text: "Activity data (created, updated, last_executed) is stored separately in `~/.x.sh/metadata.json` and shown in `x --ls`." },
            { type: "h3", text: "Inline script execution" },
            { type: "p", text: "Project-local `x.yml` entries and certain inline paths run via your configured default program (typically `bash`):" },
            { type: "code", code: `&lt;program&gt; -c '&lt;script body&gt;' x &lt;remaining-args…&gt;` },
            { type: "p", text: "Positional parameters in the script body (`$1`, `$2`, …) refer to arguments after the command name." },
        ],
    },
    {
        id: "commands",
        title: "Commands",
        subtitle: "Complete CLI reference for every flag and subcommand.",
        blocks: [
            { type: "h3", text: "`x --init [name] [script]` / `x -i`" },
            { type: "p", text: "Create a new global script or re-open an existing one in your editor." },
            { type: "ul", items: [
                "`x -i` — fully interactive (name, description, language, editor).",
                "`x -i my-script` — prompts for metadata, opens editor. If the script exists, you can rename it first; metadata and history follow the new name.",
                "`x -i my-script \"echo hello\"` — creates the script with inline content using the default or existing program.",
            ]},
            { type: "callout", variant: "info", text: "Script names must be non-empty and contain only letters, numbers, and dashes." },
            { type: "h3", text: "`x &lt;script_name&gt; [args…]`" },
            { type: "p", text: "Execute a command using the resolution order described in Architecture. Arguments are forwarded verbatim." },
            { type: "h3", text: "`x --ls` / `x -l`" },
            { type: "p", text: "List all global scripts in a table with description, program, groups, and activity timestamps." },
            { type: "h3", text: "`x --delete &lt;name&gt;` / `x -d`" },
            { type: "p", text: "Remove a global script and its metadata. Does **not** remove symlinks created with `x --ln` — delete those separately." },
            { type: "h3", text: "`x --ln &lt;name&gt; [link_name]`" },
            { type: "p", text: "Symlink a script into `~/.local/bin/` so you can invoke it without the `x` prefix." },
            { type: "code", code: `x --ln my-script          # → ~/.local/bin/my-script
x --ln my-script alias    # → ~/.local/bin/alias
x -d --ln my-script       # remove the symlink` },
            { type: "h3", text: "`x --src &lt;name&gt;`" },
            { type: "p", text: "Print the absolute path of a script or app file. Resolution: local app → global app → global script. `x.yml` keys are not matched." },
            { type: "h3", text: "`x -i --app [--local | --global] [&lt;name&gt;]`" },
            { type: "p", text: "Create a new app YAML file. Scope flags control where the file is written; without them, x prompts interactively." },
            { type: "h3", text: "`x --config`" },
            { type: "p", text: "Interactive configuration for default script language and LLM provider. See the Configuration section." },
            { type: "h3", text: "`x --ai` and `x -i --ai [name]`" },
            { type: "p", text: "LLM-powered command and script generation. Requires `~/.x.sh/config/llm.sh`. See AI Integration." },
        ],
    },
    {
        id: "global-scripts",
        title: "Global Scripts",
        subtitle: "Personal scripts available from any directory.",
        blocks: [
            { type: "p", text: "Global scripts are plain files in `~/.x.sh/scripts/`. Each script is executed with the program stored in its metadata (or the global default from `x --config`)." },
            { type: "h3", text: "Creating and editing" },
            { type: "p", text: "`x -i` is the primary workflow. x opens your `$EDITOR`, and on save, writes the file and metadata. &lt;br&gt; Re-running `x -i &lt;name&gt;` on an existing script lets you edit in place or rename." },
            { type: "h3", text: "Passing arguments" },
            { type: "code", code: `# Script: curl-google (bash)
curl -s "https://google.com$1"

$ x curl-google /search
# → curl -s "https://google.com/search"` },
            { type: "h3", text: "Program / interpreter" },
            { type: "p", text: "The `program` field in metadata determines how x invokes the file — `bash`, `python`, `node`, `go run`, etc. Set per-script during `x -i` or rely on the global default." },
            { type: "h3", text: "Groups" },
            { type: "p", text: "Metadata `groups` are free-form tags for organization. They appear in `x --ls` output to help you scan large script collections." },
            { type: "h3", text: "Symlinks vs scripts" },
            { type: "p", text: "`x --ln` creates a symlink in `~/.local/bin/`, not a copy. The symlink points at the script file in `~/.x.sh/scripts/`, so edits via `x -i` are reflected immediately." },
        ],
    },
    {
        id: "project-local",
        title: "Project-local Scripts (`x.yml`)",
        subtitle: "Version-controlled task runners in the current project.",
        blocks: [
            { type: "p", text: "Place a file named `x.yml` in your **current working directory**. When you run `x &lt;name&gt;`, x loads it and looks for a top-level key matching `&lt;name&gt;`. If found, the entry runs; otherwise x falls back to global scripts." },
            { type: "callout", variant: "warn", text: "Only the CWD is checked for `x.yml` — not parent directories. Invalid YAML fails immediately; x does not fall back to global scripts on parse errors." },
            { type: "h3", text: "String entries (inline scripts)" },
            { type: "code", title: "x.yml", code: `lint: npm run lint

build: |
  VERSION="$1"
  npm run build
  npm version "$VERSION"
  git push --tags` },
            { type: "p", text: "Extra CLI arguments after the command name become `$1`, `$2`, … in the script body." },
            { type: "h3", text: "Mapping entries (nested subcommands)" },
            { type: "code", title: "Nested commands", code: `deploy:
  $: |
    echo "Deploying (default)"
    x deploy dev
  dev: npm run deploy:dev
  prod: npm run deploy:prod
  test:
    unit: npx jest unit
    integration: npx jest integration` },
            { type: "ul", items: [
                "`$` — default handler when you run the parent with no further arguments (`x deploy`).",
                "Any other key — nested subcommand; mappings can nest arbitrarily deep.",
                "`x deploy prod` → `deploy.prod`",
                "`x deploy test unit` → `deploy.test.unit`",
            ]},
            { type: "callout", variant: "info", text: "If a mapping has nested keys but no `$` handler, invoking the parent alone (e.g. `x deploy`) produces an error asking you to add `$` or pass a subcommand." },
            { type: "h3", text: "Execution details" },
            { type: "p", text: "Local scripts use your configured **default program** (`x --config` → `default_program` in `~/.x.sh/config.json`), typically `bash`. No per-entry metadata is supported." },
            { type: "h3", text: "Activity tracking" },
            { type: "p", text: "Runs from `x.yml` are **not** recorded in `x --ls` activity data — only global scripts are tracked." },
        ],
    },
    {
        id: "app-framework",
        title: "App Framework",
        subtitle: "YAML-defined, validated multi-command CLIs.",
        blocks: [
            { type: "p", text: "Apps turn a single `&lt;name&gt;.x.yml` file into a structured CLI with options, positional arguments, nested subcommands, and bash handlers. Unlike `x.yml` inline scripts, apps get **parse-time validation** — x checks required args, defaults, choice membership, `requires:` chains, and unknown flags before any handler bash executes." },
            { type: "h3", text: "Where apps live" },
            { type: "ul", items: [
                "**Local:** `./&lt;name&gt;.x.yml` (discovered from CWD up through parents).",
                "**Global:** `~/.x.sh/apps/&lt;name&gt;.x.yml`.",
            ]},
            { type: "h3", text: "Creating an app" },
            { type: "code", code: `x -i --app my-app             # prompts for local vs global
x -i --app --local my-app     # ./my-app.x.yml
x -i --app --global my-app    # ~/.x.sh/apps/my-app.x.yml` },
            { type: "p", text: "After the editor closes, x validates the YAML structure, parses every synopsis string, and checks handler coverage. On failure you can **edit** (re-open) or **revert** (restore prior file or delete the new one)." },
            { type: "h3", text: "App vs project-local" },
            { type: "table", headers: ["Feature", "`x.yml`", "App (`&lt;name&gt;.x.yml`)"], rows: [
                ["Scope", "CWD only", "CWD + parent walk / global"],
                ["CLI parsing", "None (raw args as $1…)", "Full synopsis DSL + validation"],
                ["Options / flags", "No", "Yes"],
                ["Help generation", "No", "Auto `-h` / `--help` per command"],
                ["Handlers", "Inline YAML strings", "Bash in `$:` block or `$.import`"],
                ["Metadata", "None", "name, version, description"],
            ]},
            { type: "h3", text: "Handler imports" },
            { type: "p", text: "Large apps can split handlers into external YAML files via `$.import`. This is mutually exclusive with an inline `$:` block." },
            { type: "code", code: `$.import:
  - ./handlers/build.yml
  - ./handlers/deploy.yml` },
        ],
    },
    {
        id: "app-format",
        title: "App File Format",
        subtitle: "Anatomy of a `&lt;name&gt;.x.yml` app definition.",
        blocks: [
            { type: "code", title: "Complete example", code: `# metadata
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
      - "[--input=&lt;file&gt; [--output=&lt;dir&gt;]]"
      - "[--mode={fast|safe|deep}]"
    arguments:
      - "&lt;assets&gt;..."

  create:
    description: create things
    commands:
      file:
        description: create a file
        arguments: "&lt;path&gt; [&lt;content='empty'&gt;]"
      folder:
        arguments: "&lt;path&gt;"

# bash handlers, keyed by dotted command path
$:
  build: |
    echo "mode=$(x-opt mode), assets=$(x-arg assets)"
  create: x-usage create        # default: print help when called bare
  create.file: |
    echo "creating $(x-arg path) with $(x-arg content)"
  create.folder: |
    mkdir -p "$(x-arg path)"` },
            { type: "h3", text: "Top-level fields" },
            { type: "ul", items: [
                "`name`, `version`, `description` — metadata shown in help output.",
                "`options` — flags available on the root command.",
                "`arguments` — positional args on the root command.",
                "`commands` — nested subcommand tree.",
                "`$` or `$.import` — bash handler map (mutually exclusive).",
            ]},
            { type: "h3", text: "Command nodes" },
            { type: "p", text: "Each entry under `commands` can have `description`, `options`, `arguments`, nested `commands`, or any combination. Leaf commands need a matching handler key in `$` (e.g. `build` → `$: build:`)." },
            { type: "h3", text: "Non-leaf groups" },
            { type: "p", text: "A command with subcommands but no direct handler can use a one-liner like `create: x-usage create` to print help when invoked without a subcommand." },
            { type: "h3", text: "Imported handlers" },
            { type: "p", text: "External files referenced by `$.import` have the same flat `dotted.path: bash body` shape as the inline `$:` block." },
        ],
    },
    {
        id: "synopsis",
        title: "Synopsis DSL",
        subtitle: "The mini-language for declaring options and arguments.",
        blocks: [
            { type: "p", text: "Every string in `options` and `arguments` arrays is a **synopsis** that x parses into a structured CLI spec. All constraints are enforced before handlers run." },
            { type: "table", headers: ["Synopsis", "Meaning"], rows: [
                ["`&lt;name&gt;`", "Required positional"],
                ["`[&lt;name&gt;]`", "Optional positional"],
                ["`[&lt;name='val'&gt;]`", "Optional positional with default literal"],
                ["`&lt;name&gt;...` / `[&lt;name&gt;...]`", "Repeating positional (greedy, must be last)"],
                ["`[-s | --long]`", "Optional bool flag (alias pair)"],
                ["`[-s | --long &lt;arg&gt;]`", "Flag with one required value"],
                ["`[-s | --long &lt;arg='v'&gt;]`", "Flag value with default"],
                ["`[-s | --long &lt;arg&gt; ...]`", "Flag value repeats"],
                ["`[--long={a|b|c}]`", "Choice from set"],
                ["`[--long=&lt;arg='v'&gt;]`", "`=` form with default value"],
                ["`[--input=&lt;a&gt; [--output=&lt;b&gt;]]`", "Nesting = dependency (`--output` requires `--input`)"],
                ["`(a|b|c)`", "Required choice (becomes positional `choice`)"],
                ["`--long`", "Required option (bare, no brackets)"],
            ]},
            { type: "h3", text: "Validation guarantees" },
            { type: "ul", items: [
                "Required arguments and options must be present.",
                "Defaults are applied when optional values are omitted.",
                "Choice values must be members of the declared set.",
                "`requires:` chains from nested bracket groups are enforced.",
                "Repeating positionals must be the last argument in the list.",
                "Unknown options are rejected with a clear error.",
            ]},
            { type: "h3", text: "YAML anchors" },
            { type: "p", text: "Standard YAML anchors and aliases work for sharing option sets across commands — see the [example app](https://github.com/michaelmunson/x.sh/blob/main/docs/examples/app/exapp.x.yml) for `&demo_verbose` / `*demo_verbose` usage." },
        ],
    },
    {
        id: "builtins",
        title: "Built-in Helpers",
        subtitle: "Bash functions injected before every app handler.",
        blocks: [
            { type: "p", text: "x prepends a small bash preamble to each handler so you can read parsed values without manual environment-variable wrangling. All helpers are available in the handler's scope and exported for `x-run` subprocesses." },
            { type: "table", headers: ["Builtin", "Behavior"], rows: [
                ["`x-opt &lt;name&gt;`", "Print option value. Bool flags → `true`. Repeats join with newline."],
                ["`x-arg &lt;name&gt;`", "Print positional argument value."],
                ["`x-opts &lt;assoc-name&gt;`", "Populate a caller-named bash associative array with all options."],
                ["`x-args &lt;assoc-name&gt;`", "Populate a caller-named bash associative array with all arguments."],
                ["`x-run &lt;cmd&gt; [args…]`", "Run a command with `x-*` helpers still in scope."],
                ["`x-usage &lt;cmd-path&gt;`", "Print auto-generated help (e.g. `x-usage create.file`)."],
                ["`x-io-read …`", "Prompt for a line. `-v` / `--var` assigns to a global scalar."],
                ["`x-io-confirm …`", "Yes/no prompt (`--default yes|no`). `-v` assigns `true`/`false`."],
                ["`x-io-select …`", "Menu of `id=label` pairs. `--multi` for multiple. `-v` assigns result."],
            ]},
            { type: "h3", text: "Reading options and arguments" },
            { type: "code", code: `echo "mode=$(x-opt mode)"
echo "path=$(x-arg path)"

declare -A opts
x-opts opts
echo "\${opts[mode]}"

declare -A args
x-args args
for k in "\${!args[@]}"; do echo "$k=\${args[$k]}"; done` },
            { type: "h3", text: "Repeating options" },
            { type: "code", code: `mapfile -t dirs &lt; &lt;(x-opt dir)
for d in "\${dirs[@]}"; do echo "dir: $d"; done` },
            { type: "h3", text: "Interactive I/O" },
            { type: "code", code: `x-io-read "Name:" -v name
echo "$name"

x-io-confirm "Proceed?" --default no -v ok
[[ "$ok" == "true" ]] && echo "go"

x-io-select -v color "Pick a color" "red=Red" "green=Green" "blue=Blue"
echo "$color"

x-io-select --multi -v colors "Colors" "r=Red" "g=Green"
echo "\${colors[0]}"` },
            { type: "callout", variant: "tip", text: "With `-v` / `--var`, results land in the caller's global shell variable — not stdout. Multi-select stores an indexed array." },
        ],
    },
    {
        id: "examples",
        title: "App Examples",
        subtitle: "Real-world references in the repository.",
        blocks: [
            { type: "h3", text: "Comprehensive example app (`exapp`)" },
            { type: "p", text: "The app definition imports handlers from two sidecar files:" },
            { type: "code", title: "exapp.x.yml", code: EXAPP_X },
            { type: "code", title: "exapp.handlers-a.yml", code: EXAPP_HANDLERS_A },
            { type: "code", title: "exapp.handlers-b.yml", code: EXAPP_HANDLERS_B },
            { type: "link", href: "https://github.com/michaelmunson/x.sh/tree/main/docs/examples/app", text: "View source on GitHub", external: true },
            { type: "h3", text: "Typical workflows" },
            { type: "ul", items: [
                "**Dotfiles scripts** — `x -i` for personal utilities, `x --ln` for PATH access.",
                "**Monorepo tasks** — root `x.yml` for `build`, `test`, `lint`; no global install needed.",
                "**Team CLI tool** — commit `&lt;tool&gt;.x.yml` to the repo; teammates run `x tool subcmd` from any subdirectory.",
                "**Complex internal CLI** — global app in `~/.x.sh/apps/` with `$.import` for large handler sets.",
            ]},
        ],
    },
    {
        id: "validation-help",
        title: "Validation & Help",
        subtitle: "How x keeps apps correct and self-documenting.",
        blocks: [
            { type: "h3", text: "Automatic help" },
            { type: "p", text: "Every command supports `-h` / `--help` out of the box — no extra configuration." },
            { type: "code", code: `x my-app --help
x my-app build --help
x my-app create file --help` },
            { type: "p", text: "The same renderer powers `x-usage &lt;cmd-path&gt;` inside handlers, so you can print context-appropriate help from bash." },
            { type: "h3", text: "Save-time validation (`x -i --app`)" },
            { type: "p", text: "When you finish editing an app file, x performs a full structural check:" },
            { type: "ul", items: [
                "YAML parses cleanly.",
                "Every synopsis string parses into a valid spec.",
                "Leaf commands have corresponding `$` handlers.",
                "No orphan handler keys.",
                "No duplicate options within a command.",
                "No dangling `requires:` references.",
                "`$` and `$.import` are not both present.",
            ]},
            { type: "p", text: "On failure, x lists all errors and offers **edit** (re-open the editor) or **revert**." },
            { type: "h3", text: "Runtime validation" },
            { type: "p", text: "A lighter validation pass runs on every `x &lt;app&gt;` invocation so hand-edited or stale files fail fast with readable errors before any bash executes." },
        ],
    },
    {
        id: "configuration",
        title: "Configuration",
        subtitle: "Defaults, metadata, and persistent settings.",
        blocks: [
            { type: "h3", text: "`x --config`" },
            { type: "p", text: "Interactive menu with two options:" },
            { type: "h4", text: "Default Script Language" },
            { type: "p", text: "Sets `default_program` in `~/.x.sh/config.json`. Pre-selected during `x -i` and used for `x.yml` inline execution." },
            { type: "code", title: "config.json", code: `{
  "default_program": "bash"
}` },
            { type: "h4", text: "LLM Provider" },
            { type: "p", text: "Opens your editor with a template script saved to `~/.x.sh/config/llm.sh`. The script must accept the prompt as `$1` and print the completion to stdout." },
            { type: "code", title: "llm.sh example", code: `#!/bin/bash
PROMPT="$1"
ollama run llama2 "$PROMPT"` },
            { type: "h3", text: "Environment" },
            { type: "ul", items: [
                "`$EDITOR` — used by `x -i`, `x -i --app`, and LLM config.",
                "`$PATH` — must include `~/.local/bin` for `x --ln` symlinks.",
            ]},
        ],
    },
    {
        id: "ai",
        title: "AI Integration",
        subtitle: "Generate commands and scripts with your LLM of choice.",
        blocks: [
            { type: "p", text: "x does not bundle an LLM. Instead, you provide a shell script that forwards prompts to any backend — Ollama, OpenAI CLI, a custom API wrapper, etc." },
            { type: "h3", text: "`x --ai`" },
            { type: "p", text: "Prompts for natural-language instructions, generates a shell command, displays it, and optionally executes after confirmation." },
            { type: "code", code: `x --ai
# instructions&gt; aws command to list parameters
#
# Generated Command
# aws ssm describe-parameters
# Run command? [y/N]` },
            { type: "h3", text: "`x -i --ai [name]`" },
            { type: "p", text: "Generates a complete script via LLM and saves it through the normal `x -i` workflow (metadata + file)." },
            { type: "callout", variant: "info", text: "Configure your provider first with `x --config` → LLM Provider. Without `~/.x.sh/config/llm.sh`, AI commands will fail." },
        ],
    },
    {
        id: "languages",
        title: "Supported Languages",
        subtitle: "Interpreters and compilers available during `x -i`.",
        blocks: [
            { type: "p", text: "Each language maps to a `program` value stored in script metadata. x invokes `&lt;program&gt; &lt;script-file&gt; [args…]` for global scripts." },
            { type: "table", headers: ["Language", "Program", "Notes"], rows: [
                ["Bash", "bash", "Default for most shell scripts"],
                ["Zsh", "zsh", ""],
                ["POSIX Shell", "sh", ""],
                ["Node.js", "node", "JavaScript / TypeScript"],
                ["Python 3", "python", ""],
                ["Python 2", "python2", ""],
                ["Ruby", "ruby", ""],
                ["Perl", "perl", ""],
                ["Go", "go", "Compiled — use `go run` style in script"],
                ["Rust", "rust", ""],
                ["PHP", "php", ""],
                ["Lua", "lua", ""],
                ["Deno", "deno", "JS/TS runtime"],
                ["Swift", "swift", ""],
                ["C", "c", ""],
                ["C++", "cpp", ""],
                ["Java", "java", ""],
                ["R", "r", ""],
                ["AWK", "awk", ""],
                ["Elixir", "elixir", ""],
                ["Clojure", "clj", ""],
                ["Scala", "scala", ""],
                ["Haskell", "haskell", ""],
                ["PowerShell", "powershell", ""],
                ["Kotlin", "kotlin", ""],
            ]},
        ],
    },
    {
        id: "faq",
        title: "FAQ",
        subtitle: "Common questions and troubleshooting.",
        blocks: [
            { type: "h3", text: "Why does `x my-script` not find my `x.yml` entry?" },
            { type: "p", text: "`x.yml` is only read from the **current working directory**, not parent directories. `cd` to the directory containing `x.yml`, or use a global script / app instead." },
            { type: "h3", text: "Why does a local app work from a subdirectory but `x.yml` does not?" },
            { type: "p", text: "By design. Apps walk up the directory tree; `x.yml` does not. Put shared project commands in a root-level app or keep `x.yml` usage to single-directory workflows." },
            { type: "h3", text: "`x --ln` created a symlink but the command is not found" },
            { type: "p", text: "Ensure `~/.local/bin` is on your `PATH`. Run `echo $PATH` and verify. You may need to open a new shell after updating your rc file." },
            { type: "h3", text: "My app fails validation after editing" },
            { type: "p", text: "Run `x -i --app --local &lt;name&gt;` (or `--global`) to re-enter the edit/revert loop. Common issues: missing handler for a leaf command, invalid synopsis syntax, or both `$` and `$.import` present." },
            { type: "h3", text: "Can I use extensions in script names?" },
            { type: "p", text: "Script **names** you pass to `x` are extensionless (e.g. `x c-test`). The file on disk can include an extension (`c-test.c`); x resolves by basename match." },
            { type: "h3", text: "Does `x --delete` remove symlinks?" },
            { type: "p", text: "No. Use `x -d --ln &lt;name&gt;` to remove symlinks separately." },
            { type: "h3", text: "Where is the source code?" },
            { type: "link", href: "https://github.com/michaelmunson/x.sh", text: "github.com/michaelmunson/x.sh", external: true },
        ],
    },
];
