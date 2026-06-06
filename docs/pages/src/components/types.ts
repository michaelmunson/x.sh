export type ContentBlock =
    | { type: "p"; text: string }
    | { type: "h3"; text: string }
    | { type: "h4"; text: string }
    | { type: "code"; code: string; title?: string }
    | { type: "ul"; items: string[] }
    | { type: "ol"; items: string[] }
    | { type: "table"; headers: string[]; rows: string[][] }
    | { type: "callout"; variant: "info" | "warn" | "tip"; title?: string; text: string }
    | { type: "hr" }
    | { type: "link"; href: string; text: string; external?: boolean }
    | { type: "diagram"; variant: "resolution-pipeline" };
