#[derive(Debug)]
pub struct TreeIndex {
    pub index: Vec<usize>,
}

impl From<Vec<usize>> for TreeIndex {
    fn from(index: Vec<usize>) -> Self {
        Self { index }
    }
}

impl TreeIndex {
    #[allow(dead_code)] // TODO: remove?
    pub fn tree_index_to_flat_index(&self) -> usize {
        let mut flat_index = 0;
        for i in &self.index {
            flat_index += i + 1;
        }

        flat_index - 1
    }
}
