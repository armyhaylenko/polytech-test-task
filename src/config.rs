use crate::provider::{Provider, ProviderError};
use std::io::Write;

const WEATHER_CONFIG_FOLDER: &str = "weatherapp";
const WEATHER_CONFIG_FILENAME: &str = "provider";

/// Errors related to *configure* operations
#[derive(Debug)]
pub enum ConfigError {
    NoConfigDir,
    ProviderError(ProviderError),
    IoError(std::io::Error),
}

impl From<std::io::Error> for ConfigError {
    fn from(value: std::io::Error) -> Self {
        ConfigError::IoError(value)
    }
}

impl From<ProviderError> for ConfigError {
    fn from(value: ProviderError) -> Self {
        ConfigError::ProviderError(value)
    }
}

/// Retrieve provider.
/// Used to retrieve provider that is used to fetch weather data.
pub fn retrieve_provider() -> Result<Provider, ConfigError> {
    let system_config_dir = dirs::config_dir().ok_or(ConfigError::NoConfigDir)?;
    let app_config_dir = system_config_dir.join(WEATHER_CONFIG_FOLDER);
    if let Ok(provider_name) = std::fs::read_to_string(app_config_dir.join(WEATHER_CONFIG_FILENAME))
    {
        Provider::try_from(provider_name.as_str()).map_err(ConfigError::ProviderError)
    } else {
        log::info!(target: "config", "No previous configuration exists, creating default config...");
        let _ = std::fs::create_dir(&app_config_dir).map_err(|e| {
            log::warn!(target: "config", "Could not create app config dir!");
            e
        });
        let mut config_file = std::fs::File::create(app_config_dir.join(WEATHER_CONFIG_FILENAME))?;
        config_file.write_all(format!("{}", Provider::default()).as_bytes())?;
        Ok(Provider::default())
    }
}

/// Save provider.
/// Used to save specified provider for later use in configuration.
/// If the configuration file does not exist, creates a new config file with the specified
/// provider.
pub fn save_provider(provider: &str) -> Result<(), ConfigError> {
    let provider = Provider::try_from(provider)?;
    let system_config_dir = dirs::config_dir().ok_or(ConfigError::NoConfigDir)?;
    let app_config_dir = system_config_dir.join(WEATHER_CONFIG_FOLDER);
    let mut config_file = std::fs::File::create(app_config_dir.join(WEATHER_CONFIG_FILENAME))
        .or_else(|_| {
            let _ = std::fs::create_dir(&app_config_dir).map_err(|e| {
                log::warn!(target: "config", "Could not create app config dir!");
                e
            });
            std::fs::File::create(app_config_dir.join(WEATHER_CONFIG_FILENAME))
        })?;
    config_file.write_all(format!("{provider}").as_bytes())?;
    Ok(())
}
