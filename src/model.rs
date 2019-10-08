pub mod config;
pub mod path_node;
pub mod tree_index;

#[cfg(test)]
mod tests {
    use crate::model::path_node::PathNode;
    use crate::model::tree_index::TreeIndex;
    use crate::model::config::Config;

    #[test]
    fn test_integration_with_path_node_sort_dirs_top_simple() {
        let mut config = Config::new();
        config.setup.working_dir = String::from("./tests/test_dirs");
        config.behavior.path_node_sort = String::from("dirs_top_simple");

        let mut p = PathNode::from_config(&config);
        assert_eq!(0, p.prettify(&config).len());

        // expand_dir
        p.expand_dir(&TreeIndex::new(Vec::new()));
        assert_eq!(13, p.prettify(&config).len(), "expanding the root directory");

        p.expand_dir(&TreeIndex::new(vec![3]));
        assert_eq!(13, p.prettify(&config).len(), "expanding a file does nothing");

        p.expand_dir(&TreeIndex::new(vec![1]));
        assert_eq!(17, p.prettify(&config).len());

        p.expand_dir(&TreeIndex::new(vec![1, 0]));
        assert_eq!(23, p.prettify(&config).len());

        p.expand_dir(&TreeIndex::new(vec![1, 0, 2]));
        assert_eq!(26, p.prettify(&config).len());

        p.expand_dir(&TreeIndex::new(vec![1, 0, 2, 1]));
        assert_eq!(29, p.prettify(&config).len());

        // tree_index_to_flat_index
        let flat_index = TreeIndex::new(vec![7, 2, 4, 0, 0]).tree_index_to_flat_index();
        assert_eq!(17, flat_index);

        // flat_index_to_tree_index
        let tree_index = p.flat_index_to_tree_index(9);
        assert_eq!(vec![1, 0, 2, 1, 1], tree_index.index);

        let tree_index = p.flat_index_to_tree_index(10);
        assert_eq!(vec![1, 0, 2, 1, 2], tree_index.index);

        let tree_index = p.flat_index_to_tree_index(11);
        assert_eq!(vec![1, 0, 2, 2], tree_index.index);

        let tree_index = p.flat_index_to_tree_index(15);
        assert_eq!(vec![1, 1], tree_index.index);

        // reduce_dir
        p.reduce_dir(&TreeIndex::new(vec![1, 0, 2, 1]));
        assert_eq!(26, p.prettify(&config).len(), "reducing the last opened dir");

        p.reduce_dir(&TreeIndex::new(vec![1, 0]));
        assert_eq!(17, p.prettify(&config).len(), "reducing lots of sub dirs");
    }

    #[test]
    fn test_integration_with_path_node_sort_dirs_bot_simple() {
        let mut config = Config::new();
        config.setup.working_dir = String::from("./tests/test_dirs");
        config.behavior.path_node_sort = String::from("dirs_bot_simple");

        let mut p = PathNode::from_config(&config);
        assert_eq!(0, p.prettify(&config).len());

        // expand_dir
        p.expand_dir(&TreeIndex::new(Vec::new()));
        assert_eq!(13, p.prettify(&config).len(), "expanding the root directory");

        p.expand_dir(&TreeIndex::new(vec![3]));
        assert_eq!(13, p.prettify(&config).len(), "expanding a file does nothing");

        p.expand_dir(&TreeIndex::new(vec![11]));
        assert_eq!(17, p.prettify(&config).len());

        p.expand_dir(&TreeIndex::new(vec![11, 3]));
        assert_eq!(23, p.prettify(&config).len());

        p.expand_dir(&TreeIndex::new(vec![11, 3, 3]));
        assert_eq!(26, p.prettify(&config).len());

        p.expand_dir(&TreeIndex::new(vec![11, 3, 3, 1]));
        assert_eq!(29, p.prettify(&config).len());

        // reduce_dir
        p.reduce_dir(&TreeIndex::new(vec![11, 3, 3, 1]));
        assert_eq!(26, p.prettify(&config).len(), "reducing the last opened dir");

        p.reduce_dir(&TreeIndex::new(vec![11, 3]));
        assert_eq!(17, p.prettify(&config).len(), "reducing lots of sub dirs");
    }
}
