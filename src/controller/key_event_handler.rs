use crate::controller::event::Event;
use std::io::stdin;
use std::sync::mpsc::SyncSender;
use termion::input::TermRead;

pub struct KeyEventHandler {}

impl KeyEventHandler {
    pub fn handle(sync_sender: SyncSender<Event>) {
        let stdin = stdin();

        for key_result in stdin.keys() {
            if let Ok(key) = key_result {
                let _ = sync_sender.send(Event::Key(key));
            }
        }
    }
}
