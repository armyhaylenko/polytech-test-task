use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ProviderError {
    UnknownProvider,
}

#[derive(Debug, Default)]
pub enum Provider {
    #[default]
    OpenWeather,
    Weatherapi,
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
