mod commands;
mod config;
mod model;
mod provider;
mod utils;

use crate::commands::Commands;
use crate::config::{parse_apikey, save_provider};
use crate::model::Weather;
use crate::utils::setup_logger;
use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = Some("A CLI application to fetch current weather in some location"))]
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

        Commands::Get { address } => {
            let apikeys_str = include_str!("../apikeys.conf");
            let provider = config::retrieve_provider().expect("Could not retrieve provider");
            let apikey = parse_apikey(apikeys_str, &format!("{provider}"))
                .expect("Could not retrieve apikey. Maybe the api key is missing in config or the file is corrupted?");
            log::debug!("Get called with address {address}, provider {provider}");
            let weather_json = provider
                .get_weather_json(address, apikey)
                .expect("Failed to retrieve weather");
            let weather = Weather::from_json(weather_json, provider)
                .expect("Failed to deserialize json into readable format");
            println!("{}", weather.into_human())
        }
    }
}
