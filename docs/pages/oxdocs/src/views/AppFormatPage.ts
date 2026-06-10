import { Paragraph, Heading, CodeBlock, BulletList } from "../components";

export default function AppFormatPage() {
    return [
        CodeBlock(`name: my-app
version: 0.0.0
description: an example app

options:
  - "[-v | --version]"

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

$:
  build: |
    echo "mode=$(x-opt mode), assets=$(x-arg assets)"
  create: x-usage create
  create.file: |
    echo "creating $(x-arg path) with $(x-arg content)"
  create.folder: |
    mkdir -p "$(x-arg path)"`, "Basic app", "xsh"),
        Heading(3, "Top-level fields"),
        BulletList([
            "`name`, `version`, `description` — metadata shown in help output.",
            "`options` — flags available on the root command.",
            "`arguments` — positional args on the root command.",
            "`commands` — nested subcommand tree.",
            "`dir` — working directory handlers run in (relative to the app file, or absolute).",
            "`$` — inline bash handler map.",
            "`import` — load handlers and/or env from external files.",
            "`env` — inline environment variables and named groups.",
        ]),
        Heading(3, "Command nodes"),
        Paragraph("Each entry under `commands` can have `description`, `options`, `arguments`, nested `commands`, or any combination. Leaf commands need a matching handler key in `$` (e.g. `build` → `$: build:`)."),
        Heading(3, "Non-leaf groups"),
        Paragraph("A command with subcommands but no direct handler can use a one-liner like `create: x-usage create` to print help when invoked without a subcommand."),
        Heading(3, "Working directory (`dir`)"),
        Paragraph("Set `dir` to change the process working directory before any handler bash runs. Relative paths resolve from the directory containing the app file; absolute paths are used as-is. The directory must exist at load time."),
        CodeBlock(`name: my-app
dir: ./app

commands:
  show:
    description: print the handler working directory

$:
  show: |
    pwd`, "App with handler working directory", "xsh"),
        Paragraph("With the app file at `./my-app.x.yml` and a subdirectory `./app/`, running `x my-app show` executes the handler with `./app` as the current working directory."),
        Heading(3, "Imports"),
        Paragraph("Split handlers into sidecar files instead of an inline `$:` block. Paths resolve relative to the app file. Duplicate keys across import files are an error; inline `$:` entries override imported handlers with the same key."),
        CodeBlock(`name: my-app
version: 0.0.0

commands:
  greet:
    arguments: "<name>"

import:
  $:
    - ./handlers.yml`, "App with handler import", "xsh"),
        CodeBlock(`greet: |
  echo "Hello, $(x-arg name)"`, "handlers.yml", "xsh-handlers"),
        Paragraph("Legacy form (still supported):"),
        CodeBlock(`"$.import":
  - ./handlers.yml`, undefined, "xsh"),
        Heading(3, "Environment"),
        Paragraph("Inline `env:` defines globals available before handlers run. Named groups (keys starting with `.`) are loaded on demand via `x-env-load`. You can also import a classic `.env` file through `import.env`."),
        CodeBlock(`name: my-app
version: 0.0.0

env:
  HELLO: world
  .staging:
    API_URL: https://staging.example.com
  .production:
    API_URL: https://example.com

commands:
  deploy:
    arguments: "<target>"

import:
  env: ./shared.env
  sh: 
    - ./helpers/example.sh
    - ./helpers/example2.s
  $:
    - ./handlers/deploy.yml

# or define inline
# $:
#   deploy: |
#     x-env-load ".$(x-arg target)"
#     echo "$HELLO → $API_URL"`, "App with inline and imported env", "xsh"),
        CodeBlock(`DATABASE_URL=postgres://localhost/mydb
LOG_LEVEL=info`, "shared.env", "text"),
        Paragraph("Imported `.env` globals merge first; inline `env:` values override on duplicate keys."),
    ];
}
