use std::{fmt::Display, time::Instant};

use super::{alphabet::Alphabet, helpers::text_to_bin};

static mut ELAPSED: u128 = 0;

pub fn compress(message: &str, alphabet: &Alphabet) -> String {
	let now: Instant = Instant::now();
	let mut encoded = String::new();
	let mut encoded_letter: Option<&String>;
	for symbol in message.chars() {
		encoded_letter = alphabet.0.get(&symbol);
		match encoded_letter {
			Some(letter) => encoded.push_str(letter),
			None => encoded.push(symbol),
		}
	}

	unsafe {
		ELAPSED = now.elapsed().as_millis();
	}

	encoded
}

pub struct AnalyzeResult {
	input_chars_count: usize,
	original_size: usize,
	alphabet_length: usize,
	compressed_size: usize,
	compressed_with_alphabet: usize,
	compression_percent: f32,
	compression_percent_with_alphabet: f32,
	compression_time: u128,
}

impl Display for AnalyzeResult {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "
		Original size (chars): {},\n
		Original size (b): {},\n
		Alphabet length (pairs): {},\n
		Compressed size (b): {},\n
		Compressed size (with aplphabet, b): {},\n
		Compression percent: {},\n
		Compression percent (with aplphabet): {},\n
		Compression time (ms): {}\n
		", 
		self.input_chars_count,
		self.original_size,
		self.alphabet_length,
		self.compressed_size,
		self.compressed_with_alphabet,
		self.compression_percent * 100f32,
		self.compression_percent_with_alphabet * 100f32,
		self.compression_time)
	}
}

pub fn analyze(
	message: &str,
	compressed: &str,
	alphabet: &Alphabet
) -> AnalyzeResult {
	let compressed_with_alphabet = alphabet.stringify() + compressed;
	let original_size = text_to_bin(message).len();
	AnalyzeResult {
		input_chars_count: message.len(),
		original_size,
		alphabet_length: alphabet.len(),
		compressed_size: compressed.len(),
		compressed_with_alphabet: compressed_with_alphabet.len(),
		compression_percent:  (original_size - compressed.len()) as f32 / original_size as f32,
		compression_percent_with_alphabet:  (original_size - compressed_with_alphabet.len()) as f32 / original_size as f32,
		compression_time: unsafe { ELAPSED }
	}
}
