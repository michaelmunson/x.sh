import { CODE, B, A } from "oxidizer";

export function renderInline(text: string): (string | HTMLElement)[] {
    const parts: (string | HTMLElement)[] = [];
    const regex = /`([^`]+)`|\*\*([^*]+)\*\*|\[([^\]]+)\]\(([^)]+)\)/g;
    let last = 0;
    let match: RegExpExecArray | null;

    while ((match = regex.exec(text)) !== null) {
        if (match.index > last) parts.push(text.slice(last, match.index));
        if (match[1]) parts.push(CODE({ className: "inline-code" }, match[1]));
        else if (match[2]) parts.push(B({}, match[2]));
        else if (match[3] && match[4]) parts.push(A({ href: match[4], className: "inline-link" }, match[3]));
        last = match.index + match[0].length;
    }
    if (last < text.length) parts.push(text.slice(last));
    return parts.length ? parts : [text];
}
