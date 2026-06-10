import { Paragraph, Heading, CodeBlock, Callout, BulletList } from "../components";

export default function OverviewPage() {
    return [
        Paragraph("**x** is a script management CLI that helps you create, organize, and run scripts — global utilities in `~/.x.sh/scripts/`, project-local tasks in `./x.yml`, and full multi-command apps in `&lt;name&gt;.x.yml` files."),
        Heading(3, "What you can do"),
        BulletList([
            "**Global scripts** — create with `x -i`, run with `x &lt;name&gt;`, optionally symlink into PATH with `x --ln`.",
            "**Project-local commands** — version-controlled task runners in `./x.yml` for the current working directory.",
            "**Apps** — YAML files with options, arguments, nested subcommands, and bash handlers; input is validated before any handler runs.",
            "**AI helpers** — `x --ai` and `x -i --ai` when an LLM provider is configured via `x --config`.",
        ]),
        Heading(3, "Command resolution"),
        Paragraph("When you run `x &lt;name&gt; …`, x walks this chain — first match wins:"),
        CodeBlock(`1. ./x.yml          → project-local entry (CWD only)
2. ./<name>.x.yml    → local app (CWD)
3. ~/.x.sh/apps/     → global app
4. ~/.x.sh/scripts/  → installed global script`, undefined, "text"),
        Callout("tip", "Use `x --src &lt;name&gt;` to print the absolute path of a script or app file without executing it. Local `x.yml` keys are not matched — they are inline scripts, not file paths."),
    ];
}
