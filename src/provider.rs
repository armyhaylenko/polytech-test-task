use std::fmt::{Display, Formatter};

/// Errors related to provider operations.
#[derive(Debug)]
pub enum ProviderError {
    UnknownProvider,
    ReqwestError(reqwest::Error),
}

/// The weather API provider to use.
#[derive(Debug, Default)]
pub enum Provider {
    #[default]
    OpenWeather,
    Weatherapi,
}

impl From<reqwest::Error> for ProviderError {
    fn from(value: reqwest::Error) -> Self {
        ProviderError::ReqwestError(value)
    }
}

impl Provider {
    /// Universal function to get the weather using a specific API provider.
    pub fn get_weather_json(
        &self,
        address: String,
        api_key: String,
    ) -> Result<serde_json::Value, ProviderError> {
        match self {
            Provider::OpenWeather => self.get_weather_at_address_openweather(address, api_key),
            Provider::Weatherapi => self.get_weather_at_address_weatherapi(address, api_key),
        }
    }

    // provider-specific functions
    
    fn get_weather_at_address_openweather(
        &self,
        address: String,
        api_key: String,
    ) -> Result<serde_json::Value, ProviderError> {
        let response_json = reqwest::blocking::get(format!(
            "https://api.openweathermap.org/data/2.5/weather?q={address}&appid={api_key}"
        ))?
        .json::<serde_json::Value>()?;

        Ok(response_json)
    }

    fn get_weather_at_address_weatherapi(
        &self,
        address: String,
        api_key: String,
    ) -> Result<serde_json::Value, ProviderError> {
        let response_json = reqwest::blocking::get(format!(
            "https://api.weatherapi.com/v1/current.json?key={api_key}&q={address}&aqi=no"
        ))?
        .json::<serde_json::Value>()?;

        Ok(response_json)
    }
}

impl Display for Provider {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let provider_lowercase_name = format!("{self:?}").to_lowercase();
        write!(f, "{provider_lowercase_name}")
    }
}

impl TryFrom<&str> for Provider {
    type Error = ProviderError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "openweather" => Ok(Provider::OpenWeather),
            "weatherapi" => Ok(Provider::Weatherapi),
            _ => Err(ProviderError::UnknownProvider),
        }
    }
}
