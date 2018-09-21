/// Configuration Functions 

use std::path::Path;
use toml::Value;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    library: String
}

/// Read a toml configuration file.
fn read_config_file() -> Result<String, Box<::std::error::Error + 'static>> {
    let config_file = fs::read_to_string("library.toml")?;
    Ok(config_file)
}

pub fn parse_config() -> Result<Config, Box<::std::error::Error + 'static>> {
    let config_str = read_config_file()?;
    Ok(toml::from_str(x).unwrap()),
}
