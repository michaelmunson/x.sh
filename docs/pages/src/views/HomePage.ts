import { DIV, H1, H2, P, BUTTON, A, SPAN, CODE } from "oxidizer";
import { NAV_ITEMS } from "../content";
import { renderInline } from "../components";
import type { ShellProps } from "../layout/Sidebar";
import { goTo } from "../layout/Sidebar";

export default function HomePage(shell: ShellProps) {
    console.log('HomePage', shell.path);
    const groups = [...new Set(NAV_ITEMS.map(n => n.group ?? ""))];

    return DIV({ className: "page home-page" },
        DIV({ className: "hero" },
            H1({ className: "hero-title" },
                SPAN({ className: "hero-x" }, "x"),
                ".sh"
            ),
            P({ className: "hero-tagline" },
                "Script management and CLI App Framework."
            ),
            DIV({ className: "hero-actions" },
                BUTTON({
                    className: "btn btn-primary",
                    type: "button",
                    onclick: () => goTo("/docs/quickstart", shell)
                }, "Get Started"),
                BUTTON({
                    className: "btn btn-secondary",
                    type: "button",
                    onclick: () => goTo("/docs/commands", shell)
                }, "Command Reference"),
                A({
                    href: "https://github.com/michaelmunson/x.sh",
                    className: "btn btn-ghost",
                    target: "_blank",
                    rel: "noopener noreferrer"
                }, "View Source")
            ),
            DIV({ className: "hero-install" },
                SPAN({ className: "hero-install-label" }, "Install"),
                CODE({ className: "hero-install-code" },
                    "curl -fsSL https://raw.githubusercontent.com/michaelmunson/x.sh/main/install.sh | bash"
                )
            )
        ),
        DIV({ className: "home-sections" },
            H2({ className: "home-sections-title" }, "Browse by topic"),
            ...groups.map(group =>
                DIV({ className: "section-card-group" },
                    SPAN({ className: "section-card-group-label" }, group),
                    DIV({ className: "section-cards" },
                        ...NAV_ITEMS
                            .filter(item => item.group === group)
                            .map(item =>
                                BUTTON({
                                    className: "section-card",
                                    type: "button",
                                    onclick: () => goTo(`/docs/${item.id}`, shell)
                                },
                                    SPAN({ className: "section-card-title" }, ...renderInline(item.label)),
                                    SPAN({ className: "section-card-arrow" }, "→")
                                )
                            )
                    )
                )
            )
        )
    );
}
