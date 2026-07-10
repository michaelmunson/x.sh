# x.sh plugins

Plugins are separate Rust packages under `plugins/`. They are installed into
`~/.x.sh/plugins/` (never on `PATH`) and invoked through `x`.

## Install

```bash
x -i --plugin <name>
```

Builds `plugins/<name>` from this repository (or clones it) and installs the
binary as `~/.x.sh/plugins/<name>`.

## Run

```bash
x --plugin <name> [args…]
```

## openapi

Convert an OpenAPI 3.x specification (`.json` / `.yaml` / `.yml`) into an
x.sh app (`.x.yml`):

```bash
x -i --plugin openapi
x --plugin openapi ./petstore.openapi.yaml
# → writes ./petstore.x.yml

x --plugin openapi ./spec.json -o ./my-api.x.yml
x --plugin openapi ./spec.yaml --stdout
```

Mapping overview:

| OpenAPI | x.sh |
|---------|------|
| `info.title` | `name` (slugified; trailing `-api` dropped) |
| `info.version` | `version` |
| `info.description` | `description` |
| `servers[0].url` | `env.BASE_URL` |
| `operationId` | `.<command>` |
| `in: query` | `options` |
| `in: path` | `arguments` |
| `requestBody` JSON properties | `[-d \| --data <fieldval>...]` |
| every operation | `[-i]` |
| every operation | `[-q <expr>]` (jq query on response) |
| every operation | `[-o <{text\|json}>]` (response format) |

## ai-cmd-gen

Generate a shell command from natural-language instructions, edit it in your
shell, then run it. Requires shell completion to be sourced so the `x()`
wrapper can intercept `-A` / `--ai` (those flags only exist when this plugin
is installed):

```bash
x -i --plugin ai-cmd-gen
source <(x __complete bash)   # or: source <(x __complete zsh)

x --ai --config               # configure ~/.x.sh/config/llm.sh
x -A "list the 10 largest files here"
x --ai find all rust files modified today
x -A                          # prompts for instructions when omitted
```

If the LLM provider is not configured, generation fails with a message to run
`x --ai --config`.

Flow: plugin calls `~/.x.sh/config/llm.sh` with a shell-specific system prompt
plus your instructions → generated command is shown for editing (`read -e` in
bash, `vared` in zsh) → added to shell history → `eval`.

Direct plugin invocation (without the shell wrapper):

```bash
x --plugin ai-cmd-gen --config
x --plugin ai-cmd-gen --shell zsh list files by size
```
