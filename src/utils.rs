/// A function that converts f64 value in Kelvin to i32 value in Celsius.
pub fn kelvin_to_celsius(kelvin: f64) -> i32 {
    let celsius: f64 = kelvin - 273.15f64;
    celsius.round() as i32
}

/// A utility function to setup logger.
pub fn setup_logger() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    env_logger::init();
}
