import { H2, H3, H4 } from "oxidizer";
import { renderInline } from "./inline";

export function Heading(level: 2 | 3 | 4, text: string) {
    const cls = level === 2 ? "section-h2" : level === 3 ? "section-h3" : "section-h4";
    const Tag = level === 2 ? H2 : level === 3 ? H3 : H4;
    return Tag({ className: cls }, ...renderInline(text));
}
