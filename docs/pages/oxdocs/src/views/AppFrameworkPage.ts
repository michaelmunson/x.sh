import { Paragraph, Heading, CodeBlock, BulletList } from "../components";

export default function AppFrameworkPage() {
    return [
        Paragraph("Apps turn a single YAML file into a multi-command CLI. Unlike `x.yml` (which is a flat map of inline scripts), an app file declares **options**, **arguments**, **nested commands**, and a `$:` block of bash handlers. `x` parses the synopsis strings and validates the user's input *before* any handler runs."),
        Heading(3, "Where apps live"),
        BulletList([
            "**Local:** `./&lt;name&gt;.x.yml` in the current directory.",
            "**Global:** `~/.x.sh/apps/&lt;name&gt;.x.yml`.",
        ]),
        Heading(3, "Resolution order"),
        Paragraph("When you run `x &lt;name&gt; …`:"),
        CodeBlock(`1. ./x.yml entry          (project-local)
2. ./<name>.x.yml         (local app)
3. ~/.x.sh/apps/<name>.x.yml   (global app)
4. ~/.x.sh/scripts/<name>      (installed script)`, undefined, "text"),
        Heading(3, "Creating an app"),
        Paragraph("Use [`x -i --app`](/docs/commands) to create a seed file and open your editor:"),
        CodeBlock(`x -i --app my-app             # prompts for scope
x -i --app --local my-app     # ./my-app.x.yml
x -i --app --global my-app    # ~/.x.sh/apps/my-app.x.yml`, undefined, "shell"),
        Paragraph("After the editor closes, `x` validates the YAML structure, parses every synopsis string, and checks handler coverage. On failure you can **edit** (re-open) or **revert** (restore prior file or delete the new one)."),
        Paragraph("See [App File Format](/docs/app-format) for the full schema, [Synopsis DSL](/docs/synopsis) for option/argument strings, and [Built-in Helpers](/docs/builtins) for handler bash functions."),
    ];
}
