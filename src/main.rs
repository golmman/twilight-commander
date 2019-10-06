extern crate termion;
extern crate toml;

use config::Config;
use event_queue::EventQueue;

mod config;
mod event_queue;
mod pager;
mod path_tree;
mod utils;

fn main() {
    let config = Config::new();
    let mut event_queue = EventQueue::new(config);

    event_queue.handle_messages();
}
