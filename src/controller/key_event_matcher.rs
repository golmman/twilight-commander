use crate::controller::EventQueue;
use crate::model::path_node::PathNode;
use crate::model::tree_index::TreeIndex;
use std::io::Write;
use termion::event::Key;

impl<W: Write> EventQueue<W> {
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
