mod cli;
mod commands;
mod config;
mod db;
mod download;
mod errors;
mod hardware;
mod registry;
mod runtime;
mod tui;

fn main() -> Result<(), errors::RincliError> {
    cli::run()
}
