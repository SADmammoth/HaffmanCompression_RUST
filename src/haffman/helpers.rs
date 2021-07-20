pub fn pad(string: &str, len: usize, symbol: char) -> String {
	if len <= string.len() {
		string.to_string()
	} else {
		let spaces = std::iter::repeat(symbol)
			.take(len - string.len())
			.collect::<String>();
		format!("{}{}", spaces, string)
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
