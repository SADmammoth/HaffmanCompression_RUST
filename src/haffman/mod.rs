pub mod alphabet;
mod compression;
mod helpers;
mod init;
mod query;
pub mod tree;
use std::{fmt::Display, time::Instant};

use alphabet::Alphabet;
use compression::compress;

use self::helpers::text_to_bin;

pub struct HaffmanCompression {
    alphabet: Option<Alphabet>,
}

pub struct CompressionResult<'a> {
    message: &'a str,
    encoded: String,
    alphabet: Alphabet,
}

static mut ELAPSED: u128 = 0;

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

    pub fn compress<'a>(&self, message: &'a str) -> CompressionResult<'a> {
        let now: Instant = Instant::now();
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

        unsafe {
            ELAPSED = now.elapsed().as_millis();
        }

        CompressionResult {
            message,
            encoded,
            alphabet: alphabet_for_compression,
        }
    }
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
        write!(
            f,
            "
Original size (chars): {},\n
Original size (b): {},\n
Alphabet length (pairs): {},\n
Compressed size (b): {},\n
Compressed size (with alphabet, b): {},\n
Compression percent: {},\n
Compression percent (with alphabet): {},\n
Compression time (ms): {}\n
		",
            self.input_chars_count,
            self.original_size,
            self.alphabet_length,
            self.compressed_size,
            self.compressed_with_alphabet,
            self.compression_percent * 100f32,
            self.compression_percent_with_alphabet * 100f32,
            self.compression_time
        )
    }
}

pub fn analyze(message: &str, compressed: &str, alphabet: &Alphabet) -> AnalyzeResult {
    let compressed_with_alphabet = alphabet.stringify() + compressed;
    let original_size = text_to_bin(message).len();
    AnalyzeResult {
        input_chars_count: message.len(),
        original_size,
        alphabet_length: alphabet.len(),
        compressed_size: compressed.len(),
        compressed_with_alphabet: compressed_with_alphabet.len(),
        compression_percent: (original_size - compressed.len()) as f32 / original_size as f32,
        compression_percent_with_alphabet: (original_size - compressed_with_alphabet.len()) as f32
            / original_size as f32,
        compression_time: unsafe { ELAPSED },
    }
}

impl<'a> CompressionResult<'a> {
    pub fn get_with_injected_alphabet(&self) -> String {
        format!("{}{}", self.alphabet.encode_info(), self.encoded)
    }

    pub fn get_encoded(&self) -> &str {
        &self.encoded
    }

    pub fn get_alphabet(&self) -> &Alphabet {
        &self.alphabet
    }

    pub fn analyze(&self) -> AnalyzeResult {
        analyze(self.message, &self.encoded, &self.alphabet)
    }
}
