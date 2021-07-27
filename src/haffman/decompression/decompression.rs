use crate::haffman::tree::{ChildPosition, Tree};

use super::reverse_alphabet::ReverseAlphabet;

pub fn build_tree(alphabet: &ReverseAlphabet) -> Tree<&char> {
    let root = Tree::new_node(Tree::None, Tree::None);
    let mut curr = root.clone();
    for (code, symbol) in alphabet.get_map() {
        for direction in code.chars() {
            match direction {
                '0' => curr = curr.get_and_create(ChildPosition::Left),
                '1' => curr = curr.get_and_create(ChildPosition::Right),
                _ => {
                    panic!("Alphabet is damaged or has incorrect format")
                }
            }
        }
        curr.set_content(symbol);
    }

    root
}

pub fn decompress(encoded: &str, alphabet: &ReverseAlphabet) -> String {
    let mut decoded = String::new();
    let tree = build_tree(alphabet);
    let mut curr_tree_node = tree;

    for symbol in encoded.chars() {
        if curr_tree_node.is_leaf() {
            decoded.push(*curr_tree_node.get_content().unwrap());
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

// fn find_char(alphabet: &ReverseAlphabet, code: &str) -> Option<char> {
//     let mut candidates = alphabet.get_map().clone();
//     candidates.retain(|key, _| key.starts_with(&code));

//     if candidates.len() == 1 {
//         match candidates.values().last() {
//             Some(val) => Some(*val),
//             None => None,
//         }
//     } else {
//         None
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::haffman::{create_priority_queue, create_tree, HaffmanCompression};

    #[test]
    pub fn tree_is_recreated_correctly() {
        let message = "Haffman compression";
        let query = create_priority_queue(message);
        let tree = create_tree(query);
        let compressed = HaffmanCompression::new().compress(message);
        let (alphabet, _) =
            ReverseAlphabet::decode_from_binary(compressed.get_with_injected_alphabet());

        let recreated_tree = build_tree(&alphabet);

        assert_eq!(tree.to_string(), recreated_tree.to_string());
    }
}
