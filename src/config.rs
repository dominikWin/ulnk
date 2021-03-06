extern crate config;

use std::env;
use std::process;

const DEFAULT_CONFIG_FILE: &'static str = "ulnk.toml";

#[derive(Debug)]
pub struct Config {
    pub dbpath: String,    // Database path and port
    pub dbname: String,    // Database name
    pub dbuname: String,   // Database user name
    pub dbupasswd: String, // Database user password
}

impl Config {
    pub fn parse() -> Config {
        let env_config = env::var("CONFIG");
        let config_file: String = if env_config.is_ok() { env_config.unwrap() } else { String::from(DEFAULT_CONFIG_FILE) };

        let mut settings: config::Config = config::Config::default();
        if settings.merge(config::File::with_name(config_file.as_str())).is_err() {
            eprintln!("Config file {} not found, exiting", config_file);
            process::exit(1);
        }

        Config {
            dbpath: settings.get_str("dbpath").unwrap(),
            dbname: settings.get_str("dbname").unwrap(),
            dbuname: settings.get_str("dbuname").unwrap(),
            dbupasswd: settings.get_str("dbupasswd").unwrap(),
        }
    }
}
