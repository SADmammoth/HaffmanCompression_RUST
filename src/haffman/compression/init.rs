use super::super::tree::{ChildPosition, Tree};
use super::alphabet::Alphabet;
use super::query::Query;

use std::collections::BinaryHeap;
use std::collections::HashMap;

pub fn create_priority_queue(text: &str) -> Query<char> {
    let mut map = HashMap::new();

    for word in text.chars() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    let mut query = BinaryHeap::<Tree<char>>::with_capacity(map.len());

    let mut sorted_map = map.into_iter().collect::<Vec<_>>();
    sorted_map.sort_by(|(a_key, _), (b_key, _)| a_key.cmp(&b_key));

    for (symbol, count) in sorted_map {
        query.push(Tree::new_leaf(symbol, count));
    }

    Query(query)
}

pub fn create_tree(query: Query<char>) -> Tree<char> {
    let mut query: BinaryHeap<Tree<char>> = query.0;
    let mut right: Tree<char>;
    let mut left: Tree<char>;
    loop {
        if query.len() < 2 {
            break;
        }
        left = query.pop().unwrap_or(Tree::None);
        right = query.pop().unwrap_or(Tree::None);
        query.push(Tree::new_node(Box::new(left), Box::new(right)));
    }

    query.pop().unwrap()
}

fn create_alphabet(tree: Tree<char>) -> Alphabet {
    let paths = tree.deep_first_traversal();
    let mut alphabet = HashMap::<char, String>::new();

    for (leaf, path) in paths.iter() {
        alphabet.insert(
            leaf.content,
            path.iter()
                .map(|direction| match direction {
                    ChildPosition::Left => '0',
                    ChildPosition::Right => '1',
                })
                .collect::<String>(),
        );
    }

    Alphabet::new(alphabet)
}

pub fn init(message: &str) -> Alphabet {
    let query = create_priority_queue(message);
    let tree = create_tree(query);

    create_alphabet(tree)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn query_is_consistent() {
        let text = "Haffman compression";
        let result = create_priority_queue(text);

        for _ in 0..1000 {
            assert_eq!(result.to_string(), create_priority_queue(text).to_string());
        }
    }

    #[test]
    pub fn tree_is_consistent() {
        let text = "Haffman compression";
        let query = create_priority_queue(text);
        let result = create_tree(query.clone());

        for _ in 0..1000 {
            assert_eq!(result.to_string(), create_tree(query.clone()).to_string());
        }
    }

    #[test]
    pub fn alphabet_is_consistent() {
        let text = "Haffman compression";
        let query = create_priority_queue(text);
        let tree = create_tree(query);
        let result = create_alphabet(tree.clone());

        for _ in 0..1000 {
            assert_eq!(
                result.to_string(),
                create_alphabet(tree.clone()).to_string()
            );
        }
    }

    #[test]
    pub fn init_is_consistent() {
        let text = "Haffman compression";
        let result = init(text);

        for _ in 0..1000 {
            assert_eq!(result.to_string(), init(text).to_string());
        }
    }

    #[test]
    pub fn alphabet_has_all_letters() {
        let text = "Haffman compression";
        let alphabet = init(text);

        for symbol in text.chars() {
            assert!(alphabet.get_map().contains_key(&symbol));
        }
    }

    #[test]
    pub fn alphabet_has_no_odd_letters() {
        let text = "Haffman compression";
        let alphabet = init(text);
        let mut alphabet_map = alphabet.get_map().clone();

        for symbol in text.chars() {
            alphabet_map.remove(&symbol);
        }

        assert_eq!(alphabet_map.len(), 0)
    }
}
