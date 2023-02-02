use crate::provider::Provider;
use crate::utils::kelvin_to_celsius;

/// Errors related to weather decoding.
#[derive(Debug)]
pub enum WeatherError {
    BadResponse,
    JsonError(serde_json::Error),
}

/// Trait that unifies provider-dependent weather models.
trait WeatherT: Sized {
    /// Get a human-readable description of the weather.
    fn into_human(self) -> String;
    /// Create a `Weather` model from response json.
    fn from_json(json: serde_json::Value) -> Result<Self, WeatherError>;
}

/// Weather enum that abstracts over different provider-dependent weather models.
#[derive(Debug)]
pub enum Weather {
    OpenWeather(OpenWeather),
    Weatherapi(Weatherapi),
}

impl Weather {
    /// Get a human-readable description of the weather.
    pub fn into_human(self) -> String {
        match self {
            Weather::OpenWeather(w) => w.into_human(),
            Weather::Weatherapi(w) => w.into_human(),
        }
    }

    /// Create a `Weather` model from response json.
    pub fn from_json(json: serde_json::Value, provider: Provider) -> Result<Self, WeatherError> {
        match provider {
            Provider::OpenWeather => OpenWeather::from_json(json).map(Self::OpenWeather),
            Provider::Weatherapi => Weatherapi::from_json(json).map(Self::Weatherapi),
        }
    }
}

/// The model for OpenWeather weather info.
/// Contains min/max/current temperature, weather description,
/// humidity.
#[derive(Debug)]
pub struct OpenWeather {
    min_temperature: i32,
    max_temperature: i32,
    current_temperature: i32,
    description: String,
    humidity_percent: u8,
}

impl From<serde_json::Error> for WeatherError {
    fn from(value: serde_json::Error) -> Self {
        WeatherError::JsonError(value)
    }
}

impl OpenWeather {
    pub fn new(
        min_temperature: f64,
        max_temperature: f64,
        current_temperature: f64,
        description: &str,
        humidity_percent: u64,
    ) -> Self {
        Self {
            min_temperature: kelvin_to_celsius(min_temperature),
            max_temperature: kelvin_to_celsius(max_temperature),
            current_temperature: kelvin_to_celsius(current_temperature),
            description: String::from(description),
            humidity_percent: humidity_percent as u8,
        }
    }
}

impl WeatherT for OpenWeather {
    fn into_human(self) -> String {
        format!("Today is {}, minimum temperature: {}, maximum temperature: {}, current temperature: {}, humidity: {}%.",
                self.description,
                self.min_temperature,
                self.max_temperature,
                self.current_temperature,
                self.humidity_percent)
    }

    fn from_json(json: serde_json::Value) -> Result<Self, WeatherError> {
        let weather_description = json
            .get("weather")
            .and_then(|value| {
                value.as_array().and_then(|array| {
                    array
                        .get(0)
                        .and_then(|value| value.get("description").and_then(|value| value.as_str()))
                })
            })
            .ok_or(WeatherError::BadResponse)?;
        let current_temperature = json
            .get("main")
            .and_then(|value| value.get("temp").and_then(|value| value.as_f64()))
            .ok_or(WeatherError::BadResponse)?;
        let min_temperature = json
            .get("main")
            .and_then(|value| value.get("temp_min").and_then(|value| value.as_f64()))
            .ok_or(WeatherError::BadResponse)?;
        let max_temperature = json
            .get("main")
            .and_then(|value| value.get("temp_max").and_then(|value| value.as_f64()))
            .ok_or(WeatherError::BadResponse)?;
        let humidity = json
            .get("main")
            .and_then(|value| value.get("humidity").and_then(|value| value.as_u64()))
            .ok_or(WeatherError::BadResponse)?;
        Ok(Self::new(
            min_temperature,
            max_temperature,
            current_temperature,
            weather_description,
            humidity,
        ))
    }
}

/// WeatherAPI weather response model.
#[derive(Debug)]
pub struct Weatherapi {
    current_temperature: i32,
    feels_like: i32,
    description: String,
    humidity_percent: u8,
}

impl Weatherapi {
    pub fn new(
        current_temperature: f64,
        feels_like: f64,
        description: &str,
        humidity_percent: u64,
    ) -> Self {
        Self {
            current_temperature: current_temperature.round() as i32,
            feels_like: feels_like.round() as i32,
            description: String::from(description),
            humidity_percent: humidity_percent as u8,
        }
    }
}

impl WeatherT for Weatherapi {
    fn into_human(self) -> String {
        format!(
            "Today is {}, current temperature: {}, feels like: {}, humidity: {}%.",
            self.description, self.current_temperature, self.feels_like, self.humidity_percent
        )
    }

    fn from_json(json: serde_json::Value) -> Result<Self, WeatherError> {
        // inner "current" object to reuse
        let json = json.get("current").ok_or(WeatherError::BadResponse)?;
        let description = json
            .get("condition")
            .and_then(|value| value.get("text").and_then(|value| value.as_str()))
            .ok_or(WeatherError::BadResponse)?;
        let current_temperature = json
            .get("temp_c")
            .and_then(|value| value.as_f64())
            .ok_or(WeatherError::BadResponse)?;
        let feels_like = json
            .get("feelslike_c")
            .and_then(|value| value.as_f64())
            .ok_or(WeatherError::BadResponse)?;
        let humidity_percent = json
            .get("humidity")
            .and_then(|value| value.as_u64())
            .ok_or(WeatherError::BadResponse)?;
        Ok(Self::new(
            current_temperature,
            feels_like,
            description,
            humidity_percent,
        ))
    }
}
