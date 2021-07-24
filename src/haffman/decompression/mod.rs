use self::{decompression::decompress, reverse_alphabet::ReverseAlphabet};

use super::alphabet::Alphabet;

mod decompression;
mod reverse_alphabet;

pub struct HaffmanDecompression {
    // TODO Stor alphabet
}

pub struct DecompressionResult {
    message: String,
    alphabet: Alphabet,
    // TODO Impl
}

impl HaffmanDecompression {
    pub fn new() -> HaffmanDecompression {
        HaffmanDecompression {}
    }

    pub fn decompress(&self, encoded: &str) -> DecompressionResult {
        let (alphabet, message) = ReverseAlphabet::decode_from_binary(encoded.to_string());
        decompress(&message, &alphabet);

        DecompressionResult {
            message,
            alphabet: alphabet.convert_to_alphabet(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::haffman::{HaffmanCompression, HaffmanDecompression};

    #[test]
    pub fn alphabet_is_decoded_correctly() {
        let message = "Haffman compression";
        let compression_result = HaffmanCompression::new().compress(message);
        let decompression_result = HaffmanDecompression::new()
            .decompress(&compression_result.get_with_injected_alphabet());

        println!(
            "{}\n{}",
            decompression_result.alphabet.to_string(),
            compression_result.get_alphabet().to_string()
        );

        assert_eq!(
            decompression_result.alphabet.to_string(),
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
            decompression_result.message,
            compression_result.get_encoded()
        );
    }
}
