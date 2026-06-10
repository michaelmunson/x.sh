import { Paragraph, Heading, CodeBlock, Callout, BulletList, OrderedList } from "../components";

export default function CommandsPage() {
    return [
        Heading(3, "`x --init [name] [script]` or `x -i [name] [script]`"),
        Paragraph("Create a new script or initialize editing of an existing one."),
        Paragraph("**Interactive mode (recommended):**"),
        CodeBlock(`x --init
# or: x -i
# Prompts for:
# - Script name (validated: alphanumeric + dashes only)
# - Description (optional)
# - Programming language (from supported list)
# - Opens editor for script content`, undefined, "shell"),
        Paragraph("**With script name:**"),
        CodeBlock(`x --init my-script
# or: x -i my-script
# Prompts for description and language, then opens editor`, undefined, "shell"),
        Paragraph("If `my-script` already exists, you are prompted for the script name first so you can rename it before the editor opens (metadata and usage history follow the new name)."),
        Paragraph("**With script name and content:**"),
        CodeBlock(`x --init my-script "echo 'Hello, World!'"
# or: x -i my-script "echo 'Hello, World!'"
# Creates script with provided content (uses default or existing program)`, undefined, "shell"),
        Callout("info", "Script names must contain only letters, numbers, and dashes, and cannot be empty."),
        Heading(3, "`x &lt;script_name&gt; [args…]`"),
        Paragraph("Execute a script with optional arguments. The script runs with the program specified for that script (or the default program). Arguments are passed directly to the script."),
        CodeBlock(`x curl-google
x curl-google /docs
x my-script arg1 arg2 arg3`, undefined, "shell"),
        Paragraph("If `./x.yml` exists in the **current working directory** and defines a top-level entry matching `&lt;script_name&gt;`, that local definition runs instead of a global script. Otherwise `x` falls back to installed scripts. See [Project-local Scripts](/docs/project-local)."),
        Heading(3, "`x --ls` or `x -l`"),
        Paragraph("List all available scripts with detailed information in a formatted table."),
        CodeBlock(`x --ls
# or: x -l`, undefined, "shell"),
        Heading(3, "`x --delete &lt;name&gt;` or `x -d &lt;name&gt;`"),
        Paragraph("Remove a script and its associated metadata."),
        CodeBlock(`x --delete my-script
# or: x -d my-script`, undefined, "shell"),
        Callout("warn", "This does not remove symlinks created with `x --ln`. Remove those separately if needed."),
        Heading(3, "`x --ln &lt;name&gt; [link_name]`"),
        Paragraph("Create a symbolic link to a script in `~/.local/bin/`, allowing you to run the script without the `x` prefix."),
        CodeBlock(`x --ln my-script
# Now you can run: my-script

x --ln my-script my-alias
# Now you can run: my-alias

x --delete --ln my-script
# or: x -d --ln my-script`, undefined, "shell"),
        BulletList([
            "The `~/.local/bin/` directory is created automatically if it doesn't exist.",
            "Make sure `~/.local/bin` is in your PATH to use linked scripts directly.",
            "Existing links are overwritten when creating a new link.",
        ]),
        Heading(3, "`x --ai`"),
        Callout("info", "Requires LLM provider configuration (see `x --config` below)."),
        Paragraph("Generate CLI commands via LLM. Prompts for instructions, generates a command, and optionally executes it."),
        CodeBlock(`x --ai
# instructions> aws command to list parameters
#
# Generated Command
# aws ssm describe-parameters
# Run command? [y/N]`, undefined, "shell"),
        Heading(3, "`x --init --ai [name]`"),
        Paragraph("Generate a script via LLM and save it through the normal `x -i` workflow."),
        CodeBlock(`x --init --ai my-script`, undefined, "shell"),
        Heading(3, "`x --src &lt;name&gt;`"),
        Paragraph("Print the absolute path of a script or app file without running it. Useful for piping into editors or scripts."),
        CodeBlock(`x --src my-script
# /home/you/.x.sh/scripts/my-script

x --src my-app
# /home/you/.x.sh/apps/my-app.x.yml`, undefined, "shell"),
        Paragraph("Resolution order:"),
        OrderedList([
            "Local app: `./&lt;name&gt;.x.yml` (current directory)",
            "Global app: `~/.x.sh/apps/&lt;name&gt;.x.yml`",
            "Global script: `~/.x.sh/scripts/&lt;name&gt;` (matched with or without extension)",
        ]),
        Paragraph("Local `x.yml` keys are intentionally not matched — they are inline strings, not file paths."),
        Heading(3, "`x -i --app [--local | --global] [&lt;name&gt;]`"),
        Paragraph("Create a new app config (`&lt;name&gt;.x.yml`). See the [App Framework](/docs/app-framework) section for the file format."),
        CodeBlock(`x -i --app my-app             # prompts for scope
x -i --app --local my-app     # creates ./my-app.x.yml
x -i --app --global my-app    # creates ~/.x.sh/apps/my-app.x.yml`, undefined, "shell"),
        Paragraph("After the editor closes, `x` validates the file. On failure it prints all errors and offers **edit** (re-open the editor) or **revert** (restore the previous file or delete the new one)."),
        Heading(3, "`x --config`"),
        Paragraph("Configure default settings for `x`."),
        CodeBlock(`x --config
# Configuration Options:
#   > Default Script Language
#   > LLM Provider`, undefined, "shell"),
        Paragraph("**Default Script Language** — sets the default programming language for new scripts. The selected default is pre-selected when creating new scripts via `x -i` and used for `x.yml` inline execution."),
        Paragraph("**LLM Provider** — opens your editor with a template script saved to `~/.x.sh/config/llm.sh`. The script must accept a prompt as `$1` and output the completion to stdout."),
        CodeBlock(`#!/bin/bash
PROMPT="$1"
ollama run llama2 "$PROMPT"`, "llm.sh example", "bash"),
    ];
}
