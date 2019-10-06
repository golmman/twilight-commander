use std::sync::mpsc::sync_channel;
use crate::config::Config;
use crate::event_queue::event::Event;
use crate::event_queue::key_event_handler::KeyEventHandler;
use crate::event_queue::resize_event_handler::ResizeEventHandler;
use crate::pager::Pager;
use crate::path_tree::path_node::PathNode;
use crate::path_tree::tree_index::TreeIndex;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::SyncSender;
use std::thread;
use termion::event::Key;

mod event;
mod key_event_handler;
mod resize_event_handler;

pub struct EventQueue {
    config: Config,
    pager: Pager,
    path_node: PathNode,
    queue_receiver: Receiver<Event>,
    queue_sender: SyncSender<Event>,
    text_entries: Vec<String>,
}

impl EventQueue {
    pub fn new(config: Config) -> Self {
        let (queue_sender, queue_receiver): (SyncSender<Event>, Receiver<Event>) = sync_channel(1024);

        let mut path_node = PathNode::from_config(&config);
        path_node.expand_dir(&TreeIndex::new(Vec::new()));
        let text_entries = path_node.prettify();

        let mut pager = Pager::new(config.clone());
        pager.update(0, &text_entries, path_node.get_absolute_path());

        Self {
            config,
            pager,
            path_node,
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

        print!(
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Show,
        );
    }

    fn match_event(&mut self, event: Event) -> Option<()> {
        match event {
            Event::Key(key) => self.match_key_event(key),
            Event::Resize => {
                print!("{}", termion::clear::All);
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
                print!("{}", termion::clear::All);
                self.pager
                    .update(-1, &self.text_entries, self.path_node.get_absolute_path());
                Some(())
            }
            Key::Down => {
                print!("{}", termion::clear::All);
                self.pager
                    .update(1, &self.text_entries, self.path_node.get_absolute_path());
                Some(())
            }
            Key::Right => {
                let tree_index = self.path_node.flat_index_to_tree_index(self.pager.cursor_row as usize);
                self.path_node.expand_dir(&tree_index);
                self.text_entries = self.path_node.prettify();

                print!("{}", termion::clear::All);
                self.pager
                    .update(0, &self.text_entries, self.path_node.get_absolute_path());
                Some(())
            }
            Key::Left => {
                let tree_index = self.path_node.flat_index_to_tree_index(self.pager.cursor_row as usize);
                self.path_node.reduce_dir(&tree_index);
                self.text_entries = self.path_node.prettify();

                print!("{}", termion::clear::All);
                self.pager
                    .update(0, &self.text_entries, self.path_node.get_absolute_path());
                Some(())
            }
            Key::Char('\u{0A}') => {
                let tree_index = self.path_node.flat_index_to_tree_index(self.pager.cursor_row as usize);

                let child_node = self.path_node.get_child_path_node(&tree_index);

                if !child_node.is_dir {
                    Self::perform_file_action(&self.config, &child_node.get_absolute_path());
                }
                Some(())
            }
            Key::Char('r') => {
                // TODO: this simply resets the tree, implement a recursive method
                self.path_node = PathNode::from_config(&self.config);
                self.path_node.expand_dir(&TreeIndex::new(Vec::new()));
                self.text_entries = self.path_node.prettify();

                print!("{}", termion::clear::All);
                self.pager
                    .update(0, &self.text_entries, self.path_node.get_absolute_path());
                Some(())
            }
            _ => Some(()),
        }
    }

    fn perform_file_action(config: &Config, file_path: &str) {
        let file_action_replaced = config.behavior.file_action.replace("%s", file_path);
        let mut file_action_split = file_action_replaced.split_whitespace();

        let program = file_action_split.next().unwrap();

        if std::process::Command::new(program)
            .args(file_action_split)
            .spawn()
            .is_err()
        {
            println!("failed executing '{}'", config.behavior.file_action);
        }
    }

    // fn match_key_event(&self, key: Key) -> Option<()> {
    //     match key {
    //         Key::Char('q') => None,
    //         Key::Up => {
    //             print!("{}", termion::clear::All);
    //             pager.update(-1, &text_entries, path_node.get_absolute_path());
    //             stdout.flush().unwrap();
    //             Some(())
    //         }
    //         Key::Down => {
    //             print!("{}", termion::clear::All);
    //             pager.update(1, &text_entries, path_node.get_absolute_path());
    //             stdout.flush().unwrap();
    //             Some(())
    //         }
    //         Key::Right => {
    //             let tree_index = path_node.flat_index_to_tree_index(pager.cursor_row as usize);
    //             path_node.expand_dir(&tree_index);
    //             text_entries = path_node.prettify();

    //             print!("{}", termion::clear::All);
    //             pager.update(0, &text_entries, path_node.get_absolute_path());
    //             stdout.flush().unwrap();
    //             Some(())
    //         }
    //         Key::Left => {
    //             let tree_index = path_node.flat_index_to_tree_index(pager.cursor_row as usize);
    //             path_node.reduce_dir(&tree_index);
    //             text_entries = path_node.prettify();

    //             print!("{}", termion::clear::All);
    //             pager.update(0, &text_entries, path_node.get_absolute_path());
    //             stdout.flush().unwrap();
    //             Some(())
    //         }
    //         Key::Char('\u{0A}') => {
    //             let tree_index = path_node.flat_index_to_tree_index(pager.cursor_row as usize);

    //             let child_node = path_node.get_child_path_node(&tree_index);

    //             if !child_node.is_dir {
    //                 perform_file_action(&config, &child_node.get_absolute_path());
    //             }
    //             Some(())
    //         }
    //         Key::Char('r') => {
    //             // TODO: this simply resets the tree, implement a recursive method
    //             path_node = PathNode::from_config(&config);
    //             path_node.expand_dir(&TreeIndex::new(Vec::new()));
    //             text_entries = path_node.prettify();

    //             print!("{}", termion::clear::All);
    //             pager.update(0, &text_entries, path_node.get_absolute_path());
    //             stdout.flush().unwrap();

    //             Some(())
    //         }
    //         _ => Some(()),
    //     }
    // }
}
