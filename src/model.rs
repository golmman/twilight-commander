pub mod config;
pub mod path_node;
pub mod tree_index;

#[cfg(test)]
mod tests {
    use crate::model::path_node::PathNode;
    use crate::model::tree_index::TreeIndex;
    use crate::model::config::Config;
    use crate::view::Pager;

    #[test]
    fn test_integration_with_path_node_sort_dirs_top_simple() {
        let mut config = Config::new();
        config.setup.working_dir = String::from("./tests/test_dirs");
        config.behavior.path_node_sort = String::from("dirs_top_simple");

        let pager = Pager::new(config.clone());
        let mut path_node = PathNode::from_config(&config);
        assert_eq!(0, pager.compose_path_node(&path_node).len());

        // expand_dir
        path_node.expand_dir(&TreeIndex::new(Vec::new()));
        assert_eq!(13, pager.compose_path_node(&path_node).len(), "expanding the root directory");

        path_node.expand_dir(&TreeIndex::new(vec![3]));
        assert_eq!(13, pager.compose_path_node(&path_node).len(), "expanding a file does nothing");

        path_node.expand_dir(&TreeIndex::new(vec![1]));
        assert_eq!(17, pager.compose_path_node(&path_node).len());

        path_node.expand_dir(&TreeIndex::new(vec![1, 0]));
        assert_eq!(23, pager.compose_path_node(&path_node).len());

        path_node.expand_dir(&TreeIndex::new(vec![1, 0, 2]));
        assert_eq!(26, pager.compose_path_node(&path_node).len());

        path_node.expand_dir(&TreeIndex::new(vec![1, 0, 2, 1]));
        assert_eq!(29, pager.compose_path_node(&path_node).len());

        // tree_index_to_flat_index
        let flat_index = TreeIndex::new(vec![7, 2, 4, 0, 0]).tree_index_to_flat_index();
        assert_eq!(17, flat_index);

        // flat_index_to_tree_index
        let tree_index = path_node.flat_index_to_tree_index(9);
        assert_eq!(vec![1, 0, 2, 1, 1], tree_index.index);

        let tree_index = path_node.flat_index_to_tree_index(10);
        assert_eq!(vec![1, 0, 2, 1, 2], tree_index.index);

        let tree_index = path_node.flat_index_to_tree_index(11);
        assert_eq!(vec![1, 0, 2, 2], tree_index.index);

        let tree_index = path_node.flat_index_to_tree_index(15);
        assert_eq!(vec![1, 1], tree_index.index);

        // reduce_dir
        path_node.reduce_dir(&TreeIndex::new(vec![1, 0, 2, 1]));
        assert_eq!(26, pager.compose_path_node(&path_node).len(), "reducing the last opened dir");

        path_node.reduce_dir(&TreeIndex::new(vec![1, 0]));
        assert_eq!(17, pager.compose_path_node(&path_node).len(), "reducing lots of sub dirs");
    }

    #[test]
    fn test_integration_with_path_node_sort_dirs_bot_simple() {
        let mut config = Config::new();
        config.setup.working_dir = String::from("./tests/test_dirs");
        config.behavior.path_node_sort = String::from("dirs_bot_simple");

        let pager = Pager::new(config.clone());
        let mut path_node = PathNode::from_config(&config);
        assert_eq!(0, pager.compose_path_node(&path_node).len());

        // expand_dir
        path_node.expand_dir(&TreeIndex::new(Vec::new()));
        assert_eq!(13, pager.compose_path_node(&path_node).len(), "expanding the root directory");

        path_node.expand_dir(&TreeIndex::new(vec![3]));
        assert_eq!(13, pager.compose_path_node(&path_node).len(), "expanding a file does nothing");

        path_node.expand_dir(&TreeIndex::new(vec![11]));
        assert_eq!(17, pager.compose_path_node(&path_node).len());

        path_node.expand_dir(&TreeIndex::new(vec![11, 3]));
        assert_eq!(23, pager.compose_path_node(&path_node).len());

        path_node.expand_dir(&TreeIndex::new(vec![11, 3, 3]));
        assert_eq!(26, pager.compose_path_node(&path_node).len());

        path_node.expand_dir(&TreeIndex::new(vec![11, 3, 3, 1]));
        assert_eq!(29, pager.compose_path_node(&path_node).len());

        // reduce_dir
        path_node.reduce_dir(&TreeIndex::new(vec![11, 3, 3, 1]));
        assert_eq!(26, pager.compose_path_node(&path_node).len(), "reducing the last opened dir");

        path_node.reduce_dir(&TreeIndex::new(vec![11, 3]));
        assert_eq!(17, pager.compose_path_node(&path_node).len(), "reducing lots of sub dirs");
    }
}
