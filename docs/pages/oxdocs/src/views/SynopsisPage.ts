import { Paragraph, Heading, DataTable, BulletList } from "../components";

export default function SynopsisPage() {
    return [
        Paragraph("Every string in `options` and `arguments` arrays is a **synopsis** that `x` parses into a structured CLI spec. All `[optional]` forms have bare `(required)` equivalents — drop the brackets (or use parenthesised option groups where shown)."),
        Heading(3, "Arguments"),
        DataTable(["Synopsis", "Meaning", "Example"], [
            ["`&lt;name&gt;`", "Required positional", "`x app file.txt`"],
            ["`[&lt;name&gt;]`", "Optional positional", "`x app` or `x app file.txt`"],
            ["`[&lt;name='val'&gt;]`", "Optional positional with default", "`x app` or `x app myfile.txt`"],
            ["`&lt;name&gt;...`", "Required repeating positional", "`x app file1.txt file2.txt`"],
            ["`[&lt;name&gt;...]`", "Optional repeating positional", "`x app` or `x app file1.txt file2.txt`"],
            ["`&lt;name={a|b|c}&gt;`", "Required positional, value from set", "`x app write`"],
            ["`[&lt;name={a|b|c}&gt;]`", "Optional positional, value from set", "`x app` or `x app fast`"],
            ["`(a|b|c)`", "Required choice (becomes positional `choice`)", "`x app north`"],
        ]),
        Heading(3, "Options"),
        DataTable(["Synopsis", "Meaning", "Example"], [
            ["`--long`", "Required bool flag", "`x app --long`"],
            ["`(-l | --long)`", "Required bool flag, alias pair", "`x app -l`"],
            ["`(--long | --short)`", "Required mutually exclusive flags", "`x app --long` or `x app --short`"],
            ["`(-l | --long | -s | --short)`", "Required mutex with alias pairs", "`x app -l` or `x app -s`"],
            ["`[-l | --long]`", "Optional bool flag, alias pair", "`x app -l`"],
            ["`[-l | --long &lt;arg&gt;]`", "Optional flag with required value", "`x app -l ./dist`"],
            ["`[-l | --long &lt;arg='v'&gt;]`", "Optional flag with default value", "`x app` or `x app --long ./dist`"],
            ["`[-l | --long &lt;arg&gt; ...]`", "Optional flag, value repeats", "`x app --long \"a\" \"b\"`"],
            ["`[-l | --long &lt;arg&gt;]...`", "Optional flag repeats", "`x app --long abc --long xyz`"],
            ["`[--long=&lt;arg&gt;]`", "Optional flag, value via `=`", "`x app --long=./dist`"],
            ["`[--long=&lt;arg='v'&gt;]`", "Optional flag, `=` form with default", "`x app` or `x app --long=./dist`"],
            ["`[--long={a|b|c}]`", "Optional flag, value from set", "`x app --long=a`"],
            ["`[--input=&lt;a&gt; [--output=&lt;b&gt;]]`", "Dependent flag (`--output` requires `--input`)", "`x app --input=foo.txt --output=bar.txt`"],
        ]),
        Heading(3, "Validation"),
        Paragraph("`x` enforces all of the above before any bash runs:"),
        BulletList([
            "Required args and options must be present.",
            "Defaults are applied when optional values are omitted.",
            "Choice values must be members of the declared set.",
            "`requires:` chains from nested bracket groups are enforced.",
            "Mutually exclusive option groups are enforced.",
            "Repeating positionals must be the last argument in the list.",
            "Unknown options are rejected with a clear error.",
        ]),
    ];
}
