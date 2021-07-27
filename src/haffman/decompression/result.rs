use crate::haffman::alphabet::Alphabet;

use super::analyze::{analyze, AnalyzeResult};

pub struct DecompressionResult {
    message: String,
    decoded: String,
    alphabet: Alphabet,
    decompression_time: u128,
}

impl DecompressionResult {
    pub fn new(
        message: String,
        decoded: String,
        alphabet: Alphabet,
        decompression_time: u128,
    ) -> DecompressionResult {
        DecompressionResult {
            message,
            decoded,
            alphabet,
            decompression_time,
        }
    }

    #[allow(dead_code)] // TEMP
    pub fn get_decoded(&self) -> String {
        self.decoded.clone()
    }

    #[allow(dead_code)] // TEMP
    pub fn get_encoded_message(&self) -> String {
        self.message.clone()
    }

    #[allow(dead_code)] // TEMP
    pub fn get_alphabet(&self) -> &Alphabet {
        &self.alphabet
    }

    pub fn analyze(&self) -> AnalyzeResult {
        analyze(self.decompression_time)
    }
}
