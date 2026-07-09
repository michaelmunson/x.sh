//! OpenAPI → x.sh conversion library (used by the `openapi` plugin binary).

pub mod convert;
pub mod openapi;
pub mod render;

pub use convert::{convert, XApp};
pub use openapi::{parse_auto, parse_json, parse_yaml, OpenApiDoc};
pub use render::render;
