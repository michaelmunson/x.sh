import { DIV, TABLE, THEAD, TBODY, TR, TH, TD } from "oxidizer";
import { renderInline } from "./inline";

export function DataTable(headers: string[], rows: string[][]) {
    return DIV({ className: "table-wrap" },
        TABLE({ className: "data-table" },
            THEAD({},
                TR({}, ...headers.map(h => TH({}, ...renderInline(h))))
            ),
            TBODY({},
                ...rows.map(row =>
                    TR({}, ...row.map(cell => TD({}, ...renderInline(cell))))
                )
            )
        )
    );
}
