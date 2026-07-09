//! Conversion tests for the openapi plugin.

use std::fs;
use std::path::PathBuf;

use x_plugin_openapi::{convert, parse_json, parse_yaml, render};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn convert_fixture(name: &str) -> String {
    let raw = fs::read_to_string(fixture(name)).expect("read fixture");
    let doc = if name.ends_with(".json") {
        parse_json(&raw).expect("parse json")
    } else {
        parse_yaml(&raw).expect("parse yaml")
    };
    let app = convert(&doc).expect("convert");
    render(&app)
}

#[test]
fn petstore_yaml_converts() {
    let yaml = convert_fixture("petstore.openapi.yaml");
    assert_petstore_app(&yaml);
}

#[test]
fn petstore_json_converts() {
    let yaml = convert_fixture("petstore.openapi.json");
    assert_petstore_app(&yaml);
}

fn assert_petstore_app(yaml: &str) {
    assert!(yaml.contains("name: petstore"), "got:\n{yaml}");
    assert!(yaml.contains("version: 1.0.0"));
    assert!(yaml.contains(
        "description: A minimal pet store API for OpenAPI → x.sh conversion demos."
    ));
    assert!(yaml.contains("BASE_URL: https://api.example.com/v1"));
    assert!(yaml.contains(".list-pets:"));
    assert!(yaml.contains(".create-pet:"));
    assert!(yaml.contains(".get-pet:"));
    assert!(yaml.contains(".delete-pet:"));
    assert!(yaml.contains("[-i | --interactive]"));
    assert!(yaml.contains("[--limit=<n='10'>]"));
    assert!(yaml.contains("[--status={available|pending|sold}]"));
    assert!(yaml.contains("[-d | --data <fieldval>...]"));
    assert!(yaml.contains("[<petId>]"));
    assert!(yaml.contains("x-io-read \"Limit (default 10):\" -v limit"));
    assert!(yaml.contains("x-io-select \"Status\""));
    assert!(yaml.contains("x-io-read \"Name:\" -v name"));
    assert!(yaml.contains("x-io-read \"Tag (optional):\" -v tag"));
    assert!(yaml.contains("x-io-read \"Pet ID:\" -v petId"));
    assert!(yaml.contains("curl -s \"${BASE_URL}/pets?${query}\""));
    assert!(yaml.contains("curl -s -X POST \"${BASE_URL}/pets\""));
    assert!(yaml.contains("curl -s \"${BASE_URL}/pets/${petId}\""));
    assert!(yaml.contains("curl -s -X DELETE \"${BASE_URL}/pets/${petId}\""));
    assert!(yaml.contains("missing required field: name"));
    assert!(yaml.contains("query=\"limit=${limit}\""));
}
