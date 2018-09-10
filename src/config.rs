/// Configuration Functions 

#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::path::Path;
use toml::Value;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    library: String
}

/// Read a toml configuration file.
pub fn read_config_file() -> Result<String, Box<std::error::Error + 'static>> {
    let config_file = fs::read_to_string("library.toml")?;
    Ok(config_file)
}

/// Set configuration options.
fn config() -> {
    
}
