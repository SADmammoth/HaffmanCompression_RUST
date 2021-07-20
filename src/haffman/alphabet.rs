use super::helpers::char_to_bin;
use super::helpers::pad;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct Alphabet(pub HashMap<char, String>);

impl Clone for Alphabet {
	fn clone(&self) -> Self {
		Alphabet(self.0.clone())
	}

	fn clone_from(&mut self, other: &Self) {
		self.0 = other.0.clone();
	}
}

impl Alphabet {
	pub fn new(map: HashMap<char, String>) -> Alphabet {
		if map.len() >= 2usize.pow(32u32) {
			panic!("Alphabet larger than 2^32 is not supported");
		} else {
			Alphabet(map)
		}
	}

	pub fn stringify(&self) -> String {
		if self.0.is_empty() {
			return String::from("[empty alphabet]");
		}

		let max_char_length = self.get_max_char_length();
		let max_code_length = self.get_max_code_length();

		self.0.iter().fold(String::new(), |string, (key, code)| {
			format!(
				"{}{}{}",
				string,
				pad(&char_to_bin(*key), max_char_length, '0'),
				pad(&(code.to_string() + &code), max_code_length + 1, '0')
			)
		})
	}

	pub fn get_max_char_length(&self) -> usize {
		let character = self
			.0
			.iter()
			.max_by(|a, b| char_to_bin(*a.0).len().cmp(&char_to_bin(*b.0).len()))
			.unwrap()
			.0;

		char_to_bin(*character).to_string().len()
	}
	pub fn get_max_code_length(&self) -> usize {
		self.0
			.iter()
			.max_by(|a, b| a.1.len().cmp(&b.1.len()))
			.unwrap()
			.1
			.to_string()
			.len()
	}

	pub fn encode_info(&self) -> String {
		format!(
			"{}{}{}{}",
			pad(&format!("{:b}", self.0.len()), 32, '0'),
			pad(&format!("{:b}", self.get_max_char_length()), 6, '0'),
			pad(
				&format!("{:b}", self.get_max_code_length() + 1),
				format!("{:b}", self.0.len()).len(),
				'0',
			),
			self.stringify()
		)
	}
}

impl Display for Alphabet {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(
			f,
			"{}",
			self.0
				.iter()
				.map(|(key, value)| { format!("({}, {}); ", key, value) })
				.collect::<String>()
		)
	}
}
