use crate::controller::event::Event;
use crate::controller::key_event_handler::KeyEventHandler;
use crate::controller::resize_event_handler::ResizeEventHandler;
use crate::model::compare_functions::PathNodeCompare;
use crate::model::config::Config;
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
use termion::event::Key;

mod event;
mod key_event_handler;
mod resize_event_handler;

pub struct EventQueue<W: Write> {
    config: Config,
    composer: Composer,
    pager: Pager<W>,
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

        let path_node_compare = Self::get_path_node_compare(&config);
        path_node.expand_dir(&TreeIndex::new(Vec::new()), path_node_compare);

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

    fn match_key_event(&mut self, key: Key) -> Option<()> {
        match key {
            Key::Char('q') => None,
            Key::Up => {
                self.pager
                    .update(-1, &self.text_entries, self.path_node.get_absolute_path());
                Some(())
            }
            Key::Down => {
                self.pager
                    .update(1, &self.text_entries, self.path_node.get_absolute_path());
                Some(())
            }
            Key::Right => {
                let tree_index = self.path_node.flat_index_to_tree_index(self.pager.cursor_row as usize);
                self.path_node.expand_dir(&tree_index, self.path_node_compare);
                self.text_entries = self.composer.compose_path_node(&self.path_node);

                self.pager
                    .update(0, &self.text_entries, self.path_node.get_absolute_path());
                Some(())
            }
            Key::Left => {
                let tree_index = self.path_node.flat_index_to_tree_index(self.pager.cursor_row as usize);
                self.path_node.reduce_dir(&tree_index);
                self.text_entries = self.composer.compose_path_node(&self.path_node);

                self.pager
                    .update(0, &self.text_entries, self.path_node.get_absolute_path());
                Some(())
            }
            Key::Char('\u{0A}') => {
                let tree_index = self.path_node.flat_index_to_tree_index(self.pager.cursor_row as usize);

                let child_node = self.path_node.get_child_path_node(&tree_index);

                if !child_node.is_dir {
                    self.perform_file_action(&child_node.get_absolute_path());
                }
                Some(())
            }
            Key::Char('r') => {
                // TODO: this simply resets the tree, implement a recursive method
                self.path_node = PathNode::new(&self.config.setup.working_dir);
                self.path_node
                    .expand_dir(&TreeIndex::new(Vec::new()), self.path_node_compare);
                self.text_entries = self.composer.compose_path_node(&self.path_node);

                self.pager
                    .update(0, &self.text_entries, self.path_node.get_absolute_path());
                Some(())
            }
            _ => Some(()),
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

    fn perform_file_action(&self, file_path: &str) {
        let file_action_replaced = self.config.behavior.file_action.replace("%s", file_path);

        std::process::Command::new("bash")
            .arg("-c")
            .arg(file_action_replaced)
            .spawn()
            .unwrap();
    }
}
