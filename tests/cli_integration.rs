//! End-to-end CLI tests - spawn the `x` binary and assert on exit codes and output.

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
fn complete_project_build_subcommand() {
    x_cmd()
        .current_dir(manifest_dir())
        .args(["__complete", "bash", "x", "bu", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("build"));
}

#[test]
fn complete_project_nested_build_subcommands() {
    x_cmd()
        .current_dir(manifest_dir())
        .args(["__complete", "bash", "x", "build", "", "2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("tool"))
        .stdout(predicate::str::contains("bin"));
}

#[test]
fn complete_project_command_options() {
    x_cmd()
        .current_dir(manifest_dir())
        .args(["__complete", "bash", "x", "test", "--", "2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--complete"))
        .stdout(predicate::str::contains("--integration"));
}

#[test]
fn complete_through_alias_subcommands() {
    x_cmd()
        .current_dir(manifest_dir())
        .args(["__complete", "bash", "x", "exapp", "", "2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("demo"))
        .stdout(predicate::str::contains("nested"));
}

#[test]
fn complete_nested_alias_target_subcommands() {
    x_cmd()
        .current_dir(manifest_dir())
        .args(["__complete", "bash", "x", "exapp", "nested", "", "3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list-dir"));
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
fn opts_set_and_args_set_assign_shell_variables() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("setvars.x.yml"),
        r#"
name: setvars
.hello:
  args: '[<name="world">]'
  opts: |
    (-g | --greeting=<greeting='hello'>)
    (-p | --punctuation=<punctuation='!'>)
    [--dry-run]
  $: |
    # selective + rename
    x-args-set name=NAME
    x-opts-set greeting
    echo "partial=$greeting,$NAME"

    # all remaining (and re-set) with auto names; hyphens → underscores
    x-opts-set
    x-args-set
    echo "all=$greeting,$name$punctuation"
    [[ $dry_run == true ]] && echo "dry"
"#,
    )
    .unwrap();

    x_cmd()
        .current_dir(dir.path())
        .args([
            "setvars",
            "hello",
            "-g",
            "hi",
            "-p",
            "?",
            "--dry-run",
            "there",
        ])
        .assert()
        .success()
        .stdout("partial=hi,there\nall=hi,there?\ndry\n");
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

#[test]
fn plugin_missing_errors_helpfully() {
    let home = tempfile::tempdir().unwrap();
    x_cmd()
        .env("HOME", home.path())
        .args(["--plugin", "openapi", "--help"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"))
        .stderr(predicate::str::contains("x -i --plugin openapi"));
}

#[test]
fn plugin_openapi_converts_petstore_fixture() {
    let home = tempfile::tempdir().unwrap();
    let plugins = home.path().join(".x.sh/plugins");
    fs::create_dir_all(&plugins).unwrap();

    // Use the workspace-built openapi binary as an "installed" plugin.
    let built = manifest_dir().join("target/debug/openapi");
    let built_release = manifest_dir().join("target/release/openapi");
    let src = if built.is_file() {
        built
    } else if built_release.is_file() {
        built_release
    } else {
        // Ensure the plugin binary exists for this test.
        let status = std::process::Command::new("cargo")
            .args(["build", "-p", "x-plugin-openapi"])
            .current_dir(manifest_dir())
            .status()
            .expect("cargo build openapi");
        assert!(status.success());
        manifest_dir().join("target/debug/openapi")
    };
    fs::copy(&src, plugins.join("openapi")).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(plugins.join("openapi")).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(plugins.join("openapi"), perms).unwrap();
    }

    let work = tempfile::tempdir().unwrap();
    let spec = manifest_dir().join("plugins/openapi/tests/fixtures/petstore.openapi.yaml");
    fs::copy(&spec, work.path().join("petstore.openapi.yaml")).unwrap();

    x_cmd()
        .env("HOME", home.path())
        .current_dir(work.path())
        .args(["--plugin", "openapi", "petstore.openapi.yaml"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Wrote"));

    let out = fs::read_to_string(work.path().join("petstore.x.yml")).unwrap();
    assert!(out.contains("name: petstore"));
    assert!(out.contains(".list-pets:"));
    assert!(out.contains("BASE_URL: https://api.example.com/v1"));
    assert!(out.contains(".__request:"));
    assert!(out.contains("x-run-self __request"));
}

fn run_self_fixture() -> PathBuf {
    manifest_dir().join("tests/fixtures/run-self.x.yml")
}

#[test]
fn run_self_dispatches_subcommand() {
    x_cmd()
        .args(["__run_self", &run_self_fixture().to_string_lossy(), "via-self"])
        .assert()
        .success()
        .stdout("hello from self\n");
}

#[test]
fn run_self_via_app_file() {
    x_cmd()
        .args(["__run_self", &run_self_fixture().to_string_lossy(), "__echo", "direct"])
        .assert()
        .success()
        .stdout("direct\n");
}

#[test]
fn run_self_from_handler_builtin() {
    let fixture_dir = manifest_dir().join("tests/fixtures");
    x_cmd()
        .current_dir(&fixture_dir)
        .args(["run-self", "via-self"])
        .assert()
        .success()
        .stdout("hello from self\n");
}

fn write_exe(path: &std::path::Path, body: &str) {
    fs::write(path, body).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms).unwrap();
    }
}

fn ai_plugin_binary() -> PathBuf {
    use std::sync::OnceLock;
    static PATH: OnceLock<PathBuf> = OnceLock::new();
    PATH.get_or_init(|| {
        let status = std::process::Command::new("cargo")
            .args(["build", "-p", "x-plugin-ai-cmd-gen"])
            .current_dir(manifest_dir())
            .status()
            .expect("cargo build ai-cmd-gen");
        assert!(status.success(), "ai-cmd-gen build failed");
        let debug = manifest_dir().join("target/debug/ai-cmd-gen");
        assert!(debug.is_file(), "missing {}", debug.display());
        debug
    })
    .clone()
}

fn install_ai_plugin(home: &std::path::Path) {
    let plugins = home.join(".x.sh/plugins");
    fs::create_dir_all(&plugins).unwrap();
    let dest = plugins.join("ai-cmd-gen");
    fs::copy(ai_plugin_binary(), &dest).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&dest).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&dest, perms).unwrap();
    }
}

fn write_llm_stub(home: &std::path::Path) {
    let dir = home.join(".x.sh/config");
    fs::create_dir_all(&dir).unwrap();
    write_exe(
        &dir.join("llm.sh"),
        r#"#!/bin/sh
PROMPT="$1"
printf '%s\n' "$PROMPT" > "$HOME/llm-prompt.txt"
case "$PROMPT" in
  *"Current file contents"*)
    printf '%s\n' '#!/usr/bin/env bash' 'echo hi-revised'
    ;;
  *"x.sh apps"*)
    cat <<'EOF'
```yaml
name: demoapp
version: "0.0.0"
description: generated app

.greet:
  help: say hi
  $: echo hi-from-app
```
EOF
    ;;
  *)
    printf '%s\n' '```bash' '#!/usr/bin/env bash' 'echo hi-from-ai' '```'
    ;;
esac
"#,
    );
}

