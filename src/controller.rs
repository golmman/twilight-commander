use crate::controller::key_event_handler::KeyEventHandler;
use crate::controller::resize_event_handler::ResizeEventHandler;
use crate::model::compare_functions::PathNodeCompare;
use crate::model::config::Config;
use crate::model::event::Event;
use crate::model::path_node::PathNode;
use crate::view::composer::Composer;
use crate::view::Pager;
use std::io::Write;
use std::sync::mpsc::sync_channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::SyncSender;
use std::thread;

mod key_event_handler;
mod key_event_matcher;
mod resize_event_handler;

pub struct EventQueue<W: Write> {
    config: Config,
    composer: Composer,
    pager: Pager<W>,
    path_node_root: PathNode,
    path_node_compare: PathNodeCompare,
    queue_receiver: Receiver<Event>,
    queue_sender: SyncSender<Event>,

    // TODO: should be part of the view?
    text_entries: Vec<String>,
}

impl<W: Write> EventQueue<W> {
    pub fn new(
        config: Config,
        composer: Composer,
        mut pager: Pager<W>,
        path_node_root: PathNode,
    ) -> Self {
        let (queue_sender, queue_receiver): (
            SyncSender<Event>,
            Receiver<Event>,
        ) = sync_channel(1024);

        let path_node_compare = PathNode::get_path_node_compare(&config);

        let text_entries = composer.compose_path_node(&path_node_root);
        pager.update(0, &text_entries, path_node_root.get_absolute_path());

        Self {
            config,
            composer,
            pager,
            path_node_root,
            path_node_compare,
            queue_receiver,
            queue_sender,
            text_entries,
        }
    }

    pub fn handle_messages(&mut self) {
        let sender1 = self.queue_sender.clone();
        let sender2 = self.queue_sender.clone();
        thread::spawn(move || KeyEventHandler::handle(sender1));
        thread::spawn(move || ResizeEventHandler::handle(sender2));

        while let Some(_) =
            self.match_event(self.queue_receiver.recv().unwrap())
        {}
        // TODO: add a channel to shut down the threads?
    }

    fn match_event(&mut self, event: Event) -> Option<()> {
        match event {
            Event::Key(key) => self.match_key_event(key),
            Event::Resize => {
                self.pager.update(
                    0,
                    &self.text_entries,
                    self.path_node_root.get_absolute_path(),
                );
                Some(())
            }
        }
    }
}
