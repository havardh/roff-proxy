use std::io::prelude::*;
use std::fs::File;

use std::collections::BTreeMap;

use toml::{Parser, Value};

#[derive(Debug)]
pub struct Config {
    pub local_port: u16,
    pub remote_host: String,
    pub remote_port: u16,
    pub probe_path: String,
    pub authorization_header: Option<String>,
    pub probe_interval: u16,
}

impl Config {
    pub fn default() -> Config {
        println!("defaulting");
        Config {
            local_port: 8080,
            remote_host: String::from("localhost"),
            remote_port: 8081,
            probe_path: String::from("/"),
            authorization_header: None,
            probe_interval: 5000,
        }
    }

    pub fn from_toml(name: String) -> Config {

        println!("tomling");

        match File::open(name.as_str()) {
            Ok(mut file) => {
                let mut toml = String::new();
                match file.read_to_string(&mut toml) {
                    Ok(_) => {
                        match Parser::new(toml.as_str()).parse() {
                            Some(value) => {
                                Config {
                                    local_port: lookup_u16(&value, "local_port", 8080),
                                    remote_host: lookup_string(&value, "remote_host", "localhost"),
                                    remote_port: 8081,
                                    probe_path: String::from("/"),
                                    authorization_header: None,
                                    probe_interval: 5000,
                                }
                            }
                            _ => Self::default(),
                        }
                    }
                    _ => Self::default(),
                }
            }
            _ => Self::default(),
        }


        //let value = Parser::new(toml.as_str()).parse().unwrap();

        //println!("{:?}", value);
    }
}

fn lookup_u16(value: &BTreeMap<String, Value>, path: &str, default: u16) -> u16 {
    value.get(path).unwrap_or(&Value::Float(0.0)).as_integer().unwrap_or(default as i64) as u16
}

fn lookup_string(value: &BTreeMap<String, Value>, path: &str, default: &str) -> String {
    String::from(value.get(path).unwrap_or(&Value::Float(0.0)).as_str().unwrap_or(default))
}
