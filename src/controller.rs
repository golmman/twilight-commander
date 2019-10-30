use crate::controller::key_event_handler::KeyEventHandler;
use crate::controller::resize_event_handler::ResizeEventHandler;
use crate::model::compare_functions::PathNodeCompare;
use crate::model::config::Config;
use crate::model::event::Event;
use crate::model::path_node::PathNode;
use crate::model::tree_index::TreeIndex;
use crate::view::composer::Composer;
use crate::view::Pager;
use std::cmp::Ordering;
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
    // TODO: better name: root_path_node?
    path_node: PathNode,
    path_node_compare: PathNodeCompare,
    queue_receiver: Receiver<Event>,
    queue_sender: SyncSender<Event>,

    // TODO: should be part of the view?
    text_entries: Vec<String>,
}

impl<W: Write> EventQueue<W> {
    pub fn new(config: Config, composer: Composer, mut pager: Pager<W>, mut path_node: PathNode) -> Self {
        let (queue_sender, queue_receiver): (SyncSender<Event>, Receiver<Event>) = sync_channel(1024);

        // TODO: PathNode should have a constructor with an expanded root
        let path_node_compare = Self::get_path_node_compare(&config);
        path_node.expand_dir(&TreeIndex::from(Vec::new()), path_node_compare);

        let text_entries = composer.compose_path_node(&path_node);
        pager.update(0, &text_entries, path_node.get_absolute_path());

        Self {
            config,
            composer,
            pager,
            path_node,
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

        while let Some(_) = self.match_event(self.queue_receiver.recv().unwrap()) {}
        // TODO: add a channel to shut down the threads?
    }

    fn match_event(&mut self, event: Event) -> Option<()> {
        match event {
            Event::Key(key) => self.match_key_event(key),
            Event::Resize => {
                self.pager
                    .update(0, &self.text_entries, self.path_node.get_absolute_path());
                Some(())
            }
        }
    }

    pub fn get_path_node_compare(config: &Config) -> PathNodeCompare {
        let path_node_compare: fn(&PathNode, &PathNode) -> Ordering = match config.behavior.path_node_sort.as_str() {
            "dirs_bot_simple" => PathNode::compare_dirs_bot_simple,
            "dirs_top_simple" => PathNode::compare_dirs_top_simple,
            "none" => |_, _| Ordering::Equal,
            _ => |_, _| Ordering::Equal,
        };

        path_node_compare
    }
}
