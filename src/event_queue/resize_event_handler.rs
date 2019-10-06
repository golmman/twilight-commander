use crate::event_queue::event::Event;
use std::sync::mpsc::SyncSender;

pub struct ResizeEventHandler {}

impl ResizeEventHandler {
    pub fn handle(sync_sender: SyncSender<Event>) {
        let _ = unsafe {
            signal_hook::register(signal_hook::SIGWINCH, move || {
                sync_sender.send(Event::Resize).unwrap();
            })
        };
    }
}
