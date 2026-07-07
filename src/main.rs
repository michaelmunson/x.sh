mod app;
mod make;
mod config;
mod configure;
mod complete;
mod execute;
mod link;
mod list;
mod metadata;
mod rm;
mod srcpath;
mod utils;
mod ai;

use anyhow::Result;
use clap::Parser;
use config::XConfig;

#[derive(Parser)]
#[command(name = "x", version, about = "Create and run bash functions as commands\n\nSkills          https://github.com/michaelmunson/x.sh/tree/main/docs/skills\nExamples        https://github.com/michaelmunson/x.sh/tree/main/docs/examples\nDocumentation   https://michaelmunson.github.io/x.sh/")]
#[command(arg_required_else_help = false)]
pub(crate) struct Cli {
    /// Initialize or create a script (opens your editor)
    #[arg(short = 'i', long = "init")]
    init: bool,
    
    /// Remove a script
    #[arg(short = 'd', long = "delete")]
    delete: bool,
    
    /// Create or manage a symlink to a script
    #[arg(long = "ln")]
    ln: bool,
    
    /// List all scripts
    #[arg(short = 'l', long = "ls")]
    ls: bool,
    
    /// Configure default program
    #[arg(long = "config")]
    config: bool,
    
    /// AI-powered command generation
    #[arg(long = "ai")]
    ai: bool,
    
    /// With --init: create an app config (`<name>.x.yml`) instead of a script
    #[arg(long = "app")]
    app: bool,
    
    /// With --init --app: create the app in the current directory
    #[arg(long = "local", conflicts_with = "global")]
    local: bool,
    
    /// With --init --app: create the app in `~/.x.sh/apps`
    #[arg(long = "global", conflicts_with = "local")]
    global: bool,
    
    /// Print the absolute path of a script or app instead of running it
    #[arg(long = "src")]
    src: bool,
    
    /// Script name or arguments (positional)
    #[arg(trailing_var_arg = true)]
    args: Vec<String>,
}

fn main() -> Result<()> {
    // Hidden subcommand used by the bash `x-usage` builtin.
    // Form: `x __usage <app-file> <cmd-path>` (cmd-path may be empty)
    let raw_args: Vec<String> = std::env::args().collect();
    if raw_args.len() >= 3 && raw_args[1] == "__usage" {
        let app_file = std::path::PathBuf::from(&raw_args[2]);
        let cmd_path = raw_args.get(3).cloned().unwrap_or_default();
        return app::run::print_usage(&app_file, &cmd_path);
    }
    if raw_args.len() >= 2 && raw_args[1] == "__io" {
        let rest: Vec<String> = raw_args.iter().skip(2).cloned().collect();
        app::io::run_or_exit(&rest);
    }
    if raw_args.len() >= 2 && raw_args[1] == "__complete" {
        let rest: Vec<String> = raw_args.iter().skip(2).cloned().collect();
        return complete::run_complete(&rest);
    }

    let cli = Cli::parse();
    let config = XConfig::new()?;
    
    // Handle option flags
    if cli.init && cli.app {
        let scope = if cli.local {
            Some(app::init::Scope::Local)
        } else if cli.global {
            Some(app::init::Scope::Global)
        } else {
            None
        };
        let name = cli.args.first().cloned();
        app::init::create_app(&config, name, scope)?;
    } else if cli.src {
        let name = cli.args.first()
            .ok_or_else(|| anyhow::anyhow!("--src requires a script or app name"))?;
        srcpath::print_src(&config, name)?;
    } else if cli.init {
        // For init: args[0] = name (optional), args[1..] = script content (optional)
        let (name, script) = match cli.args.as_slice() {
            [] => (None, None),
            [name] => (Some(name.clone()), None),
            [name, script_content @ ..] => {
                let script = Some(script_content.join(" "));
                (Some(name.clone()), script)
            }
        };
        if cli.ai {
            make::add_script_with_ai(&config, name, script)?;
        } else {
            make::add_script(&config, name, script)?;
        }
    } else if cli.delete && cli.ln {
        // Special case: --delete --ln means remove the link (not the script)
        // args[0] = link_name (or script name if link_name not provided)
        let link_name = cli.args.first()
            .ok_or_else(|| anyhow::anyhow!("Link name required for link removal"))?;
        link::remove_link(&config, link_name)?;
    } else if cli.delete {
        let name = cli.args.first()
            .ok_or_else(|| anyhow::anyhow!("Script name required for removal"))?;
        rm::remove_script(&config, name)?;
    } else if cli.ln {
        // For ln: args[0] = script name, args[1] = link_name (optional)
        let name = cli.args.first()
            .ok_or_else(|| anyhow::anyhow!("Script name required for linking"))?;
        let link_name = cli.args.get(1).cloned();
        link::link_script(&config, name, link_name)?;
    } else if cli.ls {
        list::list_scripts(&config)?;
    } else if cli.config {
        configure::configure(&config)?;
    } else if cli.ai {
        ai::ai_command()?;
    } else {
        // Execute script or app.
        if let Some(script_name) = cli.args.first() {
            let script_args: Vec<String> = cli.args.iter().skip(1).cloned().collect();
            
            // Resolution order:
            //   1. command in ./x.yml (project-local)
            //   2. local <name>.x.yml app (CWD, then ancestors)
            //   3. global ~/.x.sh/apps/<name>.x.yml app
            //   4. global script
            if app::loader::project_has_command(script_name)? {
                let path = XConfig::project_x_yml_path()?
                    .expect("x.yml exists when project_has_command is true");
                app::run::run_app_file(&path, &cli.args)?;
                return Ok(());
            }
            if config.find_app(script_name)?.is_some() {
                app::run::run_app(&config, script_name, &script_args)?;
                return Ok(());
            }
            execute::execute_script(&config, script_name, script_args)?;
        } else {
            let mut cmd = <Cli as clap::CommandFactory>::command();
            cmd.print_help()?;
        }
    }
    
    Ok(())
}
