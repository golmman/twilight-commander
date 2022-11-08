use crate::model::event::Event;
use std::sync::mpsc::SyncSender;
use std::sync::mpsc;

pub struct ResizeEventHandler {}

impl ResizeEventHandler {
    pub fn handle(sync_sender: SyncSender<Event>, rx: mpsc::Receiver<()>) {
        let hook_id = unsafe {
            signal_hook::register(signal_hook::SIGWINCH, move || {
                sync_sender.send(Event::Resize).unwrap();
            })
        };
        let _ = rx.recv();
        signal_hook::unregister(hook_id.unwrap());
    }
}
