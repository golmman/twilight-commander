use crate::controller::EventQueue;
use crate::model::path_node::PathNode;
use crate::model::tree_index::TreeIndex;
use std::io::Write;

impl<W: Write> EventQueue<W> {
    pub fn do_reload(&mut self) -> Option<()> {
        self.reload_openend_dirs();

        self.text_entries =
            self.composer.compose_path_node(&self.path_node_root);

        self.update_pager(0);

        Some(())
    }

    fn reload_openend_dirs(&mut self) {
        // backup the old path node structure
        let old_path_node_root = self.path_node_root.clone();

        // reset the root path node
        self.path_node_root =
            PathNode::from(self.config.setup.working_dir.clone());
        self.path_node_root
            .expand_dir(&TreeIndex::from(Vec::new()), self.path_node_compare);

        // restore the old path nodes structure for the root path node
        self.restore_expansions(&old_path_node_root, &mut TreeIndex::new());
    }

    fn restore_expansions(
        &mut self,
        path_node: &PathNode,
        tree_index: &mut TreeIndex,
    ) {
        for (c, child) in path_node.children.iter().enumerate() {
            if child.is_expanded {
                tree_index.index.push(c);

                self.path_node_root
                    .expand_dir(tree_index, self.path_node_compare);
                self.restore_expansions(child, tree_index);

                tree_index.index.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::config::Config;
    use crate::model::path_node::PathNode;
    use crate::model::tree_index::TreeIndex;
    use crate::view::composer::Composer;
    use crate::view::Pager;

    fn get_expanded_path_node(working_dir: &str) -> PathNode {
        let mut path_node = PathNode::from(working_dir);

        path_node
            .expand_dir(&TreeIndex::new(), PathNode::compare_dirs_top_simple);
        path_node.expand_dir(
            &TreeIndex::from(vec![0]),
            PathNode::compare_dirs_top_simple,
        );
        path_node.expand_dir(
            &TreeIndex::from(vec![0, 0]),
            PathNode::compare_dirs_top_simple,
        );
        path_node.expand_dir(
            &TreeIndex::from(vec![1]),
            PathNode::compare_dirs_top_simple,
        );
        path_node.expand_dir(
            &TreeIndex::from(vec![1, 0]),
            PathNode::compare_dirs_top_simple,
        );
        path_node.expand_dir(
            &TreeIndex::from(vec![1, 0, 2]),
            PathNode::compare_dirs_top_simple,
        );
        path_node
    }

    fn prepare_event_queue(working_dir: &str) -> EventQueue<Vec<u8>> {
        let mut config = Config::default();
        config.setup.working_dir = String::from(working_dir.clone());

        let composer = Composer::from(config.clone());
        let pager = Pager::new(config.clone(), Vec::new());
        let path_node = PathNode::from(config.setup.working_dir.clone());

        let mut event_queue =
            EventQueue::new(config, composer, pager, path_node);

        event_queue.path_node_root = get_expanded_path_node(working_dir);

        event_queue
    }

    #[test]
    fn do_reload() {
        // TODO: implement proper test
        let mut event_queue = prepare_event_queue("./tests/test_dirs");
        event_queue.do_reload();
    }
}
