#[derive(Debug)]
pub struct TreeIndex {
    pub index: Vec<usize>,
}

impl TreeIndex {
    pub fn new(index: Vec<usize>) -> Self {
        Self { index }
    }

    pub fn tree_index_to_flat_index(&self) -> usize {
        let mut flat_index = 0;
        for i in &self.index {
            flat_index += i + 1;
        }

        flat_index - 1
    }
}
