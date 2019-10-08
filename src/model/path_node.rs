use crate::model::config::Config;
use crate::model::tree_index::TreeIndex;
use std::cmp::Ordering;
use std::fs::canonicalize;
use std::path::PathBuf;

pub struct PathNode {
    pub children: Vec<PathNode>,
    pub display_text: String,
    pub is_dir: bool,
    pub is_expanded: bool,
    pub path: PathBuf,

    sort_with_compare: fn(&PathNode, &PathNode) -> Ordering,
}

impl PathNode {
    fn sort_with_compare_dirs_top_simple(a: &PathNode, b: &PathNode) -> Ordering {
        if a.is_dir && !b.is_dir {
            return std::cmp::Ordering::Less;
        } else if !a.is_dir && b.is_dir {
            return std::cmp::Ordering::Greater;
        }

        a.display_text.cmp(&b.display_text)
    }

    fn sort_with_compare_dirs_bot_simple(a: &PathNode, b: &PathNode) -> Ordering {
        Self::sort_with_compare_dirs_top_simple(b, a)
    }

    // TODO: why not "new", why config as a reference?
    // TODO: config as a parameter is only needed for 'sort_with_compare', which should
    //       not be a field of the path_node but a parameter of expand_dir
    pub fn from_config(config: &Config) -> Self {
        let sort_with_compare: fn(&PathNode, &PathNode) -> Ordering = match config.behavior.path_node_sort.as_str() {
            "dirs_bot_simple" => Self::sort_with_compare_dirs_bot_simple,
            "dirs_top_simple" => Self::sort_with_compare_dirs_top_simple,
            "none" => |_, _| Ordering::Equal,
            _ => |_, _| Ordering::Equal,
        };

        Self {
            children: Vec::new(),
            display_text: config.setup.working_dir.clone(),
            is_dir: true,
            is_expanded: false,
            path: PathBuf::from(config.setup.working_dir.clone()),
            sort_with_compare,
        }
    }

    pub fn get_absolute_path(&self) -> String {
        let canonicalized_path = canonicalize(self.path.as_path()).unwrap();
        canonicalized_path.to_str().unwrap().to_string()
    }

    // TODO: errors when accessing a dir with insufficient permissios
    //       eg. /lost+found
    // TODO: why not a class method?
    fn list_path_node_children(path_node: &PathNode) -> Vec<PathNode> {
        let dirs = path_node.path.read_dir().unwrap();

        let mut path_nodes = dirs
            .map(|dir_entry| {
                let dir_entry = dir_entry.unwrap();

                PathNode {
                    children: Vec::new(),
                    display_text: dir_entry.file_name().into_string().unwrap(),
                    is_dir: dir_entry.path().is_dir(),
                    is_expanded: false,
                    path: dir_entry.path(),
                    sort_with_compare: path_node.sort_with_compare,
                }
            })
            .collect::<Vec<PathNode>>();

        path_nodes.sort_unstable_by(path_node.sort_with_compare);

        path_nodes
    }

    pub fn expand_dir(&mut self, tree_index: &TreeIndex) {
        let mut path_node = self;
        for i in &tree_index.index {
            path_node = &mut path_node.children[*i];
        }

        if !path_node.path.is_dir() {
            return;
        }

        path_node.is_expanded = true;
        path_node.children = Self::list_path_node_children(&path_node);
    }

    pub fn reduce_dir(&mut self, tree_index: &TreeIndex) {
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