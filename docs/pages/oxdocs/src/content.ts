export type NavItem = {
    id: string;
    label: string;
    group?: string;
};

export const NAV_ITEMS: NavItem[] = [
    { id: "overview", label: "Overview", group: "Getting Started" },
    { id: "installation", label: "Installation", group: "Getting Started" },
    { id: "quickstart", label: "Quick Start", group: "Getting Started" },
    { id: "commands", label: "Commands", group: "Reference" },
    { id: "project-local", label: "Project-local Scripts", group: "Reference" },
    { id: "app-framework", label: "App Framework", group: "Apps" },
    { id: "app-format", label: "App File Format", group: "Apps" },
    { id: "synopsis", label: "Synopsis DSL", group: "Apps" },
    { id: "builtins", label: "Built-in Helpers", group: "Apps" },
    { id: "validation-help", label: "Validation & Help", group: "Apps" },
    { id: "languages", label: "Supported Languages", group: "Reference" },
];

export type DocSectionMeta = {
    id: string;
    title: string;
    subtitle?: string;
};

export const SECTIONS: DocSectionMeta[] = [
    {
        id: "overview",
        title: "Overview",
        subtitle: "A useful CLI tool that helps you manage your scripts.",
    },
    {
        id: "installation",
        title: "Installation",
        subtitle: "Automated install, manual build, and shell completion.",
    },
    {
        id: "quickstart",
        title: "Quick Start",
        subtitle: "Create, run, list, and link your first script.",
    },
    {
        id: "commands",
        title: "Commands",
        subtitle: "Complete CLI reference for every flag and subcommand.",
    },
    {
        id: "project-local",
        title: "Project-local Scripts",
        subtitle: "Define repo-specific commands in `./x.yml`.",
    },
    {
        id: "app-framework",
        title: "App Framework",
        subtitle: "YAML-defined, validated multi-command CLIs.",
    },
    {
        id: "app-format",
        title: "App File Format",
        subtitle: "Metadata, commands, handlers, imports, and env.",
    },
    {
        id: "synopsis",
        title: "Synopsis DSL",
        subtitle: "Declare options and arguments with synopsis strings.",
    },
    {
        id: "builtins",
        title: "Built-in Helpers",
        subtitle: "Bash functions injected before every app handler.",
    },
    {
        id: "validation-help",
        title: "Validation & Help",
        subtitle: "Automatic help and save-time / runtime validation.",
    },
    {
        id: "languages",
        title: "Supported Languages",
        subtitle: "Interpreters and compilers available during `x -i`.",
    },
];
