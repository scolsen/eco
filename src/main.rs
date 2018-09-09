extern crate cite;

use cite::read_config_file;

fn main() {
    let r = read_config_file();
    match r {
        None => println!("No Config file."),
        Some(x) => println!("File contains: {}", x),
    }
}
