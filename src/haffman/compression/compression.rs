use super::alphabet::Alphabet;

pub fn compress(message: &str, alphabet: &Alphabet) -> String {
    let mut encoded = String::new();
    let mut encoded_letter: Option<&String>;
    for symbol in message.chars() {
        encoded_letter = alphabet.get_map().get(&symbol);
        match encoded_letter {
            Some(letter) => encoded.push_str(letter),
            None => encoded.push(symbol),
        }
    }

    encoded
}
