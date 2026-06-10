import Prism from "prismjs";
import "prismjs/components/prism-yaml";
import "prismjs/components/prism-bash";
import "prismjs/components/prism-json";
import "prismjs/themes/prism.css";

export type CodeLang = "xsh" | "xsh-handlers" | "xsh-local" | "bash" | "shell" | "json" | "text";

export function inferLang(title?: string, explicit?: CodeLang): CodeLang {
    if (explicit) return explicit;
    if (!title) return "text";
    const lower = title.toLowerCase();
    if (lower.includes(".handlers") || lower.endsWith(".x.yml")) return "xsh";
    if (lower === "x.yml" || lower.endsWith("/x.yml")) return "xsh-local";
    if (lower.endsWith(".json") || lower === "config.json") return "json";
    if (lower.endsWith(".sh") || lower.includes("llm")) return "bash";
    if (lower.includes("metadata")) return "text";
    return "shell";
}

function toPrismLang(lang: CodeLang): string | null {
    switch (lang) {
        case "xsh":
        case "xsh-handlers":
        case "xsh-local": return "yaml";
        case "bash":
        case "shell": return "bash";
        case "json": return "json";
        default: return null;
    }
}

export function highlightCode(code: string, lang: CodeLang): string | null {
    const prismLang = toPrismLang(lang);
    if (!prismLang) return null;
    const grammar = Prism.languages[prismLang];
    if (!grammar) return null;
    return Prism.highlight(code, grammar, prismLang);
}

export function prismLanguageClass(lang: CodeLang): string {
    const prismLang = toPrismLang(lang);
    return prismLang ? `language-${prismLang}` : "code-content";
}
