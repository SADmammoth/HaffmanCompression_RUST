pub fn pad(string: &String, len: usize, symbol: char) -> String {
	if len <= string.len() {
		string.clone()
	} else {
		std::iter::repeat(symbol)
			.take(len - string.len())
			.collect::<String>()
			+ string
	}
}

pub fn char_to_bin(character: char) -> String {
	let mut char_number = [0u8; 1];
	character.encode_utf8(&mut char_number);

	let mut string = String::new();
	for symbol in char_number.iter() {
		string += &format!("{:b}", symbol);
	}

	string
}
