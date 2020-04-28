use node::Node;
use node::NodeIndex;

pub mod iterators;
pub mod node;

#[derive(Clone, Debug)]
pub struct Tree<T> {
    nodes: Vec<Node<T>>,
}

impl<T> Tree<T> {
    pub fn new(value: T) -> Self {
        let node = Node {
            index: 0,
            value,
            parent: None,
            children: vec![],
        };
        Self { nodes: vec![node] }
    }

    pub fn add_node(&mut self, value: T) -> NodeIndex {
        let node = Node {
            index: self.nodes.len(),
            value,
            parent: None,
            children: vec![],
        };
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    pub fn connect(&mut self, parent: NodeIndex, child: NodeIndex) {
        self.nodes[parent].children.push(child);
        self.nodes[child].parent = Some(parent);
    }

    pub fn get_node(&self, index: NodeIndex) -> &Node<T> {
        &self.nodes[index]
    }

    pub fn add_child(&mut self, parent: NodeIndex, value: T) -> NodeIndex {
        let child = self.add_node(value);
        self.connect(parent, child);
        child
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::tree::iterators::DepthFirstIterator;

    #[derive(Debug)]
    struct TestStruct {
        number: i32,
        text: String,
    }

    #[test]
    fn build_tree() {
        let value_root = TestStruct {
            number: 0,
            text: String::from("root"),
        };
        let value_branch1 = TestStruct {
            number: 1,
            text: String::from("branch1"),
        };
        let value_branch2 = TestStruct {
            number: 2,
            text: String::from("branch2"),
        };
        let value_leaf = TestStruct {
            number: 3,
            text: String::from("leaf"),
        };
        let value_subleaf = TestStruct {
            number: 4,
            text: String::from("subleaf"),
        };

        let mut tree = Tree::new(value_root);

        let index_root = 0;
        let index_branch1 = tree.add_child(index_root, value_branch1);
        let index_branch2 = tree.add_child(index_root, value_branch2);
        let index_leaf = tree.add_child(index_branch1, value_leaf);

        // testing the connection and traversion
        let dfi = DepthFirstIterator::new(&tree);
        let mut indices: Vec<NodeIndex> = vec![];
        for index in dfi {
            indices.push(index);
        }
        assert_eq!(vec![0, 2, 1, 3], indices);

        // adding another child
        let index_leaf = tree.add_child(index_branch1, value_subleaf);
        let dfi = DepthFirstIterator::new(&tree);
        let mut indices: Vec<NodeIndex> = vec![];
        for index in dfi {
            indices.push(index);
        }
        assert_eq!(vec![0, 2, 1, 4, 3], indices);
    }
}
