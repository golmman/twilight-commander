extern crate termion;
extern crate toml;

use model::config::Config;
use controller::EventQueue;

mod controller;
mod view;
mod model;
mod utils;

fn main() {
    let config = Config::new();
    let mut event_queue = EventQueue::new(config);

    event_queue.handle_messages();
}
