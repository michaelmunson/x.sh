import { Paragraph, Heading, CodeBlock, BulletList } from "../components";

export default function ValidationHelpPage() {
    return [
        Heading(3, "Help"),
        Paragraph("Every command supports `-h` / `--help` automatically:"),
        CodeBlock(`x my-app --help
x my-app build --help
x my-app create file --help`, undefined, "shell"),
        Paragraph("The same renderer powers `x-usage &lt;cmd-path&gt;` so handlers can print context-appropriate help."),
        Heading(3, "Save-time validation"),
        Paragraph("When you save an app via `x -i --app`, `x` parses the YAML, parses every synopsis string, and runs structural checks:"),
        BulletList([
            "Missing handlers for leaf commands.",
            "Unknown handler keys.",
            "Duplicate options within a command.",
            "Dangling `requires:` references.",
            "Conflicting `$` / `$.import` usage.",
        ]),
        Paragraph("On failure you can **edit** the file again or **revert** to the prior contents."),
        Heading(3, "Runtime validation"),
        Paragraph("A lighter-weight validate also runs on every `x &lt;app&gt;` invocation so a stale or hand-edited file fails fast with a readable error before any bash executes."),
    ];
}
