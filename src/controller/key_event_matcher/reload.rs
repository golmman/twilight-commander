use crate::controller::EventQueue;
use std::io::Write;
use crate::model::path_node::PathNode;
use crate::model::tree_index::TreeIndex;

impl<W: Write> EventQueue<W> {
    pub fn do_reload(&mut self) -> Option<()> {
        // TODO: this simply resets the tree, implement a recursive method
        self.path_node = PathNode::from(self.config.setup.working_dir.clone());
        self.path_node
            .expand_dir(&TreeIndex::from(Vec::new()), self.path_node_compare);
        self.text_entries = self.composer.compose_path_node(&self.path_node);

        self.pager
            .update(0, &self.text_entries, self.path_node.get_absolute_path());
        Some(())
    }
}
