use super::{
    alphabet::Alphabet,
    analyze::{analyze, AnalyzeResult},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CompressionResult<'a> {
    message: &'a str,
    encoded: String,
    alphabet: Alphabet,
    compression_time: u128,
}

impl<'a> CompressionResult<'a> {
    pub fn new(
        message: &'a str,
        encoded: String,
        alphabet: Alphabet,
        compression_time: u128,
    ) -> CompressionResult {
        CompressionResult {
            message,
            encoded,
            alphabet,
            compression_time,
        }
    }

    pub fn get_with_injected_alphabet(&self) -> String {
        format!("{}{}", self.alphabet.encode_info(), self.encoded)
    }

    #[allow(dead_code)] // TEMP
    pub fn get_message(&self) -> &str {
        &self.message
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
