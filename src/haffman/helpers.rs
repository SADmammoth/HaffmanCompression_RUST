pub fn pad(string: &str, len: usize, symbol: char) -> String {
    if len <= string.len() {
        string.to_string()
    } else {
        let spaces = std::iter::repeat(symbol)
            .take(len - string.len())
            .collect::<String>();
        format!("{}{}", spaces, string)
    }
}

pub fn char_to_bin(character: &char) -> String {
    let mut char_number = [0u16; 1];
    character.encode_utf16(&mut char_number);

    let mut string = String::new();
    for symbol in char_number.iter() {
        string += &format!("{:b}", symbol);
    }

    string
}

pub fn bin_to_char(binary: &str) -> char {
    let char_code = u32::from_str_radix(binary, 2).unwrap();
    char::from_u32(char_code).unwrap()
}

pub fn text_to_bin(text: &str) -> String {
    text.bytes()
        .fold(String::new(), |string, x| string + &format!("{:b}", x))
}

pub fn num_to_bin(num: usize) -> String {
    format!("{:b}", num)
}

pub static MAX_ALPHABET_LENGTH_DIGITS: usize = 8;
pub static MAX_CHAR_LENGTH_DIGITS: usize = 8;
