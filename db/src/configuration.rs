extern crate serde;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Configuration {
    pub username: String,
    pub password: String,
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: i32,
    #[serde(default = "default_db")]
    pub database: String,
}

fn default_username() -> String {
    "username".to_string()
}

fn default_password() -> String {
    "password".to_string()
}

fn default_host() -> String {
    "localhost".to_string()
}

fn default_port() -> i32 {
    5432
}

fn default_db() -> String {
    "scheduler".to_string()
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            username: default_username(),
            password: default_password(),
            host: default_host(),
            port: default_port(),
            database: default_db(),
        }
    }
}
