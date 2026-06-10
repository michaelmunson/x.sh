import { P, A } from "oxidizer";

export function DocLink(href: string, text: string, external = false) {
    return P({ className: "prose-p" },
        A({
            href,
            className: "inline-link",
            ...(external ? { target: "_blank", rel: "noopener noreferrer" } : {}),
        }, text)
    );
}
