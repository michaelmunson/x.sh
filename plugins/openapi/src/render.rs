//! Render an [`XApp`] as `.x.yml` text matching x.sh v3 syntax.

use crate::convert::{curl_path, BodyField, ParamField, XApp, XCommand};

pub fn render(app: &XApp) -> String {
    let mut out = String::new();

    out.push_str(&format!("name: {}\n", app.name));
    out.push_str(&format!("version: {}\n", yaml_scalar(&app.version)));
    if let Some(desc) = &app.description {
        out.push_str(&format!("description: {}\n", yaml_scalar(desc)));
    }
    out.push('\n');

    if let Some(base) = &app.base_url {
        out.push_str("env:\n");
        out.push_str(&format!("  BASE_URL: {}\n", yaml_scalar(base)));
        out.push('\n');
    }

    render_request_utility(&mut out);
    out.push('\n');

    for cmd in &app.commands {
        render_command(&mut out, cmd);
        out.push('\n');
    }

    out
}

fn render_request_utility(out: &mut String) {
    out.push_str(".__request:\n");
    out.push_str("  help: underlying curl command\n");
    out.push_str("  options:\n");
    out.push_str("    - \"[-s | --silent]\"\n");
    out.push_str("    - \"[-I | --head]\"\n");
    out.push_str("    - \"[-X | --request <method>]\"\n");
    out.push_str("    - \"[-H | --header <header>...]\"\n");
    out.push_str("    - \"[-d | --data <data>]\"\n");
    out.push_str("  arguments:\n");
    out.push_str("    - \"[<url>]\"\n");
    out.push_str("  $: |\n");
    out.push_str("    args=()\n");
    out.push_str("    [[ $(x-opt silent) == true ]] && args+=(-s)\n");
    out.push_str("    [[ $(x-opt head) == true ]] && args+=(-I)\n");
    out.push_str("    method=$(x-opt request)\n");
    out.push_str("    [[ -n \"$method\" ]] && args+=(-X \"$method\")\n");
    out.push_str("    mapfile -t headers < <(x-opt header)\n");
    out.push_str("    for h in \"${headers[@]}\"; do\n");
    out.push_str("      args+=(-H \"$h\")\n");
    out.push_str("    done\n");
    out.push_str("    data=$(x-opt data)\n");
    out.push_str("    [[ -n \"$data\" ]] && args+=(-d \"$data\")\n");
    out.push_str("    url=$(x-arg url)\n");
    out.push_str("    [[ -n \"$url\" ]] && args+=(\"$url\")\n");
    out.push_str("    curl \"${args[@]}\"\n");
}

fn render_command(out: &mut String, cmd: &XCommand) {
    out.push_str(&format!(".{}:\n", cmd.name));
    if let Some(desc) = &cmd.description {
        out.push_str(&format!("  description: {}\n", yaml_scalar(desc)));
    }

    out.push_str("  options:\n");
    out.push_str("    - \"[-i | --interactive]\"\n");

    for q in &cmd.query_params {
        out.push_str(&format!("    - \"{}\"\n", query_option_synopsis(q)));
    }

    if cmd.has_json_body {
        out.push_str("    - \"[-d | --data <fieldval>...]\"\n");
    }

    if !cmd.path_params.is_empty() {
        out.push_str("  arguments:\n");
        for p in &cmd.path_params {
            out.push_str(&format!("    - \"{}\"\n", path_arg_synopsis(p)));
        }
    }

    let script = render_script(cmd);
    out.push_str("  $: |\n");
    for line in script.lines() {
        if line.is_empty() {
            out.push_str("    \n");
        } else {
            out.push_str("    ");
            out.push_str(line);
            out.push('\n');
        }
    }
}

fn query_option_synopsis(p: &ParamField) -> String {
    if let Some(choices) = &p.choices {
        let joined = choices.join("|");
        return format!("[--{}={{{}}}]", p.name, joined);
    }
    if let Some(default) = &p.default {
        // Match Petstore-style `[--limit=<n='10'>]` for numeric defaults.
        let placeholder = if default.chars().all(|c| c.is_ascii_digit() || c == '.') {
            "n"
        } else {
            "value"
        };
        return format!(
            "[--{}=<{}='{}'>]",
            p.name,
            placeholder,
            escape_single(default)
        );
    }
    if p.required {
        format!("(--{}=<value>)", p.name)
    } else {
        format!("[--{}=<value>]", p.name)
    }
}

fn path_arg_synopsis(p: &ParamField) -> String {
    if p.required {
        format!("[<{}>]", p.name)
    } else if let Some(default) = &p.default {
        format!("[<{}='{}'>]", p.name, escape_single(default))
    } else {
        format!("[<{}>]", p.name)
    }
}

