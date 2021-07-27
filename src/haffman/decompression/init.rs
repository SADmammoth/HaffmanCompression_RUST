use crate::haffman::tree::Tree;

use super::reverse_alphabet::ReverseAlphabet;

pub fn build_tree(alphabet: &ReverseAlphabet) -> Tree<char> {
    let mut root = Tree::new_node(Tree::None, Tree::None);
    let mut curr: &mut Tree<char>;
    for (code, symbol) in alphabet.get_map() {
        curr = &mut root;
        for direction in code.chars().take(code.len() - 1) {
            match direction {
                '0' => curr = curr.get_or_create_left(),
                '1' => curr = curr.get_or_create_right(),
                _ => {
                    panic!("Alphabet is damaged or has incorrect format")
                }
            }
        }
        match code.chars().last().unwrap() {
            '0' => curr.create_left_leaf(symbol.clone(), 0),
            '1' => curr.create_right_leaf(symbol.clone(), 0),
            _ => {
                panic!("Alphabet is damaged or has incorrect format")
            }
        }
    }

    root
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

        assert_eq!(
            tree.stringify_ignore_priority(),
            recreated_tree.stringify_ignore_priority()
        );
    }
}
