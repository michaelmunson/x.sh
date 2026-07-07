import { Paragraph, Heading, CodeBlock, Callout, BulletList } from "../components";

export default function ProjectLocalPage() {
    return [
        Paragraph("You can define **project-local** commands in a YAML file named `x.yml` in the **current working directory**. When you run `x &lt;name&gt; …`, `x` loads `./x.yml` (if present). If the file defines a top-level command matching `&lt;name&gt;`, that command is resolved and executed. If there is no `x.yml`, or it does not define `&lt;name&gt;`, `x` looks up an app or a **global** script under `~/.x.sh/scripts` as usual."),
        Callout("info", "`x.yml` uses **exactly the same file format** as app files (see [App File Format](/docs/app-format)) — the only difference is that it's invoked without an app name prefix, and `name:` is never required."),
        Heading(3, "File format"),
        Paragraph("Commands are declared with **dot-prefixed keys**. Each `.command-name:` value is either:"),
        BulletList([
            "**A string** — shorthand inline script (often a block scalar with `|`). Extra CLI arguments after `&lt;name&gt;` are passed as positional parameters (`$1`, `$2`, …), or parsed with `options`/`arguments` if declared.",
            "**A mapping** — `description`/`help`, `options`/`opts`, `arguments`/`args`, `dir`, `env`, `alias`, an own `$:` script, and/or nested `.sub-command:` keys.",
        ]),
        CodeBlock(`.build: |
  VERSION="$1"
  npm run build
  npm version "$VERSION"
  git push --tags

.deploy:
  $: |
    echo "Deploying (default)"
    x deploy dev
  .dev: npm run deploy:dev
  .prod: npm run deploy:prod
  .test:
    .unit: npx jest unit
    .integration: npx jest integration`, "x.yml", "xsh-local"),
        BulletList([
            "`x build 1.2.3` runs `build` with `1.2.3` as `$1`.",
            "`x deploy` runs `deploy`'s own `$`.",
            "`x deploy prod` runs `deploy.prod`.",
            "`x deploy test unit` runs `deploy.test.unit`.",
        ]),
        Callout("info", "If a command has subcommands but no `$:` of its own, invoking it bare (e.g. `x deploy` when only nested `.dev`/`.prod`/`.test` exist) auto-prints help instead of erroring."),
        Heading(3, "Execution"),
        Paragraph("Local scripts always run as **bash**, with the same `x-opt`/`x-arg`/`x-usage` preamble injected before every app script. Unlike global scripts (which respect `default_program` / per-script metadata), `x.yml` and app commands are bash-only."),
        Heading(3, "Notes"),
        BulletList([
            "Local scripts do not use per-script metadata files — the app engine always runs them as bash.",
            "If `x.yml` exists but is invalid YAML (or uses removed v3 syntax like `commands:`), parsing fails with a clear error before falling back to a global script — fix the file.",
            "Activity tracking in `x --ls` applies to global scripts; local `x.yml` runs are not recorded there.",
        ]),
    ];
}