fn instructions_editor(home: &std::path::Path) -> PathBuf {
    let path = home.join("edit-instructions.sh");
    write_exe(
        &path,
        r#"#!/bin/sh
cp "$1" "$HOME/instructions.md"
printf '\nEcho a short greeting.\n' >> "$1"
"#,
    );
    path
}

#[test]
fn ai_flag_without_init_explains_usage() {
    x_cmd()
        .args(["-A", "hello"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("x -i -A <name>"))
        .stderr(predicate::str::contains("x -i --app --local -A <name>"));
}

#[test]
fn ai_init_requires_installed_plugin() {
    let home = tempfile::tempdir().unwrap();
    x_cmd()
        .env("HOME", home.path())
        .args(["-i", "-A", "hello"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not installed"))
        .stderr(predicate::str::contains("x -i --plugin ai-cmd-gen"));
}

#[test]
fn ai_init_rejects_invalid_script_name() {
    let home = tempfile::tempdir().unwrap();
    x_cmd()
        .env("HOME", home.path())
        .args(["-i", "-A", "bad name"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid script name"));
}

#[test]
fn ai_init_script_opens_instructions_and_saves_generated_script() {
    let home = tempfile::tempdir().unwrap();
    install_ai_plugin(home.path());
    write_llm_stub(home.path());
    let editor = instructions_editor(home.path());

    x_cmd()
        .env("HOME", home.path())
        .env("EDITOR", &editor)
        .args(["-i", "-A", "hello"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Script 'hello' saved"));

    let seen = fs::read_to_string(home.path().join("instructions.md")).unwrap();
    assert_eq!(seen, "# hello Instructions\n");

    let script = fs::read_to_string(home.path().join(".x.sh/scripts/hello")).unwrap();
    assert_eq!(script, "#!/usr/bin/env bash\necho hi-from-ai\n");

    let prompt = fs::read_to_string(home.path().join("llm-prompt.txt")).unwrap();
    assert!(prompt.contains("Echo a short greeting."));
    assert!(prompt.contains("hello"));
    assert!(!prompt.contains("```"));

    let meta = fs::read_to_string(home.path().join(".x.sh/metadata/hello.toml")).unwrap();
    assert!(meta.contains("program = \"bash\""));
    assert!(meta.contains("Echo a short greeting."));

    x_cmd()
        .env("HOME", home.path())
        .args(["hello"])
        .assert()
        .success()
        .stdout("hi-from-ai\n");
}

#[test]
fn ai_init_script_accepts_inline_instructions() {
    let home = tempfile::tempdir().unwrap();
    install_ai_plugin(home.path());
    write_llm_stub(home.path());

    x_cmd()
        .env("HOME", home.path())
        .env("EDITOR", "/bin/false")
        .args(["-i", "-A", "hello", "print", "hi"])
        .assert()
        .success();

    let prompt = fs::read_to_string(home.path().join("llm-prompt.txt")).unwrap();
    assert!(prompt.contains("print hi"));
    let meta = fs::read_to_string(home.path().join(".x.sh/metadata/hello.toml")).unwrap();
    assert!(meta.contains("print hi"));
}

#[test]
fn ai_init_script_revises_existing_file() {
    let home = tempfile::tempdir().unwrap();
    install_ai_plugin(home.path());
    write_llm_stub(home.path());
    let editor = instructions_editor(home.path());

    x_cmd()
        .env("HOME", home.path())
        .env("EDITOR", &editor)
        .args(["-i", "-A", "hello"])
        .assert()
        .success();

    x_cmd()
        .env("HOME", home.path())
        .env("EDITOR", &editor)
        .args(["-i", "-A", "hello"])
        .assert()
        .success();

    let script = fs::read_to_string(home.path().join(".x.sh/scripts/hello")).unwrap();
    assert_eq!(script, "#!/usr/bin/env bash\necho hi-revised\n");
    let prompt = fs::read_to_string(home.path().join("llm-prompt.txt")).unwrap();
    assert!(prompt.contains("echo hi-from-ai"));
}

#[test]
fn ai_init_empty_instructions_do_not_create_a_script() {
    let home = tempfile::tempdir().unwrap();
    install_ai_plugin(home.path());
    write_llm_stub(home.path());

    x_cmd()
        .env("HOME", home.path())
        .env("EDITOR", "/bin/true")
        .args(["-i", "-A", "hello"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Instructions cannot be empty"));

    assert!(!home.path().join(".x.sh/scripts/hello").exists());
}

#[test]
fn ai_init_local_app_saves_runnable_yaml() {
    let home = tempfile::tempdir().unwrap();
    let work = tempfile::tempdir().unwrap();
    install_ai_plugin(home.path());
    write_llm_stub(home.path());
    let editor = instructions_editor(home.path());

    x_cmd()
        .env("HOME", home.path())
        .env("EDITOR", &editor)
        .current_dir(work.path())
        .args(["-i", "--app", "--local", "-A", "demoapp"])
        .assert()
        .success()
        .stdout(predicate::str::contains("demoapp.x.yml"));

    let seen = fs::read_to_string(home.path().join("instructions.md")).unwrap();
    assert_eq!(seen, "# demoapp Instructions\n");

    let app = fs::read_to_string(work.path().join("demoapp.x.yml")).unwrap();
    assert!(app.contains(".greet:"));
    assert!(!app.contains("```"));

    x_cmd()
        .env("HOME", home.path())
        .current_dir(work.path())
        .args(["demoapp", "greet"])
        .assert()
        .success()
        .stdout("hi-from-app\n");
}

#[test]
fn ai_init_global_app_saves_under_apps_dir() {
    let home = tempfile::tempdir().unwrap();
    let work = tempfile::tempdir().unwrap();
    install_ai_plugin(home.path());
    write_llm_stub(home.path());
    let editor = instructions_editor(home.path());

    x_cmd()
        .env("HOME", home.path())
        .env("EDITOR", &editor)
        .current_dir(work.path())
        .args(["-i", "--app", "--global", "-A", "demoapp"])
        .assert()
        .success();

    let app_path = home.path().join(".x.sh/apps/demoapp.x.yml");
    assert!(app_path.is_file());
    assert!(!work.path().join("demoapp.x.yml").exists());

    x_cmd()
        .env("HOME", home.path())
        .current_dir(work.path())
        .args(["demoapp", "greet"])
        .assert()
        .success()
        .stdout("hi-from-app\n");
}

#[test]
fn run_self_request_accepts_curl_options() {
    x_cmd()
        .args([
            "__run_self",
            &run_self_fixture().to_string_lossy(),
            "__request",
            "-s",
            "https://httpbin.org/get",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("httpbin.org"));
}
