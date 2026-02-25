mod client;
mod commands;
mod config;
mod models;

use anyhow::Result;
use clap::{Parser, Subcommand};

use client::ApiClient;
use commands::config::ConfigCmd;
use commands::project::ProjectCmd;
use commands::tag::TagCmd;
use commands::task::TaskCmd;
use commands::task_group::TaskGroupCmd;

#[derive(Parser)]
#[command(
    name = "singularity",
    about = "CLI for Singularity task manager API",
    long_about = "CLI wrapper around the Singularity task manager REST API (https://api.singularity-app.com/v2/).\n\n\
        Authenticate by setting SINGULARITY_TOKEN env var or running: singularity config set-token <TOKEN>\n\n\
        Entity ID formats: P-<uuid> (projects), T-<uuid> (tasks), Q-<uuid> (task groups).\n\n\
        Use --json on any command for machine-readable JSON output."
)]
struct Cli {
    #[arg(long, global = true, help = "Output raw JSON instead of formatted tables")]
    json: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Manage CLI configuration (API token)")]
    Config {
        #[command(subcommand)]
        command: ConfigCmd,
    },
    #[command(about = "Manage projects (IDs use P-<uuid> format)")]
    Project {
        #[command(subcommand)]
        command: ProjectCmd,
    },
    #[command(about = "Manage tasks (IDs use T-<uuid> format)")]
    Task {
        #[command(subcommand)]
        command: TaskCmd,
    },
    #[command(name = "task-group", about = "Manage task groups within projects (IDs use Q-<uuid> format)")]
    TaskGroup {
        #[command(subcommand)]
        command: TaskGroupCmd,
    },
    #[command(about = "Manage tags for organizing tasks and projects")]
    Tag {
        #[command(subcommand)]
        command: TagCmd,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let json = cli.json;

    match cli.command {
        Commands::Config { command } => commands::config::run(command)?,
        Commands::Project { command } => {
            let client = ApiClient::new(config::resolve_token()?);
            commands::project::run(&client, command, json)?;
        }
        Commands::Task { command } => {
            let client = ApiClient::new(config::resolve_token()?);
            commands::task::run(&client, command, json)?;
        }
        Commands::TaskGroup { command } => {
            let client = ApiClient::new(config::resolve_token()?);
            commands::task_group::run(&client, command, json)?;
        }
        Commands::Tag { command } => {
            let client = ApiClient::new(config::resolve_token()?);
            commands::tag::run(&client, command, json)?;
        }
    }

    Ok(())
}
