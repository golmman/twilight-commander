use crate::controller::EventQueue;
use crate::model::path_node::PathNode;
use crate::model::tree_index::TreeIndex;
use std::io::Write;

impl<W: Write> EventQueue<W> {
    pub fn do_reload(&mut self) -> Option<()> {
        // TODO: this simply resets the tree, implement a recursive method
        self.path_node_root =
            PathNode::from(self.config.setup.working_dir.clone());
        self.path_node_root
            .expand_dir(&TreeIndex::from(Vec::new()), self.path_node_compare);
        self.text_entries =
            self.composer.compose_path_node(&self.path_node_root);

        self.update_pager(0);
        Some(())
    }
}
