//! OpenAPI → x.sh app converter plugin.
//!
//! Usage: `x --plugin openapi <spec.json|spec.yaml> [-o <out.x.yml>]`

use anyhow::{bail, Context, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use x_plugin_openapi::{convert, parse_auto, parse_json, parse_yaml, render};

#[derive(Parser, Debug)]
#[command(
    name = "openapi",
    about = "Convert an OpenAPI specification into an x.sh app (.x.yml)"
)]
struct Cli {
    /// Path to an OpenAPI 3.x specification (.json, .yaml, or .yml)
    spec: PathBuf,

    /// Output path (default: ./<name>.x.yml from info.title)
    #[arg(short = 'o', long = "output")]
    output: Option<PathBuf>,

    /// Print the generated app YAML to stdout instead of writing a file
    #[arg(long = "stdout")]
    stdout: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    if !cli.spec.is_file() {
        bail!("OpenAPI spec not found: {}", cli.spec.display());
    }

    let raw = fs::read_to_string(&cli.spec)
        .with_context(|| format!("Failed to read {}", cli.spec.display()))?;

    let ext = cli
        .spec
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    let doc = match ext.as_str() {
        "json" => parse_json(&raw)?,
        "yaml" | "yml" => parse_yaml(&raw)?,
        "" => parse_auto(&raw)?,
        other => bail!(
            "Unsupported spec extension '.{}' (use .json, .yaml, or .yml)",
            other
        ),
    };

    let app = convert(&doc)?;
    let yaml = render(&app);

    if cli.stdout {
        print!("{}", yaml);
        return Ok(());
    }

    let out = match cli.output {
        Some(path) => path,
        None => PathBuf::from(format!("{}.x.yml", app.name)),
    };

    if let Some(parent) = out.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create {}", parent.display()))?;
        }
    }

    fs::write(&out, &yaml)
        .with_context(|| format!("Failed to write {}", out.display()))?;

    eprintln!("Wrote {}", out.display());
    Ok(())
}
