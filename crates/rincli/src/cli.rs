use clap::{Parser, Subcommand};
use crate::commands;
use crate::errors::RincliError;

#[derive(Parser)]
#[command(name = "rincli", about = "RinCLI - Local GGUF LLM Manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Doctor,
    Search {
        query: String,
        #[arg(long)]
        plain: bool,
        #[arg(long)]
        json: bool,
    },
    Install {
        model: String,
    },
    Models,
    Run {
        model: String,
    },
    Ps,
    Stop,
    Logs,
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
    Runtime {
        #[command(subcommand)]
        command: RuntimeCommands,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    Get,
    Set { key: String, value: String },
}

#[derive(Subcommand)]
pub enum RuntimeCommands {
    Install,
    Doctor,
}

pub fn run() -> Result<(), RincliError> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Doctor => commands::doctor::run(),
        Commands::Search { query, plain, json } => commands::search::run(query, plain, json),
        Commands::Install { model } => commands::install::run(model),
        Commands::Models => commands::models::run(),
        Commands::Run { model } => commands::run::run(model),
        Commands::Ps => commands::ps::run(),
        Commands::Stop => commands::stop::run(),
        Commands::Logs => commands::logs::run(),
        Commands::Config { command } => match command {
            ConfigCommands::Get => commands::config::run_get(),
            ConfigCommands::Set { key, value } => commands::config::run_set(key, value),
        },
        Commands::Runtime { command } => match command {
            RuntimeCommands::Install => commands::runtime::run_install(),
            RuntimeCommands::Doctor => commands::runtime::run_doctor(),
        },
    }
}
