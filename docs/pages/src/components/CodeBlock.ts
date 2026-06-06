import { DIV, PRE, CODE, BUTTON, SPAN } from "oxidizer";
import { renderInline } from "./inline";

export function CodeBlock(code: string, title?: string) {
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
        PRE({ className: "code-pre" }, CODE({ className: "code-content" }, code))
    );
}
