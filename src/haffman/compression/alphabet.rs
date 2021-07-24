use super::super::helpers::char_to_bin;
use super::super::helpers::num_to_bin;
use super::super::helpers::pad;
use crate::haffman::helpers::MAX_ALPHABET_LENGTH_DIGITS;
use crate::haffman::helpers::MAX_CHAR_LENGTH_DIGITS;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Eq)]
pub struct Alphabet(HashMap<char, String>);

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

    pub fn get_map(&self) -> &HashMap<char, String> {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.get_map().len()
    }

    fn stringify(&self) -> String {
        if self.get_map().is_empty() {
            return String::from("[empty alphabet]");
        }

        let max_char_length = self.get_max_char_length();
        let max_code_length = self.get_max_code_length();

        let mut sorted_alphabet = self.get_map().clone().into_iter().collect::<Vec<_>>();
        sorted_alphabet.sort_by(|(a_key, _), (b_key, _)| a_key.cmp(&b_key));

        sorted_alphabet
            .iter()
            .fold(String::new(), |string, (key, code)| {
                format!(
                    "{}{}{}",
                    string,
                    pad(&char_to_bin(key), max_char_length, '0'),
                    pad(&format!("1{}", code), max_code_length + 1, '0')
                )
            })
    }

    pub fn get_max_char_length(&self) -> usize {
        let (character, _) = self
            .get_map()
            .iter()
            .max_by(|(a_key, _), (b_key, _)| {
                char_to_bin(a_key).len().cmp(&char_to_bin(b_key).len())
            })
            .unwrap();

        char_to_bin(character).to_string().len()
    }

    pub fn get_max_code_length(&self) -> usize {
        let (_, code) = self
            .get_map()
            .iter()
            .max_by(|(_, a_code), (_, b_code)| a_code.len().cmp(&b_code.len()))
            .unwrap();

        code.to_string().len()
    }

    pub fn encode_info(&self) -> String {
        format!(
            "{}{}{}{}",
            pad(&num_to_bin(self.len()), MAX_ALPHABET_LENGTH_DIGITS, '0'),
            pad(
                &num_to_bin(self.get_max_char_length()),
                MAX_CHAR_LENGTH_DIGITS,
                '0'
            ),
            pad(
                &num_to_bin(self.get_max_code_length() + 1),
                num_to_bin(self.len()).len(),
                '0',
            ),
            self.stringify()
        )
    }
}

impl Display for Alphabet {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut sorted_alphabet = self.get_map().clone().into_iter().collect::<Vec<_>>();
        sorted_alphabet.sort_by(|(a_key, _), (b_key, _)| a_key.cmp(&b_key));

        write!(
            f,
            "{}",
            sorted_alphabet
                .iter()
                .map(|(key, value)| { format!("({}, {}); ", key, value) })
                .collect::<String>()
        )
    }
}
