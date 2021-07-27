use std::time::Instant;

use self::decompression::decompress;

use super::alphabet::Alphabet;

mod analyze;
mod decompression;
mod init;
mod result;

pub use result::DecompressionResult;

pub struct HaffmanDecompression {
    alphabet: Option<Alphabet>,
}

impl HaffmanDecompression {
    pub fn new() -> HaffmanDecompression {
        HaffmanDecompression { alphabet: None }
    }

    #[allow(dead_code)] // TEMP
    pub fn with_alphabet(&mut self, alphabet: Alphabet) -> &HaffmanDecompression {
        self.alphabet = Some(alphabet);
        self
    }

    pub fn decompress(&self, encoded: &str) -> DecompressionResult {
        let timer: Instant = Instant::now();
        let alphabet_for_compression: Alphabet;
        let message: String;

        match &self.alphabet {
            Some(alphabet) => {
                alphabet_for_compression = alphabet.clone();
                message = encoded.to_string();
            }
            None => {
                let (alphabet, encoded_message) = Alphabet::decode_from_binary(encoded.to_string());
                message = encoded_message;
                alphabet_for_compression = alphabet;
            }
        };

        let decoded: String = decompress(&message, &alphabet_for_compression);

        let decompression_time = timer.elapsed().as_millis();

        DecompressionResult::new(
            message,
            decoded,
            alphabet_for_compression,
            decompression_time,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::read_to_string, path::Path};

    use crate::haffman::{HaffmanCompression, HaffmanDecompression};

    #[test]
    pub fn alphabet_is_decoded_correctly() {
        let message = "Haffman compression";
        let compression_result = HaffmanCompression::new().compress(message);
        let decompression_result = HaffmanDecompression::new()
            .decompress(&compression_result.get_with_injected_alphabet());

        println!(
            "{}\n{}",
            decompression_result.get_alphabet().to_string(),
            compression_result.get_alphabet().to_string()
        );

        assert_eq!(
            decompression_result.get_alphabet().to_string(),
            compression_result.get_alphabet().to_string()
        );
    }

    #[test]
    pub fn compressed_message_is_received_correctly() {
        let message = "Haffman compression";
        let compression_result = HaffmanCompression::new().compress(message);
        let decompression_result = HaffmanDecompression::new()
            .decompress(&compression_result.get_with_injected_alphabet());

        assert_eq!(
            decompression_result.get_encoded_message(),
            compression_result.get_encoded()
        );
    }

    #[test]
    pub fn decompression_is_correct_for_test_data() {
        let encoded = read_to_string(Path::new("test_data/output.txt"))
            .expect("Something went wrong reading the file");

        let result = HaffmanDecompression::new().decompress(&encoded);
        let decoded = result.get_decoded();

        let original = read_to_string(Path::new("test_data/text.txt"))
            .expect("Something went wrong reading the file");

        assert_eq!(decoded, original);
    }
}
