use crate::model::event::Event;
use crate::model::event::Key;
use std::io::stdin;
use std::sync::mpsc::SyncSender;
use termion::input::TermRead;

pub struct KeyEventHandler {}

impl KeyEventHandler {
    pub fn handle(sync_sender: SyncSender<Event>) {
        let stdin = stdin();

        for termion_event in stdin.events() {
            if let Ok(termion_event) = termion_event {
                let _ = sync_sender.send(Event::Key(Key::from(termion_event)));
            }
        }
    }
}
