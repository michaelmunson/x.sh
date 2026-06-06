import { UL, LI, OL, HR, P, A } from "oxidizer";
import type { ContentBlock } from "./types";
import { renderInline } from "./inline";
import { CodeBlock } from "./CodeBlock";
import { Callout } from "./Callout";
import { DataTable } from "./DataTable";
import { Paragraph } from "./Paragraph";
import { Heading } from "./Heading";
import { ResolutionPipeline } from "./ResolutionPipeline";

function renderDiagram(variant: ContentBlock & { type: "diagram" }) {
    switch (variant.variant) {
        case "resolution-pipeline": return ResolutionPipeline();
    }
}

export function renderBlock(block: ContentBlock): HTMLElement | HTMLElement[] {
    switch (block.type) {
        case "p": return Paragraph(block.text);
        case "h3": return Heading(3, block.text);
        case "h4": return Heading(4, block.text);
        case "code": return CodeBlock(block.code, block.title);
        case "ul": return UL({ className: "prose-ul" },
            ...block.items.map(item => LI({}, ...renderInline(item)))
        );
        case "ol": return OL({ className: "prose-ol" },
            ...block.items.map(item => LI({}, ...renderInline(item)))
        );
        case "table": return DataTable(block.headers, block.rows);
        case "callout": return Callout(block.variant, block.text, block.title);
        case "hr": return HR({ className: "section-hr" });
        case "link": return P({ className: "prose-p" },
            A({
                href: block.href,
                className: "inline-link",
                ...(block.external ? { target: "_blank", rel: "noopener noreferrer" } : {})
            }, block.text)
        );
        case "diagram": return renderDiagram(block);
    }
}

export function renderBlocks(blocks: ContentBlock[]) {
    return blocks.map(b => renderBlock(b));
}
