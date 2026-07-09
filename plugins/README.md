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
| every operation | `[-i \| --interactive]` |
| every operation | `[-q \| --query <expr>]` (jq query on response) |
| every operation | `[-o \| --output {text\|json}]` (response format) |
