use std::collections::HashMap;

pub fn compress(message: &str, alphabet: HashMap<char, String>) -> String {
	let mut encoded = String::new();
	let mut encoded_letter: Option<&String>;
	for symbol in message.chars() {
		println!("{:?}", (symbol, &alphabet));
		encoded_letter = alphabet.get(&symbol);
		match encoded_letter {
			Some(letter) => encoded.push_str(letter),
			None => encoded.push(symbol),
		}
	}
	encoded
}
