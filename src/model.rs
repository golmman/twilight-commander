pub mod compare_functions;
pub mod config;
pub mod event;
pub mod path_node;
pub mod tree_index;

#[cfg(test)]
mod tests {
    use crate::{
        model::{
            config::Config,
            path_node::PathNode,
            tree_index::TreeIndex,
        },
        view::composer::Composer,
    };

    #[test]
    fn test_integration_with_path_node_sort_dirs_top_simple() {
        let mut config = Config::default();
        config.setup.working_dir = String::from("./tests/test_dirs");

        let composer = Composer::new(config.clone());
        let mut path_node = PathNode::new(&config.setup.working_dir);
        let path_node_compare = PathNode::compare_dirs_top_simple;
        assert_eq!(0, composer.compose_path_node(&path_node).len());

        // expand_dir
        path_node.expand_dir(&TreeIndex::new(Vec::new()), path_node_compare);
        assert_eq!(
            13,
            composer.compose_path_node(&path_node).len(),
            "expanding the root directory"
        );

        path_node.expand_dir(&TreeIndex::new(vec![3]), path_node_compare);
        assert_eq!(
            13,
            composer.compose_path_node(&path_node).len(),
            "expanding a file does nothing"
        );

        path_node.expand_dir(&TreeIndex::new(vec![1]), path_node_compare);
        assert_eq!(17, composer.compose_path_node(&path_node).len());

        path_node.expand_dir(&TreeIndex::new(vec![1, 0]), path_node_compare);
        assert_eq!(23, composer.compose_path_node(&path_node).len());

        path_node.expand_dir(&TreeIndex::new(vec![1, 0, 2]), path_node_compare);
        assert_eq!(26, composer.compose_path_node(&path_node).len());

        path_node.expand_dir(&TreeIndex::new(vec![1, 0, 2, 1]), path_node_compare);
        assert_eq!(29, composer.compose_path_node(&path_node).len());

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

        // collapse_dir
        path_node.collapse_dir(&TreeIndex::new(vec![1, 0, 2, 1]));
        assert_eq!(
            26,
            composer.compose_path_node(&path_node).len(),
            "reducing the last opened dir"
        );

        path_node.collapse_dir(&TreeIndex::new(vec![1, 0]));
        assert_eq!(
            17,
            composer.compose_path_node(&path_node).len(),
            "reducing lots of sub dirs"
        );
    }

    #[test]
    fn test_integration_with_path_node_sort_dirs_bot_simple() {
        let mut config = Config::default();
        config.setup.working_dir = String::from("./tests/test_dirs");

        let composer = Composer::new(config.clone());
        let mut path_node = PathNode::new(&config.setup.working_dir);
        let path_node_compare = PathNode::compare_dirs_bot_simple;
        assert_eq!(0, composer.compose_path_node(&path_node).len());

        // expand_dir
        path_node.expand_dir(&TreeIndex::new(Vec::new()), path_node_compare);
        assert_eq!(
            13,
            composer.compose_path_node(&path_node).len(),
            "expanding the root directory"
        );

        path_node.expand_dir(&TreeIndex::new(vec![3]), path_node_compare);
        assert_eq!(
            13,
            composer.compose_path_node(&path_node).len(),
            "expanding a file does nothing"
        );

        path_node.expand_dir(&TreeIndex::new(vec![11]), path_node_compare);
        assert_eq!(17, composer.compose_path_node(&path_node).len());

        path_node.expand_dir(&TreeIndex::new(vec![11, 3]), path_node_compare);
        assert_eq!(23, composer.compose_path_node(&path_node).len());

        path_node.expand_dir(&TreeIndex::new(vec![11, 3, 3]), path_node_compare);
        assert_eq!(26, composer.compose_path_node(&path_node).len());

        path_node.expand_dir(&TreeIndex::new(vec![11, 3, 3, 1]), path_node_compare);
        assert_eq!(29, composer.compose_path_node(&path_node).len());

        // collapse_dir
        path_node.collapse_dir(&TreeIndex::new(vec![11, 3, 3, 1]));
        assert_eq!(
            26,
            composer.compose_path_node(&path_node).len(),
            "reducing the last opened dir"
        );

        path_node.collapse_dir(&TreeIndex::new(vec![11, 3]));
        assert_eq!(
            17,
            composer.compose_path_node(&path_node).len(),
            "reducing lots of sub dirs"
        );
    }
}
