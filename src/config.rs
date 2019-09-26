use crate::config::behavior::Behavior;
use crate::config::color::Color;
use crate::config::debug::Debug;
use serde::Deserialize;
use std::env::args;
use std::fs::File;
use std::io::Read;
use toml;

mod behavior;
mod color;
mod debug;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    #[serde(default = "Behavior::default")]
    pub behavior: Behavior,

    #[serde(default = "Color::default")]
    pub color: Color,

    #[serde(default = "Debug::default")]
    pub debug: Debug,
}

impl Config {
    fn default_config() -> Self {
        Self {
            behavior: Behavior::default(),
            color: Color::default(),
            debug: Debug::default(),
        }
    }

    pub fn new() -> Self {
        println!("{:?}", std::env::var("HOME").ok());

        if let Some(config) = read_config_file_from_args() {
            return config;
        }

        if let Some(config) = read_config_file_from_home() {
            return config;
        }

        Self::default_config()
    }
}

fn read_file(file_name: &str) -> std::io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn get_first_arg() -> Option<String> {
    for (c, arg) in args().enumerate() {
        if c == 1 {
            return Some(arg);
        }
    }

    None
}

fn read_config_file_from_args() -> Option<Config> {
    if let Some(first_arg) = get_first_arg() {
        if let Ok(config_file) = read_file(&first_arg) {
            return toml::from_str(&config_file).ok();
        }
    }

    None
}

fn read_config_file_from_home() -> Option<Config> {
    if let Ok(home_dir) = std::env::var("HOME") {
        let home_config_path = format!("{}{}", home_dir, ".twilight-commander-rc.toml");
        if let Ok(config_file) = read_file(&home_config_path) {
            return toml::from_str(&config_file).ok();
        }
    }

    None
}
