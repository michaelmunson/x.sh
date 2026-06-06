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
