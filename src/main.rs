extern crate termion;
extern crate toml;

use controller::EventQueue;
use model::config::Config;

mod controller;
mod model;
mod utils;
mod view;

fn main() {
    let config = Config::new();
    let mut event_queue = EventQueue::new(config);

    event_queue.handle_messages();
}
