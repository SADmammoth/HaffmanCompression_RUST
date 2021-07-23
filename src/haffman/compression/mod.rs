pub mod alphabet;
mod analyze;
mod compression;
mod init;
mod query;
use std::time::Instant;

use alphabet::Alphabet;
use analyze::{analyze, AnalyzeResult};
use compression::compress;

pub struct HaffmanCompression {
    alphabet: Option<Alphabet>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CompressionResult<'a> {
    message: &'a str,
    encoded: String,
    alphabet: Alphabet,
    compression_time: u128,
}

impl HaffmanCompression {
    pub fn new() -> HaffmanCompression {
        HaffmanCompression { alphabet: None }
    }

    #[allow(dead_code)] // TEMP
    pub fn with_alphabet(&mut self, alphabet: Alphabet) -> &HaffmanCompression {
        self.alphabet = Some(alphabet);
        self
    }

    #[allow(dead_code)] // TEMP
    pub fn generate_alphabet_with_message(&mut self, message: &str) -> &HaffmanCompression {
        self.alphabet = Some(init::init(message));
        self
    }

    pub fn compress<'a>(&self, message: &'a str) -> CompressionResult<'a> {
        let timer: Instant = Instant::now();
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

        let compression_time = timer.elapsed().as_millis();

        CompressionResult {
            message,
            encoded,
            alphabet: alphabet_for_compression,
            compression_time,
        }
    }
}

impl<'a> CompressionResult<'a> {
    pub fn get_with_injected_alphabet(&self) -> String {
        format!("{}{}", self.alphabet.encode_info(), self.encoded)
    }

    #[allow(dead_code)] // TEMP
    pub fn get_encoded(&self) -> &str {
        &self.encoded
    }

    #[allow(dead_code)] // TEMP
    pub fn get_alphabet(&self) -> &Alphabet {
        &self.alphabet
    }

    pub fn analyze(&self) -> AnalyzeResult {
        analyze(
            self.message,
            &self.encoded,
            &self.alphabet,
            self.compression_time,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::haffman::helpers::text_to_bin;

    fn compress<'a>() -> CompressionResult<'a> {
        // let encoded: &str = "0010001101001010101110000101000111111010000000110001011011000001111100";
        // let alphabet: Alphabet = Alphabet::new([('r', String::from("0011")), ('e', String::from("0110")), ('i', String::from("0101")), ('m', String::from("0001")), ('f', String::from("100")), ('H', String::from("00100")), ('c', String::from("0111")), (' ', String::from("00101")), ('a', String::from("101")), ('s', String::from("110")), ('p', String::from("0100")), ('o', String::from("111")), ('n', String::from("0000"))].iter().cloned().collect());

        let message = "Haffman compression";
        let haf_comp = HaffmanCompression::new().compress(message);
        haf_comp
    }

    #[test]
    pub fn message_is_compressed() {
        let result = compress();
        // Message is changed
        assert_ne!(result.message, result.encoded);
        // Message binary is changed
        assert_ne!(text_to_bin(result.message), result.get_encoded());
        // Message size is reduced
        assert!(text_to_bin(result.message).len() > result.get_encoded().len());
        // Alphabet size is less than message length
        assert!(text_to_bin(result.message).len() > result.get_alphabet().len());
        // Compressed message with alphabet meust be larger than compressed message
        assert!(result.get_with_injected_alphabet().len() > result.get_encoded().len());
    }

    #[test]
    pub fn compression_is_consistent() {
        let result = compress();

        let alphabet = result.get_alphabet().to_string();
        let compresed = result.get_encoded();
        let compressed_with_alphabet = result.get_with_injected_alphabet();

        for _ in 0..1000 {
            let result = compress();
            assert_eq!(
                alphabet,
                result.get_alphabet().to_string(),
                "Alphabet is not consistent"
            );
            assert_eq!(
                compresed,
                result.get_encoded(),
                "Compressed message is not consistent"
            );
            assert_eq!(
                compressed_with_alphabet,
                result.get_with_injected_alphabet(),
                "Compressed message with alphabet is not consistent"
            );
        }
    }
}
