use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{self, Display};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TreeNode<T> {
	pub left: Tree<T>,
	pub right: Tree<T>,
	pub priority: u128,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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

#[derive(Clone, Copy, Debug)]
pub enum ChildPosition {
	Left,
	Right,
}

impl<T: Clone + Eq + std::hash::Hash> Tree<T> {
	pub fn deep_first_traversal(&self) -> HashMap<Box<TreeLeaf<T>>, Vec<ChildPosition>> {
		let path = Vec::<ChildPosition>::new();
		let mut paths_map = HashMap::<Box<TreeLeaf<T>>, Vec<ChildPosition>>::new();
		Tree::deep_first_recursion(self, path, &mut paths_map);

		paths_map
	}

	fn deep_first_recursion(
		root: &Tree<T>,
		path: Vec<ChildPosition>,
		paths_map: &mut HashMap<Box<TreeLeaf<T>>, Vec<ChildPosition>>,
	) {
		match root {
			Tree::Node(node) => {
				let mut left_path = path.clone();
				left_path.push(ChildPosition::Left);
				let mut right_path = path.clone();
				right_path.push(ChildPosition::Right);
				Tree::deep_first_recursion(&node.left, left_path, paths_map);
				Tree::deep_first_recursion(&node.right, right_path, paths_map);
			}
			Tree::Leaf(leaf) => {
				paths_map.insert(leaf.clone(), path);
			}
			_ => {}
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
		Some(other.cmp(self))
	}
}

impl<T: Display> Display for Tree<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.stringify())
	}
}

use std::collections::BinaryHeap;

pub struct Query<T>(pub BinaryHeap<Tree<T>>);

impl<T: Display + Ord + Clone> Display for Query<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{}",
			self.0
				.clone()
				.into_sorted_vec()
				.iter()
				.map(|x| { x.stringify() })
				.collect::<String>()
		)
	}
}
