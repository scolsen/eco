//library functions.
#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::path::Path;
use toml::Value;
use std::fs::File;

/// Read a TOML configuration file.
/// Return a Configuration struct used for the duration of the program.

#[derive(Deserialize)]
pub struct Config {
    library: String
}

pub fn read_config_file() -> Result<String, Error> {
    let mut config_file = File::open("library.toml")?;
    
    let mut contents = String::new();
    config_file.read_to_string(&mut contents)?;
    
    if contents.is_empty() {
        None     
    } else {
        Some(contents)     
    }
}

