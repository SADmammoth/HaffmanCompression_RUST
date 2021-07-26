use super::reverse_alphabet::ReverseAlphabet;

pub fn decompress(encoded: &str, alphabet: &ReverseAlphabet) -> String {
    let mut decoded = String::new();
    let mut encoded_not_processed = encoded.to_string();
    let mut i: usize = 0;

    while encoded_not_processed.len() > 0 {
        if i > encoded_not_processed.len() {
            panic!("Encoded message is damaged or has incorrect format");
        }
        match find_char(alphabet, &encoded_not_processed[0..i]) {
            Some(character) => {
                decoded.push(character);
                encoded_not_processed.drain(0..i);
                i = 0;
            }
            None => {}
        }

        i += 1;
    }

    decoded.to_string()
}

fn find_char(alphabet: &ReverseAlphabet, code: &str) -> Option<char> {
    let mut candidates = alphabet.get_map().clone();
    candidates.retain(|key, _| key.starts_with(&code));

    if candidates.len() == 1 {
        match candidates.values().last() {
            Some(val) => Some(*val),
            None => None,
        }
    } else {
        None
    }
}