fn render_script(cmd: &XCommand) -> String {
    let mut s = String::new();
    s.push_str("if [[ $(x-opt interactive) == true ]]; then\n");

    for p in &cmd.path_params {
        let label = human_label(&p.name);
        s.push_str(&format!(
            "  x-io-read \"{}:\" -v {}\n",
            escape_double(&label),
            p.name
        ));
    }
    for q in &cmd.query_params {
        render_interactive_query(&mut s, q);
    }
    for f in &cmd.body_fields {
        render_interactive_body(&mut s, f);
    }

    s.push_str("else\n");

    for p in &cmd.path_params {
        s.push_str(&format!("  {}=$(x-arg {})\n", p.name, p.name));
        if p.required {
            s.push_str(&format!(
                "  [[ -n \"${}\" ]] || {{ echo \"missing required argument: {}\" >&2; exit 1; }}\n",
                p.name, p.name
            ));
        }
    }

    for q in &cmd.query_params {
        // Synopsis defaults are applied by x-opt; do not re-default here.
        s.push_str(&format!("  {}=$(x-opt {})\n", q.name, q.name));
    }

    if cmd.has_json_body {
        s.push_str("  declare -A fields\n");
        s.push_str("  mapfile -t data < <(x-opt data)\n");
        s.push_str("  for pair in \"${data[@]}\"; do\n");
        s.push_str("    fields[\"${pair%%=*}\"]=\"${pair#*=}\"\n");
        s.push_str("  done\n");
        for f in &cmd.body_fields {
            s.push_str(&format!(
                "  {}=\"${{fields[{}]-}}\"\n",
                f.name, f.name
            ));
        }
        for f in &cmd.body_fields {
            if f.required {
                s.push_str(&format!(
                    "  [[ -n \"${}\" ]] || {{ echo \"missing required field: {} (use -d {}=...)\" >&2; exit 1; }}\n",
                    f.name, f.name, f.name
                ));
            }
        }
    }

    s.push_str("fi\n");

    if !cmd.query_params.is_empty() {
        // Prefer: always include params that have defaults; gate the rest on non-empty.
        let (with_default, without_default): (Vec<_>, Vec<_>) = cmd
            .query_params
            .iter()
            .partition(|q| q.default.is_some());

        if let Some(first) = with_default.first() {
            s.push_str(&format!(
                "query=\"{}=${{{}}}\"\n",
                first.name, first.name
            ));
            for q in with_default.iter().skip(1) {
                s.push_str(&format!(
                    "query=\"$query&{}=${{{}}}\"\n",
                    q.name, q.name
                ));
            }
            for q in &without_default {
                s.push_str(&format!("if [[ -n \"${}\" ]]; then\n", q.name));
                s.push_str(&format!(
                    "  query=\"$query&{}=${{{}}}\"\n",
                    q.name, q.name
                ));
                s.push_str("fi\n");
            }
        } else {
            s.push_str("query=\"\"\n");
            for q in &without_default {
                s.push_str(&format!("if [[ -n \"${}\" ]]; then\n", q.name));
                s.push_str("  if [[ -n \"$query\" ]]; then\n");
                s.push_str("    query=\"$query&\"\n");
                s.push_str("  fi\n");
                s.push_str(&format!(
                    "  query=\"${{query}}{}=${{{}}}\"\n",
                    q.name, q.name
                ));
                s.push_str("fi\n");
            }
        }
    }

    if cmd.has_json_body {
        s.push_str(&render_jq_body(&cmd.body_fields));
        s.push_str("echo $body\n");
    }

    let path = curl_path(&cmd.path_template);
    let url = if cmd.query_params.is_empty() {
        format!("\"${{BASE_URL}}{}\"", path)
    } else {
        format!("\"${{BASE_URL}}{}?${{query}}\"", path)
    };

    match cmd.method.as_str() {
        "GET" => {
            s.push_str(&format!("x-run-self __request -s {}\n", url));
        }
        "DELETE" => {
            s.push_str(&format!("x-run-self __request -s -X DELETE {}\n", url));
        }
        "HEAD" => {
            s.push_str(&format!("x-run-self __request -s -I {}\n", url));
        }
        method => {
            if cmd.has_json_body {
                s.push_str(&format!(
                    "x-run-self __request -s -X {} {} \\\n  -H 'Content-Type: application/json' \\\n  -d \"$body\"\n",
                    method, url
                ));
            } else {
                s.push_str(&format!("x-run-self __request -s -X {} {}\n", method, url));
            }
        }
    }

    s
}

