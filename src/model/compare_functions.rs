use crate::model::config::Config;
use crate::model::path_node::PathNode;
use std::cmp::Ordering;

pub type PathNodeCompare = fn(&PathNode, &PathNode) -> Ordering;

// TODO: add tests
impl PathNode {
    pub fn compare_dirs_top_simple(a: &PathNode, b: &PathNode) -> Ordering {
        if a.is_dir && !b.is_dir {
            return std::cmp::Ordering::Less;
        } else if !a.is_dir && b.is_dir {
            return std::cmp::Ordering::Greater;
        }

        a.display_text.cmp(&b.display_text)
    }

    pub fn compare_dirs_bot_simple(a: &PathNode, b: &PathNode) -> Ordering {
        Self::compare_dirs_top_simple(b, a)
    }

    pub fn get_path_node_compare(config: &Config) -> PathNodeCompare {
        let path_node_compare: fn(&PathNode, &PathNode) -> Ordering =
            match config.behavior.path_node_sort.as_str() {
                "dirs_bot_simple" => PathNode::compare_dirs_bot_simple,
                "dirs_top_simple" => PathNode::compare_dirs_top_simple,
                "none" => |_, _| Ordering::Equal,
                _ => |_, _| Ordering::Equal,
            };

        path_node_compare
    }
}
