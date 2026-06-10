import { P } from "oxidizer";
import { renderInline } from "./inline";

export function Paragraph(text: string) {
    return P({ className: "prose-p" }, ...renderInline(text));
}
