use crate::config::behavior::Behavior;
use crate::config::color::Color;
use crate::config::debug::Debug;
use crate::config::setup::Setup;
use serde::Deserialize;
use std::env::args;
use std::fs::File;
use std::io::Read;
use std::process::exit;
use toml;

mod args_parser;
mod behavior;
mod color;
mod debug;
mod setup;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    #[serde(default = "Behavior::default")]
    pub behavior: Behavior,

    #[serde(default = "Color::default")]
    pub color: Color,

    #[serde(default = "Debug::default")]
    pub debug: Debug,

    #[serde(default = "Setup::default")]
    pub setup: Setup,
}

impl Config {
    fn default() -> Self {
        Self {
            behavior: Behavior::default(),
            color: Color::default(),
            debug: Debug::default(),
            setup: Setup::default(),
        }
    }

    fn read_config_file_from_home() -> Option<Self> {
        if let Ok(home_dir) = std::env::var("HOME") {
            let home_config_path = format!("{}/{}", home_dir, ".twilight-commander-rc.toml");
            if let Ok(config_file) = read_file(&home_config_path) {
                return toml::from_str(&config_file).ok();
            }
        }

        None
    }

    // TODO: tests
    fn parse_args<T>(mut config: Self, args: T) -> Self
    where
        T: IntoIterator<Item = String>,
    {
        for arg in args {
            let (key, value) = split_arg(arg);

            match key.as_str() {
                "--behavior.file_action" => config.behavior.file_action = parse_value((key, value)),
                "--behavior.scrolling" => config.behavior.scrolling = parse_value((key, value)),
                "--color.background" => config.color.background = parse_value((key, value)),
                "--color.foreground" => config.color.foreground = parse_value((key, value)),
                "--debug.enabled" => config.debug.enabled = parse_value((key, value)),
                "--debug.padding_bot" => config.debug.padding_bot = parse_value((key, value)),
                "--debug.padding_top" => config.debug.padding_top = parse_value((key, value)),
                "--debug.spacing_bot" => config.debug.spacing_bot = parse_value((key, value)),
                "--debug.spacing_top" => config.debug.spacing_top = parse_value((key, value)),
                "--setup.working_dir" => config.setup.working_dir = parse_value((key, value)),

                "--help" => print_help(),
                "--" => break,
                _ => {
                    println!("unknown option {}", key);
                    exit(1);
                }
            }
        }
        config
    }

    pub fn new() -> Self {
        let config = Self::read_config_file_from_home().unwrap_or_else(Self::default);

        Self::parse_args(config, args().skip(1))
    }
}

fn print_help() {
    println!(
        r#"twilight-commander 0.5.0
usage: twilight-commander [--key1=value1 --key2=value2 ...]"#
    );
    exit(0);
}

fn split_arg(arg: String) -> (String, String) {
    let split_arg: Vec<&str> = arg.split('=').collect();

    if split_arg.len() == 1 {
        return (String::from(split_arg[0]), String::from(""));
    }

    (String::from(split_arg[0]), String::from(split_arg[1]))
}

fn parse_value<F>((key, value): (String, String)) -> F
where
    F: std::str::FromStr,
{
    value.parse().unwrap_or_else(|_| {
        println!("option '{}={}' was not parsable", key, value);
        exit(1);
    })
}

fn read_file(file_name: &str) -> std::io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
