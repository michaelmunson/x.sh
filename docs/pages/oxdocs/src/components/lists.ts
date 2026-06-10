import { UL, OL, LI } from "oxidizer";
import { renderInline } from "./inline";

export function BulletList(items: string[]) {
    return UL({ className: "prose-ul" },
        ...items.map(item => LI({}, ...renderInline(item)))
    );
}

export function OrderedList(items: string[]) {
    return OL({ className: "prose-ol" },
        ...items.map(item => LI({}, ...renderInline(item)))
    );
}
