import { createProps, createEffect, ASIDE, DIV, NAV, UL, LI, BUTTON, INPUT, A, IMG, SPAN } from "oxidizer";
import { navigate, getPathname, getSearch, setSearch } from "oxidizer-router";
import { NAV_ITEMS } from "../content";
import { renderInline } from "../components";
import { BASE_PATH } from "../App";
import { iconBase64 } from "../assets";

export type ShellProps = ReturnType<typeof createProps<{
    path: string;
    open: boolean;
    routeKey: string;
}>>;

function activeSection(path: string): string {
    const match = path.match(/^\/docs\/([^/]+)/);
    return match?.[1] ?? "";
}

export function syncRoute(shell: ShellProps) {
    shell.path = getPathname();
    shell.routeKey = window.location.search;
}

export function goTo(path: string, shell: ShellProps) {
    const { q } = getSearch();
    navigate(path.startsWith(BASE_PATH) ? path : BASE_PATH + path, q ? { search: { q } } : undefined);
    syncRoute(shell);
    shell.open = false;
}

function setSearchQuery(shell: ShellProps, q: string) {
    setSearch(q ? { q } : {});
    shell.routeKey = window.location.search;
}

export function Sidebar(shell: ShellProps) {
    
    return ASIDE(shell, p => {
        const query = (getSearch().q ?? "").toLowerCase().trim();
        const current = activeSection(p.path);
        const groups = [...new Set(NAV_ITEMS.map(n => n.group ?? ""))];
        const filtered = NAV_ITEMS.filter(item =>
            !query || item.label.toLowerCase().includes(query) || item.id.includes(query)
        );

        return [
            { className: `sidebar${p.open ? " open" : ""}` },
            DIV({ className: "sidebar-header" },
                BUTTON({
                    className: "logo",
                    type: "button",
                    onclick: () => goTo("/", p)
                },
                    IMG({ className: "logo-icon-img", src: iconBase64, alt: "x.sh", width: "28", height: "28" }),
                    SPAN({ className: "logo-text" }, "x.sh")
                ),
                BUTTON({
                    className: "sidebar-close",
                    type: "button",
                    "aria-label": "Close menu",
                    onclick: () => { p.open = false; }
                }, "×")
            ),
            DIV({ className: "sidebar-search" },
                INPUT({
                    type: "search",
                    className: "search-input",
                    placeholder: "Search docs…",
                    value: getSearch().q ?? "",
                    oninput(e: Event) {
                        setSearchQuery(p, (e.target as HTMLInputElement).value);
                    },
                    onload(this: HTMLInputElement){
                        console.log("focusing");
                        this.focus();
                    },
                    onloadstart(this: HTMLInputElement){
                        console.log("focusing");
                        this.focus();
                    }
                })
            ),
            NAV({ className: "sidebar-nav" },
                UL({ className: "nav-list" },
                    LI({},
                        BUTTON({
                            className: `nav-link${p.path === "/" ? " active" : ""}`,
                            type: "button",
                            onclick: () => goTo("/", p)
                        }, "Home")
                    )
                ),
                ...groups.map(group => {
                    const items = filtered.filter(i => (i.group ?? "") === group);
                    if (!items.length) return null;
                    return DIV({ className: "nav-group" },
                        group ? SPAN({ className: "nav-group-label" }, group) : null,
                        UL({ className: "nav-list" },
                            ...items.map(item =>
                                LI({},
                                    BUTTON({
                                        className: `nav-link${current === item.id ? " active" : ""}`,
                                        type: "button",
                                        onclick: () => goTo(`/docs/${item.id}`, p)
                                    }, ...renderInline(item.label))
                                )
                            )
                        )
                    );
                })
            ),
            DIV({ className: "sidebar-footer" },
                A({
                    href: "https://github.com/michaelmunson/x.sh",
                    className: "github-link",
                    target: "_blank",
                    rel: "noopener noreferrer"
                }, "GitHub →")
            )
        ];
    });
}
