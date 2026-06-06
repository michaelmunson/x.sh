import { DIV, MAIN, H1, P, BUTTON, SPAN } from "oxidizer";
import { getParams } from "oxidizer-router";
import { SECTIONS } from "../content";
import { renderBlocks, renderInline } from "../components";
import type { ShellProps } from "../layout/Sidebar";
import { goTo } from "../layout/Sidebar";
import NotFoundPage from "./NotFoundPage";

export default function SectionPage(shell: ShellProps) {
    console.log('SectionPage', shell.path);
    const { sectionId } = getParams();
    const section = SECTIONS.find(s => s.id === sectionId);

    if (!section) return NotFoundPage(shell);

    const idx = SECTIONS.findIndex(s => s.id === sectionId);
    const prev = SECTIONS[idx - 1];
    const next = SECTIONS[idx + 1];

    return MAIN({ className: "page doc-page" },
        DIV({ className: "page-header" },
            DIV({ className: "breadcrumb" },
                BUTTON({
                    className: "breadcrumb-link",
                    type: "button",
                    onclick: () => goTo("/", shell)
                }, "Home"),
                SPAN({ className: "breadcrumb-sep" }, "/"),
                SPAN({ className: "breadcrumb-current" }, section.title)
            ),
            H1({ className: "page-title" }, ...renderInline(section.title)),
            section.subtitle
                ? P({ className: "page-subtitle" }, section.subtitle)
                : null
        ),
        DIV({ className: "page-body" },
            ...renderBlocks(section.blocks)
        ),
        DIV({ className: "page-nav" },
            prev
                ? BUTTON({
                    className: "page-nav-btn page-nav-prev",
                    type: "button",
                    onclick: () => goTo(`/docs/${prev.id}`, shell)
                },
                    SPAN({ className: "page-nav-label" }, "Previous"),
                    SPAN({ className: "page-nav-title" }, prev.title)
                )
                : SPAN({ className: "page-nav-spacer" }),
            next
                ? BUTTON({
                    className: "page-nav-btn page-nav-next",
                    type: "button",
                    onclick: () => goTo(`/docs/${next.id}`, shell)
                },
                    SPAN({ className: "page-nav-label" }, "Next"),
                    SPAN({ className: "page-nav-title" }, next.title)
                )
                : SPAN({ className: "page-nav-spacer" })
        )
    );
}
