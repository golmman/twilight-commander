use crate::controller::EventQueue;
use crate::model::tree_index::TreeIndex;
use std::io::Write;

impl<W: Write> EventQueue<W> {
    pub fn do_collapse_dir(&mut self) -> Option<()> {
        let tree_index = self.path_node.flat_index_to_tree_index(self.pager.cursor_row as usize);

        let cursor_delta = self.get_parent_dir_cursor_delta(&tree_index);

        if cursor_delta == 0 {
            self.path_node.collapse_dir(&tree_index);
        }

        self.text_entries = self.composer.compose_path_node(&self.path_node);

        self.update_pager(cursor_delta);
        Some(())
    }

    fn get_parent_dir_cursor_delta(&mut self, tree_index: &TreeIndex) -> i32 {
        println!("{:?}", self.path_node);

        let child_path_node = self.path_node.get_child_path_node(tree_index);
        if child_path_node.is_dir && child_path_node.is_expanded {
            return 0;
        }

        let parent_path_node_tree_index = tree_index.get_parent();
        if parent_path_node_tree_index == TreeIndex::new() {
            return 0;
        }

        let parent_flat_index = self.path_node.tree_index_to_flat_index(&parent_path_node_tree_index) as i32;

        parent_flat_index - self.pager.cursor_row
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

    // TODO: duplicate code, create test utils?
    fn get_expanded_path_node() -> PathNode {
        let mut path_node = PathNode::from("./tests/test_dirs");
        path_node.expand_dir(&TreeIndex::new(), PathNode::compare_dirs_top_simple);
        path_node.expand_dir(&TreeIndex::from(vec![0]), PathNode::compare_dirs_top_simple);
        path_node.expand_dir(&TreeIndex::from(vec![0, 0]), PathNode::compare_dirs_top_simple);
        path_node.expand_dir(&TreeIndex::from(vec![1]), PathNode::compare_dirs_top_simple);
        path_node.expand_dir(&TreeIndex::from(vec![1, 0]), PathNode::compare_dirs_top_simple);
        path_node.expand_dir(&TreeIndex::from(vec![1, 0, 2]), PathNode::compare_dirs_top_simple);
        path_node
    }

    fn prepare_event_queue() -> EventQueue<Vec<u8>> {
        let config = Config::default();

        let composer = Composer::new(config.clone());
        let pager = Pager::new(config.clone(), Vec::new());
        let path_node = PathNode::from(config.setup.working_dir.clone());

        let mut event_queue = EventQueue::new(config, composer, pager, path_node);

        event_queue.path_node = get_expanded_path_node();

        event_queue
    }

    mod get_parent_dir_cursor_delta_tests {
        use super::*;

        #[test]
        fn expanded() {
            let mut event_queue = prepare_event_queue();

            let delta = event_queue.get_parent_dir_cursor_delta(&TreeIndex::from(vec![0]));

            assert_eq!(0, delta);
        }

        #[test]
        fn empty_tree_index() {
            let mut event_queue = prepare_event_queue();

            let delta = event_queue.get_parent_dir_cursor_delta(&TreeIndex::new());

            assert_eq!(0, delta);
        }

        #[test]
        fn jump() {
            let mut event_queue = prepare_event_queue();

            let delta = event_queue.get_parent_dir_cursor_delta(&TreeIndex::from(vec![1, 0, 4]));

            assert_eq!(7, delta);
        }
    }
}
