import { getParams } from "oxidizer-router";
import { SECTIONS } from "../content";
import DocPage from "./DocPage";
import NotFoundPage from "./NotFoundPage";
import { SECTION_VIEWS } from "./sectionRegistry";
import type { ShellProps } from "../layout/Sidebar";

export default function SectionPage(shell: ShellProps) {
    const { sectionId } = getParams();
    const section = SECTIONS.find(s => s.id === sectionId);
    const render = SECTION_VIEWS[sectionId];

    if (!section || !render) return NotFoundPage(shell);

    return DocPage(shell, section, render());
}
