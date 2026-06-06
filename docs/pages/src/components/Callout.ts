import { DIV, P, B } from "oxidizer";
import { renderInline } from "./inline";

export function Callout(variant: "info" | "warn" | "tip", text: string, title?: string) {
    const labels = { info: "Note", warn: "Warning", tip: "Tip" };
    return DIV({ className: `callout callout-${variant}` },
        B({}, title ?? labels[variant]),
        P({ className: "prose-p" }, ...renderInline(text))
    );
}
