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

#[test]
fn fixture_style_app_prt_emits_sgr() {
    let dir = tempfile::tempdir().unwrap();
    let fixture = manifest_dir().join("tests/fixtures/style.x.yml");
    fs::copy(fixture, dir.path().join("style.x.yml")).unwrap();

    x_cmd()
        .current_dir(dir.path())
        .args(["style", "prt"])
        .assert()
        .success()
        .stdout("\x1b[31mHi\x1b[0m");
}

#[test]
fn fixture_style_app_tui_clear_emits_escape() {
    let dir = tempfile::tempdir().unwrap();
    let fixture = manifest_dir().join("tests/fixtures/style.x.yml");
    fs::copy(fixture, dir.path().join("style.x.yml")).unwrap();

    x_cmd()
        .current_dir(dir.path())
        .args(["style", "clear"])
        .assert()
        .success()
        .stdout("\x1b[2J");
}

#[test]
fn fixture_style_app_unknown_style_fails() {
    let dir = tempfile::tempdir().unwrap();
    let fixture = manifest_dir().join("tests/fixtures/style.x.yml");
    fs::copy(fixture, dir.path().join("style.x.yml")).unwrap();

    x_cmd()
        .current_dir(dir.path())
        .args(["style", "bad"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("unknown style"));
}

fn simple_dir() -> PathBuf {
    manifest_dir().join("docs/examples/app")
}

#[test]
fn fixture_path_root_builtin_prints_app_directory() {
    let dir = tempfile::tempdir().unwrap();
    let fixture = manifest_dir().join("tests/fixtures/path-root.x.yml");
    fs::copy(fixture, dir.path().join("path-root.x.yml")).unwrap();

    x_cmd()
        .current_dir(dir.path())
        .args(["path-root", "show"])
        .assert()
        .success()
        .stdout(format!("{}\n", dir.path().display()));
}

#[test]
fn fixture_path_root_builtin_uses_app_dir_not_cwd() {
    let dir = tempfile::tempdir().unwrap();
    let fixture = manifest_dir().join("tests/fixtures/path-root.x.yml");
    fs::copy(fixture, dir.path().join("path-root.x.yml")).unwrap();
    let sub = dir.path().join("pkg");
    fs::create_dir(&sub).unwrap();

    x_cmd()
        .current_dir(&sub)
        .args(["path-root", "show"])
        .assert()
        .success()
        .stdout(format!("{}\n", dir.path().display()));
}

#[test]
fn fixture_sh_import_sources_script_before_handler() {
    let dir = tempfile::tempdir().unwrap();
    let fixture_dir = manifest_dir().join("tests/fixtures");
    fs::copy(fixture_dir.join("sh-import.x.yml"), dir.path().join("sh-import.x.yml")).unwrap();
    fs::create_dir_all(dir.path().join("helpers")).unwrap();
    fs::copy(
        fixture_dir.join("helpers/example.sh"),
        dir.path().join("helpers/example.sh"),
    )
    .unwrap();

    x_cmd()
        .current_dir(dir.path())
        .args(["sh-import", "run"])
        .assert()
        .success()
        .stdout("from-import\n");
}

#[test]
fn project_x_yml_runs_dot_command() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("x.yml"),
        ".greet: echo \"hi from x.yml\"\n",
    )
    .unwrap();

    x_cmd()
        .current_dir(dir.path())
        .arg("greet")
        .assert()
        .success()
        .stdout("hi from x.yml\n");
}

#[test]
fn project_x_yml_nested_subcommand() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("x.yml"),
        r#"
.deploy:
  .prod:
    $: echo "deploying prod"
"#,
    )
    .unwrap();

    x_cmd()
        .current_dir(dir.path())
        .args(["deploy", "prod"])
        .assert()
        .success()
        .stdout("deploying prod\n");
}

#[test]
fn alias_command_dispatches_to_another_x_file() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("other.x.yml"),
        r#"
name: other
.hello:
  arguments: "<name>"
  $: echo "hello $(x-arg name)"
"#,
    )
    .unwrap();
    fs::write(
        dir.path().join("x.yml"),
        r#"
.sub:
  alias: ./other.x.yml
"#,
    )
    .unwrap();

    x_cmd()
        .current_dir(dir.path())
        .args(["sub", "hello", "world"])
        .assert()
        .success()
        .stdout("hello world\n");
}

#[test]
fn multiline_opts_string_parses_like_a_list() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("multi.x.yml"),
        r#"
name: multi
.run:
  opts: |
    [--bool]
    [--str <arg>]
  $: |
    echo "bool=$(x-opt bool) str=$(x-opt str)"
"#,
    )
    .unwrap();

    x_cmd()
        .current_dir(dir.path())
        .args(["multi", "run", "--bool", "--str", "hi"])
        .assert()
        .success()
        .stdout("bool=true str=hi\n");
}

#[test]
fn commands_key_is_rejected_with_helpful_error() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("old.x.yml"),
        "name: old\ncommands:\n  run: {}\n",
    )
    .unwrap();

    x_cmd()
        .current_dir(dir.path())
        .args(["old", "run"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("`commands:` was removed in v3"));
}

#[test]
fn simple_app_get_env() {
    x_cmd()
        .current_dir(simple_dir())
        .args(["simple", "get-env"])
        .assert()
        .success()
        .stdout("global\nenv1\n");
}
