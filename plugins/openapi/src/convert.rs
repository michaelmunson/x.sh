//! Convert a parsed OpenAPI document into an intermediate x.sh app model.

use anyhow::{bail, Result};
use serde_json::Value;

use crate::openapi::{OpenApiDoc, Operation, Parameter, PathItem};

#[derive(Debug, Clone)]
pub struct XApp {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub base_url: Option<String>,
    pub commands: Vec<XCommand>,
}

#[derive(Debug, Clone)]
pub struct XCommand {
    pub name: String,
    pub description: Option<String>,
    pub method: String,
    pub path_template: String,
    pub path_params: Vec<ParamField>,
    pub query_params: Vec<ParamField>,
    pub body_fields: Vec<BodyField>,
    pub has_json_body: bool,
}

#[derive(Debug, Clone)]
pub struct ParamField {
    pub name: String,
    pub required: bool,
    pub default: Option<String>,
    pub choices: Option<Vec<String>>,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct BodyField {
    pub name: String,
    pub required: bool,
    pub description: Option<String>,
}

pub fn convert(doc: &OpenApiDoc) -> Result<XApp> {
    crate::openapi::validate_openapi_version(doc)?;

    let name = app_name_from_title(&doc.info.title);
    if name.is_empty() {
        bail!("info.title produced an empty app name");
    }

    let version = doc
        .info
        .version
        .clone()
        .unwrap_or_else(|| "0.0.0".to_string());
    let description = doc.info.description.clone();
    let base_url = doc.servers.first().map(|s| s.url.trim_end_matches('/').to_string());

    let mut commands = Vec::new();
    for (path, item) in &doc.paths {
        for (method, op) in operations(item) {
            let cmd = convert_operation(doc, path, item, method, op)?;
            commands.push(cmd);
        }
    }

    // Stable order: path then method (operations() already orders methods).
    // BTreeMap paths are sorted; keep that order.
    Ok(XApp {
        name,
        version,
        description,
        base_url,
        commands,
    })
}

fn operations(item: &PathItem) -> Vec<(&'static str, &Operation)> {
    let mut out = Vec::new();
    if let Some(op) = &item.get {
        out.push(("GET", op));
    }
    if let Some(op) = &item.post {
        out.push(("POST", op));
    }
    if let Some(op) = &item.put {
        out.push(("PUT", op));
    }
    if let Some(op) = &item.patch {
        out.push(("PATCH", op));
    }
    if let Some(op) = &item.delete {
        out.push(("DELETE", op));
    }
    if let Some(op) = &item.head {
        out.push(("HEAD", op));
    }
    if let Some(op) = &item.options {
        out.push(("OPTIONS", op));
    }
    if let Some(op) = &item.trace {
        out.push(("TRACE", op));
    }
    out
}

fn convert_operation(
    doc: &OpenApiDoc,
    path: &str,
    item: &PathItem,
    method: &str,
    op: &Operation,
) -> Result<XCommand> {
    let name = command_name(op, method, path);
    let description = op
        .summary
        .clone()
        .or_else(|| op.description.clone());

    let mut params: Vec<Parameter> = Vec::new();
    for p in &item.parameters {
        params.push(doc.resolve_parameter(p)?);
    }
    for p in &op.parameters {
        params.push(doc.resolve_parameter(p)?);
    }

    let mut path_params = Vec::new();
    let mut query_params = Vec::new();
    for p in params {
        let field = param_field(&p);
        match p.location.as_str() {
            "path" => path_params.push(field),
            "query" => query_params.push(field),
            // header/cookie ignored for now
            _ => {}
        }
    }

    let mut body_fields = Vec::new();
    let mut has_json_body = false;
    if let Some(body_ref) = &op.request_body {
        let body = doc.resolve_request_body(body_ref)?;
        if let Some(media) = body
            .content
            .get("application/json")
            .or_else(|| body.content.values().next())
        {
            has_json_body = true;
            if let Some(schema_or_ref) = &media.schema {
                let schema = doc.resolve_schema(schema_or_ref)?;
                body_fields = body_fields_from_schema(doc, &schema)?;
            }
        }
    }

    Ok(XCommand {
        name,
        description,
        method: method.to_string(),
        path_template: path.to_string(),
        path_params,
        query_params,
        body_fields,
        has_json_body,
    })
}

fn param_field(p: &Parameter) -> ParamField {
    let required = p.required.unwrap_or(p.location == "path");
    let (default, choices) = match &p.schema {
        Some(schema) => {
            let default = schema.default.as_ref().map(value_to_string);
            let choices = schema.enum_vals().map(|vals| {
                vals.iter().map(value_to_string).collect::<Vec<_>>()
            });
            (default, choices)
        }
        None => (None, None),
    };
    ParamField {
        name: p.name.clone(),
        required,
        default,
        choices,
        description: p.description.clone(),
    }
}

fn body_fields_from_schema(doc: &OpenApiDoc, schema: &crate::openapi::Schema) -> Result<Vec<BodyField>> {
    let mut fields = Vec::new();
    for (name, prop) in &schema.properties {
        // Resolve refs for validation; property shape itself is enough for fields.
        let _ = doc.resolve_schema(prop)?;
        let required = schema.required.iter().any(|r| r == name);
        fields.push(BodyField {
            name: name.clone(),
            required,
            description: None,
        });
    }
    Ok(fields)
}

fn command_name(op: &Operation, method: &str, path: &str) -> String {
    if let Some(id) = &op.operation_id {
        let slug = slugify(id);
        if !slug.is_empty() {
            return slug;
        }
    }
    // Fallback: method + last non-param path segment
    let segment = path
        .split('/')
        .filter(|s| !s.is_empty() && !s.starts_with('{'))
        .last()
        .unwrap_or("root");
    slugify(&format!("{}-{}", method.to_ascii_lowercase(), segment))
}

/// Slugify for x.sh command / app names: lowercase, alphanumeric + dashes.
pub fn slugify(input: &str) -> String {
    let mut out = String::new();
    let mut prev_dash = false;
    for c in input.chars() {
        let lower = c.to_ascii_lowercase();
        if lower.is_ascii_alphanumeric() {
            out.push(lower);
            prev_dash = false;
        } else if !prev_dash && !out.is_empty() {
            out.push('-');
            prev_dash = true;
        }
    }
    while out.ends_with('-') {
        out.pop();
    }
    out
}

/// App name from `info.title`: slugify, then drop a trailing `-api` suffix.
pub fn app_name_from_title(title: &str) -> String {
    let mut name = slugify(title);
    if let Some(stripped) = name.strip_suffix("-api") {
        if !stripped.is_empty() {
            name = stripped.to_string();
        }
    }
    name
}

fn value_to_string(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

/// Build a curl path with `${var}` substitutions for path params.
pub fn curl_path(template: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = template.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '{' {
            if let Some(end) = chars[i + 1..].iter().position(|&c| c == '}') {
                let name: String = chars[i + 1..i + 1 + end].iter().collect();
                out.push_str("${");
                out.push_str(&name);
                out.push('}');
                i += end + 2;
                continue;
            }
        }
        out.push(chars[i]);
        i += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugify_title() {
        assert_eq!(slugify("Petstore API"), "petstore-api");
        assert_eq!(slugify("list-pets"), "list-pets");
        assert_eq!(slugify("Get Pet"), "get-pet");
    }

    #[test]
    fn curl_path_subst() {
        assert_eq!(curl_path("/pets/{petId}"), "/pets/${petId}");
        assert_eq!(curl_path("/a/{x}/b/{y}"), "/a/${x}/b/${y}");
    }
}
