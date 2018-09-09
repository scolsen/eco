extern crate eco;

use eco::read_config_file;

fn main() {
    match read_config_file() {
        Err(e) => println!("No Config file."),
        Ok(x) => println!("File contains: {}", x),
    }
}
