use crate::model::compare_functions::PathNodeCompare;
use crate::model::config::Config;
use crate::model::tree_index::TreeIndex;
use log::info;
use std::fs::canonicalize;
use std::path::PathBuf;

mod debug;

#[derive(Clone)]
pub struct PathNode {
    pub children: Vec<PathNode>,
    pub display_text: String,
    pub is_dir: bool,
    pub is_err: bool,
    pub is_expanded: bool,
    pub path: PathBuf,
}

impl From<&str> for PathNode {
    fn from(working_dir: &str) -> Self {
        Self {
            children: Vec::new(),
            display_text: String::from(working_dir),
            is_dir: true,
            is_err: false,
            is_expanded: false,
            path: PathBuf::from(working_dir),
        }
    }
}

impl From<String> for PathNode {
    fn from(working_dir: String) -> Self {
        Self {
            children: Vec::new(),
            display_text: working_dir.clone(),
            is_dir: true,
            is_err: false,
            is_expanded: false,
            path: PathBuf::from(working_dir),
        }
    }
}

impl PathNode {
    pub fn new_expanded(config: Config) -> Self {
        info!("initializing path node");

        let mut path_node = Self::from(config.setup.working_dir.clone());
        let path_node_compare = Self::get_path_node_compare(&config);
        path_node.expand_dir(&TreeIndex::new(), path_node_compare);

        path_node
    }

    pub fn get_absolute_path(&self) -> String {
        let canonicalized_path = canonicalize(self.path.as_path()).unwrap();
        canonicalized_path.to_str().unwrap().to_string()
    }

    fn list_path_node_children(
        &mut self,
        compare: PathNodeCompare,
    ) -> Vec<PathNode> {
        let dirs = self.path.read_dir();

        if dirs.is_err() {
            self.is_err = true;
            return Vec::new();
        }

        let mut path_nodes = dirs
            .unwrap()
            .map(|dir_entry| {
                let dir_entry = dir_entry.unwrap();

                PathNode {
                    children: Vec::new(),
                    display_text: dir_entry.file_name().into_string().unwrap(),
                    is_dir: dir_entry.path().is_dir(),
                    is_err: false,
                    is_expanded: false,
                    path: dir_entry.path(),
                }
            })
            .collect::<Vec<PathNode>>();

        path_nodes.sort_unstable_by(compare);

        path_nodes
    }

    pub fn expand_dir(
        &mut self,
        tree_index: &TreeIndex,
        compare: PathNodeCompare,
    ) {
        let mut path_node = self;
        for i in &tree_index.index {
            if path_node.children.len() > *i {
                path_node = &mut path_node.children[*i];
            }
        }

        if !path_node.path.is_dir() {
            return;
        }

        path_node.is_expanded = true;
        path_node.children = path_node.list_path_node_children(compare);
    }

    pub fn collapse_dir(&mut self, tree_index: &TreeIndex) {
        let mut path_node = self;
        for i in &tree_index.index {
            path_node = &mut path_node.children[*i];
        }

        path_node.is_expanded = false;
        path_node.children = Vec::new();
    }

    fn flat_index_to_tree_index_rec(
        &self,
        flat_index: &mut usize,
        tree_index: &mut TreeIndex,
    ) -> bool {
        if *flat_index == 0 {
            return true;
        }

        for (c, child) in self.children.iter().enumerate() {
            *flat_index -= 1;

            tree_index.index.push(c);
            if child.flat_index_to_tree_index_rec(flat_index, tree_index) {
                return true;
            }
            tree_index.index.pop();
        }

        false
    }

    pub fn flat_index_to_tree_index(&self, flat_index: usize) -> TreeIndex {
        let mut tree_index = TreeIndex::from(Vec::new());
        self.flat_index_to_tree_index_rec(
            &mut (flat_index + 1),
            &mut tree_index,
        );

        tree_index
    }

    pub fn tree_index_to_flat_index_rec(
        &self,
        target_tree_index: &TreeIndex,
        current_tree_index: &TreeIndex,
    ) -> usize {
        if current_tree_index >= target_tree_index {
            return 0;
        }

        if self.children.is_empty() {
            return 1;
        }

        let mut sum = 1;

        for (index, child) in self.children.iter().enumerate() {
            let mut new_current_tree_index = current_tree_index.clone();
            new_current_tree_index.index.push(index);

            sum += child.tree_index_to_flat_index_rec(
                target_tree_index,
                &new_current_tree_index,
            );
        }

        sum
    }

    pub fn tree_index_to_flat_index(&self, tree_index: &TreeIndex) -> usize {
        // We count the root directory, hence we have to subtract 1 to get the
        // proper index.
        self.tree_index_to_flat_index_rec(tree_index, &TreeIndex::new()) - 1
    }

    pub fn get_child_path_node(&self, tree_index: &TreeIndex) -> &Self {
        let mut child_node = self;
        for i in &tree_index.index {
            child_node = &child_node.children[*i];
        }

        child_node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_expanded_path_node() -> PathNode {
        let mut path_node = PathNode::from("./tests/test_dirs");
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

    mod get_child_path_node_tests {
        use super::*;

        #[test]
        fn first_dirs() {
            let path_node = {
                let mut path_node = PathNode::from("./tests/test_dirs");
                path_node.expand_dir(
                    &TreeIndex::new(),
                    PathNode::compare_dirs_top_simple,
                );
                path_node.expand_dir(
                    &TreeIndex::from(vec![0]),
                    PathNode::compare_dirs_top_simple,
                );
                path_node.expand_dir(
                    &TreeIndex::from(vec![0, 0]),
                    PathNode::compare_dirs_top_simple,
                );
                path_node
            };

            let child_path_node =
                path_node.get_child_path_node(&TreeIndex::from(vec![0, 0, 0]));

            assert_eq!("file4", child_path_node.display_text);
        }

        #[test]
        fn complex_dirs() {
            let path_node = get_expanded_path_node();

            let child_path_node = path_node
                .get_child_path_node(&TreeIndex::from(vec![1, 0, 2, 2]));

            assert_eq!("file12", child_path_node.display_text);
        }
    }

    mod tree_index_to_flat_index_tests {
        use super::*;

        #[test]
        fn complex_dirs() {
            let path_node = get_expanded_path_node();

            let flat_index =
                path_node.tree_index_to_flat_index(&TreeIndex::from(vec![4]));

            assert_eq!(22, flat_index);
        }

        #[test]
        fn complex_dirs2() {
            let path_node = get_expanded_path_node();

            let flat_index =
                path_node.tree_index_to_flat_index(&TreeIndex::from(vec![5]));

            assert_eq!(23, flat_index);
        }

        #[test]
        fn complex_dirs3() {
            let path_node = get_expanded_path_node();

            let flat_index = path_node
                .tree_index_to_flat_index(&TreeIndex::from(vec![1, 0, 4]));

            assert_eq!(15, flat_index);
        }

        #[test]
        fn total_count() {
            let path_node = get_expanded_path_node();

            let flat_index = path_node
                .tree_index_to_flat_index(&TreeIndex::from(vec![100_000]));

            assert_eq!(31, flat_index);
        }

        #[test]
        fn zero() {
            let path_node = get_expanded_path_node();

            let flat_index =
                path_node.tree_index_to_flat_index(&TreeIndex::from(vec![0]));

            assert_eq!(0, flat_index);
        }
    }
}
