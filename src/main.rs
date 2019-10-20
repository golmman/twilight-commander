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

    let composer = Composer::new(config.clone());
    let pager = Pager::new(config.clone(), stdout().into_raw_mode().unwrap());
    let path_node = PathNode::new(&config.setup.working_dir);

    let mut event_queue = EventQueue::new(config, composer, pager, path_node);
    event_queue.handle_messages();
}
