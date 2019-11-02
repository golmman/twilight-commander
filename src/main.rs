extern crate termion;
extern crate toml;
use controller::EventQueue;
use model::config::Config;
use model::path_node::PathNode;
use std::io::stdout;
use termion::raw::IntoRawMode;
use view::composer::Composer;
use view::Pager;

mod controller;
mod model;
mod utils;
mod view;

fn main() {
    let config = Config::new();

    let composer = Composer::from(config.clone());
    let pager = Pager::new(config.clone(), stdout().into_raw_mode().unwrap());
    let path_node_root = PathNode::new_expanded(config.clone());

    let mut event_queue =
        EventQueue::new(config, composer, pager, path_node_root);
    event_queue.handle_messages();
}
