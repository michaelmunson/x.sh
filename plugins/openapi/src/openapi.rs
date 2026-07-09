//! Minimal OpenAPI 3.x document model (only fields needed for conversion).

use anyhow::{bail, Context, Result};
use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Deserialize)]
pub struct OpenApiDoc {
    pub openapi: Option<String>,
    pub info: Info,
    #[serde(default)]
    pub servers: Vec<Server>,
    #[serde(default)]
    pub paths: BTreeMap<String, PathItem>,
    #[serde(default)]
    pub components: Option<Components>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Info {
    pub title: String,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PathItem {
    pub get: Option<Operation>,
    pub post: Option<Operation>,
    pub put: Option<Operation>,
    pub patch: Option<Operation>,
    pub delete: Option<Operation>,
    pub head: Option<Operation>,
    pub options: Option<Operation>,
    pub trace: Option<Operation>,
    #[serde(default)]
    pub parameters: Vec<ParameterOrRef>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Operation {
    #[serde(rename = "operationId")]
    pub operation_id: Option<String>,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub parameters: Vec<ParameterOrRef>,
    #[serde(rename = "requestBody")]
    pub request_body: Option<RequestBodyOrRef>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum ParameterOrRef {
    Ref {
        #[serde(rename = "$ref")]
        ref_path: String,
    },
    Param(Parameter),
}

#[derive(Debug, Clone, Deserialize)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "in")]
    pub location: String,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub schema: Option<Schema>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum RequestBodyOrRef {
    Ref {
        #[serde(rename = "$ref")]
        ref_path: String,
    },
    Body(RequestBody),
}

#[derive(Debug, Clone, Deserialize)]
pub struct RequestBody {
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub content: BTreeMap<String, MediaType>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MediaType {
    #[serde(default)]
    pub schema: Option<SchemaOrRef>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum SchemaOrRef {
    Ref {
        #[serde(rename = "$ref")]
        ref_path: String,
    },
    Schema(Schema),
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Schema {
    #[serde(rename = "type")]
    pub schema_type: Option<String>,
    #[serde(default)]
    pub format: Option<String>,
    #[serde(default)]
    pub default: Option<Value>,
    #[serde(rename = "enum", default)]
    pub r#enum: Option<Vec<Value>>,
    #[serde(default)]
    pub properties: BTreeMap<String, SchemaOrRef>,
    #[serde(default)]
    pub required: Vec<String>,
    #[serde(default)]
    pub items: Option<Box<SchemaOrRef>>,
    #[serde(rename = "$ref")]
    pub ref_path: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Components {
    #[serde(default)]
    pub schemas: BTreeMap<String, Schema>,
    #[serde(default)]
    pub parameters: BTreeMap<String, Parameter>,
    #[serde(default)]
    #[serde(rename = "requestBodies")]
    pub request_bodies: BTreeMap<String, RequestBody>,
}

impl Schema {
    pub fn enum_vals(&self) -> Option<&Vec<Value>> {
        self.r#enum.as_ref()
    }
}

pub fn parse_json(raw: &str) -> Result<OpenApiDoc> {
    serde_json::from_str(raw).context("Failed to parse OpenAPI JSON")
}

pub fn parse_yaml(raw: &str) -> Result<OpenApiDoc> {
    serde_yaml::from_str(raw).context("Failed to parse OpenAPI YAML")
}

pub fn parse_auto(raw: &str) -> Result<OpenApiDoc> {
    let trimmed = raw.trim_start();
    if trimmed.starts_with('{') {
        parse_json(raw)
    } else {
        parse_yaml(raw)
    }
}

impl OpenApiDoc {
    pub fn resolve_parameter(&self, p: &ParameterOrRef) -> Result<Parameter> {
        match p {
            ParameterOrRef::Param(param) => Ok(param.clone()),
            ParameterOrRef::Ref { ref_path } => {
                let name = component_name(ref_path, "#/components/parameters/")?;
                self.components
                    .as_ref()
                    .and_then(|c| c.parameters.get(name))
                    .cloned()
                    .with_context(|| format!("Unresolved parameter ref: {}", ref_path))
            }
        }
    }

    pub fn resolve_request_body(&self, body: &RequestBodyOrRef) -> Result<RequestBody> {
        match body {
            RequestBodyOrRef::Body(b) => Ok(b.clone()),
            RequestBodyOrRef::Ref { ref_path } => {
                let name = component_name(ref_path, "#/components/requestBodies/")?;
                self.components
                    .as_ref()
                    .and_then(|c| c.request_bodies.get(name))
                    .cloned()
                    .with_context(|| format!("Unresolved requestBody ref: {}", ref_path))
            }
        }
    }

    pub fn resolve_schema(&self, schema: &SchemaOrRef) -> Result<Schema> {
        match schema {
            SchemaOrRef::Schema(s) => {
                if let Some(ref_path) = &s.ref_path {
                    return self.resolve_schema_ref(ref_path);
                }
                Ok(s.clone())
            }
            SchemaOrRef::Ref { ref_path } => self.resolve_schema_ref(ref_path),
        }
    }

    fn resolve_schema_ref(&self, ref_path: &str) -> Result<Schema> {
        let name = component_name(ref_path, "#/components/schemas/")?;
        self.components
            .as_ref()
            .and_then(|c| c.schemas.get(name))
            .cloned()
            .with_context(|| format!("Unresolved schema ref: {}", ref_path))
    }
}

fn component_name<'a>(ref_path: &'a str, prefix: &str) -> Result<&'a str> {
    ref_path
        .strip_prefix(prefix)
        .filter(|s| !s.is_empty() && !s.contains('/'))
        .ok_or_else(|| anyhow::anyhow!("Unsupported $ref: {}", ref_path))
}

pub fn validate_openapi_version(doc: &OpenApiDoc) -> Result<()> {
    match &doc.openapi {
        Some(v) if v.starts_with('3') => Ok(()),
        Some(v) => bail!("Unsupported OpenAPI version '{}' (need 3.x)", v),
        None => Ok(()), // tolerate missing; still try to convert
    }
}
