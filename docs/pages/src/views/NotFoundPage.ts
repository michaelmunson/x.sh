import { DIV, MAIN, H1, P, BUTTON } from "oxidizer";
import type { ShellProps } from "../layout/Sidebar";
import { goTo } from "../layout/Sidebar";

export default function NotFoundPage(shell: ShellProps) {
    return MAIN({ className: "page not-found-page" },
        DIV({ className: "not-found" },
            H1({ className: "not-found-code" }, "404"),
            P({ className: "not-found-msg" }, "This documentation page doesn't exist."),
            BUTTON({
                className: "btn btn-primary",
                type: "button",
                onclick: () => goTo("/", shell)
            }, "Back to Home")
        )
    );
}
