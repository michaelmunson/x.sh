//! End-to-end CLI tests — spawn the `x` binary and assert on exit codes and output.

use std::fs;
use std::path::PathBuf;

use assert_cmd::Command;
use predicates::prelude::*;

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn exapp_dir() -> PathBuf {
    manifest_dir().join("docs/examples/app")
}

fn exapp_file() -> PathBuf {
    exapp_dir().join("exapp.x.yml")
}

fn x_cmd() -> Command {
    Command::cargo_bin("x").unwrap()
}

#[test]
fn usage_hidden_command_renders_root_help() {
    x_cmd()
        .args(["__usage", &exapp_file().to_string_lossy(), ""])
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage: exapp"))
        .stdout(predicate::str::contains("topic"));
}

#[test]
fn usage_hidden_command_renders_nested_help() {
    x_cmd()
        .args(["__usage", &exapp_file().to_string_lossy(), "demo.opts"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage: exapp demo opts"))
        .stdout(predicate::str::contains("--dry-run"))
        .stdout(predicate::str::contains("--commit"));
}

#[test]
fn exapp_help_flag_from_example_dir() {
    x_cmd()
        .current_dir(&exapp_dir())
        .args(["exapp", "demo", "opts", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage: exapp demo opts"));
}

#[test]
fn exapp_demo_pick_runs_handler() {
    x_cmd()
        .current_dir(&exapp_dir())
        .args(["exapp", "demo", "pick", "north"])
        .assert()
        .success()
        .stdout(predicate::str::contains("direction=north"));
}

#[test]
fn exapp_demo_opts_applies_defaults_and_flags() {
    x_cmd()
        .current_dir(&exapp_dir())
        .args([
            "exapp",
            "demo",
            "opts",
            "--commit",
            "--dry-run",
            "--out",
            "tmp/out",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("dry-run=true"))
        .stdout(predicate::str::contains("out=tmp/out"))
        .stdout(predicate::str::contains("count=1"))
        .stdout(predicate::str::contains("commit=true"));
}

#[test]
fn exapp_root_handler_uses_default_topic() {
    x_cmd()
        .current_dir(&exapp_dir())
        .arg("exapp")
        .assert()
        .success()
        .stdout(predicate::str::contains("topic=overview"));
}

#[test]
fn exapp_demo_group_alpha_verbose() {
    x_cmd()
        .current_dir(&exapp_dir())
        .args(["exapp", "demo", "group", "alpha", "-V", "hello"])
        .assert()
        .success()
        .stdout(predicate::str::contains("[verbose] alpha ← hello"));
}

#[test]
fn exapp_invalid_choice_exits_with_error() {
    x_cmd()
        .current_dir(&exapp_dir())
        .args(["exapp", "demo", "pick", "up"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("must be one of"));
}

#[test]
fn complete_xpkg_build_subcommand() {
    x_cmd()
        .current_dir(manifest_dir())
        .args(["__complete", "bash", "x", "xpkg", "bu", "2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("build"));
}

#[test]
fn complete_xpkg_nested_build_subcommands() {
    x_cmd()
        .current_dir(manifest_dir())
        .args(["__complete", "bash", "x", "xpkg", "build", "", "3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("docs"))
        .stdout(predicate::str::contains("bin"));
}

#[test]
fn fixture_minimal_app_greet() {
    let dir = tempfile::tempdir().unwrap();
    let fixture = manifest_dir().join("tests/fixtures/minimal.x.yml");
    fs::copy(fixture, dir.path().join("minimal.x.yml")).unwrap();

    x_cmd()
        .current_dir(dir.path())
        .args(["minimal", "greet", "Ada"])
        .assert()
        .success()
        .stdout("hi Ada\n");
}

#[test]
fn fixture_minimal_app_verbose_greet() {
    let dir = tempfile::tempdir().unwrap();
    let fixture = manifest_dir().join("tests/fixtures/minimal.x.yml");
    fs::copy(fixture, dir.path().join("minimal.x.yml")).unwrap();

    x_cmd()
        .current_dir(dir.path())
        .args(["minimal", "greet", "--verbose", "Ada"])
        .assert()
        .success()
        .stdout("verbose hi Ada\n");
}
