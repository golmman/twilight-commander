use crate::path_tree::tree_index::TreeIndex;
use std::fs::canonicalize;
use std::path::PathBuf;

#[derive(Debug)]
pub struct PathNode {
    children: Vec<PathNode>,
    display_text: String,
    is_dir: bool,
    path: PathBuf,
}

impl PathNode {
    pub fn new(file_path: &str) -> Self {
        Self {
            children: Vec::new(),
            display_text: String::from(file_path),
            is_dir: true,
            path: PathBuf::from(file_path),
        }
    }

    pub fn get_path(&self) -> String {
        let canonicalized_path = canonicalize(self.path.as_path()).unwrap();
        canonicalized_path.to_str().unwrap().to_string()
    }

    fn prettify_rec(&self, texts: &mut Vec<String>, depth: usize) {
        for child in &self.children {
            let dir_indicator = if child.is_dir { "-> " } else { "-  " };

            let text = format!(
                "{}{}{}",
                "-  ".repeat(depth),
                dir_indicator,
                child.display_text.clone()
            );
            texts.push(text);
            child.prettify_rec(texts, depth + 1);
        }
    }

    pub fn prettify(&self) -> Vec<String> {
        let mut result = Vec::new();

        self.prettify_rec(&mut result, 0);

        result
    }

    fn list_path_nodes(path: &PathBuf) -> Vec<PathNode> {
        let dirs = path.read_dir().unwrap();

        dirs.map(|dir_entry| {
            let dir_entry = dir_entry.unwrap();

            PathNode {
                children: Vec::new(),
                display_text: dir_entry.file_name().into_string().unwrap(),
                is_dir: dir_entry.path().is_dir(),
                path: dir_entry.path(),
            }
        })
        .collect::<Vec<PathNode>>()
    }

    pub fn expand_dir(&mut self, tree_index: &TreeIndex) {
        let mut leaf_node = self;
        for i in &tree_index.index {
            leaf_node = &mut leaf_node.children[*i];
        }

        if !leaf_node.path.is_dir() {
            return;
        }

        leaf_node.children = Self::list_path_nodes(&leaf_node.path);
    }

    pub fn reduce_dir(&mut self, tree_index: &TreeIndex) {
        let mut leaf_node = self;
        for i in &tree_index.index {
            leaf_node = &mut leaf_node.children[*i];
        }

        leaf_node.children = Vec::new();
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
        let mut result = TreeIndex::new(Vec::new());
        self.flat_index_to_tree_index_rec(&mut (flat_index + 1), &mut result);

        result
    }
}
