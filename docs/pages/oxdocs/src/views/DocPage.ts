import { MAIN, DIV, H1, P, BUTTON, SPAN } from "oxidizer";
import type { DocSectionMeta } from "../content";
import { SECTIONS } from "../content";
import { renderInline } from "../components";
import type { ShellProps } from "../layout/Sidebar";
import { goTo } from "../layout/Sidebar";

export default function DocPage(
    shell: ShellProps,
    section: DocSectionMeta,
    body: (HTMLElement | null)[],
) {
    const idx = SECTIONS.findIndex(s => s.id === section.id);
    const prev = SECTIONS[idx - 1];
    const next = SECTIONS[idx + 1];

    return MAIN({ className: "page doc-page" },
        DIV({ className: "page-header" },
            DIV({ className: "breadcrumb" },
                BUTTON({
                    className: "breadcrumb-link",
                    type: "button",
                    onclick: () => goTo("/", shell),
                }, "Home"),
                SPAN({ className: "breadcrumb-sep" }, "/"),
                SPAN({ className: "breadcrumb-current" }, section.title)
            ),
            H1({ className: "page-title" }, ...renderInline(section.title)),
            section.subtitle
                ? P({ className: "page-subtitle" }, section.subtitle)
                : null
        ),
        DIV({ className: "page-body" }, ...body),
        DIV({ className: "page-nav" },
            prev
                ? BUTTON({
                    className: "page-nav-btn page-nav-prev",
                    type: "button",
                    onclick: () => goTo(`/docs/${prev.id}`, shell),
                },
                    SPAN({ className: "page-nav-label" }, "Previous"),
                    SPAN({ className: "page-nav-title" }, prev.title)
                )
                : SPAN({ className: "page-nav-spacer" }),
            next
                ? BUTTON({
                    className: "page-nav-btn page-nav-next",
                    type: "button",
                    onclick: () => goTo(`/docs/${next.id}`, shell),
                },
                    SPAN({ className: "page-nav-label" }, "Next"),
                    SPAN({ className: "page-nav-title" }, next.title)
                )
                : SPAN({ className: "page-nav-spacer" })
        )
    );
}
