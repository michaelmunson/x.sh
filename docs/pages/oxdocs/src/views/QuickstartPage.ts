import { Paragraph, Heading, CodeBlock } from "../components";

export default function QuickstartPage() {
    return [
        Heading(3, "1. Create your first script"),
        CodeBlock(`x --init hello-world
# or: x -i hello-world
# Follow the interactive prompts to select language and add description
# Your editor will open for you to write the script`, undefined, "shell"),
        Heading(3, "2. Run your script"),
        CodeBlock(`x hello-world`, undefined, "shell"),
        Heading(3, "3. List all scripts"),
        CodeBlock(`x --ls
# or: x -l`, undefined, "shell"),
        Heading(3, "4. Add script to PATH"),
        CodeBlock(`x --ln hello-world`, undefined, "shell"),
        Paragraph("You can now run `hello-world` directly without the `x` prefix (requires `~/.local/bin` on your PATH)."),
    ];
}
