import { Paragraph, DataTable } from "../components";

export default function LanguagesPage() {
    return [
        Paragraph("`x` supports creating scripts in the following languages:"),
        DataTable(["Language", "Program Value", "Description"], [
            ["Bash", "bash", "Bash/Shell Script"],
            ["Zsh", "zsh", "Zsh Script"],
            ["POSIX Shell", "sh", "POSIX Shell Script"],
            ["Node.js", "node", "JavaScript/TypeScript via Node.js"],
            ["Python 3", "python", "Python 3 Script"],
            ["Python 2", "python2", "Python 2 Script"],
            ["Ruby", "ruby", "Ruby Script"],
            ["Perl", "perl", "Perl Script"],
            ["Go", "go", "Go Program"],
            ["Rust", "rust", "Rust Program"],
            ["PHP", "php", "PHP Script"],
            ["Lua", "lua", "Lua Script"],
            ["Deno", "deno", "Deno (JavaScript/TypeScript runtime)"],
            ["Swift", "swift", "Swift Script"],
            ["C", "c", "C Program"],
            ["C++", "cpp", "C++ Program"],
            ["Java", "java", "Java Program"],
            ["R", "r", "R Script"],
            ["AWK", "awk", "AWK Script"],
            ["Elixir", "elixir", "Elixir Script"],
            ["Clojure", "clj", "Clojure Script"],
            ["Scala", "scala", "Scala Script"],
            ["Haskell", "haskell", "Haskell Program"],
            ["PowerShell", "powershell", "PowerShell Script"],
            ["Kotlin", "kotlin", "Kotlin Script/Program"],
        ]),
    ];
}
