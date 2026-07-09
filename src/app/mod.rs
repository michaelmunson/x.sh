//! `x` app framework.
//!
//! Loads `<name>.x.yml`, validates user input against a synopsis DSL, and
//! dispatches to bash handlers with `x-*` builtins available.

pub mod help;
pub mod init;
pub mod io;
pub mod loader;
pub mod parse;
pub mod preamble;
pub mod run;
pub mod spec;
pub mod synopsis;
pub mod validate;

#[cfg(test)]
mod pipeline_tests {
    use std::path::PathBuf;

    use super::{help, loader, parse, validate};

    fn exapp() -> super::spec::App {
        let path =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("docs/examples/app/exapp.x.yml");
        loader::load(&path).unwrap()
    }

    #[test]
    fn exapp_help_renders_demo_opts() {
        let app = exapp();
        let text = help::render(&app, &["demo".into(), "opts".into()]).unwrap();
        assert!(text.contains("Usage: exapp demo opts"));
        assert!(text.contains("--dry-run"));
        assert!(text.contains("--commit"));
    }

    #[test]
    fn exapp_parse_demo_pick_choice() {
        let app = exapp();
        let parsed = parse::parse(&app, &["demo".into(), "pick".into(), "west".into()]).unwrap();
        assert_eq!(parsed.arguments.get("choice").map(|v| v[0].as_str()), Some("west"));
    }

    #[test]
    fn exapp_parse_root_defaults() {
        let app = exapp();
        let parsed = parse::parse(&app, &[]).unwrap();
        assert!(parsed.command_path.is_empty());
        assert_eq!(parsed.arguments.get("topic").map(|v| v[0].as_str()), Some("overview"));
    }
}
