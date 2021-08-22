use std::cmp::Ordering;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TreeNode<T> {
    pub left: Box<Tree<T>>,
    pub right: Box<Tree<T>>,
    pub priority: u128,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TreeLeaf<T> {
    pub content: T,
    pub priority: u128,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Tree<T> {
    Node(TreeNode<T>),
    Leaf(TreeLeaf<T>),
    None,
}

//* For BinaryHeap compatibility
impl<T: Ord + Copy> Ord for Tree<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_priority()
            .cmp(&other.get_priority())
            .then_with(|| self.get_content().cmp(&other.get_content()))
    }
}

impl<T: Ord + Copy> PartialOrd for Tree<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cmp(self))
    }
}
