extern crate eco;

use eco::config::reader::parse_config;

fn main() {
    match parse_config() {
        Err(_e) => println!("No Config file."),
        Some(x) => println!("File contains: {}", x),
    }
}
