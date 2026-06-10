import { DIV, PRE, BUTTON, SPAN } from "oxidizer";
import { renderInline } from "./inline";
import { highlightCode, inferLang, prismLanguageClass, type CodeLang } from "../prism";

function highlightedCodeEl(code: string, lang: CodeLang): HTMLElement {
    const el = document.createElement("code");
    el.className = prismLanguageClass(lang);
    const html = highlightCode(code, lang);
    if (html) el.innerHTML = html;
    else el.textContent = code;
    return el;
}

export function CodeBlock(code: string, title?: string, lang?: CodeLang) {
    const language = inferLang(title, lang);

    return DIV({ className: "code-block" },
        title ? DIV({ className: "code-block-header" },
            SPAN({ className: "code-block-title" }, ...renderInline(title)),
            BUTTON({
                className: "copy-btn",
                type: "button",
                onclick(e: Event) {
                    const btn = e.currentTarget as HTMLButtonElement;
                    navigator.clipboard.writeText(code).then(() => {
                        const prev = btn.textContent;
                        btn.textContent = "Copied!";
                        setTimeout(() => { btn.textContent = prev; }, 1500);
                    });
                }
            }, "Copy")
        ) : BUTTON({
            className: "copy-btn copy-btn-float",
            type: "button",
            onclick(e: Event) {
                const btn = e.currentTarget as HTMLButtonElement;
                navigator.clipboard.writeText(code).then(() => {
                    const prev = btn.textContent;
                    btn.textContent = "Copied!";
                    setTimeout(() => { btn.textContent = prev; }, 1500);
                });
            }
        }, "Copy"),
        PRE({ className: "code-pre" }, highlightedCodeEl(code, language))
    );
}

export type { CodeLang };
