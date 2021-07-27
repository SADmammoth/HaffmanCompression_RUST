use crate::haffman::alphabet::Alphabet;
use crate::haffman::helpers::bin_to_char;
use crate::haffman::helpers::MAX_ALPHABET_LENGTH_DIGITS;
use crate::haffman::helpers::MAX_CHAR_LENGTH_DIGITS;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Eq)]
pub struct ReverseAlphabet(HashMap<String, char>);

impl Clone for ReverseAlphabet {
    fn clone(&self) -> Self {
        ReverseAlphabet(self.0.clone())
    }

    fn clone_from(&mut self, other: &Self) {
        self.0 = other.0.clone();
    }
}

impl ReverseAlphabet {
    pub fn new(map: HashMap<String, char>) -> ReverseAlphabet {
        if map.len() >= 2usize.pow(32u32) {
            panic!("Alphabet larger than 2^32 is not supported");
        } else {
            ReverseAlphabet(map)
        }
    }

    pub fn from_alphabet(alphabet: Alphabet) -> ReverseAlphabet {
        let map: HashMap<String, char> = alphabet
            .get_map()
            .iter()
            .map(|(key, value)| (value.clone(), key.clone()))
            .collect();

        ReverseAlphabet::new(map)
    }

    pub fn get_map(&self) -> &HashMap<String, char> {
        &self.0
    }

    #[allow(dead_code)] //TEMP
    pub fn len(&self) -> usize {
        self.get_map().len()
    }

    pub fn decode_from_binary(mut binary: String) -> (ReverseAlphabet, String) {
        let alphabet_length_raw: String = binary.drain(0..MAX_ALPHABET_LENGTH_DIGITS).collect();
        let max_char_length_raw: String = binary.drain(0..MAX_CHAR_LENGTH_DIGITS).collect();
        let max_code_length_raw: String = binary
            .drain(0..alphabet_length_raw.trim_start_matches("0").len())
            .collect();

        let alphabet_length: usize = usize::from_str_radix(&alphabet_length_raw, 2).unwrap();
        let max_char_length: usize = usize::from_str_radix(&max_char_length_raw, 2).unwrap();
        let max_code_length: usize = usize::from_str_radix(&max_code_length_raw, 2).unwrap();

        let mut map = HashMap::<String, char>::new();

        let mut character: char;
        let mut code: String;

        for _ in 0..alphabet_length {
            character = bin_to_char(&binary.drain(0..max_char_length).collect::<String>());

            code = binary.drain(0..max_code_length).collect();
            code = code.trim_start_matches("0").to_string();
            code = code.drain(1..).collect();

            map.insert(code, character);
        }

        (ReverseAlphabet(map), binary)
    }

    pub fn convert_to_alphabet(&self) -> Alphabet {
        let map: HashMap<char, String> = self
            .get_map()
            .iter()
            .map(|(key, value)| (value.clone(), key.clone()))
            .collect();
        Alphabet::new(map)
    }
}

impl Display for ReverseAlphabet {
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
