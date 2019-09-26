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

    let mut text_entries = path_node.prettify();

    let mut pager = Pager::new(config.clone());

    pager.update(0, &text_entries, path_node.get_path());

    let mut ch = getch();
    while ch != 113 {
        match ch {
            KEY_UP => {
                clear();
                pager.update(-1, &text_entries, path_node.get_path());
            }
            KEY_DOWN => {
                clear();
                pager.update(1, &text_entries, path_node.get_path());
            }
            KEY_RIGHT => {
                let tree_index = path_node.flat_index_to_tree_index(pager.cursor_row as usize);
                path_node.expand_dir(&tree_index);
                text_entries = path_node.prettify();

                clear();
                pager.update(0, &text_entries, path_node.get_path());
            }
            KEY_LEFT => {
                let tree_index = path_node.flat_index_to_tree_index(pager.cursor_row as usize);
                path_node.reduce_dir(&tree_index);
                text_entries = path_node.prettify();

                clear();
                pager.update(0, &text_entries, path_node.get_path());
            }
            _ => {}
        }
        ch = getch();
    }

    endwin();
}
