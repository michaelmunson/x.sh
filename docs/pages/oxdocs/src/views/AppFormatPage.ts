import { Paragraph, Heading, CodeBlock, BulletList, Callout } from "../components";

export default function AppFormatPage() {
    return [
        Paragraph("This is the same file format used by project-local `x.yml` files — see [Project-local Scripts](/docs/project-local). Commands are declared with **dot-prefixed keys** (`.command-name:`), never `commands:`."),
        CodeBlock(`name: my-app
version: 0.0.0
description: an example app

options:
  - "[-v | --version]"

.build:
  description: build the project
  options:
    - "[--input=<file> [--output=<dir>]]"
    - "[--mode={fast|safe|deep}]"
  arguments:
    - "<assets>..."
  $: |
    echo "mode=$(x-opt mode), assets=$(x-arg assets)"

.create:
  description: create things
  $: x-usage create

  .file:
    description: create a file
    arguments: "<path> [<content='empty'>]"
    $: |
      echo "creating $(x-arg path) with $(x-arg content)"

  .folder:
    arguments: "<path>"
    $: |
      mkdir -p "$(x-arg path)"`, "Basic app", "xsh"),
        Heading(3, "Top-level fields"),
        BulletList([
            "`name`, `version`, `description`/`help` — metadata shown in help output (`name` defaults to the filename without `.x.yml`/`.yml` if omitted; `help`/`description` are aliases).",
            "`options`/`opts` — flags available on the root command (aliases; string, multiline string, or list).",
            "`arguments`/`args` — positional args on the root command (aliases; string, multiline string, or list).",
            "`dir` — working directory scripts run in (relative to the app file, or absolute).",
            "`$` — the root command's own inline script, run on bare `x my-app`.",
            "`import` — load scripts and/or env from external files.",
            "`env` — inline environment variables and named groups.",
            "`.command-name` — a subcommand: a string (shorthand inline script) or a mapping (see below).",
        ]),
        Callout("warn", "`commands:` and a top-level `$:` handler map are **removed** — define commands with `.command-name:` keys, and put each command's script inline under its own `$:`."),
        Heading(3, "Command nodes"),
        Paragraph("Each `.command-name:` entry is either a **string** (shorthand for `{ $: <string> }`) or a **mapping** with any of `description`/`help`, `options`/`opts`, `arguments`/`args`, `dir`, `env`, `alias`, `$`, and nested `.sub-command:` keys. Every leaf command needs its own `$:` (or `alias:`)."),
        CodeBlock(`.docs: cargo doc --no-deps    # shorthand — equivalent to { $: cargo doc --no-deps }`, undefined, "xsh"),
        Heading(3, "Non-leaf groups"),
        Paragraph("A command with subcommands but no `$:` of its own auto-prints help when invoked bare. To customize that, give it its own `$:` — a one-liner like `x-usage create` prints the same generated help explicitly."),
        Heading(3, "Working directory (`dir`)"),
        Paragraph("Set `dir` to change the process working directory before a script runs. Relative paths resolve from the directory containing the app file; absolute paths are used as-is. The directory must exist at load time. `dir` set on a command applies to that command and all its subcommands, unless a subcommand sets its own."),
        CodeBlock(`name: my-app
dir: ./app

.show:
  description: print the handler working directory
  $: |
    pwd`, "App with handler working directory", "xsh"),
        Paragraph("With the app file at `./my-app.x.yml` and a subdirectory `./app/`, running `x my-app show` executes the script with `./app` as the current working directory."),
        Heading(3, "Alias commands"),
        Paragraph("An `alias:` command dispatches its remaining arguments to another x file, as if you'd invoked that file directly. Alias commands may define **only** `alias` and `help`/`description` — no `options`, `arguments`, `dir`, `env`, `$`, or nested commands."),
        CodeBlock(`.legacy:
  alias: ./tools/legacy.x.yml`, undefined, "xsh"),
        Heading(3, "Imports"),
        Paragraph("Split scripts into sidecar files instead of inline `$:` entries. Each import file is a flat YAML map of `dotted.path: <bash body>`. Paths resolve relative to the app file. Duplicate keys across import files are an error; inline `$:` entries override imported scripts with the same key."),
        CodeBlock(`name: my-app
version: 0.0.0

.greet:
  arguments: "<name>"

import:
  $:
    - ./handlers.yml`, "App with handler import", "xsh"),
        CodeBlock(`greet: |
  echo "Hello, $(x-arg name)"`, "handlers.yml", "xsh-handlers"),
        Paragraph("Legacy form (still supported, mutually exclusive with `import.$`):"),
        CodeBlock(`"$.import":
  - ./handlers.yml`, undefined, "xsh"),
        Heading(3, "Environment"),
        Paragraph("Inline `env:` defines globals available before scripts run. Named groups (keys starting with `.`) are loaded on demand via `x-env-load`. You can also import a classic `.env` file through `import.env`. Like `dir`, `env` set on a command merges downward to its subcommands."),
        CodeBlock(`name: my-app
version: 0.0.0

env:
  HELLO: world
  .staging:
    API_URL: https://staging.example.com
  .production:
    API_URL: https://example.com

.deploy:
  arguments: "<target>"
  $: |
    x-env-load ".$(x-arg target)"
    echo "$HELLO → $API_URL"

import:
  env: ./shared.env
  sh:
    - ./helpers/example.sh
    - ./helpers/example2.sh
  $:
    - ./handlers/deploy.yml`, "App with inline and imported env", "xsh"),
        CodeBlock(`DATABASE_URL=postgres://localhost/mydb
LOG_LEVEL=info`, "shared.env", "text"),
        Paragraph("Imported `.env` globals merge first; inline `env:` values override on duplicate keys."),
    ];
}
