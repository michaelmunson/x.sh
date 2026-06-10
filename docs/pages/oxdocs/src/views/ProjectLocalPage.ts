import { Paragraph, Heading, CodeBlock, Callout, BulletList } from "../components";

export default function ProjectLocalPage() {
    return [
        Paragraph("You can define **project-local** commands in a YAML file named `x.yml` in the **current working directory**. When you run `x &lt;name&gt; …`, `x` loads `./x.yml` (if present). If the file defines a top-level key matching `&lt;name&gt;`, that entry is resolved and executed. If there is no `x.yml`, or it does not define `&lt;name&gt;`, `x` looks up a **global** script under `~/.x.sh/scripts` as before."),
        Heading(3, "File format"),
        Paragraph("Each top-level key is a command name. The value is either:"),
        BulletList([
            "**A string** — inline script (often a block scalar with `|`). Extra CLI arguments after `&lt;name&gt;` are passed as positional parameters (`$1`, `$2`, …).",
            "**A mapping** — subcommands and an optional default: `$` runs when you invoke the parent with no further arguments; any other key is a nested subcommand (mappings can nest arbitrarily deep).",
        ]),
        CodeBlock(`build: |
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
    integration: npx jest integration`, "x.yml", "xsh-local"),
        BulletList([
            "`x build 1.2.3` runs `build` with `1.2.3` as `$1`.",
            "`x deploy` runs `deploy.$`.",
            "`x deploy prod` runs `deploy.prod`.",
            "`x deploy test unit` runs `deploy.test.unit`.",
        ]),
        Callout("info", "If a mapping has **no** `$` key and you invoke the parent with no subcommand arguments (e.g. `x deploy` when only nested keys exist), `x` reports an error asking you to add `$` or pass a subcommand."),
        Heading(3, "Execution"),
        Paragraph("Local scripts are executed with your configured **default program** (`x --config`, or `default_program` in `~/.x.sh/config.json`), same as other inline shell-style usage — typically **`bash`**. The implementation runs `&lt;program&gt; -c '&lt;script&gt;' x &lt;args…&gt;`, so `$1` in the script refers to the first argument after the command name."),
        Heading(3, "Notes"),
        BulletList([
            "Local scripts do not use per-script metadata files; only the default program applies.",
            "If `x.yml` exists but is invalid YAML, parsing fails before falling back to a global script — fix or rename the file.",
            "Activity tracking in `x --ls` applies to global scripts; local `x.yml` runs are not recorded there.",
        ]),
    ];
}
