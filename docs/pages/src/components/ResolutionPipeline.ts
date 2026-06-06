import { DIV, SPAN, CODE, P } from "oxidizer";

function FlowStep(
    source: string,
    sourceNote: string | null,
    action: string,
    actionNote: string | null,
) {
    return DIV({ className: "flow-step" },
        DIV({ className: "flow-step-row" },
            DIV({ className: "flow-box flow-source" },
                CODE({ className: "flow-box-code" }, source),
                sourceNote ? SPAN({ className: "flow-box-note" }, sourceNote) : null
            ),
            DIV({ className: "flow-connector" },
                SPAN({ className: "flow-connector-label" }, "match?"),
                SPAN({ className: "flow-connector-yes" }, "yes →")
            ),
            DIV({ className: "flow-box flow-action" },
                SPAN({ className: "flow-box-text" }, action),
                actionNote ? SPAN({ className: "flow-box-note" }, actionNote) : null
            )
        ),
        DIV({ className: "flow-fallback" },
            SPAN({ className: "flow-fallback-label" }, "no"),
            SPAN({ className: "flow-arrow-down" }, "↓")
        )
    );
}

export function ResolutionPipeline() {
    return DIV({ className: "flow-diagram" },
        P({ className: "flow-diagram-title" },
            CODE({ className: "inline-code" }, "x my-cmd arg1 arg2")
        ),
        DIV({ className: "flow-entry" },
            SPAN({ className: "flow-arrow-down" }, "↓")
        ),
        FlowStep("./x.yml", null, "run inline bash", null),
        FlowStep("<name>.x.yml", "CWD → parents", "parse & validate", "run $: handler"),
        FlowStep("~/.x.sh/apps/", null, "same as local app", null),
        FlowStep("~/.x.sh/scripts/", null, "run with program", "from metadata"),
        DIV({ className: "flow-step flow-step-terminal" },
            DIV({ className: "flow-fallback flow-fallback-terminal" },
                SPAN({ className: "flow-fallback-label" }, "no"),
                SPAN({ className: "flow-arrow-down" }, "↓")
            ),
            DIV({ className: "flow-box flow-error" },
                SPAN({ className: "flow-box-text" }, "error: command not found")
            )
        )
    );
}
