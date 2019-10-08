use crate::model::path_node::PathNode;
use std::cmp::Ordering;

pub type PathNodeCompare = fn(&PathNode, &PathNode) -> Ordering;

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
}
