pub fn kelvin_to_celsius(kelvin: f32) -> u32 {
    let celsius: f32 = kelvin - 273.15f32;
    celsius.round() as u32
}

pub fn setup_logger() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    env_logger::init();
}