fn render_interactive_query(s: &mut String, q: &ParamField) {
    if let Some(choices) = &q.choices {
        let label = human_label(&q.name);
        s.push_str(&format!(
            "  x-io-select \"{}\" \\\n",
            escape_double(&label)
        ));
        s.push_str("    \"_=Any\" \\\n");
        for c in choices {
            let pretty = capitalize(c);
            s.push_str(&format!(
                "    \"{}={}\" \\\n",
                escape_double(c),
                escape_double(&pretty)
            ));
        }
        s.push_str(&format!("    -v {}\n", q.name));
        s.push_str(&format!(
            "  [[ \"${}\" == \"_\" ]] && {}=\n",
            q.name, q.name
        ));
    } else if let Some(default) = &q.default {
        let label = format!("{} (default {}):", human_label(&q.name), default);
        s.push_str(&format!(
            "  x-io-read \"{}\" -v {}\n",
            escape_double(&label),
            q.name
        ));
        s.push_str(&format!(
            "  {}=${{{}:-{}}}\n",
            q.name,
            q.name,
            shell_escape_default(default)
        ));
    } else {
        let label = format!("{}:", human_label(&q.name));
        s.push_str(&format!(
            "  x-io-read \"{}\" -v {}\n",
            escape_double(&label),
            q.name
        ));
    }
}

fn render_interactive_body(s: &mut String, f: &BodyField) {
    let label = if f.required {
        format!("{}:", human_label(&f.name))
    } else {
        format!("{} (optional):", human_label(&f.name))
    };
    s.push_str(&format!(
        "  x-io-read \"{}\" -v {}\n",
        escape_double(&label),
        f.name
    ));
}

fn render_jq_body(fields: &[BodyField]) -> String {
    let mut s = String::new();
    if fields.is_empty() {
        s.push_str("body='{}'\n");
        return s;
    }

    let mut args = String::new();
    for f in fields {
        args.push_str(&format!(" --arg {} \"${}\"", f.name, f.name));
    }

    // Build a jq expression like:
    //   {name: $name} + (if $tag != "" then {tag: $tag} else {} end)
    let mut parts: Vec<String> = Vec::new();
    let required: Vec<_> = fields.iter().filter(|f| f.required).collect();
    let optional: Vec<_> = fields.iter().filter(|f| !f.required).collect();

    if required.is_empty() {
        parts.push("{}".to_string());
    } else {
        let obj = required
            .iter()
            .map(|f| format!("{}: ${}", f.name, f.name))
            .collect::<Vec<_>>()
            .join(", ");
        parts.push(format!("{{{}}}", obj));
    }

    for f in optional {
        parts.push(format!(
            "(if ${} != \"\" then {{{}: ${}}} else {{}} end)",
            f.name, f.name, f.name
        ));
    }

    let expr = parts.join(" + ");
    s.push_str(&format!("body=$(jq -n{} \\\n  '{}')\n", args, expr));
    s
}

fn human_label(name: &str) -> String {
    let mut parts = Vec::new();
    for part in name.split(|c: char| c == '_' || c == '-' || c == ' ') {
        if part.is_empty() {
            continue;
        }
        // camelCase / PascalCase split
        let mut cur = String::new();
        for (i, ch) in part.chars().enumerate() {
            if i > 0 && ch.is_ascii_uppercase() && !cur.is_empty() {
                parts.push(capitalize(&cur));
                cur.clear();
            }
            cur.push(ch);
        }
        if !cur.is_empty() {
            parts.push(capitalize(&cur));
        }
    }
    if parts.is_empty() {
        name.to_string()
    } else {
        parts.join(" ")
    }
}

fn capitalize(s: &str) -> String {
    if s.eq_ignore_ascii_case("id") {
        return "ID".to_string();
    }
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => c.to_ascii_uppercase().to_string() + &chars.as_str().to_ascii_lowercase(),
        None => String::new(),
    }
}

fn yaml_scalar(s: &str) -> String {
    if s.is_empty() {
        return "\"\"".to_string();
    }
    // Quote when a colon is a YAML mapping indicator (`: ` / trailing `:`),
    // not when it appears inside URLs like `https://…`.
    let has_mapping_colon = s.contains(": ") || s.ends_with(':');
    let needs_quote = has_mapping_colon
        || s.contains('#')
        || s.contains('{')
        || s.contains('}')
        || s.contains('[')
        || s.contains(']')
        || s.contains('&')
        || s.contains('*')
        || s.contains('!')
        || s.contains('|')
        || s.contains('>')
        || s.contains('\'')
        || s.contains('"')
        || s.contains('\n')
        || s.starts_with(' ')
        || s.ends_with(' ')
        || s.eq_ignore_ascii_case("true")
        || s.eq_ignore_ascii_case("false")
        || s.eq_ignore_ascii_case("null")
        || s.eq_ignore_ascii_case("yes")
        || s.eq_ignore_ascii_case("no");

    if needs_quote {
        format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
    } else {
        s.to_string()
    }
}

fn escape_single(s: &str) -> String {
    s.replace('\'', "'\\''")
}

fn escape_double(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn shell_escape_default(s: &str) -> String {
    // Used inside ${var:-default} — keep simple defaults unquoted when safe.
    if s.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-')
    {
        s.to_string()
    } else {
        format!("'{}'", escape_single(s))
    }
}
