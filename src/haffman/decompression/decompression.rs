use super::{init::build_tree, reverse_alphabet::ReverseAlphabet};

pub fn decompress(encoded: &str, alphabet: &ReverseAlphabet) -> String {
    let mut decoded = String::new();
    let root = build_tree(alphabet);
    let mut curr_tree_node = &root;

    for symbol in encoded.chars() {
        curr_tree_node = match symbol {
            '0' => curr_tree_node.get_left(),
            '1' => curr_tree_node.get_right(),
            _ => {
                panic!("Compressed message is damaged or has incorrect format")
            }
        };
        if curr_tree_node.is_leaf() {
            decoded.push(curr_tree_node.get_content().unwrap());
            curr_tree_node = &root;
        }
    }

    decoded
}
