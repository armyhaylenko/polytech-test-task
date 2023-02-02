mod commands;
mod config;
mod provider;
mod utils;

use crate::commands::Commands;
use crate::config::save_provider;
use crate::utils::setup_logger;
use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

fn main() {
    setup_logger();
    let args = Cli::parse();
    match args.command {
        Commands::Configure { provider_name } => {
            log::debug!("Configure called with provider {provider_name}");
            save_provider(&provider_name).expect("Could not save provider")
        }
        // TODO: complete
        Commands::Get { address } => {
            let provider = config::retrieve_provider().expect("Could not retrieve provider");
            log::debug!("Get called with address {address}, provider {provider}")
        }
    }
}
