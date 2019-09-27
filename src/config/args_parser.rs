// use std::env::Args;
// use crate::config::Config;
// use std::env::args;
// use std::process::exit;

// fn split_arg(arg: String) -> (String, String) {
//     let split_arg: Vec<&str> = arg.split('=').collect();

//     if split_arg.len() == 1 {
//         return (String::from(split_arg[0]), String::from(""));
//     }

//     (String::from(split_arg[0]), String::from(split_arg[1]))
// }

// fn parse_value<F>((key, value): (String, String)) -> F
// where
//     F: std::str::FromStr,
// {
//     value.parse().unwrap_or_else(|_| {
//         println!("option '{}={}' was not parsable", key, value);
//         exit(1);
//     })
// }

// impl Config {
//     fn parse_args(mut config: Config, args: Vec<String>) -> Config {
//         let mut args_iterator = args.into_iter();
//         args_iterator.next();

//         for arg in args_iterator {
//             let (key, value) = split_arg(arg);

//             match key.as_str() {
//                 "--behavior.scrolling" => config.behavior.scrolling = parse_value((key, value)),
//                 "--color.background" => config.color.background = parse_value((key, value)),
//                 "--color.foreground" => config.color.foreground = parse_value((key, value)),
//                 "--debug.enabled" => config.debug.enabled = parse_value((key, value)),
//                 "--debug.padding_bot" => config.debug.padding_bot = parse_value((key, value)),
//                 "--debug.padding_top" => config.debug.padding_top = parse_value((key, value)),
//                 "--debug.spacing_bot" => config.debug.spacing_bot = parse_value((key, value)),
//                 "--debug.spacing_top" => config.debug.spacing_top = parse_value((key, value)),
//                 "--setup.working_dir" => config.setup.working_dir = parse_value((key, value)),
//                 "--" => break,
//                 _ => {
//                     println!("unknown option {}", key);
//                     exit(1);
//                 }
//             }
//         }
//         config
//     }

//     pub fn new2() -> Self {
//         let default_config = Self::default_config();

//         let config = Self::parse_args(default_config, args().collect());

//         // if let Some(config) = read_config_file_from_args() {
//         //     return config;
//         // }

//         // if let Some(config) = read_config_file_from_home() {
//         //     return config;
//         // }
//         config
//     }
// }
