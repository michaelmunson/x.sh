import OverviewPage from "./OverviewPage";
import InstallationPage from "./InstallationPage";
import QuickstartPage from "./QuickstartPage";
import CommandsPage from "./CommandsPage";
import ProjectLocalPage from "./ProjectLocalPage";
import AppFrameworkPage from "./AppFrameworkPage";
import AppFormatPage from "./AppFormatPage";
import SynopsisPage from "./SynopsisPage";
import BuiltinsPage from "./BuiltinsPage";
import ValidationHelpPage from "./ValidationHelpPage";
import LanguagesPage from "./LanguagesPage";

export type SectionView = () => (HTMLElement | null)[];

export const SECTION_VIEWS: Record<string, SectionView> = {
    overview: OverviewPage,
    installation: InstallationPage,
    quickstart: QuickstartPage,
    commands: CommandsPage,
    "project-local": ProjectLocalPage,
    "app-framework": AppFrameworkPage,
    "app-format": AppFormatPage,
    synopsis: SynopsisPage,
    builtins: BuiltinsPage,
    "validation-help": ValidationHelpPage,
    languages: LanguagesPage,
};
