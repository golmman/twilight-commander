#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TreeIndex {
    pub index: Vec<usize>,
}

impl From<Vec<usize>> for TreeIndex {
    fn from(index: Vec<usize>) -> Self {
        Self { index }
    }
}

impl TreeIndex {
    pub fn new() -> Self {
        Self { index: vec![] }
    }

    pub fn get_parent(&self) -> Self {
        if self.index.is_empty() {
            return Self { index: vec![] };
        }

        let mut index = self.index.clone();
        index.pop().unwrap();

        Self { index }
    }

    #[allow(dead_code)] // TODO: remove?
    pub fn to_flat_index(&self) -> usize {
        if self.index.is_empty() {
            return 0;
        }

        let mut flat_index = 0;
        for i in &self.index {
            flat_index += i + 1;
        }

        flat_index - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod get_parent_tests {
        use super::*;

        #[test]
        fn empty() {
            let tree_index = TreeIndex::new();
            let parent = tree_index.get_parent();
            assert_eq!(TreeIndex::new(), parent);
        }

        #[test]
        fn minimal() {
            let tree_index = TreeIndex::from(vec![0]);
            let parent = tree_index.get_parent();
            assert_eq!(TreeIndex::new(), parent);
        }

        #[test]
        fn zeroes() {
            let tree_index = TreeIndex::from(vec![0, 0, 0, 0, 0]);
            let parent = tree_index.get_parent();
            assert_eq!(TreeIndex::from(vec![0, 0, 0, 0]), parent);
        }

        #[test]
        fn complex() {
            let tree_index = TreeIndex::from(vec![3, 4, 6, 7, 1]);
            let parent = tree_index.get_parent();
            assert_eq!(TreeIndex::from(vec![3, 4, 6, 7]), parent);
        }
    }

    mod to_flat_index_tests {
        use super::*;

        #[test]
        fn empty() {
            let tree_index = TreeIndex::new();
            let flat_index = tree_index.to_flat_index();
            assert_eq!(0, flat_index);
        }

        #[test]
        fn minimal() {
            let tree_index = TreeIndex::from(vec![0]);
            let flat_index = tree_index.to_flat_index();
            assert_eq!(0, flat_index);
        }

        #[test]
        fn zeroes() {
            let tree_index = TreeIndex::from(vec![0, 0, 0, 0, 0]);
            let flat_index = tree_index.to_flat_index();
            assert_eq!(4, flat_index);
        }

        #[test]
        fn complex() {
            let tree_index = TreeIndex::from(vec![3, 4, 6, 7, 1]);
            let flat_index = tree_index.to_flat_index();
            assert_eq!(25, flat_index);
        }
    }
}
