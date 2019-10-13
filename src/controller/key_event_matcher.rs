use crate::controller::EventQueue;
use crate::model::path_node::PathNode;
use crate::model::tree_index::TreeIndex;
use std::io::Write;
use termion::event::Key;

impl<W: Write> EventQueue<W> {
    // TODO: return an action function
    pub fn match_key_event(&mut self, key: Key) -> Option<()> {
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

    fn perform_file_action(&self, file_path: &str) {
        let file_action_replaced = self.config.behavior.file_action.replace("%s", file_path);

        std::process::Command::new("bash")
            .arg("-c")
            .arg(file_action_replaced)
            .spawn()
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::config::Config;
    use crate::view::composer::Composer;
    use crate::view::Pager;

    fn prepare_event_queue() -> EventQueue<Vec<u8>> {
        let config = Config::default();

        let composer = Composer::new(config.clone());
        let pager = Pager::new(config.clone(), Vec::new());
        let path_node = PathNode::new(&config.setup.working_dir);

        EventQueue::new(config, composer, pager, path_node)
    }

    #[test]
    fn match_key_event_default_test() {
        let result = {
            let mut event_queue = prepare_event_queue();
            event_queue.match_key_event(Key::__IsNotComplete)
        };

        assert!(result.is_some());
    }

    #[test]
    fn match_key_event_q_test() {
        let result = {
            let mut event_queue = prepare_event_queue();
            event_queue.match_key_event(Key::Char('q'))
        };

        assert!(result.is_none());
    }

    #[test]
    fn match_key_event_r_test() {
        let result = {
            let mut event_queue = prepare_event_queue();
            event_queue.match_key_event(Key::Char('r'))
        };

        assert!(result.is_some());
    }

    #[test]
    fn match_key_event_return_test() {
        let result = {
            let mut event_queue = prepare_event_queue();
            event_queue.match_key_event(Key::Char('\u{0A}'))
        };

        assert!(result.is_some());
    }

    #[test]
    fn match_key_event_up_test() {
        let result = {
            let mut event_queue = prepare_event_queue();
            event_queue.match_key_event(Key::Up)
        };

        assert!(result.is_some());
    }

    #[test]
    fn match_key_event_down_test() {
        let result = {
            let mut event_queue = prepare_event_queue();
            event_queue.match_key_event(Key::Down)
        };

        assert!(result.is_some());
    }

    #[test]
    fn match_key_event_left_test() {
        let result = {
            let mut event_queue = prepare_event_queue();
            event_queue.match_key_event(Key::Left)
        };

        assert!(result.is_some());
    }

    #[test]
    fn match_key_event_right_test() {
        let result = {
            let mut event_queue = prepare_event_queue();
            event_queue.match_key_event(Key::Right)
        };

        assert!(result.is_some());
    }
}
