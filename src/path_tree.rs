pub mod path_node;
pub mod tree_index;

#[cfg(test)]
mod tests {
    use crate::path_tree::tree_index::TreeIndex;
    use crate::path_tree::path_node::PathNode;

    #[test]
    fn path_tree_test() {
        let mut p = PathNode::new("./tests/test_dirs");
        assert_eq!(0, p.prettify().len());

        // expand_dir
        p.expand_dir(&TreeIndex::new(Vec::new()));
        assert_eq!(13, p.prettify().len(), "expanding the root directory");

        p.expand_dir(&TreeIndex::new(vec![1]));
        assert_eq!(13, p.prettify().len(), "expanding a file does nothing");

        p.expand_dir(&TreeIndex::new(vec![7]));
        assert_eq!(17, p.prettify().len());

        p.expand_dir(&TreeIndex::new(vec![7, 2]));
        assert_eq!(23, p.prettify().len());

        p.expand_dir(&TreeIndex::new(vec![7, 2, 4]));
        assert_eq!(26, p.prettify().len());

        p.expand_dir(&TreeIndex::new(vec![7, 2, 4, 0]));
        assert_eq!(29, p.prettify().len());

        // println!("{:#?}", p.prettify());
        // println!("------------------------------");

        // tree_index_to_flat_index
        let flat_index = TreeIndex::new(vec![7, 2, 4, 0, 0]).tree_index_to_flat_index();
        assert_eq!(17, flat_index);

        // flat_index_to_tree_index
        let tree_index = p.flat_index_to_tree_index(17);
        assert_eq!(vec![7, 2, 4, 0, 0], tree_index.index);

        let tree_index = p.flat_index_to_tree_index(18);
        assert_eq!(vec![7, 2, 4, 0, 1], tree_index.index);

        let tree_index = p.flat_index_to_tree_index(20);
        assert_eq!(vec![7, 2, 4, 1], tree_index.index);

        let tree_index = p.flat_index_to_tree_index(23);
        assert_eq!(vec![7, 3], tree_index.index);

        // reduce_dir
        p.reduce_dir(&TreeIndex::new(vec![7, 2, 4, 0]));
        assert_eq!(26, p.prettify().len(), "reducing the last opened dir");

        p.reduce_dir(&TreeIndex::new(vec![7, 2]));
        assert_eq!(17, p.prettify().len(), "reducing lots of sub dirs");
    }
}
