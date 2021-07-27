pub mod alphabet;
mod analyze;
mod compression;
mod init;
mod query;
mod result;
use std::time::Instant;

use alphabet::Alphabet;
use compression::compress;

pub struct HaffmanCompression {
    alphabet: Option<Alphabet>,
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

        CompressionResult::new(message, encoded, alphabet_for_compression, compression_time)
    }
}

#[cfg(test)]
pub use init::*;

use self::result::CompressionResult;

#[cfg(test)]
mod tests {
    use std::{fs::read_to_string, path::Path};

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
        assert_ne!(result.get_message(), result.get_encoded());
        // Message binary is changed
        assert_ne!(text_to_bin(result.get_message()), result.get_encoded());
        // Message size is reduced
        assert!(text_to_bin(result.get_message()).len() > result.get_encoded().len());
        // Alphabet size is less than message length
        assert!(text_to_bin(result.get_message()).len() > result.get_alphabet().len());
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

    #[test]
    pub fn compression_is_correct_for_test_data() {
        let input = read_to_string(Path::new("test_data/text.txt"))
            .expect("Something went wrong reading the file");

        let result = HaffmanCompression::new().compress(&input);
        let encoded = result.get_with_injected_alphabet();

        let output = read_to_string(Path::new("test_data/output.txt"))
            .expect("Something went wrong reading the file");

        assert_eq!(encoded, output);
    }
}
