use crate::model::event::Event;
use std::sync::mpsc::SyncSender;
use std::sync::mpsc::{self, TryRecvError};

pub struct ResizeEventHandler {}

impl ResizeEventHandler {
    pub fn handle(sync_sender: SyncSender<Event>, rx: mpsc::Receiver<()>) {
        let hook_id = unsafe {
            signal_hook::register(signal_hook::SIGWINCH, move || {
                sync_sender.send(Event::Resize).unwrap();
            })
        };
        loop {
            match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    // println!("Terminating.");
                    break;
                }
                Err(TryRecvError::Empty) => {}
            }
        }
        signal_hook::unregister(hook_id.unwrap());
    }
}
