use std::fs::{read_dir, DirEntry};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct PathNode {
    children: Vec<PathNode>,
    display_text: String,
    is_dir: bool,
    path: PathBuf,
}

#[derive(Debug)]
pub struct TreeIndex {
    index: Vec<usize>,
}

impl PathNode {
    fn new(file_path: &str) -> Self {
        Self {
            children: Vec::new(),
            display_text: String::from(file_path),
            is_dir: true,
            path: PathBuf::from(file_path),
        }
    }
}

impl TreeIndex {
    fn new(index: Vec<usize>) -> Self {
        Self { index }
    }
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

pub fn expand_dir(path_node: &mut PathNode, tree_index: &TreeIndex) {
    let mut leaf_node = path_node;
    for i in &tree_index.index {
        leaf_node = &mut leaf_node.children[*i];
    }

    if !leaf_node.path.is_dir() {
        return;
    }

    leaf_node.children = list_path_nodes(&leaf_node.path);
}

pub fn reduce_dir(path_node: &mut PathNode, tree_index: &TreeIndex) {
    let mut leaf_node = path_node;
    for i in &tree_index.index {
        leaf_node = &mut leaf_node.children[*i];
    }

    leaf_node.children = Vec::new();
}

pub fn tree_index_to_flat_index(tree_index: &TreeIndex) -> usize {
    let mut flat_index = 0;
    for i in &tree_index.index {
        flat_index += i + 1;
    }

    flat_index - 1
}

fn flat_index_to_tree_index_rec(
    path_node: &PathNode,
    flat_index: &mut usize,
    tree_index: &mut TreeIndex,
) -> bool {
    if *flat_index == 0 {
        return true;
    }

    for (c, child) in path_node.children.iter().enumerate() {
        *flat_index -= 1;

        tree_index.index.push(c);
        if flat_index_to_tree_index_rec(child, flat_index, tree_index) {
            return true;
        }
        tree_index.index.pop();
    }

    false
}

pub fn flat_index_to_tree_index(path_node: &PathNode, flat_index: usize) -> TreeIndex {
    let mut result = TreeIndex::new(Vec::new());
    flat_index_to_tree_index_rec(path_node, &mut (flat_index + 1), &mut result);

    result
}

fn prettify_rec(path_node: &PathNode, texts: &mut Vec<String>, depth: usize) {
    for child in &path_node.children {
        let dir_indicator = if child.is_dir { "⋅> " } else { "⋅  " };

        let text = format!(
            "{}{}{}",
            "⋅  ".repeat(depth),
            dir_indicator,
            child.display_text.clone()
        );
        texts.push(text);
        prettify_rec(child, texts, depth + 1);
    }
}

pub fn prettify(path_node: &PathNode) -> Vec<String> {
    let mut result = Vec::new();

    prettify_rec(path_node, &mut result, 0);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_tree_test() {
        let mut p = PathNode::new("./tests/test_dirs");
        assert_eq!(0, prettify(&p).len());

        // expand_dir
        expand_dir(&mut p, &TreeIndex::new(Vec::new()));
        assert_eq!(6, prettify(&p).len(), "expanding the root directory");

        expand_dir(&mut p, &TreeIndex::new(vec![1]));
        assert_eq!(6, prettify(&p).len(), "expanding a file does nothing");

        expand_dir(&mut p, &TreeIndex::new(vec![3]));
        assert_eq!(10, prettify(&p).len());

        expand_dir(&mut p, &TreeIndex::new(vec![3, 2]));
        assert_eq!(16, prettify(&p).len());

        expand_dir(&mut p, &TreeIndex::new(vec![3, 2, 4]));
        assert_eq!(19, prettify(&p).len());

        expand_dir(&mut p, &TreeIndex::new(vec![3, 2, 4, 0]));
        assert_eq!(22, prettify(&p).len());

        // println!("{:#?}", prettify(&p));
        // println!("------------------------------");

        // tree_index_to_flat_index
        let flat_index = tree_index_to_flat_index(&TreeIndex::new(vec![3, 2, 4, 0, 0]));
        assert_eq!(13, flat_index);

        // flat_index_to_tree_index
        let tree_index = flat_index_to_tree_index(&p, 13);
        assert_eq!(vec![3, 2, 4, 0, 0], tree_index.index);

        let tree_index = flat_index_to_tree_index(&p, 14);
        assert_eq!(vec![3, 2, 4, 0, 1], tree_index.index);

        let tree_index = flat_index_to_tree_index(&p, 16);
        assert_eq!(vec![3, 2, 4, 1], tree_index.index);

        let tree_index = flat_index_to_tree_index(&p, 19);
        assert_eq!(vec![3, 3], tree_index.index);

        // reduce_dir
        reduce_dir(&mut p, &TreeIndex::new(vec![3, 2, 4, 0]));
        assert_eq!(19, prettify(&p).len(), "reducing the last opened dir");

        reduce_dir(&mut p, &TreeIndex::new(vec![3, 2]));
        assert_eq!(10, prettify(&p).len(), "reducing lots of sub dirs");
    }
}
