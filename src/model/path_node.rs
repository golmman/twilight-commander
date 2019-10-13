use crate::model::compare_functions::PathNodeCompare;
use crate::model::tree_index::TreeIndex;
use std::fs::canonicalize;
use std::path::PathBuf;

pub struct PathNode {
    pub children: Vec<PathNode>,
    pub display_text: String,
    pub is_dir: bool,
    pub is_expanded: bool,
    pub path: PathBuf,
}

impl PathNode {
    pub fn new(working_dir: &str) -> Self {
        Self {
            children: Vec::new(),
            display_text: String::from(working_dir),
            is_dir: true,
            is_expanded: false,
            path: PathBuf::from(working_dir),
        }
    }

    pub fn get_absolute_path(&self) -> String {
        let canonicalized_path = canonicalize(self.path.as_path()).unwrap();
        canonicalized_path.to_str().unwrap().to_string()
    }

    // TODO: errors when accessing a dir with insufficient permissios
    //       eg. /lost+found
    fn list_path_node_children(&self, compare: PathNodeCompare) -> Vec<PathNode> {
        let dirs = self.path.read_dir().unwrap();

        let mut path_nodes = dirs
            .map(|dir_entry| {
                let dir_entry = dir_entry.unwrap();

                PathNode {
                    children: Vec::new(),
                    display_text: dir_entry.file_name().into_string().unwrap(),
                    is_dir: dir_entry.path().is_dir(),
                    is_expanded: false,
                    path: dir_entry.path(),
                }
            })
            .collect::<Vec<PathNode>>();

        path_nodes.sort_unstable_by(compare);

        path_nodes
    }

    pub fn expand_dir(&mut self, tree_index: &TreeIndex, compare: PathNodeCompare) {
        let mut path_node = self;
        for i in &tree_index.index {
            path_node = &mut path_node.children[*i];
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

    fn flat_index_to_tree_index_rec(&self, flat_index: &mut usize, tree_index: &mut TreeIndex) -> bool {
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
        let mut result = TreeIndex::new(Vec::new());
        self.flat_index_to_tree_index_rec(&mut (flat_index + 1), &mut result);

        result
    }

    // TODO: tests
    pub fn get_child_path_node(&self, tree_index: &TreeIndex) -> &Self {
        let mut child_node = self;
        for i in &tree_index.index {
            child_node = &child_node.children[*i];
        }

        child_node
    }
}
