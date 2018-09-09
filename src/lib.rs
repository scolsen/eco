//library functions.
#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::path::Path;
use toml::Value;
use std::fs;

/// Read a TOML configuration file.
/// Return a Configuration struct used for the duration of the program.

#[derive(Deserialize)]
pub struct Config {
    library: String
}

pub fn read_config_file() -> Result<String, Box<std::error::Error + 'static>> {
    let config_file = fs::read_to_string("library.toml")?;
    Ok(config_file)
}

