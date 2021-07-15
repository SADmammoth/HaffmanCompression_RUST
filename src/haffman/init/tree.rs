use std::cmp::Ordering;
use std::fmt::{self, Display};

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
	pub fn new_node(left: Tree<T>, right: Tree<T>) -> Tree<T> {
		let priority = left.get_priority() + right.get_priority();
		Tree::Node(Box::new(TreeNode {
			left,
			right,
			priority,
		}))
	}

	pub fn new_leaf(content: T, priority: u128) -> Tree<T> {
		Tree::Leaf(Box::new(TreeLeaf { content, priority }))
	}

	pub fn get_priority(&self) -> u128 {
		match self {
			Tree::Node(node) => node.priority,
			Tree::Leaf(leaf) => leaf.priority,
			Tree::None => 0,
		}
	}
}

impl<T: Display> Tree<T> {
	pub fn stringify(&self) -> String {
		let mut string = String::new();

		Tree::stringify_recursion(&mut string, self, 0, "")
	}

	fn stringify_recursion(
		string: &mut String,
		root: &Tree<T>,
		indent: usize,
		add: &str,
	) -> String {
		match root {
			Tree::Node(node) => {
				string.push_str(&format!(
					"{item:>indent$}\n",
					item = format!("{}{}", add, node.priority),
					indent = indent
				));
				Tree::stringify_recursion(string, &node.right, indent + 6, "r: ");
				Tree::stringify_recursion(string, &node.left, indent + 6, "l: ");
			}
			Tree::Leaf(leaf) => {
				string.push_str(&format!(
					"{item:>indent$}",
					item = format!("{}{}", add, leaf.priority),
					indent = indent
				));
				string.push_str(&format!("({})\n", leaf.content));
			}
			Tree::None => {}
		};

		string.clone()
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

impl<T: Display> Display for Tree<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.stringify())
	}
}
