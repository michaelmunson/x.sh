import { Paragraph, Heading, CodeBlock, Callout, DataTable } from "../components";

export default function BuiltinsPage() {
    return [
        Paragraph("`x` injects a small bash preamble before each handler so you can read the parsed values without manual env-var wrangling. All helpers are available in the handler's scope and exported for `x-run` subprocesses."),
        DataTable(["Builtin", "Behavior"], [
            ["`x-opt &lt;name&gt;`", "Print option value. Bool flags → `true`. Repeats join with newline."],
            ["`x-arg &lt;name&gt;`", "Print positional argument value."],
            ["`x-opts &lt;assoc-name&gt;`", "Populate a caller-named bash assoc array with all options (nameref via `declare -n`)."],
            ["`x-args &lt;assoc-name&gt;`", "Populate a caller-named bash assoc array with all arguments."],
            ["`x-run &lt;cmd&gt; [args…]`", "Run a command with `x-*` helpers still in scope."],
            ["`x-usage &lt;cmd-path&gt;`", "Print auto-generated help (e.g. `x-usage create.file`)."],
            ["`x-io-read …`", "Prompt for a line. `-v` / `--var` assigns to a global scalar instead of stdout."],
            ["`x-io-confirm …`", "Yes/no (`--default yes|no`, default `no`). `-v` assigns `true`/`false` to a global scalar."],
            ["`x-io-select …`", "Menu of `id=label` pairs (`--multi`, optional `--search`). `-v` assigns result to caller globals."],
            ["`x-prt …`", "Styled print via ANSI SGR codes. `(-s|--style) &lt;style&gt;` sets comma-separated styles; sticky until next `--style`. No implicit newline."],
            ["`x-tui …`", "Terminal control via ANSI escapes (`--init`, `--exit`, `--clear`, cursor moves, etc.). Unrecognized args print as literal text."],
            ["`x-env-load .&lt;group&gt;`", "Export variables from a named `env:` group (e.g. `x-env-load .env1`). Global `env:` vars are already in the shell before the handler runs."],
            ["`x-path-root`", "Print the absolute path of the directory containing the x file (app or `x.yml`)."],
        ]),
        Callout("tip", "With `-v` / `--var`, results land in the **caller's** global shell variable — not stdout. Multi-select stores an indexed array."),
        Heading(3, "Reading options and arguments"),
        CodeBlock(`declare -A opts
x-opts opts
echo "\${opts[mode]}"

declare -A args
x-args args
for k in "\${!args[@]}"; do echo "$k=\${args[$k]}"; done`, undefined, "bash"),
        Heading(3, "Repeating options"),
        CodeBlock(`mapfile -t dirs < <(x-opt dir)
for d in "\${dirs[@]}"; do echo "dir: $d"; done`, undefined, "bash"),
        Heading(3, "Interactive I/O"),
        CodeBlock(`x-io-read "Name:" -v name
echo "$name"

x-io-select --multi -v colors "Choose a color" "red=Red" "green=Green" "blue=Blue"
echo "\${colors[0]}"`, undefined, "bash"),
        Heading(3, "Terminal styling"),
        CodeBlock(`x-prt \\
  --style red,underline,bg-white \\
  "Hello" \\
  --style blue,bg-yellow \\
  " World"
echo

x-tui --init --clear --home
x-prt --style green,bold "TUI demo"
x-tui --exit`, undefined, "bash"),
    ];
}
