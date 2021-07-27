use crate::haffman::tree::ChildPosition;

use super::{init::build_tree, reverse_alphabet::ReverseAlphabet};

pub fn decompress(encoded: &str, alphabet: &ReverseAlphabet) -> String {
    let mut decoded = String::new();
    let tree = build_tree(alphabet);
    let mut curr_tree_node = tree;

    for symbol in encoded.chars() {
        if curr_tree_node.is_leaf() {
            decoded.push(curr_tree_node.get_content().unwrap());
        } else {
            curr_tree_node = match symbol {
                '0' => curr_tree_node.get_child(ChildPosition::Left),
                '1' => curr_tree_node.get_child(ChildPosition::Right),
                _ => {
                    panic!("Compressed message is damaged or has incorrect format")
                }
            }
        }
    }

    decoded
}
