use crate::model::tree::node::NodeIndex;
use crate::model::tree::Tree;

struct DepthFirstIteratorVertex {
    index: NodeIndex,
    discovered: bool,
}

pub struct DepthFirstIterator<'a, T> {
    tree: &'a Tree<T>,
    vertex_stack: Vec<DepthFirstIteratorVertex>,
}

impl<'a, T> DepthFirstIterator<'a, T> {
    pub fn new(tree: &'a Tree<T>) -> Self {
        DepthFirstIterator {
            tree,
            vertex_stack: vec![DepthFirstIteratorVertex {
                index: 0,
                discovered: false,
            }],
        }
    }
}

impl<'a, T> Iterator for DepthFirstIterator<'a, T> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(mut vertex) = self.vertex_stack.pop() {
            if !vertex.discovered {
                vertex.discovered = true;
                for child in &self.tree.nodes[vertex.index].children {
                    self.vertex_stack.push(DepthFirstIteratorVertex {
                        discovered: false,
                        index: *child,
                    });
                }
            }
            return Some(vertex.index);
        }

        None
    }
}
