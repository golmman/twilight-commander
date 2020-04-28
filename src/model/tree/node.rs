pub type NodeIndex = usize;

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub index: NodeIndex,
    pub value: T,
    pub parent: Option<NodeIndex>,
    pub children: Vec<NodeIndex>,
}

impl<T> Node<T> {}
