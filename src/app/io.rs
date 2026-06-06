//! Hidden `x __io …` entry point for interactive builtins used from app handler bash
//! (`x-io-read`, `x-io-confirm`, `x-io-select`).

use std::collections::HashSet;
use std::io::{self, Write};

use anyhow::{anyhow, Result};
use clap::Parser;
use inquire::{Confirm, MultiSelect, Select, Text};

/// Run `x __io …` after stripping the binary name and `__io`.
pub fn main(args: &[String]) -> Result<()> {
    let Some((sub, rest)) = args.split_first() else {
        anyhow::bail!("usage: x __io {{read|confirm|select}} …");
    };
    match sub.as_str() {
        "read" => cmd_read(rest),
        "confirm" => cmd_confirm(rest),
        "select" => cmd_select(rest),
        _ => anyhow::bail!("unknown __io subcommand `{}`", sub),
    }
}

fn cmd_read(args: &[String]) -> Result<()> {
    let cli = IoRead::try_parse_from(std::iter::once("x-io-read".to_string()).chain(args.iter().cloned()))?;
    let answer = Text::new(&cli.prompt).prompt().map_err(io_error)?;
    println!("{answer}");
    Ok(())
}

fn cmd_confirm(args: &[String]) -> Result<()> {
    let cli = IoConfirm::try_parse_from(std::iter::once("x-io-confirm".to_string()).chain(args.iter().cloned()))?;
    let default = match cli.default {
        DefaultYesNo::Yes => true,
        DefaultYesNo::No => false,
    };
    let answer = Confirm::new(&cli.prompt)
        .with_default(default)
        .prompt()
        .map_err(io_error)?;
    println!("{answer}");
    Ok(())
}

fn cmd_select(args: &[String]) -> Result<()> {
    // `--search` is a no-op (filtering is on by default); accept for CLI ergonomics.
    let filtered: Vec<String> = args
        .iter()
        .filter(|s| s.as_str() != "--search")
        .cloned()
        .collect();

    let cli = IoSelect::try_parse_from(std::iter::once("x-io-select".to_string()).chain(filtered.into_iter()))?;

    let allow_search = !cli.no_search;
    let (prompt, option_strs) = split_prompt_and_options(cli.rest)?;
    if option_strs.is_empty() {
        anyhow::bail!("at least one option is required (each `id=label`, split on the first `=`)");
    }

    let mut seen_ids = HashSet::new();
    let mut seen_labels = HashSet::new();
    let mut opts: Vec<SelectOpt> = Vec::new();
    for raw in &option_strs {
        let opt = parse_option(raw, &mut seen_ids, &mut seen_labels)?;
        opts.push(opt);
    }
    let labels: Vec<String> = opts.iter().map(|o| o.label.clone()).collect();

    if cli.multi {
        let mut ms = MultiSelect::new(&prompt, labels);
        if !allow_search {
            ms = ms.without_filtering();
        }
        let chosen_labels = ms.prompt().map_err(io_error)?;
        for label in chosen_labels {
            let id = opts
                .iter()
                .find(|o| o.label == label)
                .map(|o| o.id.as_str())
                .ok_or_else(|| anyhow!("internal error: unknown label"))?;
            println!("{id}");
        }
    } else {
        let mut sel = Select::new(&prompt, labels);
        if !allow_search {
            sel = sel.without_filtering();
        }
        let label = sel.prompt().map_err(io_error)?;
        let id = opts
            .iter()
            .find(|o| o.label == label)
            .map(|o| o.id.as_str())
            .ok_or_else(|| anyhow!("internal error: unknown label"))?;
        println!("{id}");
    }
    Ok(())
}

/// If the first token contains `=`, treat every token as an option (prompt `?`).
/// Otherwise the first token is the prompt and the rest are options.
fn split_prompt_and_options(args: Vec<String>) -> Result<(String, Vec<String>)> {
    if args.is_empty() {
        return Err(anyhow!("expected at least one `<id=label>` option"));
    }
    if args[0].contains('=') {
        return Ok(("?".to_string(), args));
    }
    if args.len() < 2 {
        return Err(anyhow!(
            "with a prompt that does not contain `=`, provide at least one `<id=label>` option after it"
        ));
    }
    let mut it = args.into_iter();
    let prompt = it.next().expect("non-empty");
    let opts: Vec<_> = it.collect();
    Ok((prompt, opts))
}

struct SelectOpt {
    id: String,
    label: String,
}

fn parse_option(raw: &str, seen_ids: &mut HashSet<String>, seen_labels: &mut HashSet<String>) -> Result<SelectOpt> {
    let t = raw.trim();
    if t.is_empty() {
        anyhow::bail!("empty option string");
    }
    let eq = t
        .find('=')
        .ok_or_else(|| anyhow!("option must contain `=`: `{}`", raw))?;
    let id = t[..eq].trim().to_string();
    let label = t[eq + 1..].trim().to_string();
    if id.is_empty() {
        anyhow::bail!("empty id before `=` in `{}`", raw);
    }
    if label.is_empty() {
        anyhow::bail!("empty label after `=` in `{}`", raw);
    }
    if seen_ids.contains(&id) {
        anyhow::bail!("duplicate option id `{}`", id);
    }
    if seen_labels.contains(&label) {
        anyhow::bail!("duplicate option label `{}`", label);
    }
    seen_ids.insert(id.clone());
    seen_labels.insert(label.clone());
    Ok(SelectOpt { id, label })
}

fn io_error(e: inquire::InquireError) -> anyhow::Error {
    anyhow::anyhow!("prompt failed: {e}")
}

#[derive(Parser)]
#[command(name = "x-io-read")]
struct IoRead {
    #[arg(default_value = "?")]
    prompt: String,
}

#[derive(Parser)]
#[command(name = "x-io-confirm")]
struct IoConfirm {
    #[arg(long, value_enum, default_value_t = DefaultYesNo::No)]
    default: DefaultYesNo,

    #[arg(default_value = "?")]
    prompt: String,
}

#[derive(Clone, Copy, Default, clap::ValueEnum)]
enum DefaultYesNo {
    Yes,
    #[default]
    No,
}

#[derive(Parser)]
#[command(name = "x-io-select")]
struct IoSelect {
    #[arg(long)]
    multi: bool,

    /// Disable typing to filter the option list (filtering is on by default).
    #[arg(long = "no-search")]
    no_search: bool,

    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    rest: Vec<String>,
}

/// Print error to stderr and exit with status 1. Used from [`crate::main`].
pub fn run_or_exit(args: &[String]) -> ! {
    if let Err(e) = main(args) {
        let mut stderr = io::stderr().lock();
        let _ = writeln!(stderr, "{e:#}");
        std::process::exit(1);
    }
    std::process::exit(0);
}
