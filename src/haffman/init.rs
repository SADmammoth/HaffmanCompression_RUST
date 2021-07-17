mod tree;
use tree::Query;
use tree::Tree;

use std::collections::BinaryHeap;
use std::collections::HashMap;

pub fn create_priority_queue(text: &str) -> Query<char> {
	let mut map = HashMap::new();

	for word in text.chars() {
		let count = map.entry(word).or_insert(0);
		*count += 1;
	}

	let mut query = BinaryHeap::<Tree<char>>::with_capacity(map.len());

	for (symbol, count) in map {
		query.push(Tree::new_leaf(symbol, count));
	}

	Query(query)
}

pub fn create_tree(query: Query<char>) -> Tree<char> {
	let mut query: BinaryHeap<Tree<char>> = query.0;
	let mut should_pick_right = true;
	let mut right: Tree<char> = Tree::None;
	let mut left: Tree<char>;
	loop {
		if query.len() == 1 {
			break;
		}

		if should_pick_right {
			right = query.pop().unwrap();
			if query.len() == 1 {
				left = query.pop().unwrap();
				query.push(Tree::new_node(left.clone(), right.clone()));
			}
			should_pick_right = false;
		} else {
			left = query.pop().unwrap();
			query.push(Tree::new_node(left.clone(), right.clone()));
			should_pick_right = true;
		}
	}

	query.pop().unwrap()
}

pub fn create_alphabet(tree: Tree<char>) -> HashMap<char, String> {
	let paths = tree.deep_first_traversal();
	let mut alphabet = HashMap::<char, String>::new();

	for (leaf, path) in paths.iter() {
		alphabet.insert(
			leaf.content,
			path.iter()
				.map(|direction| match direction {
					tree::ChildPosition::Left => '0',
					tree::ChildPosition::Right => '1',
				})
				.collect::<String>(),
		);
	}

	alphabet
}
