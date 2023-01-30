extern crate serde;
use fluid_db::configuration::Configuration;
use std::fs::File;
use std::io::Read;

pub fn read(filename: &str) -> Configuration {
    match File::open(filename) {
        Ok(mut file) => {
            let mut buffer = String::new();
            file.read_to_string(&mut buffer).unwrap();
            let config: Configuration = serde_json::from_str(&buffer).unwrap();
            config
        }
        _ => Configuration::default(),
    }
}
