import { Paragraph, Heading, CodeBlock, Callout, BulletList, OrderedList } from "../components";

export default function InstallationPage() {
    return [
        Heading(3, "Option 1: Automated installation script"),
        CodeBlock(`curl -fsSL https://raw.githubusercontent.com/michaelmunson/x.sh/main/install.sh | bash`, undefined, "shell"),
        Paragraph("Or download and run manually:"),
        CodeBlock(`wget https://raw.githubusercontent.com/michaelmunson/x.sh/main/install.sh
bash install.sh`, undefined, "shell"),
        Heading(4, "Requirements"),
        BulletList([
            "**Rust/Cargo** — the script will prompt if not installed.",
            "**Git** — for cloning the repository.",
            "Internet connection.",
        ]),
        Heading(4, "What the script does"),
        OrderedList([
            "Checks for Rust/Cargo installation.",
            "Clones the repository (or uses the current directory if already in the repo).",
            "Builds the binary in release mode.",
            "Installs to `~/.local/bin/`.",
            "Verifies the installation.",
        ]),
        Callout("warn", "Make sure `~/.local/bin` is in your PATH. Add `export PATH=\"$HOME/.local/bin:$PATH\"` to `~/.bashrc`, `~/.zshrc`, or `~/.profile`.", "PATH"),
        Heading(3, "Option 2: Manual build"),
        Paragraph("If you prefer to build manually:"),
        CodeBlock(`git clone https://github.com/michaelmunson/x.sh.git
cd x.sh
cargo build --release`, undefined, "shell"),
        Paragraph("The binary will be at `target/release/x`. You can copy it to a directory in your PATH (e.g. `~/.local/bin/`), create an alias in your shell configuration, or use it directly with the full path."),
        Heading(3, "Shell completion"),
        Paragraph("Tab completion follows the same command resolution order as runtime (`x.yml` → apps → global scripts), including nested subcommands."),
        Paragraph("**Bash:**"),
        CodeBlock(`mkdir -p ~/.local/share/bash-completion/completions
x __complete bash > ~/.local/share/bash-completion/completions/x`, undefined, "bash"),
        Paragraph("**Zsh** (add to `~/.zshrc`):"),
        CodeBlock(`source <(x __complete zsh)
# or (if that doesn't work)
source <(x __complete bash)`, undefined, "bash"),
    ];
}
