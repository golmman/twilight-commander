use crate::controller::EventQueue;
use std::io::Write;

impl<W: Write> EventQueue<W> {
    pub fn do_expand_dir(&mut self) -> Option<()> {
        let tree_index = self.path_node.flat_index_to_tree_index(self.pager.cursor_row as usize);
        self.path_node.expand_dir(&tree_index, self.path_node_compare);
        self.text_entries = self.composer.compose_path_node(&self.path_node);

        self.pager
            .update(0, &self.text_entries, self.path_node.get_absolute_path());
        Some(())
    }
}
