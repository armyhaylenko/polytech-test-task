use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Configure the weather provider to use.
    Configure { provider_name: String },
    /// Get the weather at the specified address.
    Get { address: String },
}
