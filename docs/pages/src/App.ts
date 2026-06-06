import { createProps, createEffect, DIV, P, A, BUTTON, SPAN } from "oxidizer";
import Router, { getPathname } from "oxidizer-router";
import { Sidebar, syncRoute } from "./layout/Sidebar";
import HomePage from "./views/HomePage";
import SectionPage from "./views/SectionPage";
import NotFoundPage from "./views/NotFoundPage";

export default function App() {
    const shell = createProps({
        path: getPathname(),
        open: false,
        routeKey: window.location.search,
    }, [
        createEffect(['routeKey'], $ => {
            if ($.routeKey) setTimeout(() => {
                const input = document.querySelector('input[type="search"]') as HTMLInputElement;
                if (input) {
                    input.focus();
                }
            }, 100)
        })
    ]);

    window.addEventListener("popstate", () => syncRoute(shell));

    return DIV({ id: "app" },
        DIV(shell, p => [
            DIV({
                className: `sidebar-overlay${p.open ? " visible" : ""}`,
                onclick: () => { p.open = false; }
            }),
            Sidebar(shell),
        ]),
        DIV({ className: "main-wrap" },
            DIV({ className: "topbar", role: "banner" },
                BUTTON({
                    className: "menu-btn",
                    type: "button",
                    "aria-label": "Open menu",
                    onclick: () => { shell.open = true; }
                }, "☰"),
                SPAN({ className: "topbar-title" }, "x.sh docs")
            ),
            Router({
                "/": () => HomePage(shell),
                "/docs": {
                    ":sectionId": () => SectionPage(shell),
                },
                "*": () => NotFoundPage(shell),
            }),
            DIV({ className: "page-footer" },
                P({},
                    "Built with ",
                    A({ href: "https://www.npmjs.com/package/oxidizer", className: "inline-link", target: "_blank", rel: "noopener noreferrer" }, "oxidizer"),
                    " & ",
                    A({ href: "https://www.npmjs.com/package/oxidizer-router", className: "inline-link", target: "_blank", rel: "noopener noreferrer" }, "oxidizer-router"),
                    " · ",
                    A({ href: "https://github.com/michaelmunson/x.sh", className: "inline-link", target: "_blank", rel: "noopener noreferrer" }, "x.sh on GitHub")
                )
            )
        )
    );
}
