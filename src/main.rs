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

fn main() {
    let config = Config::new();

    let mut path_node = PathNode::new("./tests/test_dirs");
    path_node.expand_dir(&TreeIndex::new(Vec::new()));

    let mut test_entries = path_node.prettify();
    let mut text_row = 0;
    let mut cursor_row = 0;

    let pager = Pager::new(config.clone());

    text_row = pager.update(text_row, cursor_row, &test_entries, path_node.get_path());

    let mut ch = getch();
    while ch != 113 {
        match ch {
            KEY_UP => {
                cursor_row -= 1;
                if cursor_row < 0 {
                    cursor_row = test_entries.len() as i32 - 1;
                }

                clear();
                text_row = pager.update(text_row, cursor_row, &test_entries, path_node.get_path());
            }
            KEY_DOWN => {
                cursor_row += 1;
                if cursor_row >= test_entries.len() as i32 {
                    cursor_row = 0;
                }

                clear();
                text_row = pager.update(text_row, cursor_row, &test_entries, path_node.get_path());
            }
            KEY_RIGHT => {
                let tree_index = path_node.flat_index_to_tree_index(cursor_row as usize);
                path_node.expand_dir(&tree_index);
                test_entries = path_node.prettify();

                clear();
                text_row = pager.update(text_row, cursor_row, &test_entries, path_node.get_path());
            }
            KEY_LEFT => {
                let tree_index = path_node.flat_index_to_tree_index(cursor_row as usize);
                path_node.reduce_dir(&tree_index);
                test_entries = path_node.prettify();

                clear();
                text_row = pager.update(text_row, cursor_row, &test_entries, path_node.get_path());
            }
            _ => {}
        }
        ch = getch();
    }

    endwin();
}
