extern crate termion;
extern crate toml;

use config::*;
use pager::*;
use path_tree::path_node::PathNode;
use path_tree::tree_index::TreeIndex;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod config;
mod pager;
mod path_tree;
mod utils;

fn perform_file_action(config: &Config, file_path: &str) {
    let file_action_replaced = config.behavior.file_action.replace("%s", file_path);
    let mut file_action_split = file_action_replaced.split_whitespace();

    let program = file_action_split.next().unwrap();

    if std::process::Command::new(program)
        .args(file_action_split)
        .spawn()
        .is_err()
    {
        println!("failed executing '{}'", config.behavior.file_action);
    }
}

fn main() {
    let config = Config::new();

    let mut path_node = PathNode::from_config(&config);
    path_node.expand_dir(&TreeIndex::new(Vec::new()));

    let mut text_entries = path_node.prettify();

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    print!(
        "{}{}{}",
        termion::cursor::Hide,
        termion::cursor::Goto(1, 1),
        termion::clear::All,
    );
    stdout.flush().unwrap();

    let mut pager = Pager::new(config.clone());

    pager.update(0, &text_entries, path_node.get_absolute_path());
    stdout.flush().unwrap();

    for ch in stdin.keys() {
        match ch.unwrap() {
            Key::Char('q') => {
                break;
            }
            Key::Up => {
                print!("{}", termion::clear::All);
                pager.update(-1, &text_entries, path_node.get_absolute_path());
                stdout.flush().unwrap();
            }
            Key::Down => {
                print!("{}", termion::clear::All);
                pager.update(1, &text_entries, path_node.get_absolute_path());
                stdout.flush().unwrap();
            }
            Key::Right => {
                let tree_index = path_node.flat_index_to_tree_index(pager.cursor_row as usize);
                path_node.expand_dir(&tree_index);
                text_entries = path_node.prettify();

                print!("{}", termion::clear::All);
                pager.update(0, &text_entries, path_node.get_absolute_path());
                stdout.flush().unwrap();
            }
            Key::Left => {
                let tree_index = path_node.flat_index_to_tree_index(pager.cursor_row as usize);
                path_node.reduce_dir(&tree_index);
                text_entries = path_node.prettify();

                print!("{}", termion::clear::All);
                pager.update(0, &text_entries, path_node.get_absolute_path());
                stdout.flush().unwrap();
            }
            Key::Char('\u{0A}') => {
                let tree_index = path_node.flat_index_to_tree_index(pager.cursor_row as usize);

                let child_node = path_node.get_child_path_node(&tree_index);

                if !child_node.is_dir {
                    perform_file_action(&config, &child_node.get_absolute_path());
                }
            }
            _ => {}
        }
    }

    print!(
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Show,
    );
}
