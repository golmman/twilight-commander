extern crate ncurses;
extern crate toml;

use config::*;
use ncurses::*;
use pager::*;
use path_tree::path_node::PathNode;
use path_tree::tree_index::TreeIndex;

mod config;
mod pager;
mod path_tree;

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

    let mut path_node = PathNode::new(&config.setup.working_dir);
    path_node.expand_dir(&TreeIndex::new(Vec::new()));

    let mut text_entries = path_node.prettify();

    let mut pager = Pager::new(config.clone());

    pager.update(0, &text_entries, path_node.get_absolute_path());

    let mut ch = getch();
    while ch != 113 {
        match ch {
            KEY_UP => {
                erase();
                pager.update(-1, &text_entries, path_node.get_absolute_path());
            }
            KEY_DOWN => {
                erase();
                pager.update(1, &text_entries, path_node.get_absolute_path());
            }
            KEY_RIGHT => {
                let tree_index = path_node.flat_index_to_tree_index(pager.cursor_row as usize);
                path_node.expand_dir(&tree_index);
                text_entries = path_node.prettify();

                erase();
                pager.update(0, &text_entries, path_node.get_absolute_path());
            }
            KEY_LEFT => {
                let tree_index = path_node.flat_index_to_tree_index(pager.cursor_row as usize);
                path_node.reduce_dir(&tree_index);
                text_entries = path_node.prettify();

                erase();
                pager.update(0, &text_entries, path_node.get_absolute_path());
            }
            10 => {
                // 
                let tree_index = path_node.flat_index_to_tree_index(pager.cursor_row as usize);

                let child_node = path_node.get_child_path_node(&tree_index);

                if !child_node.is_dir {
                    perform_file_action(&config, &child_node.get_absolute_path());
                }
            }
            _ => {}
        }
        refresh();

        ch = getch();
    }

    endwin();
}
