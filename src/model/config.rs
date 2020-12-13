use crate::model::config::behavior::Behavior;
use crate::model::config::color::Color;
use crate::model::config::composition::Composition;
use crate::model::config::debug::Debug;
use crate::model::config::keybinding::Keybinding;
use crate::model::config::setup::Setup;
use crate::utils::get_config_dir;
use crate::utils::print_help;
use crate::utils::read_file;
use log::{info, warn};
use serde::Deserialize;
use std::env::args;
use std::process::exit;

mod behavior;
mod color;
mod composition;
mod debug;
mod keybinding;
mod setup;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub behavior: Behavior,

    #[serde(default)]
    pub color: Color,

    #[serde(default)]
    pub composition: Composition,

    #[serde(default)]
    pub debug: Debug,

    #[serde(default)]
    pub keybinding: Keybinding,

    #[serde(default)]
    pub setup: Setup,
}

impl Config {
    pub fn new() -> Self {
        info!("initializing config");

        let config = Self::read_config_file().unwrap_or_default();

        Self::parse_args(config, args().skip(1))
    }

    #[rustfmt::skip]
    fn parse_args<T>(mut config: Self, args: T) -> Self
    where
        T: IntoIterator<Item = String>,
    {
        for arg in args {
            let (key, value) = Self::split_arg(arg);
            match key.as_str() {
                "--behavior.file_action" => config.behavior.file_action = Self::parse_value((key, value)),
                "--behavior.path_node_sort" => config.behavior.path_node_sort = Self::parse_value((key, value)),
                "--behavior.scrolling" => config.behavior.scrolling = Self::parse_value((key, value)),
                "--color.background" => config.color.background = Self::parse_value((key, value)),
                "--color.foreground" => config.color.foreground = Self::parse_value((key, value)),
                "--composition.indent" => config.composition.indent = Self::parse_value((key, value)),
                "--composition.show_indent" => config.composition.show_indent = Self::parse_value((key, value)),
                "--composition.use_utf8" => config.composition.use_utf8 = Self::parse_value((key, value)),
                "--debug.enabled" => config.debug.enabled = Self::parse_value((key, value)),
                "--debug.padding_bot" => config.debug.padding_bot = Self::parse_value((key, value)),
                "--debug.padding_top" => config.debug.padding_top = Self::parse_value((key, value)),
                "--debug.spacing_bot" => config.debug.spacing_bot = Self::parse_value((key, value)),
                "--debug.spacing_top" => config.debug.spacing_top = Self::parse_value((key, value)),
                "--keybinding.collapse_dir" => config.keybinding.collapse_dir = Self::parse_value((key, value)),
                "--keybinding.entry_down" => config.keybinding.entry_down = Self::parse_value((key, value)),
                "--keybinding.entry_up" => config.keybinding.entry_up = Self::parse_value((key, value)),
                "--keybinding.expand_dir" => config.keybinding.expand_dir = Self::parse_value((key, value)),
                "--keybinding.file_action" => config.keybinding.file_action = Self::parse_value((key, value)),
                "--keybinding.quit" => config.keybinding.quit = Self::parse_value((key, value)),
                "--keybinding.reload" => config.keybinding.reload = Self::parse_value((key, value)),
                "--setup.working_dir" => config.setup.working_dir = Self::parse_value((key, value)),

                "--help" | "--version" => print_help(),
                "--" => break,
                _ => {
                    warn!("unknown option {}", key);
                }
            }
        }

        info!("config loaded as:\n{:?}", config);
        config
    }

