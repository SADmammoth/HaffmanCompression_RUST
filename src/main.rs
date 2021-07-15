use std::collections::BinaryHeap;
use std::collections::HashMap;
mod tree;

use tree::{Tree, TreeLeaf, TreeNode};

fn main() {
    let query = create_priority_queue("FreedommmmeinnFd");
    let tree = create_tree(query);
    println!("{}", tree);
}

fn create_priority_queue(text: &str) -> BinaryHeap<Tree<char>> {
    let mut map = HashMap::new();

    for word in text.chars() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    let mut query = BinaryHeap::<Tree<char>>::with_capacity(map.len());

    for (symbol, count) in map {
        query.push(Tree::Leaf(Box::new(TreeLeaf {
            content: symbol,
            priority: count,
        })));
    }

    query
}

fn create_tree(mut query: BinaryHeap<Tree<char>>) -> Tree<char> {
    let mut left_right_flag = true;
    let mut saved_right: Tree<char> = Tree::None;
    let mut saved_left: Tree<char>;
    loop {
        if query.len() == 1 {
            break;
        }
        if left_right_flag {
            saved_right = query.pop().unwrap();
            if query.len() == 1 {
                saved_left = query.pop().unwrap();
                query.push(Tree::Node(Box::new(TreeNode {
                    left: saved_right.clone(),
                    right: saved_left.clone(),
                    priority: saved_right.get_priority() + saved_left.get_priority(),
                })));
            }
            left_right_flag = false;
        } else {
            saved_left = query.pop().unwrap();
            query.push(Tree::Node(Box::new(TreeNode {
                left: saved_right.clone(),
                right: saved_left.clone(),
                priority: saved_right.get_priority() + saved_left.get_priority(),
            })));
            left_right_flag = true;
        }
    }

    query.pop().unwrap()
}
