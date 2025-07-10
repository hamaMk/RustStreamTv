use serde::Deserialize;
use std::path::Path;

/// Represents the configuration settings loaded from a file.
///
/// This config struct is typically deserialized from a TOML, YAML, or JSON file,
/// and holds essential runtime parameters like:
/// - `folder`: The directory to serve or scan
/// - `port`: The TCP port the application will bind to
/// - `device_name`: A friendly identifier for this device instance
#[derive(Debug, Deserialize)]
pub struct Config {
    pub folder: String,
    pub port: u16,
    pub device_name: String,
}

impl Config {
    /// Loads the configuration from a file and deserializes it into a `Config` struct.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice that holds the file path to the config file.
    ///
    /// # Errors
    ///
    /// Returns a [`config::ConfigError`] if the file can't be read or parsed.
    pub fn from_file(path: &str) -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::File::from(Path::new(path)))
            .build()?
            .try_deserialize()
    }
}