    fn split_arg(arg: String) -> (String, String) {
        println!("{}", arg);
        if let Some(equal_sign_index) = arg.find('=') {
            let before_split = arg.split_at(equal_sign_index);
            let after_split = arg.split_at(equal_sign_index + 1);
            return (String::from(before_split.0), String::from(after_split.1));
        }
        (arg, String::from(""))
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

    fn read_config_file() -> std::io::Result<Self> {
        let config_dir = get_config_dir()?;

        let config_file = format!("{}/twilight-commander.toml", config_dir);

        let config_file_content = read_file(&config_file)?;

        toml::from_str(&config_file_content).map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "could not read the config file",
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args() {
        let default_config = Config::default();
        let args_vec = vec![
            String::from("--behavior.file_action=file_action_test"),
            String::from("--behavior.path_node_sort=path_node_sort_test"),
            String::from("--behavior.scrolling=scrolling_test"),
            String::from("--color.background=background_test"),
            String::from("--color.foreground=foreground_test"),
            String::from("--debug.enabled=true"),
            String::from("--debug.padding_bot=111"),
            String::from("--debug.padding_top=222"),
            String::from("--debug.spacing_bot=333"),
            String::from("--debug.spacing_top=444"),
            String::from("--setup.working_dir=working_dir_test"),
        ];

        let config = Config::parse_args(default_config, args_vec.into_iter());

        assert_eq!(
            config.behavior.file_action,
            String::from("file_action_test")
        );
        assert_eq!(
            config.behavior.path_node_sort,
            String::from("path_node_sort_test")
        );
        assert_eq!(config.behavior.scrolling, String::from("scrolling_test"));
        assert_eq!(config.color.background, String::from("background_test"));
        assert_eq!(config.color.foreground, String::from("foreground_test"));
        assert_eq!(config.debug.enabled, true);
        assert_eq!(config.debug.padding_bot, 111);
        assert_eq!(config.debug.padding_top, 222);
        assert_eq!(config.debug.spacing_bot, 333);
        assert_eq!(config.debug.spacing_top, 444);
        assert_eq!(config.setup.working_dir, String::from("working_dir_test"));
    }

    #[test]
    fn test_parse_args_with_stopper() {
        let default_config = Config::default();
        let args_vec = vec![
            String::from("--behavior.file_action=file_action_test"),
            String::from("--behavior.path_node_sort=path_node_sort_test"),
            String::from("--behavior.scrolling=scrolling_test"),
            String::from("--color.background=background_test"),
            String::from("--color.foreground=foreground_test"),
            String::from("--"),
            String::from("--debug.enabled=true"),
            String::from("--debug.padding_bot=111"),
            String::from("--debug.padding_top=222"),
            String::from("--debug.spacing_bot=333"),
            String::from("--debug.spacing_top=444"),
            String::from("--setup.working_dir=working_dir_test"),
        ];

        let config = Config::parse_args(default_config, args_vec.into_iter());
        let def_conf = Config::default();

        assert_eq!(
            config.behavior.file_action,
            String::from("file_action_test")
        );
        assert_eq!(
            config.behavior.path_node_sort,
            String::from("path_node_sort_test")
        );
        assert_eq!(config.behavior.scrolling, String::from("scrolling_test"));
        assert_eq!(config.color.background, String::from("background_test"));
        assert_eq!(config.color.foreground, String::from("foreground_test"));
        assert_eq!(config.debug.enabled, def_conf.debug.enabled);
        assert_eq!(config.debug.padding_bot, def_conf.debug.padding_bot);
        assert_eq!(config.debug.padding_top, def_conf.debug.padding_top);
        assert_eq!(config.debug.spacing_bot, def_conf.debug.spacing_bot);
        assert_eq!(config.debug.spacing_top, def_conf.debug.spacing_top);
        assert_eq!(config.setup.working_dir, def_conf.setup.working_dir);
    }

    #[test]
    fn test_parse_args_with_multiple_equals() {
        let default_config = Config::default();
        let args_vec =
            vec![String::from("--behavior.file_action=(x=1; y=2; echo $x$y)")];

        let config = Config::parse_args(default_config, args_vec.into_iter());

        assert_eq!(
            config.behavior.file_action,
            String::from("(x=1; y=2; echo $x$y)")
        );
    }
}
