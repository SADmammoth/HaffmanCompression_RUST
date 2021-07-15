use std::cmp::Ordering;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TreeNode<T> {
	pub left: Tree<T>,
	pub right: Tree<T>,
	pub priority: u128,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TreeLeaf<T> {
	pub content: T,
	pub priority: u128,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Tree<T> {
	Node(Box<TreeNode<T>>),
	Leaf(Box<TreeLeaf<T>>),
	None,
}

impl<T> Tree<T> {
	pub fn get_priority(&self) -> u128 {
		match self {
			Tree::Node(node) => node.priority,
			Tree::Leaf(leaf) => leaf.priority,
			Tree::None => 0,
		}
	}
}

impl<T: Ord> Ord for Tree<T> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.get_priority().cmp(&other.get_priority())
	}
}

impl<T: Ord> PartialOrd for Tree<T> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(other.get_priority().cmp(&self.get_priority()))
	}
}
