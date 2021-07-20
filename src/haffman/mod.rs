pub mod alphabet;
mod compression;
mod helpers;
mod init;
mod query;
pub mod tree;
use alphabet::Alphabet;
use compression::compress;

pub struct HaffmanCompression {
	alphabet: Option<Alphabet>,
}

pub struct CompressionResult {
	encoded: String,
	alphabet: Alphabet,
}

impl HaffmanCompression {
	pub fn new() -> HaffmanCompression {
		HaffmanCompression { alphabet: None }
	}

	pub fn with_alphabet(&mut self, alphabet: Alphabet) -> &HaffmanCompression {
		self.alphabet = Some(alphabet);
		self
	}

	pub fn generate_alphabet_with_message(&mut self, message: &str) -> &HaffmanCompression {
		self.alphabet = Some(init::init(message));
		self
	}

	pub fn compress(&self, message: &str) -> CompressionResult {
		let alphabet_for_compression: Alphabet;
		match &self.alphabet {
			Some(alphabet) => {
				alphabet_for_compression = alphabet.clone();
			}
			None => {
				alphabet_for_compression = init::init(message);
			}
		};

		let encoded = compress(message, &alphabet_for_compression);

		CompressionResult {
			encoded,
			alphabet: alphabet_for_compression,
		}
	}
}

impl CompressionResult {
	pub fn get_with_injected_alphabet(&self) -> String {
		format!("{}{}", self.alphabet.encode_info(), self.encoded)
	}

	pub fn get_encoded(&self) -> String {
		self.encoded.clone()
	}

	pub fn get_alphabet(&self) -> Alphabet {
		self.alphabet.clone()
	}
}
