//! Bash preamble compiled into the binary.

/// The literal contents of `app/preamble.sh`. Concatenated before the user's
/// handler body in [`crate::app::run`].
pub const PREAMBLE: &str = include_str!("preamble.sh");
